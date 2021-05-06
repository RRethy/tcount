use crate::language::Language;
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
    Ignore(ignore::Error),
    LanguageNotWhitelisted(Language),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO Error: {}\n", err),
            Error::UnsupportedLanguage => write!(f, "Unsupported Language\n"),
            Error::Parser => write!(f, "Parser Error\n"),
            Error::LanguageError(err) => write!(f, "Tree-sitter Language Error: {}\n", err),
            Error::QueryError(err) => write!(f, "Tree-sitter Query Error: {:?}\n", err),
            Error::Ignore(err) => write!(f, "Error while walking filetree: {:?}\n", err),
            Error::LanguageNotWhitelisted(lang) => {
                write!(f, "Language({}) is not on the non-empty whitelist\n", lang)
            }
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

impl From<ignore::Error> for Error {
    fn from(err: ignore::Error) -> Error {
        Error::Ignore(err)
    }
}
