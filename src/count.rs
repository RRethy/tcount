use crate::error::{Error, Result};
use crate::language::Language;
use crate::query::Queries;
use crate::tree;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::ops::AddAssign;
use std::path::Path;
use tree_sitter::{Node, Parser, QueryCursor};

/// Counts contains the cumulative totals for the how many files, number of tokens, number of nodes
/// matching each kind specified by --kind, and number of matches for each query specified by
/// --query.
#[derive(Debug, Eq, PartialEq)]
pub struct Counts<'a> {
    pub nfiles: u64,
    pub ntokens: u64,
    pub nkinds: Vec<u64>,
    pub nkind_patterns: Vec<u64>,
    pub nqueries: HashMap<&'a String, u64>,
}

// TODO test this
impl<'a> AddAssign for Counts<'a> {
    fn add_assign(&mut self, other: Self) {
        #[inline(always)]
        fn add(l: &mut Vec<u64>, r: &Vec<u64>) {
            l.iter_mut().zip(r).for_each(|(a, b)| *a += b);
        }
        self.nfiles += other.nfiles;
        self.ntokens += other.ntokens;
        add(&mut self.nkinds, &other.nkinds);
        add(&mut self.nkind_patterns, &other.nkind_patterns);
        other.nqueries.iter().for_each(|(query, v1)| {
            self.nqueries
                .entry(query)
                .and_modify(|v2| *v2 += v1)
                .or_insert(*v1);
        });
    }
}

