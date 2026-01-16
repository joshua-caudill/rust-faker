use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io;
use std::path::Path;

use crate::generators::addresses::Address;

pub struct CsvWriter {
    quiet: bool,
}

impl CsvWriter {
    pub fn new(quiet: bool) -> Self {
        Self { quiet }
    }

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

    pub fn write_addresses(&self, path: &str, addresses: &[Address]) -> io::Result<()> {
        // Create parent directories if needed
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_writer_creation() {
        let writer = CsvWriter::new(false);
        assert_eq!(writer.quiet, false);
    }

    #[test]
    fn test_create_progress_bar_quiet() {
        let writer = CsvWriter::new(true);
        let pb = writer.create_progress_bar(1000, "Testing");
        pb.finish_and_clear();
    }

    #[test]
    fn test_create_progress_bar_not_quiet() {
        let writer = CsvWriter::new(false);
        let pb = writer.create_progress_bar(1000, "Testing");
        pb.finish_and_clear();
    }

    #[test]
    fn test_write_addresses() {
        use tempfile::NamedTempFile;
        use std::io::Read;

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
            Address::new(
                "456 Oak Ave".to_string(),
                "".to_string(),
                "Chicago".to_string(),
                "IL".to_string(),
                "60601".to_string(),
            ),
        ];

        let writer = CsvWriter::new(true);
        writer.write_addresses(path, &addresses).unwrap();

        // Read the file and verify contents
        let mut file = std::fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("Address1|Address2|City|State|Zip"));
        assert!(contents.contains("123 Main St|Apt 4B|Springfield|IL|62701"));
        assert!(contents.contains("456 Oak Ave||Chicago|IL|60601"));
    }
}
