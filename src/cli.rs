use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tc", about = "Count source code tokens, and TODO.")]
pub struct Cli {
    #[structopt(
        short,
        long,
        help = "Prints errors encountered (e.g. file reading, parsing, etc.)"
    )]
    pub verbose: bool,

    #[structopt(short, long)]
    pub kinds: Vec<String>,

    #[structopt(short = "p", long)]
    pub kind_patterns: Vec<Regex>,

    #[structopt(
        long,
        default_value = "queries",
        help = "The directory too look for the named queries provided by --query."
    )]
    pub query_dir: PathBuf,

    #[structopt(
        short,
        long,
        help = "Names of the tree-sitter queries found under {--query-dir}/{language}/ to count.\n\nFor example, for a --query-dir of `/foo/bar/` and a --queries `foobar`, then /foo/bar/{language}/foobar.scm will be counted for all files of type {language}.\n\nSee https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries"
    )]
    pub queries: Vec<String>,

    #[structopt(
        long,
        help = "Shows counts for individual files instead of grouping by Language"
    )]
    pub show_files: bool,

    #[structopt()]
    pub files: Vec<PathBuf>,
}
