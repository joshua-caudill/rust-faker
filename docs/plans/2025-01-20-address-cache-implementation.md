# Address Cache & Download Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add functionality to download real addresses from OpenAddresses.io, cache them locally by state, and use them for generating test data.

**Architecture:** New `download` subcommand fetches regional zip files from OpenAddresses.io, extracts state CSVs, samples addresses, and caches to `~/.rust-faker/cache/addresses/`. The `addresses` command gains a `--state` flag to load from cache instead of generating fake data.

**Tech Stack:** reqwest (HTTP), zip (archive extraction), serde/serde_json (manifest), dirs (home directory)

---

## Task 1: Add Dependencies

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add new dependencies**

Add to `[dependencies]` section in `Cargo.toml`:

```toml
reqwest = { version = "0.11", features = ["blocking"] }
zip = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"
```

**Step 2: Verify dependencies resolve**

Run: `cargo check`
Expected: Compiles successfully (may take time to download deps)

**Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: add dependencies for address cache feature"
```

---

## Task 2: Create State-to-Region Mapping

**Files:**
- Create: `src/regions.rs`
- Modify: `src/main.rs` (add module declaration)

**Step 1: Create regions module with state mapping**

Create `src/regions.rs`:

```rust
use std::collections::HashMap;

/// OpenAddresses.io region URLs
pub const REGION_NORTHEAST: &str = "https://data.openaddresses.io/openaddr-collected-us_northeast.zip";
pub const REGION_MIDWEST: &str = "https://data.openaddresses.io/openaddr-collected-us_midwest.zip";
pub const REGION_SOUTH: &str = "https://data.openaddresses.io/openaddr-collected-us_south.zip";
pub const REGION_WEST: &str = "https://data.openaddresses.io/openaddr-collected-us_west.zip";

/// All valid US state codes
pub const ALL_STATES: &[&str] = &[
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA",
    "HI", "ID", "IL", "IN", "IA", "KS", "KY", "LA", "ME", "MD",
    "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ",
    "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC",
    "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY",
    "DC",
];

/// Returns the OpenAddresses region URL for a given state code
pub fn get_region_url(state: &str) -> Option<&'static str> {
    let state_upper = state.to_uppercase();

    match state_upper.as_str() {
        // Northeast
        "CT" | "ME" | "MA" | "NH" | "NJ" | "NY" | "PA" | "RI" | "VT" => {
            Some(REGION_NORTHEAST)
        }
        // Midwest
        "IL" | "IN" | "IA" | "KS" | "MI" | "MN" | "MO" | "NE" | "ND" | "OH" | "SD" | "WI" => {
            Some(REGION_MIDWEST)
        }
        // South
        "AL" | "AR" | "DE" | "DC" | "FL" | "GA" | "KY" | "LA" | "MD" | "MS" |
        "NC" | "OK" | "SC" | "TN" | "TX" | "VA" | "WV" => {
            Some(REGION_SOUTH)
        }
        // West
        "AK" | "AZ" | "CA" | "CO" | "HI" | "ID" | "MT" | "NV" | "NM" | "OR" |
        "UT" | "WA" | "WY" => {
            Some(REGION_WEST)
        }
        _ => None,
    }
}

/// Validates a state code
pub fn is_valid_state(state: &str) -> bool {
    ALL_STATES.contains(&state.to_uppercase().as_str())
}

