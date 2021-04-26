use std::io::{self, Write};
use std::process;
use structopt::StructOpt;

mod cli;
mod error;

use error::Error;

fn run(cli: cli::Cli, mut out: impl Write) -> Result<(), Error> {
    write!(out, "Hello, World!")?;
    Ok(())
}

fn main() {
    let cli = cli::Cli::from_args();

    match run(cli, io::stdout()) {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
        _ => {}
    }
    println!("Hello, world!");
}
