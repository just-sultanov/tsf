use std::fmt::Display;
use std::path::PathBuf;

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
