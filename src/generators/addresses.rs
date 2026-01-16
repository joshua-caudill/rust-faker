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
}
