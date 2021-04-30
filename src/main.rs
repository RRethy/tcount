use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod cli;
mod count;
mod error;
mod parser;

use count::{count, Counter, KindCounter, TokenCounter};
use error::Error;
use parser::Language;

fn make_counters(kinds: &Vec<String>) -> Vec<Box<dyn Counter>> {
    let mut counters: Vec<Box<dyn Counter>> = Vec::new();
    counters.push(Box::new(TokenCounter::new()));
    kinds
        .iter()
        .for_each(|kind| counters.push(Box::new(KindCounter::new(kind.clone()))));
    counters
}

fn get_counts(
    files: &Vec<PathBuf>,
    kinds: &Vec<String>,
) -> (Vec<(Language, Vec<u64>)>, Vec<Error>) {
    let mut counters = make_counters(&kinds);
    let (counts, errors): (Vec<_>, Vec<_>) = files
        .iter()
        .map(|path| {
            counters.iter_mut().for_each(|counter| counter.reset());
            let (tree, lang) = parser::parse(&path)?;
            count(&tree, &mut counters);
            Ok((
                lang,
                counters.iter().map(|counter| counter.get_count()).collect(),
            ))
        })
        .partition(Result::is_ok);
    (
        counts.into_iter().map(Result::unwrap).collect(),
        errors.into_iter().map(Result::unwrap_err).collect(),
    )
}

fn group_by_language(counts: Vec<(Language, Vec<u64>)>) -> BTreeMap<parser::Language, Vec<u64>> {
    counts
        .into_iter()
        .fold(BTreeMap::new(), |mut acc, (lang, counts)| {
            if let Some(cur) = acc.get_mut(&lang) {
                cur.iter_mut()
                    .zip(&counts)
                    .for_each(|(cum, val)| *cum += val);
            } else {
                acc.insert(lang, counts);
            }
            acc
        })
}

fn run(cli: cli::Cli) -> Result<(), Error> {
    let (counts, errors) = get_counts(&cli.files, &cli.kind);
    let lang_counts = group_by_language(counts);
    lang_counts
        .iter()
        .for_each(|(lang, count)| println!("{:?}: {:?}", lang, count));

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
        expected.insert(parser::Language::Rust, vec![35 + 20]);
        let kinds = vec![];
        let (got, _) = get_counts(&files, &kinds);
        let got = group_by_language(got);
        assert_eq!(got, expected);
    }
}
