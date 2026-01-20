use crate::cache::{self, CachedRegion, StateCache};
use crate::generators::addresses::Address;
use crate::regions;
use rand::seq::SliceRandom;
use std::fs::{self, File};
use std::io::{self, Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Default limit for addresses per state
#[allow(dead_code)]
pub const DEFAULT_LIMIT: usize = 10_000;

/// Downloads address data for specified states from OpenAddresses.io.
///
/// # Arguments
/// * `states` - Slice of state codes to download
/// * `limit` - Maximum number of addresses per state
/// * `force` - If true, re-download even if already cached
/// * `quiet` - If true, suppress progress output
///
/// # Returns
/// * `Ok(())` - If all downloads succeeded
/// * `Err(io::Error)` - If validation or download failed
pub fn download_states(
    states: &[String],
    limit: usize,
    force: bool,
    quiet: bool,
) -> io::Result<()> {
    // Validate all states first
    for state in states {
        if !regions::is_valid_state(state) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid state code: {}", state),
            ));
        }
    }

    // Load current manifest
    let mut manifest = cache::load_manifest()?;
    if manifest.version == 0 {
        manifest.version = 1;
    }

    // Filter out already-cached states (unless force=true)
    let mut states_to_download: Vec<String> = Vec::new();
    for state in states {
        let state_upper = state.to_uppercase();
        if force || !cache::is_state_cached(&state_upper)? {
            states_to_download.push(state_upper);
        } else if !quiet {
            println!(
                "State {} already cached (use --force to re-download)",
                state_upper
            );
        }
    }

    if states_to_download.is_empty() {
        if !quiet {
            println!("All requested states are already cached");
        }
        return Ok(());
    }

    // Group states by region to minimize downloads
    let mut regions_map: std::collections::HashMap<&'static str, Vec<String>> =
        std::collections::HashMap::new();

    for state in &states_to_download {
        if let Some(region_url) = regions::get_region_url(state) {
            regions_map
                .entry(region_url)
                .or_default()
                .push(state.clone());
        }
    }

    // Download each region and extract state data
    for (region_url, region_states) in regions_map {
        // Check if we have the regional data cached (ZIP or directory)
        let cached_region = cache::get_cached_region(region_url)?;

        // Extract and cache each state from this region
        for state in region_states.clone() {
            if !quiet {
                println!("Extracting addresses for {}...", state);
            }

            let addresses = match &cached_region {
                Some(CachedRegion::Zip(zip_path)) => {
                    if !quiet {
                        println!("Using cached ZIP: {}", zip_path.display());
                    }
                    let zip_data = fs::read(zip_path)?;
                    extract_state_from_zip(&zip_data, &state, limit)?
                }
                Some(CachedRegion::Directory(dir_path)) => {
                    if !quiet {
                        println!("Using cached directory: {}", dir_path.display());
                    }
                    extract_state_from_directory(dir_path, &state, limit)?
                }
                None => {
                    if !quiet {
                        println!("Downloading region: {}", region_url);
                        println!("(This file is large and may take several minutes)");
                    }

                    // Download the regional zip file
                    let zip_data = download_region(region_url, quiet)?;

                    // Cache the ZIP for future use
                    let zip_path = cache::save_region_zip(region_url, &zip_data)?;
                    if !quiet {
                        println!("Cached regional ZIP to: {}", zip_path.display());
                    }

                    extract_state_from_zip(&zip_data, &state, limit)?
                }
            };

            if !quiet {
                println!("Found {} addresses for {}", addresses.len(), state);
            }

            // Write to cache
            let cache_path = cache::get_state_cache_path(&state)?;
            write_addresses_to_cache(&cache_path, &addresses)?;

            // Update manifest
            manifest.states.insert(
                state.clone(),
                StateCache {
                    downloaded_at: chrono_now(),
                    source_url: region_url.to_string(),
                    record_count: addresses.len(),
                },
            );

            // Save manifest after each state
            cache::save_manifest(&manifest)?;

            if !quiet {
                println!("Cached {} addresses for {}", addresses.len(), state);
            }
        }
    }

    if !quiet {
        println!(
            "Successfully downloaded {} state(s)",
            states_to_download.len()
        );
    }

    Ok(())
}

