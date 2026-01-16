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
}
