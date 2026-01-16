use fake::Fake;
use fake::faker::name::en::*;
use rand::Rng;

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

pub fn generate_clean_name() -> Name {
    let mut rng = rand::thread_rng();

    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();

    // 50% chance of having a middle name
    let middle_name = if rng.gen_bool(0.5) {
        FirstName().fake()
    } else {
        String::new()
    };

    Name::new(first_name, middle_name, last_name)
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

    #[test]
    fn test_generate_clean_name() {
        let name = generate_clean_name();
        assert!(!name.first_name.is_empty());
        assert!(!name.last_name.is_empty());
        // middle_name can be empty (50% chance)
    }

    #[test]
    fn test_generate_clean_name_randomness() {
        let name1 = generate_clean_name();
        let name2 = generate_clean_name();
        // Very unlikely to generate identical names
        assert_ne!(name1, name2);
    }
}
