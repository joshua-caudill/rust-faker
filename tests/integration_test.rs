use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to get the path to the compiled binary
fn get_binary_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug_path = format!("{}/target/debug/rust-faker", manifest_dir);

    // Check if binary exists, if not provide helpful message
    if !std::path::Path::new(&debug_path).exists() {
        panic!(
            "Binary not found at {}. Run 'cargo build' first.",
            debug_path
        );
    }

    debug_path
}

/// Helper function to read and return file contents
fn read_file_contents(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

/// Helper function to create a test CSV file with standard columns
fn create_test_csv(dir: &TempDir, filename: &str, content: &str) -> String {
    let path = dir.path().join(filename);
    let mut file = File::create(&path).expect("Failed to create test CSV");
    file.write_all(content.as_bytes())
        .expect("Failed to write test CSV");
    path.to_str().unwrap().to_string()
}

#[test]
fn test_addresses_command_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count",
            "10",
            "--output",
            output_str,
            "--error-rate",
            "0.5",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_path.exists(), "Output file was not created");

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // Should have header + 10 records = 11 lines
    assert_eq!(lines.len(), 11, "Expected 11 lines (header + 10 records)");
}

#[test]
fn test_addresses_correct_format() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count",
            "5",
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // Check header
    assert_eq!(lines[0], "Address1|Address2|City|State|Zip");

    // Check each record has 5 fields (some may be empty)
    for (i, line) in lines.iter().skip(1).enumerate() {
        let fields: Vec<&str> = line.split('|').collect();
        assert_eq!(
            fields.len(),
            5,
            "Line {} should have 5 fields: {}",
            i + 2,
            line
        );
    }
}

#[test]
fn test_addresses_pipe_delimiter() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count",
            "3",
            "--output",
            output_str,
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let contents = read_file_contents(output_str);

    // Verify pipe delimiter is used
    assert!(contents.contains('|'), "Output should use pipe delimiter");
    assert!(
        !contents.contains(','),
        "Output should not use comma delimiter"
    );
}

#[test]
fn test_names_command_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "names",
            "--count",
            "10",
            "--output",
            output_str,
            "--error-rate",
            "0.5",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_path.exists(), "Output file was not created");

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // Should have header + 10 records = 11 lines
    assert_eq!(lines.len(), 11, "Expected 11 lines (header + 10 records)");
}

#[test]
fn test_names_correct_format() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "names",
            "--count",
            "5",
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // Check header
    assert_eq!(lines[0], "FirstName|MiddleName|LastName");

    // Check each record has 3 fields (some may be empty)
    for (i, line) in lines.iter().skip(1).enumerate() {
        let fields: Vec<&str> = line.split('|').collect();
        assert_eq!(
            fields.len(),
            3,
            "Line {} should have 3 fields: {}",
            i + 2,
            line
        );
    }
}

#[test]
fn test_names_pipe_delimiter() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "names",
            "--count",
            "3",
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // Verify pipe delimiter is used in header
    assert!(lines[0].contains('|'), "Header should use pipe delimiter");
    // Verify each line has exactly 2 pipe delimiters (3 fields)
    for line in &lines {
        let pipe_count = line.matches('|').count();
        assert_eq!(
            pipe_count, 2,
            "Each line should have 2 pipe delimiters: {}",
            line
        );
    }
}

#[test]
fn test_invalid_count_zero() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&["addresses", "--count", "0", "--output", output_str])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Command should fail with count=0");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Count must be greater than 0"),
        "Should have appropriate error message"
    );
}

#[test]
fn test_invalid_error_rate_negative() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count",
            "10",
            "--output",
            output_str,
            "--error-rate",
            "-0.5",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        !output.status.success(),
        "Command should fail with negative error rate"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    // clap will parse -0.5 as an unexpected argument flag '-0', not as a negative number
    assert!(
        stderr.contains("unexpected argument")
            || stderr.contains("Error rate must be between 0.0 and 1.0"),
        "Should have appropriate error message"
    );
}

#[test]
fn test_invalid_error_rate_above_one() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "names",
            "--count",
            "10",
            "--output",
            output_str,
            "--error-rate",
            "1.5",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        !output.status.success(),
        "Command should fail with error rate > 1.0"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Error rate must be between 0.0 and 1.0"),
        "Should have appropriate error message"
    );
}

