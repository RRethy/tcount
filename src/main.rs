use std::collections::BTreeMap;
use std::process;
use structopt::StructOpt;

mod cli;
mod count;
mod error;
mod language;
mod query;
mod tree;

use count::Counts;
use error::Result;
use query::get_queries;

fn run(cli: cli::Cli) -> Result<()> {
    let queries = get_queries(&cli.query_dir, &cli.queries)?;

    let (file_counts, errors): (Vec<_>, Vec<_>) = cli
        .files
        .iter()
        .map(|path| Counts::from_path(path, &cli.kinds, &cli.kind_patterns, &queries))
        .partition(Result::is_ok);

    if cli.show_files {
        println!("{:?}", file_counts);
    } else {
        let grouped_counts = file_counts.into_iter().map(Result::unwrap).fold(
            BTreeMap::new(),
            |mut acc, (lang, counts)| {
                if let Some(cur) = acc.get_mut(&lang) {
                    *cur += counts;
                } else {
                    acc.insert(lang, counts);
                }
                acc
            },
        );
        println!("{:?}", grouped_counts);
    }

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
            eprintln!("{:?}", err);
            process::exit(1);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::query::Queries;
    // use std::collections::HashMap;
    // use std::path::PathBuf;

    // TODO move this to count.rs
    // #[test]
    // fn test_counting_rust() {
    //     let files = vec![
    //         PathBuf::from("./test_data/rust1.rs"),
    //         PathBuf::from("./test_data/rust2.rs"),
    //     ];
    //     let mut expected = BTreeMap::new();
    //     expected.insert(Language::Rust, vec![35 + 20, 4 + 0, 3 + 1]);
    //     let kinds = vec!["\"".into(), ";".into()];
    //     let queries = HashMap::new();
    //     let (got, _) = get_counts(&files, &kinds, &queries);
    //     let got = group_by_language(got);
    //     assert_eq!(got, expected);
    // }
}
