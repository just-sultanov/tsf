use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::debug;

use crate::config::Config;

#[derive(Debug, Parser)]
#[command(name = "tsf")]
#[command(about = "Tree-Sitter based Formatting tool (TSF)")]
#[command(long_about = None)]
pub struct Cli {
    /// Use a custom configuration file
    #[arg(long, value_name = "FILE", default_value = "tsf.toml")]
    pub config_path: Option<PathBuf>,

    /// Show debug info
    #[arg(long)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
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

pub fn parse() -> Cli {
    Cli::parse()
}

pub fn check(config: Config, path: Vec<PathBuf>) {
    debug!("Check: {:?}", path);
    debug!("{:?}", config)
}

pub fn fix(config: Config, path: Vec<PathBuf>) {
    debug!("Fix: {:?}", path);
    debug!("{:?}", config)
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