#[test]
fn test_addresses_with_subdirectory() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("subdir/addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count",
            "5",
            "--output",
            output_str,
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(
        output_path.exists(),
        "Output file should be created in subdirectory"
    );
}

#[test]
fn test_names_with_subdirectory() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("subdir/names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&["names", "--count", "5", "--output", output_str, "--quiet"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(
        output_path.exists(),
        "Output file should be created in subdirectory"
    );
}

#[test]
fn test_addresses_zero_error_rate() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count",
            "10",
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // With 0.0 error rate, all addresses should have required fields populated
    for line in lines.iter().skip(1) {
        let fields: Vec<&str> = line.split('|').collect();
        // Address1, City, State, Zip should not be empty (Address2 can be)
        assert!(
            !fields[0].is_empty(),
            "Address1 should not be empty with 0.0 error rate"
        );
        assert!(
            !fields[2].is_empty(),
            "City should not be empty with 0.0 error rate"
        );
        assert!(
            !fields[3].is_empty(),
            "State should not be empty with 0.0 error rate"
        );
        assert!(
            !fields[4].is_empty(),
            "Zip should not be empty with 0.0 error rate"
        );
    }
}

#[test]
fn test_names_zero_error_rate() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "names",
            "--count",
            "10",
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // With 0.0 error rate, all names should have first and last name populated
    for line in lines.iter().skip(1) {
        let fields: Vec<&str> = line.split('|').collect();
        // FirstName and LastName should not be empty (MiddleName can be)
        assert!(
            !fields[0].is_empty(),
            "FirstName should not be empty with 0.0 error rate"
        );
        assert!(
            !fields[2].is_empty(),
            "LastName should not be empty with 0.0 error rate"
        );
    }
}

#[test]
fn test_help_command() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("rust-faker"),
        "Help should contain program name"
    );
    assert!(
        stdout.contains("addresses"),
        "Help should mention addresses command"
    );
    assert!(
        stdout.contains("names"),
        "Help should mention names command"
    );
}

#[test]
fn test_addresses_help_command() {
    let output = Command::new(get_binary_path())
        .args(&["addresses", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("count"), "Help should mention count option");
    assert!(
        stdout.contains("output"),
        "Help should mention output option"
    );
    assert!(
        stdout.contains("error-rate"),
        "Help should mention error-rate option"
    );
}

#[test]
fn test_names_help_command() {
    let output = Command::new(get_binary_path())
        .args(&["names", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("count"), "Help should mention count option");
    assert!(
        stdout.contains("output"),
        "Help should mention output option"
    );
    assert!(
        stdout.contains("error-rate"),
        "Help should mention error-rate option"
    );
}

// ============================================================================
// Input CSV Loading Tests
// ============================================================================

#[test]
fn test_addresses_load_from_csv_standard_columns() {
    let temp_dir = TempDir::new().unwrap();

    // Create input CSV with standard column names
    let input_csv = create_test_csv(
        &temp_dir,
        "input.csv",
        "address1,address2,city,state,zip
123 Main St,Apt 1,Springfield,IL,62701
456 Oak Ave,,Chicago,IL,60601
789 Pine Rd,Suite 100,Naperville,IL,60540",
    );

    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--input",
            &input_csv,
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // Should have header + 3 records = 4 lines
    assert_eq!(lines.len(), 4, "Expected 4 lines (header + 3 records)");

    // Verify the addresses were loaded correctly (with 0 error rate, should be unchanged)
    assert!(contents.contains("123 Main St"));
    assert!(contents.contains("Springfield"));
}

#[test]
fn test_addresses_load_from_csv_with_count_limit() {
    let temp_dir = TempDir::new().unwrap();

    // Create input CSV with 5 addresses
    let input_csv = create_test_csv(
        &temp_dir,
        "input.csv",
        "address1,city,state,zip
123 Main St,Springfield,IL,62701
456 Oak Ave,Chicago,IL,60601
789 Pine Rd,Naperville,IL,60540
321 Elm St,Evanston,IL,60201
654 Maple Dr,Aurora,IL,60505",
    );

    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--input",
            &input_csv,
            "--count",
            "2",
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // Should have header + 2 records = 3 lines
    assert_eq!(lines.len(), 3, "Expected 3 lines (header + 2 records)");
}

#[test]
fn test_addresses_load_from_csv_alternate_column_names() {
    let temp_dir = TempDir::new().unwrap();

    // Create input CSV with alternate column names (like some external sources use)
    let input_csv = create_test_csv(
        &temp_dir,
        "input.csv",
        "street,city_name,region,postal_code
123 Main St,Springfield,IL,62701
456 Oak Ave,Chicago,IL,60601",
    );

    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--input",
            &input_csv,
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let contents = read_file_contents(output_str);
    assert!(contents.contains("123 Main St"));
    assert!(contents.contains("Springfield"));
}

#[test]
fn test_addresses_load_from_csv_openaddresses_format() {
    let temp_dir = TempDir::new().unwrap();

    // Create input CSV in OpenAddresses.io format (number + street separate)
    let input_csv = create_test_csv(
        &temp_dir,
        "input.csv",
        "NUMBER,STREET,CITY,REGION,POSTCODE
123,Main St,Springfield,IL,62701
456,Oak Ave,Chicago,IL,60601",
    );

    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--input",
            &input_csv,
            "--output",
            output_str,
            "--error-rate",
            "0.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let contents = read_file_contents(output_str);
    // Verify number and street were combined
    assert!(contents.contains("123 Main St"));
    assert!(contents.contains("456 Oak Ave"));
}

#[test]
fn test_addresses_load_from_csv_with_variance() {
    let temp_dir = TempDir::new().unwrap();

    // Create input CSV
    let input_csv = create_test_csv(
        &temp_dir,
        "input.csv",
        "address1,city,state,zip
123 Main Street,Springfield,IL,62701
456 Oak Avenue,Chicago,IL,60601
789 Pine Road,Naperville,IL,60540",
    );

    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    // Run with 100% error rate to ensure variance is applied
    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--input",
            &input_csv,
            "--output",
            output_str,
            "--error-rate",
            "1.0",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    let contents = read_file_contents(output_str);
    let lines: Vec<&str> = contents.lines().collect();

    // Should still have header + 3 records
    assert_eq!(lines.len(), 4, "Expected 4 lines (header + 3 records)");
}

#[test]
fn test_addresses_load_from_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--input",
            "/nonexistent/path/to/file.csv",
            "--output",
            output_str,
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        !output.status.success(),
        "Command should fail for nonexistent file"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Error loading addresses"),
        "Should have appropriate error message"
    );
}

