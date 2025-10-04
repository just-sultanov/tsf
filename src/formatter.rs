use std::fmt::Display;
use std::path::PathBuf;

use tracing::debug;

use crate::config::Config;

pub enum Language {
    Unsupported,
    Clojure,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Language::Unsupported => "Unsupported",
            Language::Clojure => "Clojure",
        };
        write!(f, "{}", name)
    }
}

pub fn detect_language(path: PathBuf) -> Language {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("clj" | "cljs" | "cljc" | "edn") => Language::Clojure,
        _ => Language::Unsupported,
    }
}

pub fn check(config: Config, files: Vec<PathBuf>) {
    debug!("Check formatting");
    debug!("Config: {:?}", config);
    debug!("Files: {:?}", files)
}

pub fn fix(config: Config, files: Vec<PathBuf>) {
    debug!("Fix formatting");
    debug!("Config: {:?}", config);
    debug!("Files: {:?}", files)
}
