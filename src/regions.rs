/// OpenAddresses.io regional data source URLs
pub const REGION_NORTHEAST: &str =
    "https://data.openaddresses.io/openaddr-collected-us_northeast.zip";
pub const REGION_MIDWEST: &str = "https://data.openaddresses.io/openaddr-collected-us_midwest.zip";
pub const REGION_SOUTH: &str = "https://data.openaddresses.io/openaddr-collected-us_south.zip";
pub const REGION_WEST: &str = "https://data.openaddresses.io/openaddr-collected-us_west.zip";

/// All valid US state codes (50 states + DC)
pub const ALL_STATES: [&str; 51] = [
    "AK", "AL", "AR", "AZ", "CA", "CO", "CT", "DC", "DE", "FL", "GA", "HI", "IA", "ID", "IL", "IN",
    "KS", "KY", "LA", "MA", "MD", "ME", "MI", "MN", "MO", "MS", "MT", "NC", "ND", "NE", "NH", "NJ",
    "NM", "NV", "NY", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN", "TX", "UT", "VA", "VT", "WA",
    "WI", "WV", "WY",
];

/// Returns the OpenAddresses.io region URL for a given state code.
///
/// # Arguments
/// * `state` - Two-letter state code (case-insensitive)
///
/// # Returns
/// * `Some(&str)` - The region URL if the state is valid
/// * `None` - If the state code is not recognized
///
/// # Examples
/// ```
/// use rust_faker::regions::get_region_url;
///
/// assert_eq!(
///     get_region_url("CA"),
///     Some("https://data.openaddresses.io/openaddr-collected-us_west.zip")
/// );
/// assert_eq!(get_region_url("invalid"), None);
/// ```
pub fn get_region_url(state: &str) -> Option<&'static str> {
    let state_upper = state.to_uppercase();
    match state_upper.as_str() {
        // Northeast: CT, ME, MA, NH, NJ, NY, PA, RI, VT
        "CT" | "ME" | "MA" | "NH" | "NJ" | "NY" | "PA" | "RI" | "VT" => Some(REGION_NORTHEAST),

        // Midwest: IL, IN, IA, KS, MI, MN, MO, NE, ND, OH, SD, WI
        "IL" | "IN" | "IA" | "KS" | "MI" | "MN" | "MO" | "NE" | "ND" | "OH" | "SD" | "WI" => {
            Some(REGION_MIDWEST)
        }

        // South: AL, AR, DE, DC, FL, GA, KY, LA, MD, MS, NC, OK, SC, TN, TX, VA, WV
        "AL" | "AR" | "DE" | "DC" | "FL" | "GA" | "KY" | "LA" | "MD" | "MS" | "NC" | "OK"
        | "SC" | "TN" | "TX" | "VA" | "WV" => Some(REGION_SOUTH),

        // West: AK, AZ, CA, CO, HI, ID, MT, NV, NM, OR, UT, WA, WY
        "AK" | "AZ" | "CA" | "CO" | "HI" | "ID" | "MT" | "NV" | "NM" | "OR" | "UT" | "WA"
        | "WY" => Some(REGION_WEST),

        _ => None,
    }
}

/// Validates if a state code is recognized.
///
/// # Arguments
/// * `state` - Two-letter state code (case-insensitive)
///
/// # Returns
/// * `true` - If the state code is valid (including DC)
/// * `false` - If the state code is not recognized
///
/// # Examples
/// ```
/// use rust_faker::regions::is_valid_state;
///
/// assert!(is_valid_state("CA"));
/// assert!(is_valid_state("ca"));
/// assert!(is_valid_state("DC"));
/// assert!(!is_valid_state("ZZ"));
/// ```
pub fn is_valid_state(state: &str) -> bool {
    let state_upper = state.to_uppercase();
    ALL_STATES.contains(&state_upper.as_str())
}

