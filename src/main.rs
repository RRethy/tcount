use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod cli;
mod count;
mod error;
mod parser;

use count::count_tokens;
use error::Error;

fn counts(files: &Vec<PathBuf>) -> (BTreeMap<parser::Language, u64>, Vec<Error>) {
    let (parsed, errors): (Vec<_>, Vec<_>) = files
        .iter()
        .map(|path| match parser::parse(&path) {
            Ok((tree, lang)) => Ok((lang, count_tokens(&tree))),
            Err(err) => Err(err),
        })
        .partition(Result::is_ok);
    let counts =
        parsed
            .into_iter()
            .map(Result::unwrap)
            .fold(BTreeMap::new(), |mut acc, (lang, count)| {
                *acc.entry(lang).or_insert(0) += count;
                acc
            });
    (counts, errors.into_iter().map(Result::unwrap_err).collect())
}

fn run(cli: cli::Cli) -> Result<(), Error> {
    let (langs, errors) = counts(&cli.files);
    langs
        .iter()
        .for_each(|(lang, count)| println!("{:?}: {}", lang, count));
    if cli.verbose {
        // TODO print this nicer
        eprintln!("{:?}", errors);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_rust() {
        let files = vec![
            PathBuf::from("./test_data/rust1.rs"),
            PathBuf::from("./test_data/rust2.rs"),
        ];
        let mut expected = BTreeMap::new();
        expected.insert(parser::Language::RUST, 35 + 20);
        let (got, _) = counts(&files);
        assert_eq!(got, expected);
    }
}
