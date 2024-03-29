use crate::error::Result;
use crate::language::Language;
use std::collections::HashMap;
use std::env::var;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum QueryKind {
    Match,
    Captures(Vec<String>),
}

#[derive(Debug)]
pub struct Query {
    pub name: String,
    pub kind: QueryKind,
    pub langs: HashMap<Language, tree_sitter::Query>,
}

impl FromStr for Query {
    type Err = String;

    /// Searches for tree-sitter queries based on the based on the string argument with the syntax
    /// "{query name}(@{capture name}(,{capture_name})*)?"
    /// For example,
    ///     "foo" -> Query name of "foo"
    ///     "foo@bar" -> Query name of "foo" and a capture name of "bar"
    ///     "foo@bar,baz" -> Query name of "foo" and capture names of "bar" and "baz"
    ///
    /// A query directory is as a directory with subdirectories that contain query files with the
    /// name "{query name}.scm". These subdirectories are named based on the language those queries
    /// are written for. For example, a query directory tcount_queries/ could have a "comments" query
    /// for rust and ruby named queries/rust/comments.scm and queries/ruby/comments.scm, respectively.
    ///
    /// When searching for these query files, first, the present working directory is searched for
    /// a query directory named .tcount_queries/, then $XDG_CONFIG_HOME/tcount (defaults to $HOME/.config/tcount)
    /// is searched for query directories that match $XDG_CONFIG_HOME/tcount/* (conflicting query files
    /// result in undefined behaviour).
    fn from_str(name: &str) -> std::result::Result<Self, Self::Err> {
        let (kind, name) = match name.find('@') {
            Some(i) => (
                QueryKind::Captures(name[i + 1..].split(',').map(String::from).collect()),
                &name[..i],
            ),
            None => (QueryKind::Match, name),
        };

        let queries: Option<HashMap<Language, tree_sitter::Query>> = vec![
            // look in pwd for a .tcount_queries/ dir
            format!(".tcount_queries/*/{}.scm", name),
            // look in $XDG_CONFIG_HOME/tcount/* for a dir with queries
            format!(
                "{}/tcount/*/*/{}.scm",
                if !var("XDG_CONFIG_HOME").unwrap_or(String::new()).is_empty() {
                    var("XDG_CONFIG_HOME").unwrap()
                } else {
                    "~/.config".into()
                },
                name
            ),
        ]
        .iter()
        .map(|dir_glob| glob::glob(dir_glob.as_str()))
        .filter_map(|res| res.ok())
        .map(|entries| {
            entries
                .into_iter()
                .filter_map(|res| res.ok())
                .map(|path| {
                    let lang = Language::from(path.parent().unwrap_or(&PathBuf::new()));
                    let tree_sitter_lang = lang.get_treesitter_language()?;
                    let query_str = fs::read_to_string(&path)?;
                    let mut query = tree_sitter::Query::new(tree_sitter_lang, &query_str)?;
                    match &kind {
                        QueryKind::Captures(captures) => {
                            // Disable all captures that aren't used.
                            let unused_captures: Vec<String> = query
                                .capture_names()
                                .iter()
                                .filter(|name| !captures.contains(name))
                                .map(String::clone)
                                .collect();
                            unused_captures
                                .iter()
                                .for_each(|name| query.disable_capture(name));
                        }
                        QueryKind::Match => {
                            let names: Vec<String> = query.capture_names().into();
                            names.iter().for_each(|name| query.disable_capture(name));
                        }
                    }
                    Ok((lang, query))
                })
                .filter_map(Result::ok)
                .collect()
        })
        .find(|map: &HashMap<Language, tree_sitter::Query>| !map.is_empty());

        if let Some(queries) = queries {
            Ok(Query {
                name: name.to_string(),
                kind,
                langs: queries,
            })
        } else {
            Err(format!("Unabled to find query for {}", name))
        }
    }
}
