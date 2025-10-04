use std::{collections::VecDeque, fs, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::cli;
use crate::config;
use crate::formatter;
use crate::logger;

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

pub fn collect_files(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut queue: VecDeque<PathBuf> = paths.into_iter().collect();

    while let Some(path) = queue.pop_front() {
        if path.is_dir() {
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.flatten() {
                    queue.push_back(entry.path());
                }
            }
        } else if path.is_file() {
            files.push(path);
        }
    }
    files
}

pub fn run() {
    let cli = cli::parse();
    logger::init(cli.debug);
    let config = config::load(cli.config_path);
    let command = cli.command.unwrap();

    match command {
        Commands::Version => {
            println!("{}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Check { paths } => {
            let files = collect_files(paths.clone());
            formatter::check(config, files)
        }
        Commands::Fix { paths } => {
            let files = collect_files(paths.clone());
            formatter::fix(config, files)
        }
    }
}
