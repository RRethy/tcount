use crate::error::Error;
use rayon::prelude::*;
use std::path::PathBuf;

pub fn iter_paths<'a>(
    paths: &Vec<PathBuf>,
    no_git: bool,
    count_hidden: bool,
    no_dot_ignore: bool,
    no_parent_ignore: bool,
) -> impl ParallelIterator<Item = Result<PathBuf, Error>> {
    let mut builder = ignore::WalkBuilder::new(paths.first().unwrap());
    &paths[1..].iter().for_each(|path| {
        builder.add(path);
    });
    // We synchronously walk the filesystem and using rayon's .par_bridge to create a parallel
    // iterator over these results, this iterator the parses and counts each path. This is just as
    // efficient as parallel walking of the filesystem and using some mechanism (like channels) to
    // aggregate the results afterwards (which is how tokei works).
    builder
        .git_exclude(!no_git)
        .git_global(!no_git)
        .git_ignore(!no_git)
        .hidden(!count_hidden)
        .ignore(!no_dot_ignore)
        .parents(!no_parent_ignore)
        .build()
        .into_iter()
        .par_bridge()
        .filter_map(|entry| match entry {
            Ok(dir) => {
                if dir.file_type().map_or(false, |ft| ft.is_file()) {
                    Some(Ok(dir.into_path()))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(Error::from(err))),
        })
}
