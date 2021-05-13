use crate::output::Format;
use crate::query::Query;
use regex::Regex;
use std::format;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "tc",
    about = "Count your code by tokens, node kinds, and patterns in the syntax tree."
)]
pub struct Cli {
    #[structopt(
        long,
        help = "Logging level. 0 to not print errors. 1 to print IO and filesystem errors. 2 to print parsing errors. 3 to print everything else.",
        default_value = "0"
    )]
    pub verbose: u8,

    #[structopt(
        short,
        long,
        help = "kinds of nodes in the syntax tree to count. See node-types.json in the parser's repo to see the names of nodes or use https://tree-sitter.github.io/tree-sitter/playground."
    )]
    pub kind: Vec<String>,

    #[structopt(
        short = "p",
        long,
        help = "Patterns of node kinds to count in the syntax tree (e.g. \".*comment\" to match nodes of type \"line_comment\", \"block_comment\", and \"comment\"). Supports Rust regular expressions"
    )]
    pub kind_pattern: Vec<Regex>,

    #[structopt(
        long,
        help = "Tree-sitter queries to match and count. Captures can also be counted with --query=query_name@capture_name,capture_name2. See https://github.com/RRethy/tc/blob/master/QUERIES.md for more information"
    )]
    pub query: Vec<Query>,

    #[structopt(
        long,
        default_value = "tokens",
        help = "One of group|numfiles|tokens. \"group\" will sort based on --groupby value"
    )]
    pub sort_by: SortBy,

    #[structopt(
        long,
        default_value = "language",
        help = "One of language|file|arg. \"arg\" will group by the `paths` arguments provided"
    )]
    pub groupby: GroupBy,

    #[structopt(long, default_value = "table", help = "One of table|csv")]
    pub format: Format,

    #[structopt(long, help = "Don't respect gitignore and .git/info/exclude files")]
    pub no_git: bool,

    #[structopt(long, help = "Don't respect .ignore files")]
    pub no_dot_ignore: bool,

    #[structopt(long, help = "Don't respect ignore files from parent directories")]
    pub no_parent_ignore: bool,

    #[structopt(long, help = "Count hidden files")]
    pub count_hidden: bool,

    #[structopt(
        long,
        help = "Whitelist of languages to parse. This overrides --blacklist and must be an exact match"
    )]
    pub whitelist: Vec<String>,

    #[structopt(
        long,
        help = "Blacklist of languages not to parse. This is overriden by --whitelist and must be an exact match"
    )]
    pub blacklist: Vec<String>,

    #[structopt(long, help = "Show a list of supported languages for parsing")]
    pub list_languages: bool,

    #[structopt(long, help = "Show column totals. This is not affected by --top")]
    pub show_totals: bool,

    #[structopt(long, help = "How many of the top results to show")]
    pub top: Option<usize>,

    #[structopt(
        default_value = ".",
        help = "Files and directories to parse and count."
    )]
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SortBy {
    Group,
    NumFiles,
    Tokens,
}

impl FromStr for SortBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "group" => Ok(SortBy::Group),
            "numfiles" => Ok(SortBy::NumFiles),
            "tokens" => Ok(SortBy::Tokens),
            _ => Err(format!(
                "\"{}\" is not a supported argument to --sort-by. Use one of group|numfiles|tokens",
                s
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GroupBy {
    Language,
    File,
    Arg,
}

impl FromStr for GroupBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "language" => Ok(GroupBy::Language),
            "file" => Ok(GroupBy::File),
            "arg" => Ok(GroupBy::Arg),
            _ => Err(format!(
                "\"{}\" is not a supported argument to --groupby. Use one of language|file",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_by_from_str() {
        assert_eq!(GroupBy::Language, GroupBy::from_str("language").unwrap());
        assert_eq!(GroupBy::File, GroupBy::from_str("file").unwrap());
        assert_eq!(GroupBy::Arg, GroupBy::from_str("arg").unwrap());
    }

    #[test]
    fn sort_by_from_str() {
        assert_eq!(SortBy::Group, SortBy::from_str("group").unwrap());
        assert_eq!(SortBy::NumFiles, SortBy::from_str("numfiles").unwrap());
        assert_eq!(SortBy::Tokens, SortBy::from_str("tokens").unwrap());
    }
}
