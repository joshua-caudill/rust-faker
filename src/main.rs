use clap::{Parser, Subcommand};
use std::process;

mod cache;
mod download;
mod generators;
mod regions;
mod writer;

use generators::addresses::{
    apply_variance_to_addresses, generate_addresses, load_addresses_from_cache,
    load_addresses_from_csv,
};
use generators::names::generate_names;
use writer::CsvWriter;

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
        /// Number of records to generate (required unless using --input)
        #[arg(short, long)]
        count: Option<usize>,

        /// Input CSV file with real addresses to load
        #[arg(short, long)]
        input: Option<String>,

        /// Load from cached state(s) - comma-separated or 'all'
        #[arg(short, long)]
        state: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Error rate (0.0-1.0) - percentage of records with variance applied
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
    /// Download address data from OpenAddresses.io
    Download {
        /// State codes to download (e.g., IL CA TX)
        #[arg(value_name = "STATES")]
        states: Vec<String>,

        /// Download all 50 states + DC
        #[arg(long)]
        all: bool,

        /// List cached states
        #[arg(long)]
        list: bool,

        /// Maximum addresses per state
        #[arg(long, default_value = "10000")]
        limit: usize,

        /// Force re-download even if cached
        #[arg(long)]
        force: bool,

        /// Suppress progress output
        #[arg(short, long)]
        quiet: bool,
    },
}

fn validate_error_rate(error_rate: f64) -> Result<(), String> {
    if !(0.0..=1.0).contains(&error_rate) {
        return Err("Error rate must be between 0.0 and 1.0".to_string());
    }
    Ok(())
}

fn validate_count(count: usize) -> Result<(), String> {
    if count == 0 {
        return Err("Count must be greater than 0".to_string());
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Addresses {
            count,
            input,
            state,
            output,
            error_rate,
            quiet,
        } => {
            if let Err(e) = validate_error_rate(error_rate) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }

            // Check mutual exclusivity
            if input.is_some() && state.is_some() {
                eprintln!("Error: Cannot use --input and --state together. Choose one.");
                process::exit(1);
            }

            let addresses = if let Some(input_path) = input {
                // Load addresses from input CSV
                match load_addresses_from_csv(&input_path, count) {
                    Ok(loaded) => {
                        if !quiet {
                            println!("Loaded {} addresses from {}", loaded.len(), input_path);
                        }
                        // Apply variance to loaded addresses
                        apply_variance_to_addresses(loaded, error_rate)
                    }
                    Err(e) => {
                        eprintln!("Error loading addresses from {}: {}", input_path, e);
                        process::exit(1);
                    }
                }
            } else if let Some(state_input) = state {
                // Load addresses from cache
                let states_to_load: Vec<String> = if state_input.to_lowercase() == "all" {
                    match cache::list_cached_states() {
                        Ok(cached) => {
                            if cached.is_empty() {
                                eprintln!(
                                    "Error: No states specified or cached. Run 'rust-faker download <STATE>' first."
                                );
                                process::exit(1);
                            }
                            cached.into_iter().map(|(state, _)| state).collect()
                        }
                        Err(e) => {
                            eprintln!("Error listing cached states: {}", e);
                            process::exit(1);
                        }
                    }
                } else {
                    state_input
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                };

                match load_addresses_from_cache(&states_to_load, count) {
                    Ok(loaded) => {
                        if !quiet {
                            println!(
                                "Loaded {} addresses from cache (states: {})",
                                loaded.len(),
                                states_to_load.join(", ")
                            );
                        }
                        // Apply variance to loaded addresses
                        apply_variance_to_addresses(loaded, error_rate)
                    }
                    Err(e) => {
                        eprintln!("Error loading addresses from cache: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                // Generate fake addresses (count is required in this case)
                let count = count.unwrap_or_else(|| {
                    eprintln!("Error: --count is required when not using --input or --state");
                    process::exit(1);
                });
                if let Err(e) = validate_count(count) {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
                generate_addresses(count, error_rate)
            };

            let final_count = addresses.len();
            let writer = CsvWriter::new(quiet);
            if let Err(e) = writer.write_addresses(&output, &addresses) {
                eprintln!("Error writing addresses: {}", e);
                process::exit(1);
            }

            if !quiet {
                println!("Successfully wrote {} addresses to {}", final_count, output);
            }
        }
        Commands::Names {
            count,
            output,
            error_rate,
            quiet,
        } => {
            if let Err(e) = validate_count(count) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
            if let Err(e) = validate_error_rate(error_rate) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }

            let names = generate_names(count, error_rate);
            let writer = CsvWriter::new(quiet);
            if let Err(e) = writer.write_names(&output, &names) {
                eprintln!("Error writing names: {}", e);
                process::exit(1);
            }

            if !quiet {
                println!("Successfully generated {} names to {}", count, output);
            }
        }
        Commands::Download {
            states,
            all,
            list,
            limit,
            force,
            quiet,
        } => {
            if list {
                if let Err(e) = download::print_cache_list() {
                    eprintln!("Error listing cache: {}", e);
                    process::exit(1);
                }
                return;
            }

            let states_to_download: Vec<String> = if all {
                regions::ALL_STATES.iter().map(|s| s.to_string()).collect()
            } else if states.is_empty() {
                eprintln!("Error: Specify states to download or use --all");
                process::exit(1);
            } else {
                states
            };

            if let Err(e) = download::download_states(&states_to_download, limit, force, quiet) {
                eprintln!("Error downloading: {}", e);
                process::exit(1);
            }
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
        assert!(validate_count(0).is_err());
    }

    #[test]
    fn test_validate_error_rate_below_zero() {
        assert!(validate_error_rate(-0.1).is_err());
    }

    #[test]
    fn test_validate_error_rate_above_one() {
        assert!(validate_error_rate(1.5).is_err());
    }

    #[test]
    fn test_validate_valid_inputs() {
        assert!(validate_count(100).is_ok());
        assert!(validate_error_rate(0.5).is_ok());
    }
}
