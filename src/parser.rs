use crate::error::Error;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::Path;
use tree_sitter::{Parser, Tree};

extern "C" {
    fn tree_sitter_rust() -> tree_sitter::Language;
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum Language {
    Rust,
    Unsupported,
}

static LANGUAGES: phf::Map<&'static str, Language> = phf::phf_map! {
    "rs" => Language::Rust,
};

fn language(path: impl AsRef<Path>) -> Language {
    let ext = path
        .as_ref()
        .extension()
        .map(OsString::from)
        .unwrap_or(OsString::new());
    LANGUAGES
        .get(ext.to_string_lossy().as_ref())
        .unwrap_or(&Language::Unsupported)
        .clone()
}

pub fn parse(path: impl AsRef<Path>) -> Result<(Tree, Language), Error> {
    let lang = language(path.as_ref());
    let tslang = match lang {
        Language::Rust => unsafe { tree_sitter_rust() },
        Language::Unsupported => return Err(Error::Unsupported(String::new())),
    };

    let mut parser = Parser::new();
    parser.set_language(tslang)?;
    match parser.parse(read_to_string(&path)?, None) {
        Some(tree) => Ok((tree, lang.clone())),
        None => Err(Error::Parser),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_language_from_extension() {
        let ext_to_language = vec![("main.rs", Language::Rust)];
        ext_to_language
            .into_iter()
            .for_each(|(ext, lang)| assert_eq!(language(ext), lang));
    }
}