/// Returns the state name in the format used by OpenAddresses file paths
pub fn get_state_path_name(state: &str) -> Option<String> {
    if !is_valid_state(state) {
        return None;
    }
    // OpenAddresses uses lowercase state codes in paths
    Some(state.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_region_url_northeast() {
        assert_eq!(get_region_url("NY"), Some(REGION_NORTHEAST));
        assert_eq!(get_region_url("ny"), Some(REGION_NORTHEAST));
        assert_eq!(get_region_url("MA"), Some(REGION_NORTHEAST));
    }

    #[test]
    fn test_get_region_url_midwest() {
        assert_eq!(get_region_url("IL"), Some(REGION_MIDWEST));
        assert_eq!(get_region_url("OH"), Some(REGION_MIDWEST));
    }

    #[test]
    fn test_get_region_url_south() {
        assert_eq!(get_region_url("TX"), Some(REGION_SOUTH));
        assert_eq!(get_region_url("FL"), Some(REGION_SOUTH));
        assert_eq!(get_region_url("DC"), Some(REGION_SOUTH));
    }

    #[test]
    fn test_get_region_url_west() {
        assert_eq!(get_region_url("CA"), Some(REGION_WEST));
        assert_eq!(get_region_url("WA"), Some(REGION_WEST));
    }

    #[test]
    fn test_get_region_url_invalid() {
        assert_eq!(get_region_url("XX"), None);
        assert_eq!(get_region_url(""), None);
    }

    #[test]
    fn test_is_valid_state() {
        assert!(is_valid_state("IL"));
        assert!(is_valid_state("il"));
        assert!(is_valid_state("CA"));
        assert!(!is_valid_state("XX"));
        assert!(!is_valid_state(""));
    }

    #[test]
    fn test_all_states_count() {
        assert_eq!(ALL_STATES.len(), 51); // 50 states + DC
    }
}
```

**Step 2: Add module declaration to main.rs**

In `src/main.rs`, after the existing `mod` declarations, add:

```rust
mod regions;
```

**Step 3: Run tests**

Run: `cargo test regions`
Expected: All tests pass

**Step 4: Commit**

```bash
git add src/regions.rs src/main.rs
git commit -m "feat: add state-to-region mapping for OpenAddresses"
```

---

## Task 3: Create Cache Module

**Files:**
- Create: `src/cache.rs`
- Modify: `src/main.rs` (add module declaration)

**Step 1: Create cache module with path management and manifest types**

Create `src/cache.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Manifest tracking cached state data
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CacheManifest {
    pub version: u32,
    pub states: HashMap<String, StateCache>,
}

/// Metadata for a cached state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateCache {
    pub downloaded_at: String,
    pub source_url: String,
    pub record_count: usize,
}

impl CacheManifest {
    pub fn new() -> Self {
        CacheManifest {
            version: 1,
            states: HashMap::new(),
        }
    }
}

/// Returns the cache directory path (~/.rust-faker/cache/addresses/)
pub fn get_cache_dir() -> io::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;

    let cache_dir = home.join(".rust-faker").join("cache").join("addresses");
    Ok(cache_dir)
}

/// Ensures the cache directory exists
pub fn ensure_cache_dir() -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

/// Returns the path to the manifest file
pub fn get_manifest_path() -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    Ok(cache_dir.join("manifest.json"))
}

/// Returns the path to a state's cache file
pub fn get_state_cache_path(state: &str) -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    Ok(cache_dir.join(format!("{}.csv", state.to_uppercase())))
}

/// Loads the cache manifest, or returns empty manifest if not found
pub fn load_manifest() -> io::Result<CacheManifest> {
    let manifest_path = get_manifest_path()?;

    if !manifest_path.exists() {
        return Ok(CacheManifest::new());
    }

    let content = fs::read_to_string(&manifest_path)?;
    let manifest: CacheManifest = serde_json::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(manifest)
}

/// Saves the cache manifest
pub fn save_manifest(manifest: &CacheManifest) -> io::Result<()> {
    ensure_cache_dir()?;
    let manifest_path = get_manifest_path()?;
    let content = serde_json::to_string_pretty(manifest)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(&manifest_path, content)?;
    Ok(())
}

/// Checks if a state is cached
pub fn is_state_cached(state: &str) -> io::Result<bool> {
    let manifest = load_manifest()?;
    let state_upper = state.to_uppercase();

    if !manifest.states.contains_key(&state_upper) {
        return Ok(false);
    }

    // Also verify the file exists
    let cache_path = get_state_cache_path(state)?;
    Ok(cache_path.exists())
}