/// Prints a formatted list of cached states with their metadata.
pub fn print_cache_list() -> io::Result<()> {
    let cached_states = cache::list_cached_states()?;
    let cache_dir = cache::get_cache_dir()?;

    if cached_states.is_empty() {
        println!("No cached states found.");
        println!("\nCache location: {}", cache_dir.display());
        println!("\nTo manually add regional data, download from:");
        println!("  https://data.openaddresses.io/openaddr-collected-us_south.zip");
        println!("  https://data.openaddresses.io/openaddr-collected-us_northeast.zip");
        println!("  https://data.openaddresses.io/openaddr-collected-us_midwest.zip");
        println!("  https://data.openaddresses.io/openaddr-collected-us_west.zip");
        println!("\nThen place in the cache directory as ZIP or extracted folder:");
        println!("  {}/us_south.zip  OR  {}/us_south/", cache_dir.display(), cache_dir.display());
        println!("  (If your browser auto-extracts, the folder works too!)");
        return Ok(());
    }

    println!("\nCached States:");
    println!("{:-<80}", "");
    println!("{:<10} {:<15} {:<30}", "State", "Records", "Downloaded");
    println!("{:-<80}", "");

    let mut total_records = 0;
    for (state, cache_info) in &cached_states {
        println!(
            "{:<10} {:<15} {:<30}",
            state, cache_info.record_count, cache_info.downloaded_at
        );
        total_records += cache_info.record_count;
    }

    println!("{:-<80}", "");
    println!(
        "Total: {} states, {} records",
        cached_states.len(),
        total_records
    );

    println!("\nCache location: {}", cache_dir.display());

    // Check for cached regional data (ZIPs or directories)
    let regions = ["us_south", "us_northeast", "us_midwest", "us_west"];
    let mut cached_regions = Vec::new();
    for region in &regions {
        let zip_path = cache_dir.join(format!("{}.zip", region));
        let dir_path = cache_dir.join(region);
        if zip_path.exists() {
            cached_regions.push(format!("{}.zip", region));
        } else if dir_path.exists() && dir_path.is_dir() {
            cached_regions.push(format!("{}/ (dir)", region));
        }
    }
    if !cached_regions.is_empty() {
        println!("Cached regional data: {}", cached_regions.join(", "));
    }

    Ok(())
}

/// Downloads a regional zip file from OpenAddresses.io with retry logic.
///
/// # Arguments
/// * `url` - The URL of the regional zip file
/// * `quiet` - If true, suppress progress output
///
/// # Returns
/// * `Ok(Vec<u8>)` - The downloaded zip file data
/// * `Err(io::Error)` - If the download failed after all retries
fn download_region(url: &str, quiet: bool) -> io::Result<Vec<u8>> {
    // Create client with extended timeout for large files (regional zips can be 100MB+)
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(600)) // 10 minute timeout
        .connect_timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| io::Error::other(format!("Failed to create HTTP client: {}", e)))?;

    let max_retries = 3;
    let mut last_error = String::new();

    for attempt in 0..=max_retries {
        if attempt > 0 {
            // Exponential backoff: 30s, 60s, 120s
            let wait_secs = 30 * (1 << (attempt - 1));
            if !quiet {
                println!(
                    "Rate limited. Waiting {} seconds before retry {}/{}...",
                    wait_secs, attempt, max_retries
                );
            }
            std::thread::sleep(Duration::from_secs(wait_secs));
        }

        let response = match client.get(url).send() {
            Ok(r) => r,
            Err(e) => {
                last_error = format!("HTTP request failed: {}", e);
                if attempt < max_retries {
                    continue;
                }
                return Err(io::Error::other(last_error));
            }
        };

        let status = response.status();
        if status.is_success() {
            let bytes = response
                .bytes()
                .map_err(|e| io::Error::other(format!("Failed to read response: {}", e)))?;
            return Ok(bytes.to_vec());
        }

        if status.as_u16() == 429 {
            last_error = format!("HTTP error: {} (rate limited)", status);
            // Will retry on next iteration
            continue;
        }

        // Non-retryable error
        return Err(io::Error::other(format!("HTTP error: {}", status)));
    }

    Err(io::Error::other(format!(
        "{} - exhausted all {} retries",
        last_error, max_retries
    )))
}

/// Extracts addresses for a specific state from a regional ZIP file.
fn extract_state_from_zip(zip_data: &[u8], state: &str, limit: usize) -> io::Result<Vec<Address>> {
    let cursor = Cursor::new(zip_data);
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|e| io::Error::other(format!("Invalid zip file: {}", e)))?;

    let state_lower = state.to_lowercase();
    let prefix = format!("us/{}/", state_lower);

    let mut all_addresses: Vec<Address> = Vec::new();

    // Iterate through all files in the zip
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| io::Error::other(format!("Failed to read zip entry: {}", e)))?;

        let file_name = file.name().to_string();

        // Check if this file is for our state
        if file_name.starts_with(&prefix) && file_name.ends_with(".csv") {
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(|e| {
                io::Error::other(format!("Failed to read file {}: {}", file_name, e))
            })?;

            let addresses = parse_openaddresses_csv(&contents)?;
            all_addresses.extend(addresses);
        }
    }

    shuffle_and_limit(&mut all_addresses, limit);
    Ok(all_addresses)
}

