# rust-faker

A Rust CLI tool for generating test data with configurable variance. This tool generates realistic address and name datasets with intentional formatting variations, perfect for testing data standardization and deduplication systems.

## Features

- Generate address records with realistic US addresses
- Generate name records with various formatting patterns
- Configurable variance/error rates (0.0 to 1.0)
- Pipe-delimited CSV output
- Progress bar for large datasets
- Creates output directories automatically

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-faker
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/rust-faker`

4. Optionally, install it to your system:
```bash
cargo install --path .
```

## Usage

### Generate Addresses

Generate address records with the `addresses` command:

```bash
rust-faker addresses --count 1000 --output addresses.csv --error-rate 0.5
```

**Options:**
- `-c, --count <COUNT>` - Number of records to generate (required)
- `-o, --output <OUTPUT>` - Output file path (required)
- `-e, --error-rate <ERROR_RATE>` - Error rate between 0.0 and 1.0 (default: 0.5)
- `-q, --quiet` - Suppress progress output

**Example:**
```bash
# Generate 500 clean addresses (no variance)
rust-faker addresses -c 500 -o clean_addresses.csv -e 0.0

# Generate 1000 addresses with 30% variance
rust-faker addresses -c 1000 -o varied_addresses.csv -e 0.3

# Generate 5000 addresses quietly
rust-faker addresses -c 5000 -o addresses.csv -e 0.5 -q
```

### Generate Names

Generate name records with the `names` command:

```bash
rust-faker names --count 1000 --output names.csv --error-rate 0.5
```

**Options:**
- `-c, --count <COUNT>` - Number of records to generate (required)
- `-o, --output <OUTPUT>` - Output file path (required)
- `-e, --error-rate <ERROR_RATE>` - Error rate between 0.0 and 1.0 (default: 0.5)
- `-q, --quiet` - Suppress progress output

**Example:**
```bash
# Generate 500 clean names (no variance)
rust-faker names -c 500 -o clean_names.csv -e 0.0

# Generate 1000 names with 70% variance
rust-faker names -c 1000 -o varied_names.csv -e 0.7

# Generate 5000 names quietly
rust-faker names -c 5000 -o names.csv -e 0.5 -q
```

## Output Format

### Address Output

The address CSV file uses pipe (`|`) as the delimiter with the following columns:

```
Address1|Address2|City|State|Zip
123 Main St|Apt 4B|Springfield|IL|62701
456 Oak Ave||Chicago|IL|60601
```

**Columns:**
- `Address1` - Primary street address (may include street number, name, and suffix)
- `Address2` - Secondary address (apartment, suite, etc.) - may be empty
- `City` - City name
- `State` - Two-letter state abbreviation
- `Zip` - ZIP code

### Name Output

The name CSV file uses pipe (`|`) as the delimiter with the following columns:

```
FirstName|MiddleName|LastName
Joshua|Allen|Caudill
John||Doe
```

**Columns:**
- `FirstName` - First name (may include prefixes, nicknames, or full names with variance)
- `MiddleName` - Middle name - may be empty
- `LastName` - Last name (may include suffixes or compound names)

## Data Variance

The `error-rate` parameter controls how much variance is introduced into the data:

- **0.0** - Clean data with no variance (all records properly formatted)
- **0.5** - 50% of records have variance applied (default)
- **1.0** - All records have variance applied

### Address Variance Patterns

When variance is applied to addresses, 1-3 random patterns may be introduced:

1. **Street suffix abbreviations** - "Street" becomes "St", "Avenue" becomes "Ave"
2. **PO Boxes** - Replace street address with "PO Box 1234" or "P.O. Box 1234"
3. **Apartment formats** - Various formats like "Apt 5", "Unit 5", "#5", "Suite 5"
4. **Missing fields** - State, ZIP, or city may be empty
5. **Case variations** - ALL CAPS or MiXeD CaSe
6. **Extra spaces** - Multiple spaces between words
7. **Inconsistent periods** - "St." vs "St"

**Example:**
```
123 Main St|Apt 4B|Springfield|IL|62701
456 OAK AVE||CHICAGO|IL|60601
PO Box 789||Boston|MA|02101
123 ELM  STREET|Unit 12|Miami||33101
```

### Name Variance Patterns

When variance is applied to names, 1-3 random patterns may be introduced:

1. **Field swapping** - First and last names swapped
2. **Combined fields** - Full name in one field: "John Michael Doe"
3. **Comma format** - "Doe, John" format
4. **Hyphenated names** - "Mary-Jane" or "Smith-Johnson"
5. **Multiple last names** - "Garcia Lopez"
6. **Prefixes** - "Dr.", "Mr.", "Mrs.", "Ms.", "Prof.", "Rev."
7. **Suffixes** - "Jr.", "Sr.", "II", "III", "IV", "MD", "PhD", "Esq."
8. **Nicknames** - "\"Bob\"" or "Robert (Bob)"
9. **Case variations** - ALL CAPS, lowercase, or MiXeD CaSe
10. **Typos** - Realistic typing errors (doubled letters, transpositions, missing letters)

**Example:**
```
Joshua|Allen|Caudill
JOHN||DOE
Doe, Mary||
Dr. Robert||Smith
Jane||Johnson-Williams
michael||o'brien
JoHn|RoBeRt|DoE
```

## Testing

### Run Unit Tests

```bash
cargo test
```

This runs all unit tests for the generators and writer modules.

### Run Integration Tests

```bash
cargo test --test integration_test
```

This runs end-to-end tests of the CLI commands.

### Run All Tests

```bash
cargo test
```

### Check Code Quality

```bash
# Run clippy for linting
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

