use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::Config;

pub mod config;

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
        path: Vec<PathBuf>,
    },
    /// Fix formatting
    Fix {
        /// Paths to files or directories
        path: Vec<PathBuf>,
    },
    /// Show version
    Version,
}

fn load_config(path: Option<PathBuf>) -> Config {
    match path {
        // TODO: Configuration file not found - show error message
        Some(path) => Config::load(path).unwrap_or_default(),
        None => Config::default(),
    }
}

fn check(config: Config, path: Vec<PathBuf>) {
    println!("Check: {:?}", path);
    println!("{:?}", config);
}

fn fix(config: Config, path: Vec<PathBuf>) {
    println!("Fix: {:?}", path);
    println!("{:?}", config);
}

fn show_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

fn main() {
    let cli = Cli::parse();

    match &cli.command.unwrap() {
        Commands::Check { path } => {
            let config = load_config(cli.config);
            check(config, path.clone())
        }

        Commands::Fix { path } => {
            let config = load_config(cli.config);
            fix(config, path.clone())
        }

        Commands::Version => show_version(),
    }
}
