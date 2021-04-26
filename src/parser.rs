use crate::error::Error;
use crate::language::language;
use std::fs::read_to_string;
use std::path::Path;
use tree_sitter::{Parser, Tree};

pub fn parse(path: impl AsRef<Path>) -> Result<Tree, Error> {
    let lang = language(&path)?;
    let mut parser = Parser::new();
    parser.set_language(lang)?;
    match parser.parse(read_to_string(&path)?, None) {
        Some(tree) => Ok(tree),
        None => Err(Error::Parser),
    }
}
