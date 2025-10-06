use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
        paths: Vec<PathBuf>,
    },
    /// Fix formatting
    Fix {
        /// Paths to files or directories
        paths: Vec<PathBuf>,
    },
    /// Show version
    Version,
}

pub fn parse() -> Cli {
    Cli::parse()
}
