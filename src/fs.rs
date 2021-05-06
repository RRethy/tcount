use crate::cli::Cli;
use crate::count::Counts;
use crate::error::Result;
use crate::language::Language;
use crate::query::Queries;
use rayon::prelude::*;
use std::path::PathBuf;

pub fn walk_paths<'a>(
    cli: &Cli,
    queries: &'a Queries,
) -> (
    Vec<Result<(Language, PathBuf, Counts<'a>)>>,
    Vec<Result<(Language, PathBuf, Counts<'a>)>>,
) {
    let paths = &cli.paths;
    let kinds = &cli.kinds;
    let kind_patterns = &cli.kind_patterns;

    let mut builder = ignore::WalkBuilder::new(paths.first().unwrap());
    &paths[1..].iter().for_each(|path| {
        builder.add(path);
    });
    // We synchronously walk the filesystem and using rayon's .par_bridge to create a parallel
    // iterator over these results, this iterator the parses and counts each path. This is just as
    // efficient as parallel walking of the filesystem and using some mechanism (like channels) to
    // aggregate the results afterwards (which is how tokei works).
    builder
        .git_exclude(!cli.no_git)
        .git_global(!cli.no_git)
        .git_ignore(!cli.no_git)
        .hidden(!cli.count_hidden)
        .ignore(!cli.no_dot_ignore)
        .parents(!cli.no_parent_ignore)
        .build()
        .into_iter()
        .par_bridge()
        .filter_map(|entry| {
            let res = match entry {
                Ok(dir) => {
                    if dir.file_type().map_or(false, |ft| ft.is_file()) {
                        let path = dir.into_path();
                        Some(
                            match Counts::from_path(&path, &kinds, &kind_patterns, &queries) {
                                Ok((lang, counts)) => Ok((lang, path, counts)),
                                Err(err) => Err(err),
                            },
                        )
                    } else {
                        None
                    }
                }
                Err(err) => Some(Err(err.into())),
            };
            res
        })
        .partition(Result::is_ok)
}