impl<'a> Counts<'a> {
    pub fn from_path(
        path: impl AsRef<Path>,
        kinds: &Vec<String>,
        kind_patterns: &Vec<Regex>,
        queries: &'a Queries,
    ) -> Result<(Language, Self)> {
        let lang = Language::from(path.as_ref());
        let ts_lang = {
            match lang.get_treesitter_language() {
                Ok(ts_lang) => ts_lang,
                Err(_) => {
                    return Ok((
                        Language::Unsupported,
                        Counts {
                            nfiles: 1,
                            ntokens: 0,
                            nkinds: vec![0; kinds.len()],
                            nkind_patterns: vec![0; kind_patterns.len()],
                            nqueries: HashMap::new(),
                        },
                    ));
                }
            }
        };

        let mut ntokens = 0;
        let mut nkinds = vec![0; kinds.len()];
        let mut nkind_patterns = vec![0; kind_patterns.len()];
        let mut nqueries = HashMap::new();

        let text = fs::read_to_string(path)?;
        let mut parser = Parser::new();
        parser.set_language(ts_lang)?;
        let mut qcursor = QueryCursor::new();
        let text_callback = |n: Node| &text[n.byte_range()];
        match parser.parse(&text, None) {
            Some(tree) => {
                if let Some(queries) = queries.get(&lang) {
                    queries.iter().for_each(|(name, query)| {
                        nqueries.insert(
                            name,
                            qcursor
                                .matches(query, tree.root_node(), text_callback)
                                .count() as u64,
                        );
                    });
                }

                tree::traverse(&tree, |node| {
                    if !node.is_missing() {
                        // count each terminal node which is the closest we can get to counting
                        // tokens. For some tokens this is a bit misleading since they can have
                        // children (e.g. string_literal in rust), but it's the closest we can
                        // acheive with tree-sitter.
                        if node.child_count() == 0 && !node.is_extra() && node.parent().is_some() {
                            ntokens += 1;
                        }

                        // count each --kinds that matches the current nodes kind
                        kinds.iter().enumerate().for_each(|(i, kind)| {
                            if kind == node.kind() {
                                nkinds[i] += 1;
                            }
                        });

                        kind_patterns.iter().enumerate().for_each(|(i, kind)| {
                            if kind.is_match(node.kind()) {
                                nkind_patterns[i] += 1;
                            }
                        });
                    }
                });
                Ok((
                    lang,
                    Counts {
                        nfiles: 1,
                        ntokens,
                        nkinds,
                        nkind_patterns,
                        nqueries,
                    },
                ))
            }
            None => Err(Error::Parser),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;
    use std::collections::HashMap;

    fn queries() -> Queries {
        let mut map = HashMap::new();
        let rust = Language::Rust;
        let go = Language::Go;
        map.insert(
            rust.clone(),
            hashmap! {
                "comment".into() => tree_sitter::Query::new(
                        rust.get_treesitter_language().unwrap(),
                        "[(line_comment) (block_comment)]",
                    )
                    .unwrap(),
                "string_literal".into() => tree_sitter::Query::new(
                    rust.get_treesitter_language().unwrap(),
                    "[(string_literal) (raw_string_literal)]",
                )
                .unwrap(),
            },
        );
        map.insert(
            go.clone(),
            hashmap! {
                    "string_literal".into() => tree_sitter::Query::new(
                        go.get_treesitter_language().unwrap(),
                        "[(interpreted_string_literal) (raw_string_literal)]",
                    )
                    .unwrap(),
            },
        );
        map
    }

    #[test]
    fn counting_unsupported_language() {
        let queries = HashMap::new();
        let got = Counts::from_path(
            "test_data/unsupported.abc",
            &Vec::new(),
            &Vec::new(),
            &queries,
        );
        let expected = (
            Language::Unsupported,
            Counts {
                nfiles: 1,
                ntokens: 0,
                nkinds: Vec::new(),
                nkind_patterns: Vec::new(),
                nqueries: HashMap::new(),
            },
        );
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_nothing() {
        let queries = queries();
        let got = Counts::from_path("test_data/empty.rs", &Vec::new(), &Vec::new(), &queries);
        let comment = String::from("comment");
        let string_literal = String::from("string_literal");
        let expected = (
            Language::Rust,
            Counts {
                nfiles: 1,
                ntokens: 0,
                nkinds: Vec::new(),
                nkind_patterns: Vec::new(),
                nqueries: hashmap! {&comment => 0, &string_literal => 0},
            },
        );
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_tokens() {
        let queries = HashMap::new();
        let got = Counts::from_path("test_data/rust1.rs", &Vec::new(), &Vec::new(), &queries);
        let expected = (
            Language::Rust,
            Counts {
                nfiles: 1,
                ntokens: 33,
                nkinds: Vec::new(),
                nkind_patterns: Vec::new(),
                nqueries: HashMap::new(),
            },
        );
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_tokens_for_invalid_syntax() {
        let queries = HashMap::new();
        let got = Counts::from_path("test_data/invalid.rs", &Vec::new(), &Vec::new(), &queries);
        let expected = (
            Language::Rust,
            Counts {
                nfiles: 1,
                ntokens: 30,
                nkinds: Vec::new(),
                nkind_patterns: Vec::new(),
                nqueries: HashMap::new(),
            },
        );
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_node_kinds() {
        let queries = HashMap::new();
        let got = Counts::from_path(
            "test_data/rust1.rs",
            &vec!["identifier".into(), "::".into()],
            &Vec::new(),
            &queries,
        );
        let expected = (
            Language::Rust,
            Counts {
                nfiles: 1,
                ntokens: 33,
                nkinds: vec![8, 3],
                nkind_patterns: Vec::new(),
                nqueries: HashMap::new(),
            },
        );
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_node_kind_patterns() {
        let queries = HashMap::new();
        let got = Counts::from_path(
            "test_data/rust1.rs",
            &vec!["block_comment".into(), "line_comment".into()],
            &vec![Regex::new(".*comment").unwrap()],
            &queries,
        );
        let expected = (
            Language::Rust,
            Counts {
                nfiles: 1,
                ntokens: 33,
                nkinds: vec![1, 3],
                nkind_patterns: vec![4],
                nqueries: HashMap::new(),
            },
        );
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_queries() {
        let queries = queries();
        let got = Counts::from_path("test_data/rust1.rs", &Vec::new(), &Vec::new(), &queries);
        let comment = String::from("comment");
        let string_literal = String::from("string_literal");
        let expected = (
            Language::Rust,
            Counts {
                nfiles: 1,
                ntokens: 33,
                nkinds: Vec::new(),
                nkind_patterns: Vec::new(),
                nqueries: hashmap! {&comment => 4, &string_literal => 2},
            },
        );
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_queries_for_other_languages_are_not_used() {
        let queries = queries();
        let got = Counts::from_path("test_data/ruby.rb", &Vec::new(), &Vec::new(), &queries);
        let expected = (
            Language::Ruby,
            Counts {
                nfiles: 1,
                ntokens: 10,
                nkinds: Vec::new(),
                nkind_patterns: Vec::new(),
                nqueries: HashMap::new(),
            },
        );
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_everything() {
        let queries = queries();
        let got = Counts::from_path(
            "test_data/rust1.rs",
            &vec!["block_comment".into(), "line_comment".into()],
            &vec![Regex::new(".*comment").unwrap()],
            &queries,
        );
        let comment = String::from("comment");
        let string_literal = String::from("string_literal");
        let expected = (
            Language::Rust,
            Counts {
                nfiles: 1,
                ntokens: 33,
                nkinds: vec![1, 3],
                nkind_patterns: vec![4],
                nqueries: hashmap! {&comment => 4, &string_literal => 2},
            },
        );
        assert_eq!(expected, got.unwrap());
    }
}
