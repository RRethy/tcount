use std::fmt::{self, Display};
use std::io;
use tree_sitter::{LanguageError, QueryError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    UnsupportedLanguage,
    Parser,
    LanguageError(LanguageError),
    QueryError(QueryError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO Error: {}\n", err),
            Error::UnsupportedLanguage => write!(f, "Unsupported Language\n"),
            Error::Parser => write!(f, "Parser Error\n"),
            Error::LanguageError(err) => write!(f, "Tree-sitter Language Error: {}\n", err),
            Error::QueryError(err) => write!(f, "Tree-sitter Query Error: {:?} \n", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<LanguageError> for Error {
    fn from(err: LanguageError) -> Error {
        Error::LanguageError(err)
    }
}

impl From<QueryError> for Error {
    fn from(err: QueryError) -> Error {
        Error::QueryError(err)
    }
}
