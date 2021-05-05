use std::collections::HashMap;
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
use error::Result;
use output::print;
use query::get_queries;

fn run(cli: cli::Cli) -> Result<()> {
    let queries = get_queries(&cli.queries_dir, &cli.queries)?;
    let (file_counts, errors): (Vec<_>, Vec<_>) =
        fs::walk_paths(&cli.paths, &cli.kinds, &cli.kind_patterns, &queries);

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

    if counts.len() > 0 {
        print(
            &cli.format,
            &counts,
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
    match run(cli) {
        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1);
        }
        _ => {}
    }
}