## Performance

- Progress bar is shown for datasets larger than 100 records (unless `--quiet` is used)
- Generates approximately 10,000-50,000 records per second (depending on system)
- Memory-efficient streaming writes to CSV files
- Supports generating millions of records

## Technical Details

### Dependencies

- **clap** (v4.5) - Command-line argument parsing
- **fake** (v2.9) - Fake data generation
- **rand** (v0.8) - Random number generation
- **csv** (v1.3) - CSV file writing
- **indicatif** (v0.17) - Progress bars

### Project Structure

```
rust-faker/
├── src/
│   ├── main.rs           # CLI entry point and command handling
│   ├── writer.rs         # CSV writing with progress bars
│   └── generators/
│       ├── mod.rs        # Generator module exports
│       ├── addresses.rs  # Address generation and variance
│       └── names.rs      # Name generation and variance
├── tests/
│   └── integration_test.rs  # Integration tests
├── Cargo.toml            # Project dependencies
└── README.md             # This file
```

## Use Cases

This tool is designed for:

- Testing data standardization systems
- Training machine learning models for data cleaning
- Benchmarking deduplication algorithms
- Creating realistic test datasets for databases
- Stress testing data processing pipelines
- Demonstrating data quality issues

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Examples

### Generate Multiple Datasets

```bash
# Generate different error rates for testing
rust-faker addresses -c 10000 -o addresses_clean.csv -e 0.0
rust-faker addresses -c 10000 -o addresses_low.csv -e 0.2
rust-faker addresses -c 10000 -o addresses_medium.csv -e 0.5
rust-faker addresses -c 10000 -o addresses_high.csv -e 0.8
rust-faker addresses -c 10000 -o addresses_full.csv -e 1.0

# Generate matching name datasets
rust-faker names -c 10000 -o names_clean.csv -e 0.0
rust-faker names -c 10000 -o names_varied.csv -e 0.5
```

### Subdirectories

The tool automatically creates directories as needed:

```bash
# Creates data/ directory if it doesn't exist
rust-faker addresses -c 1000 -o data/addresses.csv

# Creates nested directories
rust-faker names -c 1000 -o output/test/names.csv
```
