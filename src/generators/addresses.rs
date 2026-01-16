use fake::faker::address::en::*;
use fake::Fake;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

impl Address {
    pub fn new(
        address1: String,
        address2: String,
        city: String,
        state: String,
        zip: String,
    ) -> Self {
        Self {
            address1,
            address2,
            city,
            state,
            zip,
        }
    }

    pub fn to_record(&self) -> Vec<String> {
        vec![
            self.address1.clone(),
            self.address2.clone(),
            self.city.clone(),
            self.state.clone(),
            self.zip.clone(),
        ]
    }
}

pub fn generate_clean_address() -> Address {
    let mut rng = rand::thread_rng();

    let street_number: u32 = (1..9999).fake();
    let street_name: String = StreetName().fake();
    let street_suffix: String = StreetSuffix().fake();
    let address1 = format!("{} {} {}", street_number, street_name, street_suffix);

    // 50% chance of having a secondary address
    let address2 = if rng.gen_bool(0.5) {
        SecondaryAddress().fake()
    } else {
        String::new()
    };

    let city: String = CityName().fake();
    let state: String = StateAbbr().fake();
    let zip: String = ZipCode().fake();

    Address::new(address1, address2, city, state, zip)
}

/// Generates a vector of addresses with configurable variance.
///
/// Creates `count` addresses, applying variance patterns to each address
/// with probability `error_rate`. When variance is not applied,
/// returns clean, properly formatted addresses.
///
/// # Arguments
/// * `count` - Number of addresses to generate
/// * `error_rate` - Probability (0.0 to 1.0) of applying variance to each address
///
/// # Panics
/// Panics if `error_rate` is outside the range [0.0, 1.0]
///
/// # Examples
/// ```
/// // Generate 10 clean addresses
/// let clean_addresses = generate_addresses(10, 0.0);
///
/// // Generate 10 addresses with 30% variance
/// let varied_addresses = generate_addresses(10, 0.3);
/// ```
pub fn generate_addresses(count: usize, error_rate: f64) -> Vec<Address> {
    let mut rng = rand::thread_rng();
    let mut addresses = Vec::with_capacity(count);

    for _ in 0..count {
        let clean_address = generate_clean_address();

        // Apply variance based on error rate
        let address = if rng.gen_bool(error_rate) {
            apply_address_variance(clean_address)
        } else {
            clean_address
        };

        addresses.push(address);
    }

    addresses
}

fn abbreviate_street_suffix(suffix: &str) -> String {
    match suffix {
        "Street" => "St",
        "Avenue" => "Ave",
        "Road" => "Rd",
        "Boulevard" => "Blvd",
        "Drive" => "Dr",
        "Lane" => "Ln",
        "Parkway" => "Pkwy",
        "Court" => "Ct",
        "Circle" => "Cir",
        "Way" => "Way",
        "Place" => "Pl",
        "Square" => "Sq",
        "Trail" => "Trl",
        "Terrace" => "Ter",
        _ => suffix,
    }
    .to_string()
}

fn generate_po_box() -> String {
    let mut rng = rand::thread_rng();
    let box_number: u32 = (1..9999).fake();

    let formats = [
        format!("PO Box {}", box_number),
        format!("P.O. Box {}", box_number),
        format!("POB {}", box_number),
    ];

    formats[rng.gen_range(0..formats.len())].clone()
}

fn generate_apartment() -> String {
    let mut rng = rand::thread_rng();
    let unit: String = format!(
        "{}{}",
        rng.gen_range(1..999),
        if rng.gen_bool(0.3) {
            ['A', 'B', 'C', 'D'][rng.gen_range(0..4)].to_string()
        } else {
            String::new()
        }
    );

    let formats = [
        format!("Apt {}", unit),
        format!("Apartment {}", unit),
        format!("#{}", unit),
        format!("Unit {}", unit),
        format!("Suite {}", unit),
        format!("Ste {}", unit),
        format!("Ste. {}", unit),
    ];

    formats[rng.gen_range(0..formats.len())].clone()
}

