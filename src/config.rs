use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Clojure {}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub clojure: Option<Clojure>,
}

impl Default for Config {
    fn default() -> Self {
        parse(include_str!("../tsf.toml")).unwrap()
    }
}

pub fn parse(content: &str) -> anyhow::Result<Config> {
    let config = toml::from_str(content)?;
    Ok(config)
}

pub fn load(path: Option<PathBuf>) -> Config {
    match path {
        // TODO: Configuration file not found - show error message
        Some(path) => {
            if path.exists() {
                let content = fs::read_to_string(path).unwrap();
                parse(&content).unwrap_or(Config::default())
            } else {
                Config::default()
            }
        }
        None => Config::default(),
    }
}
