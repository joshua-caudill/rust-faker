use fake::Fake;
use fake::faker::address::en::*;
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
    pub fn new(address1: String, address2: String, city: String, state: String, zip: String) -> Self {
        Self { address1, address2, city, state, zip }
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
}
