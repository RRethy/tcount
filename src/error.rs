use crate::language::Language;
use glob::GlobError;
use std::fmt::{self, Display};
use std::io;
use std::path::PathBuf;
use tree_sitter::QueryError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    UnsupportedLanguage,
    Parser(PathBuf),
    QueryError(QueryError),
    Ignore(ignore::Error),
    LanguageIgnored(PathBuf, Language),
    Glob(GlobError),
}

impl Error {
    pub fn should_show(&self, verbose_lvl: u8) -> bool {
        match self {
            Error::IO(_) => verbose_lvl >= 1,
            Error::UnsupportedLanguage => verbose_lvl >= 3,
            Error::Parser(_) => verbose_lvl >= 2,
            Error::QueryError(_) => true,
            Error::Ignore(_) => verbose_lvl >= 1,
            Error::LanguageIgnored(_, _) => verbose_lvl >= 3,
            Error::Glob(_) => verbose_lvl >= 3,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO Error: {}\n", err),
            Error::UnsupportedLanguage => write!(f, "Unsupported Language\n"),
            Error::Parser(path) => write!(f, "Parser Error for path {}\n", path.display()),
            Error::QueryError(err) => write!(f, "Tree-sitter Query Error: {:?}\n", err),
            Error::Ignore(err) => write!(f, "Error while walking filetree: {}\n", err),
            Error::LanguageIgnored(path, lang) => {
                write!(
                    f,
                    "Language({}) for path({}) is ignored due to whitelist/blacklist options\n",
                    lang,
                    path.display()
                )
            }
            Error::Glob(err) => write!(f, "Error with globbing {}\n", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
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

impl From<GlobError> for Error {
    fn from(err: GlobError) -> Error {
        Error::Glob(err)
    }
}