#[test]
fn test_addresses_no_count_no_input_fails() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&["addresses", "--output", output_str, "--quiet"])
        .output()
        .expect("Failed to execute command");

    assert!(
        !output.status.success(),
        "Command should fail when neither --count nor --input is provided"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--count is required"),
        "Should have appropriate error message"
    );
}

#[test]
fn test_addresses_help_mentions_input_flag() {
    let output = Command::new(get_binary_path())
        .args(&["addresses", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("input"), "Help should mention input option");
}

// ============================================================================
// Download Command Tests
// ============================================================================

#[test]
fn test_download_help_command() {
    let output = Command::new(get_binary_path())
        .args(&["download", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--all"), "Help should mention --all option");
    assert!(
        stdout.contains("--list"),
        "Help should mention --list option"
    );
    assert!(
        stdout.contains("--limit"),
        "Help should mention --limit option"
    );
}

#[test]
fn test_download_list_empty() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(get_binary_path())
        .args(&["download", "--list"])
        .env("HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No") || stdout.contains("cached"),
        "Should show cache status"
    );
}

#[test]
fn test_download_invalid_state() {
    let output = Command::new(get_binary_path())
        .args(&["download", "XX"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Invalid state"),
        "Should reject invalid state code"
    );
}

// ============================================================================
// State Flag Tests
// ============================================================================

#[test]
fn test_addresses_state_flag_uncached() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&["addresses", "--state", "IL", "--output", output_str])
        .env("HOME", temp_dir.path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not cached"),
        "Should error when state not cached"
    );
}

#[test]
fn test_addresses_state_and_input_mutual_exclusion() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--state",
            "IL",
            "--input",
            "file.csv",
            "--output",
            output_str,
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Cannot use --input and --state together"),
        "Should reject combining --input and --state"
    );
}

#[test]
fn test_addresses_help_mentions_state_flag() {
    let output = Command::new(get_binary_path())
        .args(&["addresses", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--state"),
        "Help should mention --state option"
    );
}
