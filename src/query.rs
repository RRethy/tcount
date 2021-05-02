use crate::error::Result;
use crate::language::Language;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tree_sitter::Query;

// TODO these have to be named for printing later
pub type Queries = HashMap<Language, Vec<Query>>;

pub fn get_queries(query_dir: &PathBuf, queries: &Vec<String>) -> Result<Queries> {
    let mut langs = HashMap::new();

    if queries.len() == 0 {
        return Ok(langs.into());
    }

    for path in fs::read_dir(query_dir)? {
        if let Ok(path) = path {
            let path = path.path();
            let lang = Language::from(path.as_path().as_ref());
            if let Ok(ts_lang) = lang.get_treesitter_language() {
                for query in queries {
                    let path = path.join(format!("{}.scm", query));
                    if path.is_file() {
                        langs
                            .entry(lang.clone())
                            .or_insert(Vec::new())
                            .push(Query::new(ts_lang, fs::read_to_string(&path)?.as_ref())?);
                    }
                }
            }
        }
    }

    Ok(langs.into())
}
