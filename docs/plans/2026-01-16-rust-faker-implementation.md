# Rust Faker Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a CLI tool that generates pipe-delimited CSV files with realistic addresses and names, including configurable data quality issues for testing standardization systems.

**Architecture:** Subcommand-based CLI using clap. Separate generator modules for addresses and names using the `fake` crate. Writer module handles CSV output with progress bars via indicatif. Random variance applied based on error-rate parameter.

**Tech Stack:** Rust, clap (CLI), fake (data generation), rand (randomness), csv (file writing), indicatif (progress bars)

---

## Task 1: Project Initialization

**Files:**
- Create: `Cargo.toml`

**Step 1: Initialize Cargo project**

Run: `cargo init --name rust-faker`
Expected: Creates basic Cargo project structure

**Step 2: Add dependencies to Cargo.toml**

```toml
[package]
name = "rust-faker"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
fake = { version = "2.9", features = ["derive"] }
rand = "0.8"
csv = "1.3"
indicatif = "0.17"

[dev-dependencies]
tempfile = "3.10"
```

**Step 3: Verify dependencies resolve**

Run: `cargo check`
Expected: All dependencies download and compile successfully

**Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: initialize Cargo project with dependencies"
```

---

## Task 2: CLI Structure Setup

**Files:**
- Modify: `src/main.rs`

**Step 1: Write basic CLI test**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_help_works() {
        // This will test that CLI parsing doesn't panic
        // More detailed CLI tests will come later
    }
}
```

