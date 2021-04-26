use std::collections::BTreeMap;
use std::process;
use structopt::StructOpt;

mod cli;
mod count;
mod error;
mod parser;

use count::count_tokens;
use error::Error;

fn run(cli: cli::Cli) -> Result<(), Error> {
    let (parsed, failed): (Vec<_>, Vec<_>) = cli
        .files
        .into_iter()
        .map(|path| match parser::parse(&path) {
            Ok((tree, lang)) => Ok((lang, count_tokens(&tree))),
            Err(err) => Err(err),
        })
        .partition(Result::is_ok);
    parsed
        .into_iter()
        .map(Result::unwrap)
        .fold(BTreeMap::new(), |mut acc, (lang, count)| {
            *acc.entry(lang).or_insert(0) += count;
            acc
        })
        .iter()
        .for_each(|(lang, count)| println!("{:?}: {}", lang, count));
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