/// Extracts addresses for a specific state from an extracted regional directory.
fn extract_state_from_directory(
    dir_path: &Path,
    state: &str,
    limit: usize,
) -> io::Result<Vec<Address>> {
    let state_lower = state.to_lowercase();

    // Look for the state directory: dir_path/us/ky/ or dir_path/openaddr-collected-us_south/us/ky/
    let possible_paths = [
        dir_path.join("us").join(&state_lower),
        dir_path
            .join(dir_path.file_name().unwrap_or_default())
            .join("us")
            .join(&state_lower),
    ];

    let mut state_dir: Option<PathBuf> = None;
    for path in &possible_paths {
        if path.exists() && path.is_dir() {
            state_dir = Some(path.clone());
            break;
        }
    }

    // Also try to find it recursively
    if state_dir.is_none() {
        state_dir = find_state_directory(dir_path, &state_lower)?;
    }

    let state_dir = state_dir.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "State directory '{}' not found in {}",
                state_lower,
                dir_path.display()
            ),
        )
    })?;

    let mut all_addresses: Vec<Address> = Vec::new();

    // Read all CSV files in the state directory
    for entry in fs::read_dir(&state_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |e| e == "csv") {
            let contents = fs::read_to_string(&path)?;
            let addresses = parse_openaddresses_csv(&contents)?;
            all_addresses.extend(addresses);
        }
    }

    shuffle_and_limit(&mut all_addresses, limit);
    Ok(all_addresses)
}

/// Recursively finds a state directory within a path.
fn find_state_directory(base: &Path, state: &str) -> io::Result<Option<PathBuf>> {
    // Look for us/<state> pattern
    let us_dir = base.join("us");
    if us_dir.exists() {
        let state_dir = us_dir.join(state);
        if state_dir.exists() && state_dir.is_dir() {
            return Ok(Some(state_dir));
        }
    }

    // Check subdirectories (for nested structures like openaddr-collected-us_south/us/ky)
    if base.is_dir() {
        for entry in fs::read_dir(base)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_state_directory(&path, state)? {
                    return Ok(Some(found));
                }
            }
        }
    }

    Ok(None)
}

/// Shuffles addresses and truncates to limit.
fn shuffle_and_limit(addresses: &mut Vec<Address>, limit: usize) {
    let mut rng = rand::thread_rng();
    addresses.shuffle(&mut rng);

    if addresses.len() > limit {
        addresses.truncate(limit);
    }
}

/// Parses a CSV file in OpenAddresses format.
///
/// # Arguments
/// * `content` - The CSV file content as a string
///
/// # Returns
/// * `Ok(Vec<Address>)` - The parsed addresses
/// * `Err(io::Error)` - If parsing failed
fn parse_openaddresses_csv(content: &str) -> io::Result<Vec<Address>> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(content.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to read CSV headers: {}", e),
            )
        })?
        .clone();

    // Map column names to indices
    let mut column_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (idx, header) in headers.iter().enumerate() {
        let header_lower = header.to_lowercase();
        column_map.insert(header_lower, idx);
    }

    // Find relevant columns (OpenAddresses format)
    let number_idx = column_map
        .get("number")
        .or_else(|| column_map.get("house_number"));
    let street_idx = column_map
        .get("street")
        .or_else(|| column_map.get("street_name"));
    let unit_idx = column_map
        .get("unit")
        .or_else(|| column_map.get("apartment"));
    let city_idx = column_map
        .get("city")
        .or_else(|| column_map.get("locality"));
    let state_idx = column_map.get("region").or_else(|| column_map.get("state"));
    let zip_idx = column_map
        .get("postcode")
        .or_else(|| column_map.get("zip"))
        .or_else(|| column_map.get("postal_code"));

    let mut addresses: Vec<Address> = Vec::new();

    for result in reader.records() {
        let record = result.map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to read CSV record: {}", e),
            )
        })?;

        // Extract fields
        let number = number_idx
            .and_then(|&idx| record.get(idx))
            .unwrap_or("")
            .trim();
        let street = street_idx
            .and_then(|&idx| record.get(idx))
            .unwrap_or("")
            .trim();
        let unit = unit_idx
            .and_then(|&idx| record.get(idx))
            .unwrap_or("")
            .trim();
        let city = city_idx
            .and_then(|&idx| record.get(idx))
            .unwrap_or("")
            .trim();
        let state = state_idx
            .and_then(|&idx| record.get(idx))
            .unwrap_or("")
            .trim();
        let zip = zip_idx
            .and_then(|&idx| record.get(idx))
            .unwrap_or("")
            .trim();

        // Skip records missing essential fields
        if street.is_empty() || city.is_empty() {
            continue;
        }

        // Combine number and street for address1
        let address1 = if !number.is_empty() && !street.is_empty() {
            format!("{} {}", number, street)
        } else {
            street.to_string()
        };

        addresses.push(Address::new(
            address1,
            unit.to_string(),
            city.to_string(),
            state.to_string(),
            zip.to_string(),
        ));
    }

    Ok(addresses)
}

