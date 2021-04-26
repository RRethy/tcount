use std::fmt::{self, Display};
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Unsupported(String),
    Parser,
    LanguageError(tree_sitter::LanguageError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error")
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<tree_sitter::LanguageError> for Error {
    fn from(err: tree_sitter::LanguageError) -> Error {
        Error::LanguageError(err)
    }
}
