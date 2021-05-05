use crate::output::Format;
use regex::Regex;
use std::format;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
pub enum OrderBy {
    Language,
    File,
    NumFiles,
    Tokens,
}

impl FromStr for OrderBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "language" => Ok(OrderBy::Language),
            "file" => Ok(OrderBy::File),
            "numfiles" => Ok(OrderBy::NumFiles),
            "tokens" => Ok(OrderBy::Tokens),
            _ => Err(format!(
                "\"{}\" is not supported. Use one of language|file|numfiles|tokens",
                s
            )),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "tc",
    about = "Count your code by tokens, token kinds, and patterns in the syntax tree."
)]
pub struct Cli {
    #[structopt(
        short,
        long,
        help = "Prints errors encountered (e.g. file reading, parsing, etc.)"
    )]
    pub verbose: bool,

    #[structopt(
        short,
        long,
        help = "kinds of node in the syntax tree to count. See node-types.json in the parsers repo."
    )]
    pub kinds: Vec<String>,

    #[structopt(
        short = "p",
        long,
        help = "Patterns of node kinds to count in the syntax tree."
    )]
    pub kind_patterns: Vec<Regex>,

    #[structopt(
        long,
        default_value = "queries",
        help = "The directory too look for the named queries provided by --queries."
    )]
    pub queries_dir: PathBuf,

    #[structopt(
        short,
        long,
        help = "Names of the tree-sitter queries found under {--queries-dir}/{language}/ to count.\n\nFor example, for a --queries-dir of `/foo/bar/` and a --queries of `foobar`, then /foo/bar/{language}/foobar.scm will be counted for all files of kind {language}.\n\nSee https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries"
    )]
    pub queries: Vec<String>,

    #[structopt(
        long,
        help = "Shows counts for individual files instead of grouping by Language"
    )]
    pub show_files: bool,

    #[structopt(long, default_value = "tokens", help = "TODO")]
    pub order_by: OrderBy,

    #[structopt(long, default_value = "table", help = "TODO")]
    pub format: Format,

    #[structopt(help = "Files to parse and count.")]
    pub files: Vec<PathBuf>,
}
