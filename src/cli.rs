use regex::Regex;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tc", about = "Count source code tokens, and TODO.")]
pub struct Cli {
    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(short, long)]
    pub kinds: Vec<String>,

    #[structopt(short = "p", long)]
    pub kind_patterns: Vec<Regex>,

    #[structopt(long)]
    pub query_dir: Option<PathBuf>,

    #[structopt(short, long)]
    pub query: Vec<String>,

    #[structopt(long)]
    pub show_files: bool,

    #[structopt()]
    pub files: Vec<PathBuf>,
}
