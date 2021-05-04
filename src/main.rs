use std::collections::HashMap;
use std::process;
use structopt::StructOpt;

mod cli;
mod count;
mod error;
mod language;
mod print;
mod query;
mod tree;

use count::Counts;
use error::Result;
use query::get_queries;

fn run(cli: cli::Cli) -> Result<()> {
    let queries = get_queries(&cli.queries_dir, &cli.queries)?;

    let (file_counts, errors): (Vec<_>, Vec<_>) = cli
        .files
        .iter()
        .map(|path| Counts::from_path(path, &cli.kinds, &cli.kind_patterns, &queries))
        .partition(Result::is_ok);

    if cli.show_files {
        // print::grouped_by_file(&file_counts.into_iter().map(Result::unwrap).collect());
    } else {
        let grouped_counts = file_counts.into_iter().map(Result::unwrap).fold(
            HashMap::new(),
            |mut acc, (lang, counts)| {
                if let Some(cur) = acc.get_mut(&lang) {
                    *cur += counts;
                } else {
                    acc.insert(lang, counts);
                }
                acc
            },
        );
        print::grouped_by_language(
            &grouped_counts,
            &cli.kinds,
            &cli.kind_patterns,
            &cli.queries,
        );
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
