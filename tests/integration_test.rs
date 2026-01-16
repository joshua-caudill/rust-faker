use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to get the path to the compiled binary
fn get_binary_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let debug_path = format!("{}/target/debug/rust-faker", manifest_dir);

    // Check if binary exists, if not provide helpful message
    if !std::path::Path::new(&debug_path).exists() {
        panic!("Binary not found at {}. Run 'cargo build' first.", debug_path);
    }

    debug_path
}

/// Helper function to read and return file contents
fn read_file_contents(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}

#[test]
fn test_addresses_command_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count", "10",
            "--output", output_str,
            "--error-rate", "0.5",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", String::from_utf8_lossy(&output.stderr));
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
            "--count", "5",
            "--output", output_str,
            "--error-rate", "0.0",
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
        assert_eq!(fields.len(), 5, "Line {} should have 5 fields: {}", i+2, line);
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
            "--count", "3",
            "--output", output_str,
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let contents = read_file_contents(output_str);

    // Verify pipe delimiter is used
    assert!(contents.contains('|'), "Output should use pipe delimiter");
    assert!(!contents.contains(','), "Output should not use comma delimiter");
}

#[test]
fn test_names_command_creates_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "names",
            "--count", "10",
            "--output", output_str,
            "--error-rate", "0.5",
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Command failed: {:?}", String::from_utf8_lossy(&output.stderr));
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
            "--count", "5",
            "--output", output_str,
            "--error-rate", "0.0",
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
        assert_eq!(fields.len(), 3, "Line {} should have 3 fields: {}", i+2, line);
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
            "--count", "3",
            "--output", output_str,
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let contents = read_file_contents(output_str);

    // Verify pipe delimiter is used
    assert!(contents.contains('|'), "Output should use pipe delimiter");
    assert!(!contents.contains(','), "Output should not use comma delimiter");
}

#[test]
fn test_invalid_count_zero() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count", "0",
            "--output", output_str,
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Command should fail with count=0");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Count must be greater than 0"), "Should have appropriate error message");
}

#[test]
fn test_invalid_error_rate_negative() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count", "10",
            "--output", output_str,
            "--error-rate", "-0.5",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Command should fail with negative error rate");
    let stderr = String::from_utf8_lossy(&output.stderr);
    // clap will parse -0.5 as an unexpected argument flag '-0', not as a negative number
    assert!(stderr.contains("unexpected argument") || stderr.contains("Error rate must be between 0.0 and 1.0"),
            "Should have appropriate error message");
}

#[test]
fn test_invalid_error_rate_above_one() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "names",
            "--count", "10",
            "--output", output_str,
            "--error-rate", "1.5",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Command should fail with error rate > 1.0");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error rate must be between 0.0 and 1.0"), "Should have appropriate error message");
}

#[test]
fn test_addresses_with_subdirectory() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("subdir/addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count", "5",
            "--output", output_str,
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output_path.exists(), "Output file should be created in subdirectory");
}

#[test]
fn test_names_with_subdirectory() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("subdir/names.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "names",
            "--count", "5",
            "--output", output_str,
            "--quiet",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(output_path.exists(), "Output file should be created in subdirectory");
}

#[test]
fn test_addresses_zero_error_rate() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("addresses.csv");
    let output_str = output_path.to_str().unwrap();

    let output = Command::new(get_binary_path())
        .args(&[
            "addresses",
            "--count", "10",
            "--output", output_str,
            "--error-rate", "0.0",
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
        assert!(!fields[0].is_empty(), "Address1 should not be empty with 0.0 error rate");
        assert!(!fields[2].is_empty(), "City should not be empty with 0.0 error rate");
        assert!(!fields[3].is_empty(), "State should not be empty with 0.0 error rate");
        assert!(!fields[4].is_empty(), "Zip should not be empty with 0.0 error rate");
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
            "--count", "10",
            "--output", output_str,
            "--error-rate", "0.0",
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
        assert!(!fields[0].is_empty(), "FirstName should not be empty with 0.0 error rate");
        assert!(!fields[2].is_empty(), "LastName should not be empty with 0.0 error rate");
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
    assert!(stdout.contains("rust-faker"), "Help should contain program name");
    assert!(stdout.contains("addresses"), "Help should mention addresses command");
    assert!(stdout.contains("names"), "Help should mention names command");
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
    assert!(stdout.contains("output"), "Help should mention output option");
    assert!(stdout.contains("error-rate"), "Help should mention error-rate option");
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
    assert!(stdout.contains("output"), "Help should mention output option");
    assert!(stdout.contains("error-rate"), "Help should mention error-rate option");
}
