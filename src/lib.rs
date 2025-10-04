use std::path::PathBuf;

use clap::{Parser, Subcommand};
use config::Config;

pub mod config;

#[derive(Debug, Parser)]
#[command(name = "tsf")]
#[command(about = "Tree-Sitter based Formatting tool (TSF)")]
#[command(long_about = None)]
pub struct Cli {
    /// Use a custom configuration file
    #[arg(long, value_name = "FILE", default_value = "tsf.toml")]
    pub config_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}

#[derive(Debug, Subcommand)]
pub enum Commands {
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

pub fn load_config(path: Option<PathBuf>) -> Config {
    match path {
        // TODO: Configuration file not found - show error message
        Some(path) => Config::load(path).unwrap_or_default(),
        None => Config::default(),
    }
}

pub fn check(config: Config, path: Vec<PathBuf>) {
    println!("Check: {:?}", path);
    println!("{:?}", config)
}

pub fn fix(config: Config, path: Vec<PathBuf>) {
    println!("Fix: {:?}", path);
    println!("{:?}", config)
}

pub fn show_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

pub fn run(config: Config, command: Commands) {
    match command {
        Commands::Version => show_version(),
        Commands::Check { path } => check(config, path.clone()),
        Commands::Fix { path } => fix(config, path.clone()),
    }
}
