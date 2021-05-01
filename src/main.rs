// use std::collections::BTreeMap;
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
// use language::Language;
use query::get_queries;

// fn group_by_language(counts: Vec<(Language, Vec<u64>)>) -> BTreeMap<Language, Vec<u64>> {
//     counts
//         .into_iter()
//         .fold(BTreeMap::new(), |mut acc, (lang, counts)| {
//             if let Some(cur) = acc.get_mut(&lang) {
//                 cur.iter_mut()
//                     .zip(&counts)
//                     .for_each(|(cum, val)| *cum += val);
//             } else {
//                 acc.insert(lang, counts);
//             }
//             acc
//         })
// }

fn run(cli: cli::Cli) -> Result<()> {
    let queries = get_queries(&cli.query_dir, &cli.query)?;

    let (file_counts, errors): (Vec<_>, Vec<_>) = cli
        .files
        .iter()
        .map(|path| Counts::from_file(path, &cli.kind, &queries))
        .partition(Result::is_ok);

    // let (counts, errors) = get_counts(&cli.files, &cli.kind, &queries);
    file_counts
        .into_iter()
        .map(Result::unwrap)
        .for_each(|counts| println!("{:?}", counts));

    // lang_counts
    //     .iter()
    //     .for_each(|(lang, count)| println!("{:?}: {:?}", lang, count));

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