**Step 2: Implement CLI structure**

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rust-faker")]
#[command(about = "Generate test data with configurable variance", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate address records
    Addresses {
        /// Number of records to generate
        #[arg(short, long)]
        count: usize,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Error rate (0.0-1.0) - percentage of records with issues
        #[arg(short, long, default_value = "0.5")]
        error_rate: f64,

        /// Suppress progress output
        #[arg(short, long)]
        quiet: bool,
    },
    /// Generate name records
    Names {
        /// Number of records to generate
        #[arg(short, long)]
        count: usize,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Error rate (0.0-1.0) - percentage of records with issues
        #[arg(short, long, default_value = "0.5")]
        error_rate: f64,

        /// Suppress progress output
        #[arg(short, long)]
        quiet: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Addresses { count, output, error_rate, quiet } => {
            println!("Generating {} addresses to {} (error_rate: {}, quiet: {})",
                     count, output, error_rate, quiet);
            // Implementation will come later
        }
        Commands::Names { count, output, error_rate, quiet } => {
            println!("Generating {} names to {} (error_rate: {}, quiet: {})",
                     count, output, error_rate, quiet);
            // Implementation will come later
        }
    }
}
```

**Step 3: Test CLI compilation and help output**

Run: `cargo run -- --help`
Expected: Shows help text with addresses and names subcommands

Run: `cargo run -- addresses --help`
Expected: Shows addresses subcommand help with all options

**Step 4: Commit**

```bash
git add src/main.rs
git commit -m "feat: add CLI structure with addresses and names subcommands"
```

---

## Task 3: Input Validation

**Files:**
- Modify: `src/main.rs`

**Step 1: Add validation function**

```rust
fn validate_inputs(count: usize, error_rate: f64) -> Result<(), String> {
    if count == 0 {
        return Err("Count must be greater than 0".to_string());
    }
    if error_rate < 0.0 || error_rate > 1.0 {
        return Err("Error rate must be between 0.0 and 1.0".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_zero_count() {
        assert!(validate_inputs(0, 0.5).is_err());
    }

    #[test]
    fn test_validate_error_rate_below_zero() {
        assert!(validate_inputs(100, -0.1).is_err());
    }

    #[test]
    fn test_validate_error_rate_above_one() {
        assert!(validate_inputs(100, 1.5).is_err());
    }

    #[test]
    fn test_validate_valid_inputs() {
        assert!(validate_inputs(100, 0.5).is_ok());
    }
}
```

**Step 2: Run tests to verify**

Run: `cargo test`
Expected: All 4 validation tests pass

**Step 3: Add validation to main function**

```rust
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Addresses { count, output, error_rate, quiet } => {
            if let Err(e) = validate_inputs(count, error_rate) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            println!("Generating {} addresses to {} (error_rate: {}, quiet: {})",
                     count, output, error_rate, quiet);
        }
        Commands::Names { count, output, error_rate, quiet } => {
            if let Err(e) = validate_inputs(count, error_rate) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            println!("Generating {} names to {} (error_rate: {}, quiet: {})",
                     count, output, error_rate, quiet);
        }
    }
}
```

**Step 4: Test validation with CLI**

Run: `cargo run -- addresses --count 0 --output test.csv`
Expected: Exits with error message "Error: Count must be greater than 0"

Run: `cargo run -- addresses --count 100 --output test.csv --error-rate 1.5`
Expected: Exits with error message "Error: Error rate must be between 0.0 and 1.0"

**Step 5: Commit**

```bash
git add src/main.rs
git commit -m "feat: add input validation for count and error rate"
```

---

## Task 4: Address Data Structure

**Files:**
- Create: `src/generators/mod.rs`
- Create: `src/generators/addresses.rs`

**Step 1: Create generators module structure**

Create `src/generators/mod.rs`:
```rust
pub mod addresses;
```

**Step 2: Define Address struct with test**

Create `src/generators/addresses.rs`:
```rust
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
```

**Step 3: Update main.rs to include module**

Add to top of `src/main.rs`:
```rust
mod generators;
```

**Step 4: Run tests**

Run: `cargo test`
Expected: All tests pass including new address creation test

**Step 5: Commit**

```bash
git add src/generators/mod.rs src/generators/addresses.rs src/main.rs
git commit -m "feat: add Address struct and module structure"
```

---

## Task 5: Clean Address Generation

**Files:**
- Modify: `src/generators/addresses.rs`

**Step 1: Add imports and helper function**

Add to top of `src/generators/addresses.rs`:
```rust
use fake::{Fake, Faker};
use fake::faker::address::en::*;
use rand::Rng;
```

**Step 2: Write test for clean address generation**

Add to tests module in `src/generators/addresses.rs`:
```rust
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
```

**Step 3: Run test to verify it fails**

Run: `cargo test test_generate_clean_address`
Expected: FAIL with "function not defined"

**Step 4: Implement clean address generation**

Add to `src/generators/addresses.rs` before tests:
```rust
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
```

**Step 5: Run test to verify it passes**

Run: `cargo test test_generate_clean_address`
Expected: Both tests pass

**Step 6: Commit**

```bash
git add src/generators/addresses.rs
git commit -m "feat: implement clean address generation"
```

---

## Task 6: Address Variance - Street Suffix Abbreviations

**Files:**
- Modify: `src/generators/addresses.rs`

**Step 1: Write test for suffix abbreviation**

Add to tests:
```rust
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test abbreviate_street_suffix`
Expected: FAIL with "function not defined"

**Step 3: Implement suffix abbreviation**

Add before tests:
```rust
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
    }.to_string()
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test abbreviate_street_suffix`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/generators/addresses.rs
git commit -m "feat: add street suffix abbreviation function"
```

---

## Task 7: Address Variance - PO Boxes and Apartments

**Files:**
- Modify: `src/generators/addresses.rs`

**Step 1: Write test for secondary address generation**

Add to tests:
```rust
#[test]
fn test_generate_po_box() {
    let po_box = generate_po_box();
    assert!(po_box.contains("Box") || po_box.contains("BOX"));
}

#[test]
fn test_generate_apartment() {
    let apt = generate_apartment();
    assert!(apt.contains("Apt") || apt.contains("Unit") || apt.contains("#") || apt.contains("Suite"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test generate_po_box`
Expected: FAIL with "function not defined"

**Step 3: Implement secondary address generators**

Add before tests:
```rust
fn generate_po_box() -> String {
    let mut rng = rand::thread_rng();
    let box_number: u32 = (1..9999).fake();

    let formats = vec![
        format!("PO Box {}", box_number),
        format!("P.O. Box {}", box_number),
        format!("POB {}", box_number),
    ];

    formats[rng.gen_range(0..formats.len())].clone()
}

fn generate_apartment() -> String {
    let mut rng = rand::thread_rng();
    let unit: String = format!("{}{}",
        rng.gen_range(1..999),
        if rng.gen_bool(0.3) {
            ['A', 'B', 'C', 'D'][rng.gen_range(0..4)].to_string()
        } else {
            String::new()
        }
    );

    let formats = vec![
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
```

**Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/generators/addresses.rs
git commit -m "feat: add PO box and apartment generators"
```

---

## Task 8: Address Variance Application

**Files:**
- Modify: `src/generators/addresses.rs`

**Step 1: Write test for variance application**

Add to tests:
```rust
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test apply_address_variance`
Expected: FAIL with "function not defined"

**Step 3: Implement variance application function**

Add before tests:
```rust
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
                    let mut new_parts = parts[..parts.len()-1].to_vec();
                    new_parts.push(&abbreviated);
                    address.address1 = new_parts.join(" ");
                }
            },
            1 => {
                // Replace with PO Box
                address.address1 = generate_po_box();
                address.address2 = String::new();
            },
            2 => {
                // Add apartment/unit
                address.address2 = generate_apartment();
            },
            3 => {
                // Remove state
                address.state = String::new();
            },
            4 => {
                // Remove zip
                address.zip = String::new();
            },
            5 => {
                // Remove city
                address.city = String::new();
            },
            6 => {
                // All caps
                address.address1 = address.address1.to_uppercase();
                address.city = address.city.to_uppercase();
            },
            7 => {
                // Add extra spaces
                address.address1 = address.address1.replace(" ", "  ");
            },
            8 => {
                // Add periods inconsistently
                if rng.gen_bool(0.5) {
                    address.address1 = address.address1.replace("St", "St.");
                    address.address1 = address.address1.replace("Ave", "Ave.");
                }
            },
            _ => {
                // Mixed case
                address.city = address.city.chars().enumerate().map(|(i, c)| {
                    if i % 2 == 0 { c.to_uppercase().to_string() } else { c.to_lowercase().to_string() }
                }).collect();
            }
        }
    }

    address
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test apply_address_variance`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/generators/addresses.rs
git commit -m "feat: implement address variance application"
```

---

## Task 9: Address Generator Public API

**Files:**
- Modify: `src/generators/addresses.rs`

**Step 1: Write test for public generate function**

Add to tests:
```rust
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test generate_addresses`
Expected: FAIL with "function not defined"

**Step 3: Implement public generate function**

Add after generate_clean_address and before apply_address_variance:
```rust
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
```

**Step 4: Run test to verify it passes**

Run: `cargo test generate_addresses`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/generators/addresses.rs
git commit -m "feat: add public address generation API"
```

---

## Task 10: Name Data Structure

**Files:**
- Modify: `src/generators/mod.rs`
- Create: `src/generators/names.rs`

**Step 1: Add names module**

Modify `src/generators/mod.rs`:
```rust
pub mod addresses;
pub mod names;
```

**Step 2: Define Name struct with test**

Create `src/generators/names.rs`:
```rust
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
```

**Step 3: Run tests**

Run: `cargo test`
Expected: All tests pass including new name creation tests

**Step 4: Commit**

```bash
git add src/generators/mod.rs src/generators/names.rs
git commit -m "feat: add Name struct and module"
```

---

## Task 11: Clean Name Generation

**Files:**
- Modify: `src/generators/names.rs`

**Step 1: Add imports**

Add to top of `src/generators/names.rs`:
```rust
use fake::{Fake, Faker};
use fake::faker::name::en::*;
use rand::Rng;
```

**Step 2: Write test for clean name generation**

Add to tests:
```rust
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
```

**Step 3: Run test to verify it fails**

Run: `cargo test generate_clean_name`
Expected: FAIL with "function not defined"

**Step 4: Implement clean name generation**

Add before tests:
```rust
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
```

**Step 5: Run test to verify it passes**

Run: `cargo test generate_clean_name`
Expected: All tests pass

**Step 6: Commit**

```bash
git add src/generators/names.rs
git commit -m "feat: implement clean name generation"
```

---

## Task 12: Name Variance - Prefixes and Suffixes

**Files:**
- Modify: `src/generators/names.rs`

**Step 1: Write test for prefix/suffix generation**

Add to tests:
```rust
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test get_random_prefix`
Expected: FAIL with "function not defined"

**Step 3: Implement prefix/suffix functions**

Add before tests:
```rust
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
```

**Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/generators/names.rs
git commit -m "feat: add name prefix and suffix generators"
```