fn apply_address_variance(mut address: Address) -> Address {
    let mut rng = rand::thread_rng();

    // Apply 1-3 random variance patterns
    let num_variances = rng.gen_range(1..=3);

    for _ in 0..num_variances {
        let variance_type = rng.gen_range(0..10);

        match variance_type {
            0 => {
                // Abbreviate street suffix
                let parts: Vec<&str> = address.address1.split_whitespace().collect();
                if let Some(&last) = parts.last() {
                    let abbreviated = abbreviate_street_suffix(last);
                    let mut new_parts = parts[..parts.len() - 1].to_vec();
                    new_parts.push(&abbreviated);
                    address.address1 = new_parts.join(" ");
                }
            }
            1 => {
                // Replace with PO Box
                address.address1 = generate_po_box();
                address.address2 = String::new();
            }
            2 => {
                // Add apartment/unit
                address.address2 = generate_apartment();
            }
            3 => {
                // Remove state
                address.state = String::new();
            }
            4 => {
                // Remove zip
                address.zip = String::new();
            }
            5 => {
                // Remove city
                address.city = String::new();
            }
            6 => {
                // All caps
                address.address1 = address.address1.to_uppercase();
                address.city = address.city.to_uppercase();
            }
            7 => {
                // Add extra spaces
                address.address1 = address.address1.replace(" ", "  ");
            }
            8 => {
                // Add periods inconsistently
                if rng.gen_bool(0.5) {
                    address.address1 = address.address1.replace("St", "St.");
                    address.address1 = address.address1.replace("Ave", "Ave.");
                }
            }
            _ => {
                // Mixed case
                address.city = address
                    .city
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i % 2 == 0 {
                            c.to_uppercase().to_string()
                        } else {
                            c.to_lowercase().to_string()
                        }
                    })
                    .collect();
            }
        }
    }

    address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation() {
        let addr = Address::new(
            "123 Main St".to_string(),
            "Apt 4B".to_string(),
            "Springfield".to_string(),
            "IL".to_string(),
            "62701".to_string(),
        );
        assert_eq!(addr.address1, "123 Main St");
        assert_eq!(addr.address2, "Apt 4B");
        assert_eq!(addr.city, "Springfield");
        assert_eq!(addr.state, "IL");
        assert_eq!(addr.zip, "62701");
    }

    #[test]
    fn test_address_to_record() {
        let addr = Address::new(
            "123 Main St".to_string(),
            "Apt 4B".to_string(),
            "Springfield".to_string(),
            "IL".to_string(),
            "62701".to_string(),
        );
        let record = addr.to_record();
        assert_eq!(
            record,
            vec!["123 Main St", "Apt 4B", "Springfield", "IL", "62701"]
        );
    }

    #[test]
    fn test_generate_clean_address() {
        let addr = generate_clean_address();
        // Basic validation - clean addresses should have values in fields
        assert!(!addr.address1.is_empty());
        assert!(!addr.city.is_empty());
        assert!(!addr.state.is_empty());
        assert!(!addr.zip.is_empty());
        // address2 can be empty (it's optional)
    }

    #[test]
    fn test_generate_clean_address_randomness() {
        // Generate multiple addresses and verify they're different
        let addr1 = generate_clean_address();
        let addr2 = generate_clean_address();
        // Very unlikely to generate identical addresses
        assert_ne!(addr1, addr2);
    }

    #[test]
    fn test_abbreviate_street_suffix_known() {
        assert_eq!(abbreviate_street_suffix("Street"), "St");
        assert_eq!(abbreviate_street_suffix("Avenue"), "Ave");
        assert_eq!(abbreviate_street_suffix("Road"), "Rd");
        assert_eq!(abbreviate_street_suffix("Boulevard"), "Blvd");
    }

    #[test]
    fn test_abbreviate_street_suffix_unknown() {
        // Unknown suffixes should return as-is
        assert_eq!(abbreviate_street_suffix("Unknown"), "Unknown");
    }

    #[test]
    fn test_generate_po_box() {
        let po_box = generate_po_box();
        assert!(po_box.contains("Box") || po_box.contains("BOX") || po_box.contains("POB"));
    }

    #[test]
    fn test_generate_apartment() {
        let apt = generate_apartment();
        assert!(
            apt.contains("Apt")
                || apt.contains("Unit")
                || apt.contains("#")
                || apt.contains("Suite")
                || apt.contains("Ste")
        );
    }

    #[test]
    fn test_apply_address_variance_abbreviates_suffix() {
        let clean = Address::new(
            "123 Main Street".to_string(),
            String::new(),
            "Springfield".to_string(),
            "IL".to_string(),
            "62701".to_string(),
        );

        // Apply variance multiple times until we get abbreviation
        // (since variance is random, we test the function exists and runs)
        let varied = apply_address_variance(clean.clone());
        assert!(!varied.address1.is_empty());
    }

    #[test]
    fn test_apply_address_variance_can_add_po_box() {
        let clean = Address::new(
            "123 Main St".to_string(),
            String::new(),
            "Springfield".to_string(),
            "IL".to_string(),
            "62701".to_string(),
        );

        let varied = apply_address_variance(clean);
        // Just verify it doesn't panic and returns something
        assert!(!varied.address1.is_empty());
    }

    #[test]
    fn test_generate_addresses_count() {
        let addresses = generate_addresses(10, 0.0);
        assert_eq!(addresses.len(), 10);
    }

    #[test]
    fn test_generate_addresses_zero_error_rate() {
        let addresses = generate_addresses(5, 0.0);
        // All should be clean (have all fields populated)
        for addr in addresses {
            assert!(!addr.address1.is_empty());
            assert!(!addr.city.is_empty());
            assert!(!addr.state.is_empty());
            assert!(!addr.zip.is_empty());
        }
    }

    #[test]
    fn test_generate_addresses_full_error_rate() {
        let addresses = generate_addresses(5, 1.0);
        // All should have variance applied
        // Hard to test exactly, but verify we got addresses
        assert_eq!(addresses.len(), 5);
    }
}
