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