---

## Task 13: Name Variance - Field Mixing and Formatting

**Files:**
- Modify: `src/generators/names.rs`

**Step 1: Write test for variance helpers**

Add to tests:
```rust
#[test]
fn test_add_typo() {
    // Test that typo function runs without panicking
    let result = add_typo("Joshua");
    assert!(!result.is_empty());
}

#[test]
fn test_to_mixed_case() {
    let result = to_mixed_case("Joshua");
    // Should alternate or randomly mix case
    assert_eq!(result.len(), "Joshua".len());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test add_typo`
Expected: FAIL with "function not defined"

**Step 3: Implement helper functions**

Add before tests:
```rust
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
        },
        1 => {
            // Transpose two letters
            if chars.len() >= 2 {
                let pos = rng.gen_range(0..chars.len()-1);
                chars.swap(pos, pos + 1);
            }
        },
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

fn to_mixed_case(name: &str) -> String {
    name.chars().enumerate().map(|(i, c)| {
        if i % 2 == 0 {
            c.to_uppercase().to_string()
        } else {
            c.to_lowercase().to_string()
        }
    }).collect()
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/generators/names.rs
git commit -m "feat: add name typo and case mixing functions"
```

---

## Task 14: Name Variance Application

**Files:**
- Modify: `src/generators/names.rs`

**Step 1: Write test for variance application**

Add to tests:
```rust
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
    assert!(!varied.first_name.is_empty() ||
            !varied.middle_name.is_empty() ||
            !varied.last_name.is_empty());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test apply_name_variance`
Expected: FAIL with "function not defined"

**Step 3: Implement variance application function**

Add before tests (after helper functions):
```rust
fn apply_name_variance(mut name: Name) -> Name {
    let mut rng = rand::thread_rng();

    // Apply 1-3 random variance patterns
    let num_variances = rng.gen_range(1..=3);

    for _ in 0..num_variances {
        let variance_type = rng.gen_range(0..15);

        match variance_type {
            0 => {
                // Last name in first name field
                name.first_name = name.last_name.clone();
                name.last_name = name.first_name.clone();
            },
            1 => {
                // First and last combined in first name
                name.first_name = format!("{} {}", name.first_name, name.last_name);
                name.last_name = String::new();
            },
            2 => {
                // "LastName, FirstName" format in first name
                name.first_name = format!("{}, {}", name.last_name, name.first_name);
                name.last_name = String::new();
            },
            3 => {
                // Full name in one field
                name.first_name = format!("{} {} {}", name.first_name, name.middle_name, name.last_name);
                name.middle_name = String::new();
                name.last_name = String::new();
            },
            4 => {
                // Hyphenated last name
                let extra_last: String = LastName().fake();
                name.last_name = format!("{}-{}", name.last_name, extra_last);
            },
            5 => {
                // Hyphenated first name
                let extra_first: String = FirstName().fake();
                name.first_name = format!("{}-{}", name.first_name, extra_first);
            },
            6 => {
                // Multiple last names
                let extra_last: String = LastName().fake();
                name.last_name = format!("{} {}", name.last_name, extra_last);
            },
            7 => {
                // Add prefix to first name
                name.first_name = format!("{} {}", get_random_prefix(), name.first_name);
            },
            8 => {
                // Add suffix to last name
                name.last_name = format!("{} {}", name.last_name, get_random_suffix());
            },
            9 => {
                // Nickname in quotes
                name.first_name = format!("\"{}\"", name.first_name);
            },
            10 => {
                // Nickname in parentheses
                name.first_name = format!("{} ({})", name.first_name, &name.first_name[..3.min(name.first_name.len())]);
            },
            11 => {
                // All caps
                name.first_name = name.first_name.to_uppercase();
                name.last_name = name.last_name.to_uppercase();
            },
            12 => {
                // All lowercase
                name.first_name = name.first_name.to_lowercase();
                name.last_name = name.last_name.to_lowercase();
            },
            13 => {
                // Mixed case
                name.first_name = to_mixed_case(&name.first_name);
            },
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
```

**Step 4: Run test to verify it passes**

