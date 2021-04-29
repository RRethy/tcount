use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "tc", about = "Count source code tokens, and TODO.")]
pub struct Cli {
    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt()]
    pub files: Vec<PathBuf>,
}