/// Returns list of cached states
pub fn list_cached_states() -> io::Result<Vec<(String, StateCache)>> {
    let manifest = load_manifest()?;
    let mut states: Vec<_> = manifest.states.into_iter().collect();
    states.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(states)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_manifest_new() {
        let manifest = CacheManifest::new();
        assert_eq!(manifest.version, 1);
        assert!(manifest.states.is_empty());
    }

    #[test]
    fn test_get_cache_dir() {
        let cache_dir = get_cache_dir().unwrap();
        assert!(cache_dir.ends_with(".rust-faker/cache/addresses"));
    }

    #[test]
    fn test_get_state_cache_path() {
        let path = get_state_cache_path("il").unwrap();
        assert!(path.to_string_lossy().ends_with("IL.csv"));

        let path = get_state_cache_path("CA").unwrap();
        assert!(path.to_string_lossy().ends_with("CA.csv"));
    }

    #[test]
    fn test_manifest_serialization() {
        let mut manifest = CacheManifest::new();
        manifest.states.insert(
            "IL".to_string(),
            StateCache {
                downloaded_at: "2025-01-20T10:00:00Z".to_string(),
                source_url: "https://example.com".to_string(),
                record_count: 10000,
            },
        );

        let json = serde_json::to_string(&manifest).unwrap();
        let parsed: CacheManifest = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.version, 1);
        assert!(parsed.states.contains_key("IL"));
        assert_eq!(parsed.states["IL"].record_count, 10000);
    }
}
```

**Step 2: Add module declaration to main.rs**

In `src/main.rs`, add after other `mod` declarations:

```rust
mod cache;
```

**Step 3: Run tests**

Run: `cargo test cache`
Expected: All tests pass

**Step 4: Commit**

```bash
git add src/cache.rs src/main.rs
git commit -m "feat: add cache module for manifest and path management"
```

---

## Task 4: Create Download Module (Core Logic)

**Files:**
- Create: `src/download.rs`
- Modify: `src/main.rs` (add module declaration)

**Step 1: Create download module with fetch and extraction logic**

Create `src/download.rs`:

```rust
use crate::cache::{self, CacheManifest, StateCache};
use crate::generators::addresses::Address;
use crate::regions;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Cursor, Write};
use std::path::PathBuf;

/// Default number of addresses to cache per state
pub const DEFAULT_LIMIT: usize = 10_000;

/// Downloads and caches addresses for the specified states
pub fn download_states(
    states: &[String],
    limit: usize,
    force: bool,
    quiet: bool,
) -> io::Result<()> {
    let mut manifest = cache::load_manifest()?;

    // Group states by region to minimize downloads
    let mut regions_needed: HashSet<&'static str> = HashSet::new();

    for state in states {
        let state_upper = state.to_uppercase();

        if !regions::is_valid_state(&state_upper) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid state code: {}", state),
            ));
        }

        if !force && cache::is_state_cached(&state_upper)? {
            if !quiet {
                println!("State {} already cached, skipping (use --force to re-download)", state_upper);
            }
            continue;
        }

        if let Some(region_url) = regions::get_region_url(&state_upper) {
            regions_needed.insert(region_url);
        }
    }

    // Download each needed region
    for region_url in regions_needed {
        if !quiet {
            println!("Downloading from {}...", region_url);
        }

        let zip_data = download_region(region_url)?;

        // Extract addresses for each state in this region
        for state in states {
            let state_upper = state.to_uppercase();

            if regions::get_region_url(&state_upper) != Some(region_url) {
                continue;
            }

            if !force && cache::is_state_cached(&state_upper)? {
                continue;
            }

            if !quiet {
                println!("Extracting addresses for {}...", state_upper);
            }

            let addresses = extract_state_addresses(&zip_data, &state_upper, limit)?;

            if addresses.is_empty() {
                eprintln!("Warning: No addresses found for state {}", state_upper);
                continue;
            }

            // Write to cache
            let cache_path = cache::get_state_cache_path(&state_upper)?;
            cache::ensure_cache_dir()?;
            write_addresses_to_cache(&cache_path, &addresses)?;

            // Update manifest
            manifest.states.insert(
                state_upper.clone(),
                StateCache {
                    downloaded_at: chrono_now(),
                    source_url: region_url.to_string(),
                    record_count: addresses.len(),
                },
            );

            if !quiet {
                println!("Cached {} addresses for {}", addresses.len(), state_upper);
            }
        }
    }

    cache::save_manifest(&manifest)?;
    Ok(())
}

