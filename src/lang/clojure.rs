use std::cell::RefCell;

use serde::Deserialize;
use tracing::debug;

use crate::lang;

unsafe extern "C" {
    fn tree_sitter_clojure() -> tree_sitter::Language;
}

pub fn language() -> tree_sitter::Language {
    unsafe { tree_sitter_clojure() }
}

#[derive(Debug, Deserialize)]
pub struct Config {}

pub struct Formatter {
    pub config: Config,
    pub parser: RefCell<tree_sitter::Parser>,
}

impl lang::Formatter for Formatter {
    fn format(&self, content: &str) {
        let mut parser = self.parser.borrow_mut();
        let tree = parser.parse(content, None);
        debug!("clojure::format - Tree: {:?}", tree);
        debug!("clojure::format - Content: {}", content)
    }
}

pub fn create_formatter(config: Config) -> anyhow::Result<Formatter> {
    let lang = language();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&lang)?;
    let formatter = Formatter {
        config,
        parser: RefCell::new(parser),
    };
    Ok(formatter)
}
