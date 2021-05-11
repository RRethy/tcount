use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
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

fn run(cli: cli::Cli) -> Result<()> {
    let whitelist: HashSet<String> = HashSet::from_iter(cli.whitelist.iter().cloned());
    let blacklist: HashSet<String> = HashSet::from_iter(cli.blacklist.iter().cloned());

    let (file_counts, errors): (Vec<_>, Vec<_>) = fs::iter_paths(
        &cli.paths,
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
            let counts =
                Counts::from_path(&path, &lang, &cli.kinds, &cli.kind_patterns, &cli.query)?;
            Ok((lang, path, counts))
        } else {
            Err(Error::LanguageIgnored(path, lang))
        }
    })
    .partition(Result::is_ok);

    let counts = file_counts.into_iter().map(Result::unwrap);
    let mut counts: Vec<(String, Counts)> = match cli.group_by {
        GroupBy::Language => counts
            .fold(HashMap::new(), |mut acc, (lang, _path, counts)| {
                if let Some(cur) = acc.get_mut(&lang.to_string()) {
                    *cur += counts;
                } else {
                    acc.insert(lang.to_string(), counts);
                }
                acc
            })
            .into_iter()
            .collect(),
        GroupBy::File => counts
            .map(|(_lang, path, count)| (path.display().to_string(), count))
            .collect(),
    };

    match cli.sort_by {
        // sort asc lexographical order on either language or file
        SortBy::Group => counts.sort_by(|(l1, _c1), (l2, _c2)| l1.cmp(l2)),
        // sort desc numerical order
        SortBy::NumFiles => counts.sort_by(|(_l1, c1), (_l2, c2)| c2.nfiles.cmp(&c1.nfiles)),
        // sort desc numerical order
        SortBy::Tokens => counts.sort_by(|(_l1, c1), (_l2, c2)| c2.ntokens.cmp(&c1.ntokens)),
    }

    let totals: Option<Counts> = if cli.hide_totals {
        None
    } else {
        Some(counts.iter().fold(
            Counts::empty(cli.kinds.len(), cli.kind_patterns.len(), &cli.query),
            |mut cur, (_, counts)| {
                cur += counts.clone();
                cur
            },
        ))
    };

    if counts.len() > 0 {
        print(
            &cli.format,
            counts,
            totals,
            &cli.kinds,
            &cli.kind_patterns,
            &cli.query,
        );
    } else {
        println!("No files found.");
    }

    errors
        .into_iter()
        .map(Result::unwrap_err)
        .filter(|err| err.should_show(cli.verbose))
        .for_each(|err| {
            eprintln!("{}", err);
        });
    Ok(())
}

fn main() {
    let cli = cli::Cli::from_args();

    if cli.list_languages {
        println!("{}", Language::list_all());
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
