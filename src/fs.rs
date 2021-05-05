use crate::count::Counts;
use crate::error::Result;
use crate::language::Language;
use crate::query::Queries;
use rayon::prelude::*;
use regex::Regex;
use std::path::PathBuf;

pub fn walk_paths<'a>(
    paths: &Vec<PathBuf>,
    kinds: &Vec<String>,
    kind_patterns: &Vec<Regex>,
    queries: &'a Queries,
) -> (
    Vec<Result<(Language, PathBuf, Counts<'a>)>>,
    Vec<Result<(Language, PathBuf, Counts<'a>)>>,
) {
    let mut builder = ignore::WalkBuilder::new(paths.first().unwrap());
    &paths[1..].iter().for_each(|path| {
        builder.add(path);
    });
    // Synchronously walking the filesystem and using rayon's .par_bridge to create a parallel
    // iterator and collect the results outperforms using ignore::WalkBuilder::build_parallel to
    // walk the file system asynchronously then using channels to collect the results.
    builder
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