/// Writes addresses to a cache CSV file.
///
/// # Arguments
/// * `path` - The path to the cache file
/// * `addresses` - The addresses to write
///
/// # Returns
/// * `Ok(())` - If writing succeeded
/// * `Err(io::Error)` - If writing failed
fn write_addresses_to_cache(path: &PathBuf, addresses: &[Address]) -> io::Result<()> {
    let mut file = File::create(path)?;

    // Write header
    writeln!(file, "address1,address2,city,state,zip")?;

    // Write records
    for address in addresses {
        writeln!(
            file,
            "{},{},{},{},{}",
            escape_csv(&address.address1),
            escape_csv(&address.address2),
            escape_csv(&address.city),
            escape_csv(&address.state),
            escape_csv(&address.zip)
        )?;
    }

    Ok(())
}

/// Escapes a CSV field by quoting it if necessary.
///
/// # Arguments
/// * `field` - The field to escape
///
/// # Returns
/// * The escaped field
fn escape_csv(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

/// Returns the current timestamp as a human-readable string.
fn chrono_now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_csv_simple() {
        assert_eq!(escape_csv("simple"), "simple");
    }

    #[test]
    fn test_escape_csv_with_comma() {
        assert_eq!(escape_csv("hello, world"), "\"hello, world\"");
    }

    #[test]
    fn test_escape_csv_with_quote() {
        assert_eq!(escape_csv("say \"hello\""), "\"say \"\"hello\"\"\"");
    }

    #[test]
    fn test_escape_csv_with_newline() {
        assert_eq!(escape_csv("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn test_chrono_now_format() {
        let timestamp = chrono_now();
        // Should be in format "YYYY-MM-DD HH:MM:SS"
        assert_eq!(timestamp.len(), 19);
        assert!(timestamp.contains('-'));
        assert!(timestamp.contains(':'));
    }

    #[test]
    fn test_parse_openaddresses_csv_basic() {
        let csv_content = "NUMBER,STREET,CITY,REGION,POSTCODE\n123,Main St,Springfield,IL,62701\n456,Oak Ave,Chicago,IL,60601";
        let addresses = parse_openaddresses_csv(csv_content).unwrap();

        assert_eq!(addresses.len(), 2);
        assert_eq!(addresses[0].address1, "123 Main St");
        assert_eq!(addresses[0].city, "Springfield");
        assert_eq!(addresses[0].state, "IL");
        assert_eq!(addresses[0].zip, "62701");
    }

    #[test]
    fn test_parse_openaddresses_csv_missing_street() {
        let csv_content = "NUMBER,STREET,CITY,REGION,POSTCODE\n123,,Springfield,IL,62701\n456,Oak Ave,Chicago,IL,60601";
        let addresses = parse_openaddresses_csv(csv_content).unwrap();

        // Should skip record with missing street
        assert_eq!(addresses.len(), 1);
        assert_eq!(addresses[0].address1, "456 Oak Ave");
    }

    #[test]
    fn test_parse_openaddresses_csv_missing_city() {
        let csv_content = "NUMBER,STREET,CITY,REGION,POSTCODE\n123,Main St,,IL,62701\n456,Oak Ave,Chicago,IL,60601";
        let addresses = parse_openaddresses_csv(csv_content).unwrap();

        // Should skip record with missing city
        assert_eq!(addresses.len(), 1);
        assert_eq!(addresses[0].city, "Chicago");
    }

    #[test]
    fn test_parse_openaddresses_csv_case_insensitive() {
        let csv_content = "number,street,city,region,postcode\n123,Main St,Springfield,IL,62701";
        let addresses = parse_openaddresses_csv(csv_content).unwrap();

        assert_eq!(addresses.len(), 1);
        assert_eq!(addresses[0].address1, "123 Main St");
    }

    #[test]
    fn test_default_limit() {
        assert_eq!(DEFAULT_LIMIT, 10_000);
    }
}
