# Rust Faker - Test Data Generator Design

**Date:** 2026-01-16
**Purpose:** Generate realistic test data with configurable variance for testing data standardization processes

## Overview

A Rust CLI application that generates pipe-delimited CSV files containing addresses or names with configurable data quality issues. Used to test the robustness and auto-correction capabilities of standardization systems.

## Architecture

### Project Structure
```
rust-faker/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI setup and entry point
│   ├── generators/
│   │   ├── mod.rs
│   │   ├── addresses.rs  # Address generation logic
│   │   └── names.rs      # Name generation logic
│   └── writer.rs         # CSV writing with progress bars
```

### Dependencies
- `clap` (v4) with derive feature - CLI argument parsing
- `fake` (v2.9) with derive feature - realistic fake data generation
- `rand` (v0.8) - controlling randomness and error rates
- `indicatif` (v0.17) - progress bars
- `csv` (v1.3) - CSV writing configured for pipe delimiters

### Design Pattern
Each record type implements a generator module with a `generate()` function that takes count and error rate, returning a Vec of records. A separate writer module handles file output with progress tracking. This keeps data generation separate from I/O concerns.

## CLI Interface

### Command Structure
```bash
# Generate addresses
rust-faker addresses --count 10000 --output addresses.csv --error-rate 0.5

# Generate names
rust-faker names --count 5000 --output names.csv --error-rate 0.3

# With quiet mode
rust-faker addresses --count 10000 --output addresses.csv --error-rate 0.5 --quiet
```

### Arguments
**Shared Arguments:**
- `--count <N>` (required) - number of records to generate
- `--output <PATH>` (required) - output file path
- `--error-rate <0.0-1.0>` (optional, default: 0.5) - percentage of records with issues
- `--quiet` (optional flag) - suppress progress output

### Error Rate Behavior
The error rate controls what percentage of records will have issues. With `--error-rate 0.8`, 80% of records will have some variance (abbreviations, missing fields, garbled data, etc.) and 20% will be clean. The specific issues applied will be randomly selected from the available variance patterns.

### File Output
All files use pipe `|` delimiters and include a header row by default. The writer will create parent directories if they don't exist.

## Address Generation

### Record Structure
```
Address1|Address2|City|State|Zip
123 Main Street|Apt 4B|Springfield|IL|62701
```

### Clean Address Generation
Uses the `fake` crate to generate:
- Street address (number + street name + suffix)
- Optional secondary address (Apt, Suite, Unit, etc.)
- Real US city names
- Real US state codes
- Real US ZIP codes (5-digit format)

### Variance Patterns

**Street Suffixes:**
Mix full and abbreviated forms: Street/St, Avenue/Ave, Road/Rd, Boulevard/Blvd, Drive/Dr, Lane/Ln, Parkway/Pkwy, Court/Ct, etc.

**Secondary Addresses:**
- PO Box formats: "PO Box 123", "P.O. Box 123", "POB 123"
- Apartments: "Apt 4B", "Apartment 4B", "#4B", "Unit 4B"
- Suites: "Suite 200", "Ste 200", "Ste. 200"

**Missing Data:**
- Some records missing Address2 (common)
- Some missing State
- Some missing Zip
- Some missing City

**Data Quality Issues:**
- Extra spaces
- All caps or mixed case inconsistently
- Periods scattered randomly (St. vs St)
- Misspellings in street names

## Name Generation

### Record Structure
```
FirstName|MiddleName|LastName
Joshua|Allen|Caudill
```

### Clean Name Generation
Uses the `fake` crate to generate:
- FirstName: Common US first names
- MiddleName: Optional (50% of records will have empty middle name even in "clean" data)
- LastName: Common US last names

### Variance Patterns

**Field Swapping/Mixing:**
- Last name in FirstName field: "Caudill||Joshua"
- First and last combined in FirstName: "Joshua Caudill||"
- "LastName, FirstName" format in FirstName: "Caudill, Joshua||"
- Full name in one field: "Joshua Allen Caudill||"

**Hyphenation & Multiple Names:**
- Hyphenated last names: "||Smith-Jones"
- Hyphenated first names: "Mary-Ann||"
- Multiple last names: "||Garcia Lopez"

**Prefixes & Suffixes:**
- Prefixes: Dr., Mr., Mrs., Ms., Prof., Rev.
- Suffixes: Jr., Sr., II, III, IV, MD, PhD, Esq.
- Can appear in any field

**Nicknames:**
- Quotes: "\"Josh\"||Caudill"
- Parentheses: "Joshua (Josh)||Caudill"

**Data Quality Issues:**
- All caps: "JOSHUA||CAUDILL"
- All lowercase: "joshua||caudill"
- Mixed case: "JoShUa||"
- Special characters: O'Brien, St. James, José
- Common typos/misspellings: doubled letters, transposed letters
- Extra spaces, leading/trailing spaces

## Implementation Details

### Random Distribution Logic
When error_rate is 0.5:
1. For each record, roll a random number 0.0-1.0
2. If roll < error_rate, apply 1-3 random variance patterns to that record
3. Otherwise generate a clean record (except MiddleName still optional)

### Error Handling
- Validate error-rate is between 0.0 and 1.0
- Validate count is positive integer
- Handle file write errors (permissions, disk space)
- Create parent directories if they don't exist
- Provide clear error messages for all failures

### Progress Bar
- Shows: `Generating addresses: [=====>    ] 50% (5000/10000)`
- Updates every 100 records (or 1% of total, whichever is larger)
- Only displays for count > 100
- Suppressed with `--quiet` flag

### File Writing
- Use `csv` crate with custom delimiter (`|`)
- Write header row first
- Stream records to avoid memory issues with large datasets
- Flush and close file properly

### Testing Strategy
- Unit tests for each variance pattern
- Integration tests for full file generation
- Test edge cases (error_rate 0.0, 1.0, small/large counts)
- Verify no commas or quotes in output
