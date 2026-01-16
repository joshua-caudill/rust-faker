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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_help_works() {
        // This will test that CLI parsing doesn't panic
        // More detailed CLI tests will come later
    }
}
