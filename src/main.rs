use std::process;
use structopt::StructOpt;

mod cli;
mod count;
mod error;
mod language;
mod parser;

use count::count_tokens;
use error::Error;

fn run(cli: cli::Cli) -> Result<(), Error> {
    cli.files.iter().for_each(|path| match parser::parse(path) {
        Ok(tree) => println!("{}", count_tokens(tree)),
        Err(err) => eprintln!("{:?}", err),
    });
    Ok(())
}

fn main() {
    let cli = cli::Cli::from_args();

    match run(cli) {
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
        _ => {}
    }
}