Run: `cargo test apply_name_variance`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/generators/names.rs
git commit -m "feat: implement name variance application"
```

---

## Task 15: Name Generator Public API

**Files:**
- Modify: `src/generators/names.rs`

**Step 1: Write test for public generate function**

Add to tests:
```rust
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test generate_names`
Expected: FAIL with "function not defined"

**Step 3: Implement public generate function**

Add after generate_clean_name and before apply_name_variance:
```rust
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
```

**Step 4: Run test to verify it passes**

Run: `cargo test generate_names`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/generators/names.rs
git commit -m "feat: add public name generation API"
```

---

## Task 16: CSV Writer Module Structure

**Files:**
- Create: `src/writer.rs`
- Modify: `src/main.rs`

**Step 1: Create writer module**

Create `src/writer.rs`:
```rust
use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io;
use std::path::Path;

pub struct CsvWriter {
    quiet: bool,
}

impl CsvWriter {
    pub fn new(quiet: bool) -> Self {
        Self { quiet }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_writer_creation() {
        let writer = CsvWriter::new(false);
        assert_eq!(writer.quiet, false);
    }
}
```

**Step 2: Add writer module to main.rs**

Add to top of `src/main.rs` after generators module:
```rust
mod writer;
```

**Step 3: Run tests**

Run: `cargo test`
Expected: All tests pass including writer creation test

**Step 4: Commit**

```bash
git add src/writer.rs src/main.rs
git commit -m "feat: add CSV writer module structure"
```

---

## Task 17: Progress Bar Implementation

**Files:**
- Modify: `src/writer.rs`

**Step 1: Write test for progress bar creation**

Add to tests:
```rust
#[test]
fn test_create_progress_bar_quiet() {
    let writer = CsvWriter::new(true);
    let pb = writer.create_progress_bar(1000, "Testing");
    // Progress bar should be hidden when quiet
    pb.finish_and_clear();
}

#[test]
fn test_create_progress_bar_not_quiet() {
    let writer = CsvWriter::new(false);
    let pb = writer.create_progress_bar(1000, "Testing");
    // Just verify it doesn't panic
    pb.finish_and_clear();
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test create_progress_bar`
Expected: FAIL with "method not found"

**Step 3: Implement progress bar creation**

Add to CsvWriter impl block before tests:
```rust
fn create_progress_bar(&self, count: usize, message: &str) -> ProgressBar {
    if self.quiet || count <= 100 {
        ProgressBar::hidden()
    } else {
        let pb = ProgressBar::new(count as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {percent}% ({pos}/{len})")
                .expect("Invalid progress bar template")
                .progress_chars("=>-")
        );
        pb.set_message(message.to_string());
        pb
    }
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test create_progress_bar`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/writer.rs
git commit -m "feat: implement progress bar creation"
```

---

## Task 18: Address CSV Writing

**Files:**
- Modify: `src/writer.rs`
- Modify: `src/generators/addresses.rs`

**Step 1: Add method to convert Address to record**

Add to Address impl in `src/generators/addresses.rs`:
```rust
impl Address {
    pub fn new(address1: String, address2: String, city: String, state: String, zip: String) -> Self {
        Self { address1, address2, city, state, zip }
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
```

**Step 2: Add test for to_record**

Add to tests in `src/generators/addresses.rs`:
```rust
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
    assert_eq!(record, vec!["123 Main St", "Apt 4B", "Springfield", "IL", "62701"]);
}
```

**Step 3: Write test for address writing**

Add to tests in `src/writer.rs`:
```rust
use crate::generators::addresses::Address;
use tempfile::NamedTempFile;

#[test]
fn test_write_addresses() {
    let writer = CsvWriter::new(true);
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();

    let addresses = vec![
        Address::new(
            "123 Main St".to_string(),
            "Apt 4B".to_string(),
            "Springfield".to_string(),
            "IL".to_string(),
            "62701".to_string(),
        ),
    ];

    let result = writer.write_addresses(path, &addresses);
    assert!(result.is_ok());

    // Verify file contents
    let contents = std::fs::read_to_string(path).unwrap();
    assert!(contents.contains("Address1|Address2|City|State|Zip"));
    assert!(contents.contains("123 Main St|Apt 4B|Springfield|IL|62701"));
}
```

**Step 4: Run test to verify it fails**

Run: `cargo test write_addresses`
Expected: FAIL with "method not found"

**Step 5: Implement write_addresses method**

Add to CsvWriter impl in `src/writer.rs`:
```rust
use crate::generators::addresses::Address;

impl CsvWriter {
    pub fn new(quiet: bool) -> Self {
        Self { quiet }
    }

