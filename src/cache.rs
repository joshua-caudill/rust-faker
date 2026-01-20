use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Cache manifest tracking downloaded state data
#[derive(Serialize, Deserialize, Default)]
pub struct CacheManifest {
    pub version: u32,
    pub states: HashMap<String, StateCache>,
}

/// Metadata for a cached state
#[derive(Serialize, Deserialize, Clone)]
pub struct StateCache {
    pub downloaded_at: String,
    pub source_url: String,
    pub record_count: usize,
}

/// Returns the cache directory path: ~/.rust-faker/cache/addresses/
pub fn get_cache_dir() -> io::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

    Ok(home.join(".rust-faker").join("cache").join("addresses"))
}

/// Creates the cache directory if it doesn't exist and returns the path
pub fn ensure_cache_dir() -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

/// Returns the path to the manifest.json file
pub fn get_manifest_path() -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    Ok(cache_dir.join("manifest.json"))
}

/// Returns the path to a state's CSV file (state code in uppercase)
pub fn get_state_cache_path(state: &str) -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    let state_upper = state.to_uppercase();
    Ok(cache_dir.join(format!("{}.csv", state_upper)))
}

/// Loads the manifest from disk, or returns an empty manifest if it doesn't exist
pub fn load_manifest() -> io::Result<CacheManifest> {
    let manifest_path = get_manifest_path()?;

    if !manifest_path.exists() {
        return Ok(CacheManifest::default());
    }

    let contents = fs::read_to_string(manifest_path)?;
    let manifest: CacheManifest = serde_json::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(manifest)
}

/// Saves the manifest to disk as pretty-printed JSON
pub fn save_manifest(manifest: &CacheManifest) -> io::Result<()> {
    ensure_cache_dir()?;
    let manifest_path = get_manifest_path()?;

    let json = serde_json::to_string_pretty(manifest)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    fs::write(manifest_path, json)?;
    Ok(())
}

/// Checks if a state is cached (both in manifest and file exists on disk)
pub fn is_state_cached(state: &str) -> io::Result<bool> {
    let manifest = load_manifest()?;
    let state_upper = state.to_uppercase();

    // Check if state is in manifest
    if !manifest.states.contains_key(&state_upper) {
        return Ok(false);
    }

    // Check if file exists
    let state_path = get_state_cache_path(state)?;
    Ok(state_path.exists())
}

/// Returns a sorted list of cached states with their metadata
pub fn list_cached_states() -> io::Result<Vec<(String, StateCache)>> {
    let manifest = load_manifest()?;

    let mut states: Vec<(String, StateCache)> = manifest.states.into_iter().collect();

    states.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(states)
}

/// Extracts the region name from a URL (e.g., "us_south" from the URL)
fn extract_region_name(region_url: &str) -> String {
    region_url
        .rsplit('/')
        .next()
        .unwrap_or("region")
        .trim_end_matches(".zip")
        .replace("openaddr-collected-", "")
}

/// Returns the path to a regional ZIP file in cache
pub fn get_region_zip_path(region_url: &str) -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    let region_name = extract_region_name(region_url);
    Ok(cache_dir.join(format!("{}.zip", region_name)))
}

/// Returns the path to a regional directory in cache (for extracted ZIPs)
pub fn get_region_dir_path(region_url: &str) -> io::Result<PathBuf> {
    let cache_dir = get_cache_dir()?;
    let region_name = extract_region_name(region_url);
    Ok(cache_dir.join(region_name))
}

/// Enum to represent cached region format
pub enum CachedRegion {
    Zip(PathBuf),
    Directory(PathBuf),
}

/// Checks if a regional data is cached (either as ZIP or directory)
#[allow(dead_code)]
pub fn is_region_cached(region_url: &str) -> io::Result<bool> {
    Ok(get_cached_region(region_url)?.is_some())
}

/// Gets the cached region if it exists (ZIP takes precedence)
pub fn get_cached_region(region_url: &str) -> io::Result<Option<CachedRegion>> {
    let zip_path = get_region_zip_path(region_url)?;
    if zip_path.exists() {
        return Ok(Some(CachedRegion::Zip(zip_path)));
    }

    let dir_path = get_region_dir_path(region_url)?;
    if dir_path.exists() && dir_path.is_dir() {
        return Ok(Some(CachedRegion::Directory(dir_path)));
    }

    Ok(None)
}

