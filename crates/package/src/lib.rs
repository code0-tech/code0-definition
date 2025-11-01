#[path = "../../parser/src/lib.rs"]
mod parser;

pub mod package {
    pub use super::parser::*;
}