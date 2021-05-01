use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use tree_sitter::Parser;

mod cli;
mod error;
mod language;
mod query;
mod tree;

use error::{Error, Result};
use language::Language;

fn get_counts(
    files: &Vec<PathBuf>,
    kinds: &Vec<String>,
) -> (Vec<(Language, Vec<u64>)>, Vec<Error>) {
    let (counts, errors): (Vec<_>, Vec<_>) = files
        .iter()
        .map(|path| {
            let lang: Language = path.into();
            let ts_lang = lang.get_treesitter_language()?;
            let text = fs::read_to_string(path)?;

            let mut parser = Parser::new();
            parser.set_language(ts_lang)?;
            match parser.parse(text, None) {
                Some(tree) => {
                    // the first element is the total number of tokens while the rest are the
                    // counts for the specified --kind
                    let mut counts = vec![0; kinds.len() + 1];

                    tree::traverse(&tree, |node| {
                        if !node.is_missing() {
                            // count each terminal node which is the closest we can get to counting
                            // tokens
                            if node.child_count() == 0 && !node.is_extra() {
                                counts[0] += 1;
                            }

                            // count each --kind that matches the current nodes kind
                            kinds.iter().enumerate().for_each(|(i, kind)| {
                                if kind == node.kind() {
                                    counts[i + 1] += 1;
                                }
                            });
                        }
                    });
                    Ok((lang, counts))
                }
                None => Err(Error::Parser),
            }
        })
        .partition(Result::is_ok);
    (
        counts.into_iter().map(Result::unwrap).collect(),
        errors.into_iter().map(Result::unwrap_err).collect(),
    )
}

fn group_by_language(counts: Vec<(Language, Vec<u64>)>) -> BTreeMap<Language, Vec<u64>> {
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

fn run(cli: cli::Cli) -> Result<()> {
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
        expected.insert(Language::Rust, vec![35 + 20, 4 + 0, 3 + 1]);
        let kinds = vec!["\"".into(), ";".into()];
        let (got, _) = get_counts(&files, &kinds);
        let got = group_by_language(got);
        assert_eq!(got, expected);
    }
}
