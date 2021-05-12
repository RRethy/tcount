use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

mod cli;
mod count;
mod error;
mod fs;
mod language;
mod output;
mod query;
mod tree;

use cli::{GroupBy, SortBy};
use count::Counts;
use error::{Error, Result};
use language::Language;
use output::print;

fn get_counts_for_paths(
    paths: &Vec<impl AsRef<Path>>,
    cli: &cli::Cli,
    whitelist: &HashSet<String>,
    blacklist: &HashSet<String>,
) -> (Vec<(Language, PathBuf, Counts)>, Vec<Error>) {
    let (file_counts, errors): (Vec<_>, Vec<_>) = fs::iter_paths(
        paths,
        cli.no_git,
        cli.count_hidden,
        cli.no_dot_ignore,
        cli.no_parent_ignore,
    )
    .map(|res| {
        let path = res?;
        let lang = Language::from(path.as_ref());
        let ignore_path = if whitelist.len() == 0 {
            blacklist.len() == 0 || !blacklist.contains(&lang.to_string())
        } else {
            whitelist.contains(&lang.to_string())
        };

        if ignore_path {
            let counts = Counts::from_path(&path, &lang, &cli.kind, &cli.kind_pattern, &cli.query)?;
            Ok((lang, path, counts))
        } else {
            Err(Error::LanguageIgnored(path, lang))
        }
    })
    .partition(Result::is_ok);
    (
        file_counts.into_iter().map(Result::unwrap).collect(),
        errors.into_iter().map(Result::unwrap_err).collect(),
    )
}

fn run(cli: cli::Cli) -> Result<()> {
    let whitelist: HashSet<String> = HashSet::from_iter(cli.whitelist.iter().cloned());
    let blacklist: HashSet<String> = HashSet::from_iter(cli.blacklist.iter().cloned());

    let (mut counts, errors): (Vec<(String, Counts)>, Vec<Error>) = match cli.group_by {
        GroupBy::Language => {
            let (counts, errors) = get_counts_for_paths(&cli.paths, &cli, &whitelist, &blacklist);
            let counts = counts
                .into_iter()
                .fold(HashMap::new(), |mut acc, (lang, _path, counts)| {
                    if let Some(cur) = acc.get_mut(&lang.to_string()) {
                        *cur += counts;
                    } else {
                        acc.insert(lang.to_string(), counts);
                    }
                    acc
                })
                .into_iter()
                .collect();
            (counts, errors)
        }
        GroupBy::File => {
            let (counts, errors) = get_counts_for_paths(&cli.paths, &cli, &whitelist, &blacklist);
            let counts = counts
                .into_iter()
                .map(|(_lang, path, count)| (path.display().to_string(), count))
                .collect();
            (counts, errors)
        }
        GroupBy::Arg => {
            let (counts, errors): (Vec<_>, Vec<_>) = cli
                .paths
                .par_iter()
                .map(|path| {
                    let (counts, errors) =
                        get_counts_for_paths(&vec![path], &cli, &whitelist, &blacklist);
                    let counts = counts.into_iter().fold(
                        Counts::empty(cli.kind.len(), cli.kind_pattern.len(), &cli.query),
                        |mut acc, (_lang, _path, counts)| {
                            acc += counts.clone();
                            acc
                        },
                    );
                    ((path.display().to_string(), counts), errors)
                })
                .unzip();
            (counts, errors.into_iter().flatten().collect())
        }
    };

    match cli.sort_by {
        // sort asc lexographical order on either language or file
        SortBy::Group => counts.sort_by(|(l1, _c1), (l2, _c2)| l1.cmp(l2)),
        // sort desc numerical order
        SortBy::NumFiles => counts.sort_by(|(_l1, c1), (_l2, c2)| c2.nfiles.cmp(&c1.nfiles)),
        // sort desc numerical order
        SortBy::Tokens => counts.sort_by(|(_l1, c1), (_l2, c2)| c2.ntokens.cmp(&c1.ntokens)),
    }

    let totals: Option<Counts> = if cli.show_totals {
        Some(counts.iter().fold(
            Counts::empty(cli.kind.len(), cli.kind_pattern.len(), &cli.query),
            |mut cur, (_, counts)| {
                cur += counts.clone();
                cur
            },
        ))
    } else {
        None
    };

    if counts.len() > 0 {
        print(
            &cli.format,
            counts,
            totals,
            &cli.kind,
            &cli.kind_pattern,
            &cli.query,
        );
    } else {
        println!("No files found.");
    }

    errors
        .into_iter()
        .filter(|err| err.should_show(cli.verbose))
        .for_each(|err| {
            eprintln!("{}", err);
        });
    Ok(())
}

fn main() {
    let cli = cli::Cli::from_args();

    if cli.list_languages {
        Language::print_all();
    } else {
        match run(cli) {
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
            _ => {}
        }
    }
}