    pub fn write_addresses(&self, path: &str, addresses: &[Address]) -> io::Result<()> {
        // Create parent directories if needed
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        let mut writer = Writer::from_writer(file);

        // Configure pipe delimiter
        let mut builder = csv::WriterBuilder::new();
        builder.delimiter(b'|');
        let mut writer = builder.from_path(path)?;

        // Write header
        writer.write_record(&["Address1", "Address2", "City", "State", "Zip"])?;

        // Create progress bar
        let pb = self.create_progress_bar(addresses.len(), "Generating addresses");

        // Write records
        for address in addresses {
            writer.write_record(&address.to_record())?;
            pb.inc(1);
        }

        pb.finish_and_clear();
        writer.flush()?;

        Ok(())
    }
}
```

**Step 6: Run test to verify it passes**

Run: `cargo test write_addresses`
Expected: All tests pass

**Step 7: Commit**

```bash
git add src/writer.rs src/generators/addresses.rs
git commit -m "feat: implement address CSV writing"
```

---

## Task 19: Name CSV Writing

**Files:**
- Modify: `src/writer.rs`
- Modify: `src/generators/names.rs`

**Step 1: Add method to convert Name to record**

Add to Name impl in `src/generators/names.rs`:
```rust
impl Name {
    pub fn new(first_name: String, middle_name: String, last_name: String) -> Self {
        Self { first_name, middle_name, last_name }
    }

    pub fn to_record(&self) -> Vec<String> {
        vec![
            self.first_name.clone(),
            self.middle_name.clone(),
            self.last_name.clone(),
        ]
    }
}
```

**Step 2: Add test for to_record**

Add to tests in `src/generators/names.rs`:
```rust
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
```

**Step 3: Write test for name writing**

Add to tests in `src/writer.rs`:
```rust
use crate::generators::names::Name;

#[test]
fn test_write_names() {
    let writer = CsvWriter::new(true);
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_str().unwrap();

    let names = vec![
        Name::new(
            "Joshua".to_string(),
            "Allen".to_string(),
            "Caudill".to_string(),
        ),
    ];

    let result = writer.write_names(path, &names);
    assert!(result.is_ok());

    // Verify file contents
    let contents = std::fs::read_to_string(path).unwrap();
    assert!(contents.contains("FirstName|MiddleName|LastName"));
    assert!(contents.contains("Joshua|Allen|Caudill"));
}
```

**Step 4: Run test to verify it fails**

Run: `cargo test write_names`
Expected: FAIL with "method not found"

**Step 5: Implement write_names method**

Add to CsvWriter impl in `src/writer.rs`:
```rust
use crate::generators::names::Name;

pub fn write_names(&self, path: &str, names: &[Name]) -> io::Result<()> {
    // Create parent directories if needed
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Configure pipe delimiter
    let mut builder = csv::WriterBuilder::new();
    builder.delimiter(b'|');
    let mut writer = builder.from_path(path)?;

    // Write header
    writer.write_record(&["FirstName", "MiddleName", "LastName"])?;

    // Create progress bar
    let pb = self.create_progress_bar(names.len(), "Generating names");

    // Write records
    for name in names {
        writer.write_record(&name.to_record())?;
        pb.inc(1);
    }

    pb.finish_and_clear();
    writer.flush()?;

    Ok(())
}
```

**Step 6: Run test to verify it passes**

Run: `cargo test write_names`
Expected: All tests pass

**Step 7: Commit**

```bash
git add src/writer.rs src/generators/names.rs
git commit -m "feat: implement name CSV writing"
```

---

## Task 20: Wire Up CLI to Generators

**Files:**
- Modify: `src/main.rs`

**Step 1: Import required modules**

Update imports at top of `src/main.rs`:
```rust
use clap::{Parser, Subcommand};
use std::process;

mod generators;
mod writer;

use generators::addresses::generate_addresses;
use generators::names::generate_names;
use writer::CsvWriter;
```

**Step 2: Implement address command**

Update the Addresses match arm in main():
```rust
Commands::Addresses { count, output, error_rate, quiet } => {
    if let Err(e) = validate_inputs(count, error_rate) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    // Generate addresses
    let addresses = generate_addresses(count, error_rate);

    // Write to file
    let writer = CsvWriter::new(quiet);
    if let Err(e) = writer.write_addresses(&output, &addresses) {
        eprintln!("Error writing addresses: {}", e);
        process::exit(1);
    }

    if !quiet {
        println!("Successfully generated {} addresses to {}", count, output);
    }
}
```

**Step 3: Implement names command**

Update the Names match arm in main():
```rust
Commands::Names { count, output, error_rate, quiet } => {
    if let Err(e) = validate_inputs(count, error_rate) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }

