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

fn get_random_prefix() -> String {
    let mut rng = rand::thread_rng();
    let prefixes = vec!["Dr.", "Mr.", "Mrs.", "Ms.", "Prof.", "Rev."];
    prefixes[rng.gen_range(0..prefixes.len())].to_string()
}

fn get_random_suffix() -> String {
    let mut rng = rand::thread_rng();
    let suffixes = vec!["Jr.", "Sr.", "II", "III", "IV", "MD", "PhD", "Esq."];
    suffixes[rng.gen_range(0..suffixes.len())].to_string()
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

    #[test]
    fn test_get_random_prefix() {
        let prefix = get_random_prefix();
        let valid_prefixes = vec!["Dr.", "Mr.", "Mrs.", "Ms.", "Prof.", "Rev."];
        assert!(valid_prefixes.contains(&prefix.as_str()));
    }

    #[test]
    fn test_get_random_suffix() {
        let suffix = get_random_suffix();
        let valid_suffixes = vec!["Jr.", "Sr.", "II", "III", "IV", "MD", "PhD", "Esq."];
        assert!(valid_suffixes.contains(&suffix.as_str()));
    }
}