/// Saves a regional ZIP file to the cache
pub fn save_region_zip(region_url: &str, data: &[u8]) -> io::Result<PathBuf> {
    ensure_cache_dir()?;
    let zip_path = get_region_zip_path(region_url)?;
    fs::write(&zip_path, data)?;
    Ok(zip_path)
}

/// Loads a regional ZIP file from the cache
#[allow(dead_code)]
pub fn load_region_zip(region_url: &str) -> io::Result<Vec<u8>> {
    let zip_path = get_region_zip_path(region_url)?;
    fs::read(zip_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_manifest_serialization() {
        let mut manifest = CacheManifest {
            version: 1,
            states: HashMap::new(),
        };

        manifest.states.insert(
            "CA".to_string(),
            StateCache {
                downloaded_at: "2024-01-01T00:00:00Z".to_string(),
                source_url: "https://example.com/ca.zip".to_string(),
                record_count: 1000,
            },
        );

        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: CacheManifest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.version, 1);
        assert_eq!(deserialized.states.len(), 1);
        assert!(deserialized.states.contains_key("CA"));
    }

    #[test]
    fn test_default_cache_manifest() {
        let manifest = CacheManifest::default();
        assert_eq!(manifest.version, 0);
        assert_eq!(manifest.states.len(), 0);
    }

    #[test]
    fn test_state_cache_clone() {
        let cache = StateCache {
            downloaded_at: "2024-01-01T00:00:00Z".to_string(),
            source_url: "https://example.com/ca.zip".to_string(),
            record_count: 1000,
        };

        let cloned = cache.clone();
        assert_eq!(cache.downloaded_at, cloned.downloaded_at);
        assert_eq!(cache.source_url, cloned.source_url);
        assert_eq!(cache.record_count, cloned.record_count);
    }

    #[test]
    fn test_get_cache_dir_structure() {
        let cache_dir = get_cache_dir().unwrap();
        let path_str = cache_dir.to_string_lossy();

        assert!(path_str.contains(".rust-faker"));
        assert!(path_str.contains("cache"));
        assert!(path_str.contains("addresses"));
    }

    #[test]
    fn test_get_manifest_path() {
        let manifest_path = get_manifest_path().unwrap();
        let path_str = manifest_path.to_string_lossy();

        assert!(path_str.contains("manifest.json"));
        assert!(path_str.contains(".rust-faker"));
    }

    #[test]
    fn test_get_state_cache_path_uppercase() {
        let path_lower = get_state_cache_path("ca").unwrap();
        let path_upper = get_state_cache_path("CA").unwrap();
        let path_mixed = get_state_cache_path("Ca").unwrap();

        // All should result in CA.csv
        assert!(path_lower.to_string_lossy().ends_with("CA.csv"));
        assert!(path_upper.to_string_lossy().ends_with("CA.csv"));
        assert!(path_mixed.to_string_lossy().ends_with("CA.csv"));

        // All should be equal
        assert_eq!(path_lower, path_upper);
        assert_eq!(path_upper, path_mixed);
    }

    #[test]
    fn test_load_manifest_nonexistent() {
        // This will try to load from actual home directory, which may or may not have a manifest
        // We test that it returns successfully (either with existing manifest or default)
        let result = load_manifest();
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_cached_states_empty() {
        // This may return actual cached states or empty list depending on system state
        let result = list_cached_states();
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_cached_states_sorted() {
        // Test that if we have states, they're sorted
        let states = list_cached_states().unwrap();

        // Check if sorted
        for i in 1..states.len() {
            assert!(
                states[i - 1].0 <= states[i].0,
                "States should be sorted alphabetically"
            );
        }
    }

    #[test]
    fn test_is_state_cached_nonexistent() {
        // Test with a state that's unlikely to be cached
        let result = is_state_cached("ZZ");
        assert!(result.is_ok());
        // Should be false for invalid state
        assert!(!result.unwrap());
    }

    #[test]
    fn test_state_cache_path_contains_state_code() {
        let path = get_state_cache_path("NY").unwrap();
        assert!(path.to_string_lossy().contains("NY.csv"));

        let path = get_state_cache_path("tx").unwrap();
        assert!(path.to_string_lossy().contains("TX.csv"));
    }
}
