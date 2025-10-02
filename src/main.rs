use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tsf")]
#[command(about = "TSF - Tree-Sitter based Formatting tool")]
#[command(long_about = None)]
struct Cli {
    /// Use a custom configuration file
    #[arg(long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Check formatting
    Check {
        /// Paths to files or directories
        path: Vec<String>,
    },
    /// Fix formatting
    Fix {
        /// Paths to files or directories
        path: Vec<String>,
    },
    /// Show version
    Version,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command.unwrap() {
        Commands::Check { path } => {
            println!("Check: {:?}", path);
        }

        Commands::Fix { path } => {
            println!("Fix: {:?}", path);
        }

        Commands::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
    }
}
