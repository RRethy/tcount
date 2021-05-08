use crate::error::Result;
use crate::language::Language;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
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
    /// A query directory is defined as a directory with subdirectories that contain query files
    /// with the name "{query name}.scm". These subdirectories are named based on the language
    /// those queries are written for. For example, a query directory queries/ could have a
    /// "comments" query for rust and ruby named queries/rust/comments.scm and
    /// queries/ruby/comments.scm, respectively.
    ///
    /// First, the present working directory is searched for a query directory named tc_queries/,
    /// then $XDG_CONFIG_HOME/tc (defaults to $HOME/.config/tc) is searched and each subdirectory
    /// is considered a query directory and searched (conflicting query files result in undefined
    /// behaviour), lastly the builtin queries directory is searched.
    fn from_str(name: &str) -> std::result::Result<Self, Self::Err> {
        let queries: Option<HashMap<Language, tree_sitter::Query>> = vec![
            // look in pwd for a tc_queries/ dir
            format!("tc_queries/*/{}.scm", name),
            // look in $XDG_CONFIG_HOME/tc/* for a dir with queries
            format!(
                "{}/tc/*/*/{}.scm",
                if env!("XDG_CONFIG_HOME").len() > 0 {
                    env!("XDG_CONFIG_HOME").to_string()
                } else {
                    format!("{}/{}", env!("HOME"), ".config")
                },
                name
            ),
            // look in the root of this repo for it's queries/ dir
            format!("{}/queries/*/{}.scm", env!("CARGO_MANIFEST_DIR"), name),
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
                    let query = tree_sitter::Query::new(tree_sitter_lang, &query_str)?;
                    Ok((lang, query))
                })
                .filter_map(Result::ok)
                .collect()
        })
        .filter(|map: &HashMap<Language, tree_sitter::Query>| map.len() > 0)
        .next();

        if let Some(queries) = queries {
            match name.find('@') {
                Some(i) => Ok(Query {
                    name: name[..i].to_string(),
                    kind: QueryKind::Captures(name[i + 1..].split(',').map(String::from).collect()),
                    langs: queries,
                }),
                None => Ok(Query {
                    name: name.to_string(),
                    kind: QueryKind::Match,
                    langs: queries,
                }),
            }
        } else {
            Err(String::from("TODO"))
        }
    }
}
