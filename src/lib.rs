use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::cli::Commands;
use crate::lang::clojure;

pub mod cli;
pub mod lang;
pub mod logger;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub clojure: Option<clojure::Config>,
}

impl Default for Config {
    fn default() -> Self {
        parse_config(include_str!("../tsf.toml")).unwrap()
    }
}

pub fn parse_config(content: &str) -> anyhow::Result<Config> {
    let config = toml::from_str(content)?;
    Ok(config)
}

pub fn read_config(path: Option<PathBuf>) -> Config {
    match path {
        // TODO: Configuration file not found - show error message
        Some(path) => {
            if path.exists() {
                let content = fs::read_to_string(path).unwrap();
                parse_config(&content).unwrap_or_default()
            } else {
                Config::default()
            }
        }
        None => Config::default(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Unsupported,
    Clojure,
}

pub struct Formatter {
    formatters: HashMap<Language, Box<dyn lang::Formatter>>,
}

pub fn create_formatter(config: Config) -> anyhow::Result<Formatter> {
    let mut formatters = HashMap::new();
    if config.clojure.is_some() {
        let formatter = clojure::create_formatter(config.clojure.unwrap())?;
        let boxed: Box<dyn lang::Formatter> = Box::new(formatter);
        formatters.insert(Language::Clojure, boxed);
    };
    Ok(Formatter { formatters })
}

impl Formatter {
    pub fn check(&self, path: &Path) {
        let lang = detect_language(path);
        if let Some(formatter) = self.formatters.get(&lang) {
            let content = fs::read_to_string(path).unwrap();
            formatter.format(&content)
        }
    }

    pub fn fix(&self, path: &Path) {
        // FIXME: [2025-10-08, Ilshat Sultanov] implement this
        self.check(path)
    }
}

pub fn detect_language(path: &Path) -> Language {
    if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
        match ext {
            "clj" | "cljs" | "cljc" | "edn" => Language::Clojure,
            _ => Language::Unsupported,
        }
    } else {
        Language::Unsupported
    }
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

pub fn version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

pub fn check(config_path: Option<PathBuf>, paths: Vec<PathBuf>) {
    let files = collect_files(paths);
    if !files.is_empty() {
        let config = read_config(config_path);
        if let Ok(formatter) = create_formatter(config) {
            for file in files {
                formatter.check(&file)
            }
        }
    }
}

pub fn fix(config_path: Option<PathBuf>, paths: Vec<PathBuf>) {
    let files = collect_files(paths);
    if !files.is_empty() {
        let config = read_config(config_path);
        if let Ok(formatter) = create_formatter(config) {
            for file in files {
                formatter.fix(&file)
            }
        }
    }
}

pub fn run() {
    let cli = cli::parse();
    let command = cli.command.unwrap();
    logger::init(cli.debug);

    match command {
        Commands::Version => version(),
        Commands::Check { paths } => check(cli.config_path, paths),
        Commands::Fix { paths } => fix(cli.config_path, paths),
    }
}