/// Returns the lowercase state code for use in file paths.
///
/// # Arguments
/// * `state` - Two-letter state code (case-insensitive)
///
/// # Returns
/// * `Some(String)` - Lowercase state code if valid
/// * `None` - If the state code is not recognized
///
/// # Examples
/// ```
/// use rust_faker::regions::get_state_path_name;
///
/// assert_eq!(get_state_path_name("CA"), Some("ca".to_string()));
/// assert_eq!(get_state_path_name("ca"), Some("ca".to_string()));
/// assert_eq!(get_state_path_name("DC"), Some("dc".to_string()));
/// assert_eq!(get_state_path_name("invalid"), None);
/// ```
#[allow(dead_code)]
pub fn get_state_path_name(state: &str) -> Option<String> {
    if is_valid_state(state) {
        Some(state.to_lowercase())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_region_url_northeast() {
        // Test all Northeast states
        let northeast_states = ["CT", "ME", "MA", "NH", "NJ", "NY", "PA", "RI", "VT"];
        for state in &northeast_states {
            assert_eq!(get_region_url(state), Some(REGION_NORTHEAST));
        }
    }

    #[test]
    fn test_get_region_url_midwest() {
        // Test all Midwest states
        let midwest_states = [
            "IL", "IN", "IA", "KS", "MI", "MN", "MO", "NE", "ND", "OH", "SD", "WI",
        ];
        for state in &midwest_states {
            assert_eq!(get_region_url(state), Some(REGION_MIDWEST));
        }
    }

    #[test]
    fn test_get_region_url_south() {
        // Test all South states (including DC)
        let south_states = [
            "AL", "AR", "DE", "DC", "FL", "GA", "KY", "LA", "MD", "MS", "NC", "OK", "SC", "TN",
            "TX", "VA", "WV",
        ];
        for state in &south_states {
            assert_eq!(get_region_url(state), Some(REGION_SOUTH));
        }
    }

    #[test]
    fn test_get_region_url_west() {
        // Test all West states
        let west_states = [
            "AK", "AZ", "CA", "CO", "HI", "ID", "MT", "NV", "NM", "OR", "UT", "WA", "WY",
        ];
        for state in &west_states {
            assert_eq!(get_region_url(state), Some(REGION_WEST));
        }
    }

    #[test]
    fn test_get_region_url_case_insensitive() {
        assert_eq!(get_region_url("ca"), Some(REGION_WEST));
        assert_eq!(get_region_url("CA"), Some(REGION_WEST));
        assert_eq!(get_region_url("Ca"), Some(REGION_WEST));
        assert_eq!(get_region_url("ny"), Some(REGION_NORTHEAST));
        assert_eq!(get_region_url("NY"), Some(REGION_NORTHEAST));
    }

    #[test]
    fn test_get_region_url_invalid() {
        assert_eq!(get_region_url("ZZ"), None);
        assert_eq!(get_region_url("invalid"), None);
        assert_eq!(get_region_url(""), None);
        assert_eq!(get_region_url("XXX"), None);
    }

    #[test]
    fn test_is_valid_state_valid_states() {
        // Test a sample of valid states
        assert!(is_valid_state("CA"));
        assert!(is_valid_state("NY"));
        assert!(is_valid_state("TX"));
        assert!(is_valid_state("DC"));
        assert!(is_valid_state("AK"));
        assert!(is_valid_state("HI"));
    }

    #[test]
    fn test_is_valid_state_case_insensitive() {
        assert!(is_valid_state("ca"));
        assert!(is_valid_state("CA"));
        assert!(is_valid_state("Ca"));
        assert!(is_valid_state("cA"));
    }

    #[test]
    fn test_is_valid_state_invalid() {
        assert!(!is_valid_state("ZZ"));
        assert!(!is_valid_state("invalid"));
        assert!(!is_valid_state(""));
        assert!(!is_valid_state("XXX"));
        assert!(!is_valid_state("12"));
    }

    #[test]
    fn test_all_states_count() {
        // Verify we have exactly 51 states (50 states + DC)
        assert_eq!(ALL_STATES.len(), 51);
    }

    #[test]
    fn test_all_states_have_regions() {
        // Verify every state in ALL_STATES has a region mapping
        for state in &ALL_STATES {
            assert!(
                get_region_url(state).is_some(),
                "State {} should have a region mapping",
                state
            );
        }
    }

    #[test]
    fn test_get_state_path_name_valid() {
        assert_eq!(get_state_path_name("CA"), Some("ca".to_string()));
        assert_eq!(get_state_path_name("ca"), Some("ca".to_string()));
        assert_eq!(get_state_path_name("NY"), Some("ny".to_string()));
        assert_eq!(get_state_path_name("DC"), Some("dc".to_string()));
    }

    #[test]
    fn test_get_state_path_name_invalid() {
        assert_eq!(get_state_path_name("ZZ"), None);
        assert_eq!(get_state_path_name("invalid"), None);
        assert_eq!(get_state_path_name(""), None);
    }

    #[test]
    fn test_get_state_path_name_lowercase() {
        // Verify output is always lowercase regardless of input case
        assert_eq!(get_state_path_name("CA"), Some("ca".to_string()));
        assert_eq!(get_state_path_name("Ca"), Some("ca".to_string()));
        assert_eq!(get_state_path_name("cA"), Some("ca".to_string()));
    }

    #[test]
    fn test_dc_is_in_south_region() {
        // Verify DC is correctly mapped to South region
        assert_eq!(get_region_url("DC"), Some(REGION_SOUTH));
        assert!(is_valid_state("DC"));
    }
}