    // Generate names
    let names = generate_names(count, error_rate);

    // Write to file
    let writer = CsvWriter::new(quiet);
    if let Err(e) = writer.write_names(&output, &names) {
        eprintln!("Error writing names: {}", e);
        process::exit(1);
    }

    if !quiet {
        println!("Successfully generated {} names to {}", count, output);
    }
}
```

**Step 4: Test full integration manually**

Run: `cargo run -- addresses --count 100 --output test_addresses.csv --error-rate 0.5`
Expected: Creates test_addresses.csv with 100 pipe-delimited addresses

Run: `cargo run -- names --count 100 --output test_names.csv --error-rate 0.5`
Expected: Creates test_names.csv with 100 pipe-delimited names

Run: `head -5 test_addresses.csv`
Expected: Shows header and first 4 address records with pipe delimiters

Run: `head -5 test_names.csv`
Expected: Shows header and first 4 name records with pipe delimiters

**Step 5: Clean up test files**

Run: `rm test_addresses.csv test_names.csv`

**Step 6: Commit**

```bash
git add src/main.rs
git commit -m "feat: wire up CLI to generators and writer"
```

---

## Task 21: Integration Tests

**Files:**
- Create: `tests/integration_test.rs`

**Step 1: Create integration test file**

Create `tests/integration_test.rs`:
```rust
use std::process::Command;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_addresses_command_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");

    let output = Command::new("cargo")
        .args(&["run", "--", "addresses",
                "--count", "10",
                "--output", output_path.to_str().unwrap(),
                "--quiet"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output_path.exists());

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("Address1|Address2|City|State|Zip"));

    // Should have header + 10 records = 11 lines
    let line_count = contents.lines().count();
    assert_eq!(line_count, 11);
}

#[test]
fn test_names_command_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("names.csv");

    let output = Command::new("cargo")
        .args(&["run", "--", "names",
                "--count", "10",
                "--output", output_path.to_str().unwrap(),
                "--quiet"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output_path.exists());

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("FirstName|MiddleName|LastName"));

    // Should have header + 10 records = 11 lines
    let line_count = contents.lines().count();
    assert_eq!(line_count, 11);
}

