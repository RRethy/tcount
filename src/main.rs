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

use cli::OrderBy;
use count::Counts;
use error::{Error, Result};
use language::Language;
use output::print;
use query::get_queries;

fn run(cli: cli::Cli) -> Result<()> {
    let lang_whitelist: HashSet<String> =
        HashSet::from_iter(cli.language_whitelist.iter().cloned());
    let queries = get_queries(&cli.queries_dir, &cli.queries)?;

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
        if lang_whitelist.len() == 0 || lang_whitelist.contains(&lang.to_string()) {
            match Counts::from_path(&path, &lang, &cli.kinds, &cli.kind_patterns, &queries) {
                Ok(counts) => Ok((lang, path, counts)),
                Err(err) => Err(err),
            }
        } else {
            Err(Error::LanguageNotWhitelisted(lang))
        }
    })
    .partition(Result::is_ok);

    let mut counts: Vec<(String, Counts)> = if cli.show_files {
        file_counts
            .into_iter()
            .map(Result::unwrap)
            .map(|(_lang, path, count)| (path.display().to_string(), count))
            .collect()
    } else {
        file_counts
            .into_iter()
            .map(Result::unwrap)
            .fold(HashMap::new(), |mut acc, (lang, _path, counts)| {
                if let Some(cur) = acc.get_mut(&lang.to_string()) {
                    *cur += counts;
                } else {
                    acc.insert(lang.to_string(), counts);
                }
                acc
            })
            .into_iter()
            .collect()
    };
    match cli.order_by {
        OrderBy::Language | OrderBy::File => counts.sort_by(|(l1, _c1), (l2, _c2)| l1.cmp(l2)),
        OrderBy::NumFiles => counts.sort_by(|(_l1, c1), (_l2, c2)| c2.nfiles.cmp(&c1.nfiles)),
        OrderBy::Tokens => counts.sort_by(|(_l1, c1), (_l2, c2)| c2.ntokens.cmp(&c1.ntokens)),
    }

    let totals: Option<Counts> = if cli.hide_totals {
        None
    } else {
        Some(counts.iter().fold(
            Counts {
                nfiles: 0,
                ntokens: 0,
                nkinds: vec![0; cli.kinds.len()],
                nkind_patterns: vec![0; cli.kind_patterns.len()],
                nqueries: HashMap::new(),
            },
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
            &cli.queries,
        );
    } else {
        println!("No files found.");
    }

    if cli.verbose {
        errors.into_iter().map(Result::unwrap_err).for_each(|err| {
            eprintln!("{}", err);
        });
    }
    Ok(())
}

fn main() {
    let cli = cli::Cli::from_args();

    if cli.list_languages {
        println!("{}", Language::list_all());
    } else {
        match run(cli) {
            Err(err) => {
                eprintln!("{:?}", err);
                process::exit(1);
            }
            _ => {}
        }
    }
}
