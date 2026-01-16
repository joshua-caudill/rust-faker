#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
}

impl Name {
    pub fn new(first_name: String, middle_name: String, last_name: String) -> Self {
        Self { first_name, middle_name, last_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_creation() {
        let name = Name::new(
            "Joshua".to_string(),
            "Allen".to_string(),
            "Caudill".to_string(),
        );
        assert_eq!(name.first_name, "Joshua");
        assert_eq!(name.middle_name, "Allen");
        assert_eq!(name.last_name, "Caudill");
    }

    #[test]
    fn test_name_creation_no_middle() {
        let name = Name::new(
            "Joshua".to_string(),
            String::new(),
            "Caudill".to_string(),
        );
        assert_eq!(name.first_name, "Joshua");
        assert_eq!(name.middle_name, "");
        assert_eq!(name.last_name, "Caudill");
    }
}
