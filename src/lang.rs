#![allow(unused_variables)]

pub mod clojure;

pub trait Formatter {
    fn format(&self, content: &str) {}
}
