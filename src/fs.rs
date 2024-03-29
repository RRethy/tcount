use crate::error::{Error, Result};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

/// Recursively iterate over @paths and produce a parallel and unordered iterator over the files encountered.
pub fn iter_paths(
    paths: &[impl AsRef<Path>],
    no_git: bool,
    count_hidden: bool,
    no_dot_ignore: bool,
    no_parent_ignore: bool,
) -> impl ParallelIterator<Item = Result<PathBuf>> {
    let mut builder = ignore::WalkBuilder::new(paths.first().unwrap());
    let _ = &paths[1..].iter().for_each(|path| {
        builder.add(path);
    });
    // We synchronously walk the filesystem and use rayon's .par_bridge to create a parallel
    // iterator over these results for processing. This is just as efficient (and sometimes more
    // so) as asynchronously walking the filesystem (with channels for inter-thread communication)
    // since the limiting factor is the parsing of each file, not the walking of the filesystem.
    builder
        .git_exclude(!no_git)
        .git_global(!no_git)
        .git_ignore(!no_git)
        .hidden(!count_hidden)
        .ignore(!no_dot_ignore)
        .parents(!no_parent_ignore)
        .build()
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
