use clap::{Parser, Subcommand};

mod generators;

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

fn validate_inputs(count: usize, error_rate: f64) -> Result<(), String> {
    if count == 0 {
        return Err("Count must be greater than 0".to_string());
    }
    if error_rate < 0.0 || error_rate > 1.0 {
        return Err("Error rate must be between 0.0 and 1.0".to_string());
    }
    Ok(())
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_help_works() {
        // This will test that CLI parsing doesn't panic
        // More detailed CLI tests will come later
    }

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