/// Downloads a region zip file
fn download_region(url: &str) -> io::Result<Vec<u8>> {
    let response = reqwest::blocking::get(url)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Download failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Download failed with status: {}", response.status()),
        ));
    }

    let bytes = response.bytes()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to read response: {}", e)))?;

    Ok(bytes.to_vec())
}

/// Extracts addresses for a specific state from a region zip
fn extract_state_addresses(
    zip_data: &[u8],
    state: &str,
    limit: usize,
) -> io::Result<Vec<Address>> {
    let cursor = Cursor::new(zip_data);
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid zip: {}", e)))?;

    let state_lower = state.to_lowercase();
    let mut all_addresses: Vec<Address> = Vec::new();

    // Look for CSV files matching this state
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let name = file.name().to_string();

        // OpenAddresses structure: us/{state}/*.csv
        if !name.contains(&format!("us/{}/", state_lower)) || !name.ends_with(".csv") {
            continue;
        }

        // Read and parse CSV
        let mut content = String::new();
        io::Read::read_to_string(&mut file, &mut content)?;

        let addresses = parse_openaddresses_csv(&content)?;
        all_addresses.extend(addresses);

        // Stop if we have enough
        if all_addresses.len() >= limit * 2 {
            break;
        }
    }

    // Shuffle and limit
    let mut rng = rand::thread_rng();
    all_addresses.shuffle(&mut rng);
    all_addresses.truncate(limit);

    Ok(all_addresses)
}

/// Parses OpenAddresses CSV format into Address structs
fn parse_openaddresses_csv(content: &str) -> io::Result<Vec<Address>> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(content.as_bytes());

    let headers = reader.headers()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
        .clone();

    // Find column indices (OpenAddresses uses various column names)
    let find_col = |names: &[&str]| -> Option<usize> {
        for name in names {
            for (i, h) in headers.iter().enumerate() {
                if h.to_uppercase() == name.to_uppercase() {
                    return Some(i);
                }
            }
        }
        None
    };

    let number_col = find_col(&["NUMBER", "HOUSE_NUMBER"]);
    let street_col = find_col(&["STREET", "STREET_NAME"]);
    let unit_col = find_col(&["UNIT", "APARTMENT"]);
    let city_col = find_col(&["CITY", "LOCALITY"]);
    let state_col = find_col(&["REGION", "STATE"]);
    let zip_col = find_col(&["POSTCODE", "ZIP", "POSTAL_CODE"]);

    let mut addresses = Vec::new();

    for result in reader.records() {
        let record = match result {
            Ok(r) => r,
            Err(_) => continue,
        };

        let get_field = |col: Option<usize>| -> String {
            col.and_then(|i| record.get(i))
                .unwrap_or("")
                .trim()
                .to_string()
        };

        let number = get_field(number_col);
        let street = get_field(street_col);
        let unit = get_field(unit_col);
        let city = get_field(city_col);
        let state = get_field(state_col);
        let zip = get_field(zip_col);

        // Build address1 from number + street
        let address1 = if !number.is_empty() && !street.is_empty() {
            format!("{} {}", number, street)
        } else if !street.is_empty() {
            street
        } else {
            continue; // Skip records without street
        };

        // Skip if missing critical fields
        if city.is_empty() || state.is_empty() {
            continue;
        }

        addresses.push(Address::new(address1, unit, city, state, zip));
    }

    Ok(addresses)
}

/// Writes addresses to a cache file
fn write_addresses_to_cache(path: &PathBuf, addresses: &[Address]) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "address1,address2,city,state,zip")?;

    for addr in addresses {
        writeln!(
            file,
            "{},{},{},{},{}",
            escape_csv(&addr.address1),
            escape_csv(&addr.address2),
            escape_csv(&addr.city),
            escape_csv(&addr.state),
            escape_csv(&addr.zip),
        )?;
    }

    Ok(())
}

/// Escapes a field for CSV output
fn escape_csv(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

/// Returns current timestamp in ISO format
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}Z", duration.as_secs())
}

