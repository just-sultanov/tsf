use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct General {
    pub indent_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct Clojure {
    pub indent_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub clojure: Option<Clojure>,
}

impl Default for Config {
    fn default() -> Self {
        Self::parse(include_str!("../tsf.toml")).unwrap()
    }
}

impl Config {
    pub fn parse(content: &str) -> anyhow::Result<Self> {
        let config = toml::from_str(content)?;
        Ok(config)
    }

    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        Self::parse(&content)
    }
}

pub fn load(path: Option<PathBuf>) -> Config {
    match path {
        // TODO: Configuration file not found - show error message
        Some(path) => Config::load(path).unwrap_or_default(),
        None => Config::default(),
    }
}
