use fake::faker::name::en::*;
use fake::Fake;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
}

impl Name {
    pub fn new(first_name: String, middle_name: String, last_name: String) -> Self {
        Self {
            first_name,
            middle_name,
            last_name,
        }
    }

    pub fn to_record(&self) -> Vec<String> {
        vec![
            self.first_name.clone(),
            self.middle_name.clone(),
            self.last_name.clone(),
        ]
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

/// Generates a vector of names with configurable variance.
///
/// Creates `count` names, applying variance patterns to each name
/// with probability `error_rate`. When variance is not applied,
/// returns clean, properly formatted names.
///
/// # Arguments
/// * `count` - Number of names to generate
/// * `error_rate` - Probability (0.0 to 1.0) of applying variance to each name
///
/// # Panics
/// Panics if `error_rate` is outside the range [0.0, 1.0]
///
/// # Examples
/// ```
/// // Generate 10 clean names
/// let clean_names = generate_names(10, 0.0);
///
/// // Generate 10 names with 30% variance
/// let varied_names = generate_names(10, 0.3);
/// ```
pub fn generate_names(count: usize, error_rate: f64) -> Vec<Name> {
    let mut rng = rand::thread_rng();
    let mut names = Vec::with_capacity(count);

    for _ in 0..count {
        let clean_name = generate_clean_name();

        // Apply variance based on error rate
        let name = if rng.gen_bool(error_rate) {
            apply_name_variance(clean_name)
        } else {
            clean_name
        };

        names.push(name);
    }

    names
}

fn get_random_prefix() -> String {
    let mut rng = rand::thread_rng();
    let prefixes = ["Dr.", "Mr.", "Mrs.", "Ms.", "Prof.", "Rev."];
    prefixes[rng.gen_range(0..prefixes.len())].to_string()
}

fn get_random_suffix() -> String {
    let mut rng = rand::thread_rng();
    let suffixes = ["Jr.", "Sr.", "II", "III", "IV", "MD", "PhD", "Esq."];
    suffixes[rng.gen_range(0..suffixes.len())].to_string()
}

/// Adds a realistic typo to a name string.
///
/// Randomly applies one of three typo types:
/// - Double a letter (e.g., "John" -> "Johhn")
/// - Transpose two adjacent letters (e.g., "John" -> "Jhon")
/// - Remove a letter (e.g., "John" -> "Jon")
///
/// Returns the original string unchanged if it's empty or has less than 2 characters.
fn add_typo(name: &str) -> String {
    if name.is_empty() {
        return name.to_string();
    }

    let mut rng = rand::thread_rng();
    let mut chars: Vec<char> = name.chars().collect();

    if chars.len() < 2 {
        return name.to_string();
    }

    let typo_type = rng.gen_range(0..3);
    match typo_type {
        0 => {
            // Double a letter
            let pos = rng.gen_range(0..chars.len());
            chars.insert(pos, chars[pos]);
        }
        1 => {
            // Transpose two letters
            if chars.len() >= 2 {
                let pos = rng.gen_range(0..chars.len() - 1);
                chars.swap(pos, pos + 1);
            }
        }
        _ => {
            // Remove a letter (but keep at least one)
            if chars.len() > 1 {
                let pos = rng.gen_range(0..chars.len());
                chars.remove(pos);
            }
        }
    }

    chars.into_iter().collect()
}

/// Converts a string to alternating case (e.g., "Joshua" -> "JoShUa").
///
/// Characters at even indices are uppercased, odd indices are lowercased.
/// Used for testing case-insensitive matching in standardization systems.
fn to_mixed_case(name: &str) -> String {
    name.chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}

/// Applies 1-3 random variance patterns to a Name.
///
/// Randomly selects and applies between 1 and 3 variance types from 15 possible patterns:
/// - Field swapping and combining (0-3)
/// - Hyphenation patterns (4-6)
/// - Prefixes and suffixes (7-8)
/// - Nickname formats (9-10)
/// - Case variations (11-13)
/// - Typos (14)
///
/// Variance patterns can be applied multiple times, potentially creating cumulative effects.
fn apply_name_variance(mut name: Name) -> Name {
    let mut rng = rand::thread_rng();

    // Apply 1-3 random variance patterns
    let num_variances = rng.gen_range(1..=3);

    for _ in 0..num_variances {
        let variance_type = rng.gen_range(0..15);

        match variance_type {
            0 => {
                // Swap first and last names
                std::mem::swap(&mut name.first_name, &mut name.last_name);
            }
            1 => {
                // First and last combined in first name
                name.first_name = format!("{} {}", name.first_name, name.last_name);
                name.last_name = String::new();
            }
            2 => {
                // "LastName, FirstName" format in first name
                name.first_name = format!("{}, {}", name.last_name, name.first_name);
                name.last_name = String::new();
            }
            3 => {
                // Full name in one field
                name.first_name = format!(
                    "{} {} {}",
                    name.first_name, name.middle_name, name.last_name
                );
                name.middle_name = String::new();
                name.last_name = String::new();
            }
            4 => {
                // Hyphenated last name
                let extra_last: String = LastName().fake();
                name.last_name = format!("{}-{}", name.last_name, extra_last);
            }
            5 => {
                // Hyphenated first name
                let extra_first: String = FirstName().fake();
                name.first_name = format!("{}-{}", name.first_name, extra_first);
            }
            6 => {
                // Multiple last names
                let extra_last: String = LastName().fake();
                name.last_name = format!("{} {}", name.last_name, extra_last);
            }
            7 => {
                // Add prefix to first name
                name.first_name = format!("{} {}", get_random_prefix(), name.first_name);
            }
            8 => {
                // Add suffix to last name
                if !name.last_name.is_empty() {
                    name.last_name = format!("{} {}", name.last_name, get_random_suffix());
                }
            }
            9 => {
                // Nickname in quotes
                name.first_name = format!("\"{}\"", name.first_name);
            }
            10 => {
                // Nickname in parentheses
                if !name.first_name.is_empty() {
                    name.first_name = format!(
                        "{} ({})",
                        name.first_name,
                        &name.first_name[..3.min(name.first_name.len())]
                    );
                }
            }
            11 => {
                // All caps
                name.first_name = name.first_name.to_uppercase();
                name.middle_name = name.middle_name.to_uppercase();
                name.last_name = name.last_name.to_uppercase();
            }
            12 => {
                // All lowercase
                name.first_name = name.first_name.to_lowercase();
                name.middle_name = name.middle_name.to_lowercase();
                name.last_name = name.last_name.to_lowercase();
            }
            13 => {
                // Mixed case
                name.first_name = to_mixed_case(&name.first_name);
                name.middle_name = to_mixed_case(&name.middle_name);
                name.last_name = to_mixed_case(&name.last_name);
            }
            _ => {
                // Add typo
                if rng.gen_bool(0.5) {
                    name.first_name = add_typo(&name.first_name);
                } else {
                    name.last_name = add_typo(&name.last_name);
                }
            }
        }
    }

    name
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
        let name = Name::new("Joshua".to_string(), String::new(), "Caudill".to_string());
        assert_eq!(name.first_name, "Joshua");
        assert_eq!(name.middle_name, "");
        assert_eq!(name.last_name, "Caudill");
    }

    #[test]
    fn test_name_to_record() {
        let name = Name::new(
            "Joshua".to_string(),
            "Allen".to_string(),
            "Caudill".to_string(),
        );
        let record = name.to_record();
        assert_eq!(record, vec!["Joshua", "Allen", "Caudill"]);
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

    #[test]
    fn test_add_typo() {
        let result = add_typo("Joshua");
        assert!(!result.is_empty());
        // Verify length changed by at most 1 character
        assert!((result.len() as i32 - "Joshua".len() as i32).abs() <= 1);
    }

    #[test]
    fn test_to_mixed_case() {
        let result = to_mixed_case("Joshua");
        assert_eq!(result, "JoShUa");
        assert_eq!(result.len(), "Joshua".len());

        // Test with all lowercase
        assert_eq!(to_mixed_case("hello"), "HeLlO");

        // Test with all uppercase
        assert_eq!(to_mixed_case("HELLO"), "HeLlO");
    }

    #[test]
    fn test_apply_name_variance() {
        let clean = Name::new(
            "Joshua".to_string(),
            "Allen".to_string(),
            "Caudill".to_string(),
        );

        // Apply variance and verify it doesn't panic
        let varied = apply_name_variance(clean);
        // At least one field should have content
        assert!(
            !varied.first_name.is_empty()
                || !varied.middle_name.is_empty()
                || !varied.last_name.is_empty()
        );
    }

    #[test]
    fn test_generate_names_count() {
        let names = generate_names(10, 0.0);
        assert_eq!(names.len(), 10);
    }

    #[test]
    fn test_generate_names_zero_error_rate() {
        let names = generate_names(5, 0.0);
        // All should be clean
        for name in names {
            assert!(!name.first_name.is_empty());
            assert!(!name.last_name.is_empty());
        }
    }

    #[test]
    fn test_generate_names_full_error_rate() {
        let names = generate_names(5, 1.0);
        // All should have variance applied
        assert_eq!(names.len(), 5);
    }
}