/// Prints the cache status
pub fn print_cache_list() -> io::Result<()> {
    let states = cache::list_cached_states()?;

    if states.is_empty() {
        println!("No addresses cached yet.");
        println!("Run 'rust-faker download <STATE>' to download addresses.");
        return Ok(());
    }

    println!("Cached addresses:");
    let mut total = 0;

    for (state, info) in &states {
        println!(
            "  {}    {:>6} addresses  (downloaded {})",
            state, info.record_count, info.downloaded_at
        );
        total += info.record_count;
    }

    println!();
    println!("Total: {} addresses across {} states", total, states.len());

    if let Ok(cache_dir) = cache::get_cache_dir() {
        println!("Cache location: {}", cache_dir.display());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_csv_simple() {
        assert_eq!(escape_csv("hello"), "hello");
    }

    #[test]
    fn test_escape_csv_with_comma() {
        assert_eq!(escape_csv("hello, world"), "\"hello, world\"");
    }

    #[test]
    fn test_escape_csv_with_quotes() {
        assert_eq!(escape_csv("say \"hello\""), "\"say \"\"hello\"\"\"");
    }

    #[test]
    fn test_parse_openaddresses_csv() {
        let csv = "NUMBER,STREET,CITY,REGION,POSTCODE\n123,Main St,Springfield,IL,62701\n456,Oak Ave,Chicago,IL,60601";
        let addresses = parse_openaddresses_csv(csv).unwrap();

        assert_eq!(addresses.len(), 2);
        assert_eq!(addresses[0].address1, "123 Main St");
        assert_eq!(addresses[0].city, "Springfield");
        assert_eq!(addresses[0].state, "IL");
    }
}
```

**Step 2: Add module declaration to main.rs**

In `src/main.rs`, add after other `mod` declarations:

```rust
mod download;
```

**Step 3: Run tests**

Run: `cargo test download`
Expected: All tests pass

**Step 4: Commit**

```bash
git add src/download.rs src/main.rs
git commit -m "feat: add download module for fetching OpenAddresses data"
```

---

## Task 5: Add Download Subcommand to CLI

**Files:**
- Modify: `src/main.rs`

**Step 1: Add Download command to enum**

In `src/main.rs`, add to the `Commands` enum:

```rust
    /// Download address data from OpenAddresses.io
    Download {
        /// State codes to download (e.g., IL CA TX)
        #[arg(value_name = "STATES")]
        states: Vec<String>,

        /// Download all 50 states + DC
        #[arg(long)]
        all: bool,

        /// List cached states
        #[arg(long)]
        list: bool,

        /// Maximum addresses per state
        #[arg(long, default_value = "10000")]
        limit: usize,

        /// Force re-download even if cached
        #[arg(long)]
        force: bool,

        /// Suppress progress output
        #[arg(short, long)]
        quiet: bool,
    },
```

**Step 2: Add handler for Download command**

In the `main()` function's match block, add:

```rust
        Commands::Download {
            states,
            all,
            list,
            limit,
            force,
            quiet,
        } => {
            if list {
                if let Err(e) = download::print_cache_list() {
                    eprintln!("Error listing cache: {}", e);
                    process::exit(1);
                }
                return;
            }

            let states_to_download: Vec<String> = if all {
                regions::ALL_STATES.iter().map(|s| s.to_string()).collect()
            } else if states.is_empty() {
                eprintln!("Error: Specify states to download or use --all");
                process::exit(1);
            } else {
                states
            };

            if let Err(e) = download::download_states(&states_to_download, limit, force, quiet) {
                eprintln!("Error downloading: {}", e);
                process::exit(1);
            }
        }
```

**Step 3: Add necessary imports at top of main.rs**

```rust
use regions::ALL_STATES;
```

**Step 4: Test the download help**

Run: `cargo run -- download --help`
Expected: Shows download command options

**Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat: add download subcommand to CLI"
```

---

## Task 6: Add --state Flag to Addresses Command

**Files:**
- Modify: `src/main.rs`
- Modify: `src/generators/addresses.rs`

**Step 1: Add load_addresses_from_cache function to addresses.rs**

Add to `src/generators/addresses.rs`:

```rust
use crate::cache;

/// Loads addresses from the cache for specified states
pub fn load_addresses_from_cache(
    states: &[String],
    count: Option<usize>,
) -> io::Result<Vec<Address>> {
    let mut all_addresses: Vec<Address> = Vec::new();

    for state in states {
        let state_upper = state.to_uppercase();

        if !cache::is_state_cached(&state_upper)? {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("State '{}' not cached. Run 'rust-faker download {}' first.", state_upper, state_upper),
            ));
        }

        let cache_path = cache::get_state_cache_path(&state_upper)?;
        let addresses = load_addresses_from_csv(cache_path.to_str().unwrap(), None)?;
        all_addresses.extend(addresses);
    }

    // Handle count
    if let Some(requested_count) = count {
        if requested_count < all_addresses.len() {
            let mut rng = rand::thread_rng();
            all_addresses.shuffle(&mut rng);
            all_addresses.truncate(requested_count);
        }
    }

    Ok(all_addresses)
}
```

**Step 2: Update Addresses command in main.rs**

Add `--state` flag to the Addresses command:

```rust
    /// Generate address records
    Addresses {
        /// Number of records to generate (required unless using --input or --state)
        #[arg(short, long)]
        count: Option<usize>,

        /// Input CSV file with real addresses to load
        #[arg(short, long)]
        input: Option<String>,

        /// Load from cached state(s) - comma-separated or 'all'
        #[arg(short, long)]
        state: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Error rate (0.0-1.0) - percentage of records with variance applied
        #[arg(short = 'e', long, default_value = "0.5")]
        error_rate: f64,

        /// Suppress progress output
        #[arg(short, long)]
        quiet: bool,
    },
```

**Step 3: Update Addresses handler in main.rs**

Replace the Addresses command handler:

```rust
        Commands::Addresses {
            count,
            input,
            state,
            output,
            error_rate,
            quiet,
        } => {
            if let Err(e) = validate_error_rate(error_rate) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }

            // Check mutual exclusivity
            let mode_count = [input.is_some(), state.is_some(), input.is_none() && state.is_none() && count.is_some()]
                .iter()
                .filter(|&&x| x)
                .count();

            if input.is_some() && state.is_some() {
                eprintln!("Error: Cannot use --input and --state together. Choose one.");
                process::exit(1);
            }

            let addresses = if let Some(input_path) = input {
                // Load from input file
                match load_addresses_from_csv(&input_path, count) {
                    Ok(loaded) => {
                        if !quiet {
                            println!("Loaded {} addresses from {}", loaded.len(), input_path);
                        }
                        apply_variance_to_addresses(loaded, error_rate)
                    }
                    Err(e) => {
                        eprintln!("Error loading addresses from {}: {}", input_path, e);
                        process::exit(1);
                    }
                }
            } else if let Some(state_str) = state {
                // Load from cache
                let states: Vec<String> = if state_str.to_lowercase() == "all" {
                    match cache::list_cached_states() {
                        Ok(cached) => cached.into_iter().map(|(s, _)| s).collect(),
                        Err(e) => {
                            eprintln!("Error reading cache: {}", e);
                            process::exit(1);
                        }
                    }
                } else {
                    state_str.split(',').map(|s| s.trim().to_string()).collect()
                };

                if states.is_empty() {
                    eprintln!("Error: No states specified or cached. Run 'rust-faker download <STATE>' first.");
                    process::exit(1);
                }

                match generators::addresses::load_addresses_from_cache(&states, count) {
                    Ok(loaded) => {
                        if !quiet {
                            println!("Loaded {} addresses from cache", loaded.len());
                        }
                        apply_variance_to_addresses(loaded, error_rate)
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                // Generate fake addresses
                let count = count.unwrap_or_else(|| {
                    eprintln!("Error: --count is required when not using --input or --state");
                    process::exit(1);
                });
                if let Err(e) = validate_count(count) {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
                generate_addresses(count, error_rate)
            };

            let final_count = addresses.len();
            let writer = CsvWriter::new(quiet);
            if let Err(e) = writer.write_addresses(&output, &addresses) {
                eprintln!("Error writing addresses: {}", e);
                process::exit(1);
            }

            if !quiet {
                println!("Successfully wrote {} addresses to {}", final_count, output);
            }
        }
```

**Step 4: Add cache import to main.rs**

At top of file, add:

```rust
mod cache;
```

**Step 5: Test help output**

Run: `cargo run -- addresses --help`
Expected: Shows --state option

**Step 6: Commit**

```bash
git add src/main.rs src/generators/addresses.rs
git commit -m "feat: add --state flag to addresses command for cache loading"
```

---

## Task 7: Add Integration Tests

**Files:**
- Modify: `tests/integration_test.rs`

**Step 1: Add integration tests for download command**

Add to `tests/integration_test.rs`:

```rust
#[test]
fn test_download_help_command() {
    let output = Command::new(get_binary_path())
        .args(&["download", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--all"), "Help should mention --all option");
    assert!(stdout.contains("--list"), "Help should mention --list option");
    assert!(stdout.contains("--limit"), "Help should mention --limit option");
}

#[test]
fn test_download_list_empty() {
    // Create a temp home directory to isolate cache
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(get_binary_path())
        .args(&["download", "--list"])
        .env("HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No addresses cached") || stdout.contains("Cached addresses"),
        "Should show cache status"
    );
}

#[test]
fn test_download_invalid_state() {
    let output = Command::new(get_binary_path())
        .args(&["download", "XX"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid state"),
        "Should reject invalid state code"
    );
}

#[test]
fn test_addresses_state_flag_uncached() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&["addresses", "--state", "IL", "--output", output_str])
        .env("HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not cached"),
        "Should error when state not cached"
    );
}

#[test]
fn test_addresses_state_and_input_mutual_exclusion() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--state", "IL",
            "--input", "file.csv",
            "--output", output_str,
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Cannot use --input and --state together"),
        "Should reject combining --input and --state"
    );
}

#[test]
fn test_addresses_help_mentions_state_flag() {
    let output = Command::new(get_binary_path())
        .args(&["addresses", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--state"), "Help should mention --state option");
}
```

**Step 2: Run tests**

Run: `cargo test`
Expected: All tests pass

**Step 3: Commit**

```bash
git add tests/integration_test.rs
git commit -m "test: add integration tests for download and --state features"
```

---

## Task 8: Update README

**Files:**
- Modify: `README.md`

**Step 1: Add documentation for download command and --state flag**

Add new section after "Load Addresses from CSV" in README.md:

```markdown
### Download Real Addresses

Download and cache real addresses from OpenAddresses.io for use in testing:

```bash
# Download addresses for specific states
rust-faker download IL CA TX

# Download all 50 states + DC
rust-faker download --all

# Check what's cached
rust-faker download --list

# Customize addresses per state (default: 10,000)
rust-faker download IL --limit 50000

# Force re-download
rust-faker download IL --force
```

**Cache location:** `~/.rust-faker/cache/addresses/`

### Use Cached Addresses

Once addresses are cached, use the `--state` flag:

```bash
# Use all cached addresses from Illinois
rust-faker addresses --state IL -o output.csv -e 0.5

# Sample 1000 addresses from Illinois
rust-faker addresses --state IL -c 1000 -o output.csv

# Sample across multiple states
rust-faker addresses --state IL,CA,TX -c 5000 -o output.csv

# Sample from all cached states
rust-faker addresses --state all -c 10000 -o output.csv
```

The `--state` flag is mutually exclusive with `--input`. When using `--state`, the `--count` flag optionally limits the sample size.
```

**Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add documentation for download and --state features"
```

---

## Task 9: Final Testing & Cleanup

**Step 1: Run full test suite**

Run: `cargo test`
Expected: All tests pass

**Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Format code**

Run: `cargo fmt`

**Step 4: Build release**

Run: `cargo build --release`
Expected: Compiles successfully

**Step 5: Manual smoke test (optional)**

```bash
# Test download list (empty)
./target/release/rust-faker download --list

# Test help
./target/release/rust-faker download --help
./target/release/rust-faker addresses --help
```

**Step 6: Final commit if any changes**

```bash
git add -A
git commit -m "chore: final cleanup and formatting"
```