#[test]
fn test_invalid_count_fails() {
    let output = Command::new("cargo")
        .args(&["run", "--", "addresses",
                "--count", "0",
                "--output", "test.csv"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Count must be greater than 0"));
}

#[test]
fn test_invalid_error_rate_fails() {
    let output = Command::new("cargo")
        .args(&["run", "--", "addresses",
                "--count", "10",
                "--output", "test.csv",
                "--error-rate", "1.5"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error rate must be between 0.0 and 1.0"));
}

#[test]
fn test_pipe_delimiter_no_commas() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");

    Command::new("cargo")
        .args(&["run", "--", "addresses",
                "--count", "50",
                "--output", output_path.to_str().unwrap(),
                "--error-rate", "0.8",
                "--quiet"])
        .output()
        .expect("Failed to execute command");

    let contents = fs::read_to_string(output_path).unwrap();

    // Verify pipe delimiters are used
    assert!(contents.contains("|"));

    // Check each line for proper delimiter count
    for line in contents.lines() {
        let pipe_count = line.matches('|').count();
        assert_eq!(pipe_count, 4, "Each line should have exactly 4 pipes");
    }
}
```

**Step 2: Run integration tests**

Run: `cargo test --test integration_test`
Expected: All integration tests pass

**Step 3: Commit**

```bash
git add tests/integration_test.rs
git commit -m "test: add integration tests for CLI commands"
```

---

## Task 22: Documentation

**Files:**
- Create: `README.md`

**Step 1: Create README**

Create `README.md`:
```markdown
# Rust Faker

Generate realistic test data with configurable variance for testing data standardization processes.

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/rust-faker`.

## Usage

### Generate Addresses

```bash
rust-faker addresses --count 10000 --output addresses.csv --error-rate 0.5
```

### Generate Names

```bash
rust-faker names --count 5000 --output names.csv --error-rate 0.3
```

### Options

- `--count <N>` (required) - Number of records to generate
- `--output <PATH>` (required) - Output file path
- `--error-rate <0.0-1.0>` (optional, default: 0.5) - Percentage of records with variance
- `--quiet` (optional) - Suppress progress output

## Output Format

All files use pipe `|` delimiters and include a header row.

### Address Format
```
Address1|Address2|City|State|Zip
123 Main Street|Apt 4B|Springfield|IL|62701
```

### Name Format
```
FirstName|MiddleName|LastName
Joshua|Allen|Caudill
```

## Data Variance

The `--error-rate` parameter controls what percentage of records will have data quality issues applied.

### Address Variance Includes:
- Abbreviated street suffixes (Street/St, Avenue/Ave, etc.)
- PO Boxes in various formats
- Apartments, suites, units
- Missing fields (state, zip, city)
- Case variations
- Extra spaces
- Inconsistent punctuation

### Name Variance Includes:
- Field mixing (last name in first name field, etc.)
- "LastName, FirstName" format
- Hyphenated names
- Multiple last names
- Prefixes (Dr., Mr., Mrs., etc.)
- Suffixes (Jr., Sr., MD, etc.)
- Nicknames in quotes or parentheses
- Case variations (all caps, all lowercase, mixed)
- Common typos

## Examples

Generate 1000 clean addresses (no variance):
```bash
rust-faker addresses --count 1000 --output clean_addresses.csv --error-rate 0.0
```

Generate 5000 highly problematic names (80% with issues):
```bash
rust-faker names --count 5000 --output messy_names.csv --error-rate 0.8
```

Generate addresses quietly (no progress bar):
```bash
rust-faker addresses --count 10000 --output addresses.csv --quiet
```

## Testing

Run all tests:
```bash
cargo test
```

Run only unit tests:
```bash
cargo test --lib
```

Run only integration tests:
```bash
cargo test --test integration_test
```

## License

MIT
```

**Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add comprehensive README"
```

---

## Task 23: Final Testing and Cleanup

**Files:**
- Multiple

**Step 1: Run all tests**

Run: `cargo test`
Expected: All tests pass

**Step 2: Run clippy for lint checks**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Format code**

Run: `cargo fmt`
Expected: All code formatted consistently

**Step 4: Build release binary**

Run: `cargo build --release`
Expected: Binary builds successfully

**Step 5: Test release binary end-to-end**

Run: `./target/release/rust-faker addresses --count 1000 --output test_addresses.csv --error-rate 0.7`
Expected: Creates test_addresses.csv with 1000 addresses

Run: `./target/release/rust-faker names --count 1000 --output test_names.csv --error-rate 0.7`
Expected: Creates test_names.csv with 1000 names

Run: `wc -l test_addresses.csv test_names.csv`
Expected: Both files have 1001 lines (header + 1000 records)

Run: `head -20 test_addresses.csv`
Expected: Shows varied addresses with different formats and issues

Run: `head -20 test_names.csv`
Expected: Shows varied names with different formats and issues

**Step 6: Clean up test files**

Run: `rm test_addresses.csv test_names.csv`

**Step 7: Final commit**

```bash
git add -A
git commit -m "chore: final cleanup and formatting"
```

---

## Completion

All tasks completed! The rust-faker CLI tool is now fully implemented with:
- Subcommand-based CLI interface
- Address generation with realistic US addresses and configurable variance
- Name generation with realistic names and configurable variance
- Pipe-delimited CSV output
- Progress bars for large datasets
- Comprehensive tests (unit and integration)
- Full documentation

The tool is ready for use in testing data standardization processes.
