use crate::error::{Error, Result};
use crate::language::Language;
use crate::query::{Query, QueryKind};
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
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Counts {
    pub nfiles: u64,
    pub ntokens: u64,
    pub nkinds: Vec<u64>,
    pub nkind_patterns: Vec<u64>,
    pub nqueries: Vec<u64>,
}

impl Counts {
    pub fn empty(nkinds: usize, nkind_patterns: usize, queries: &Vec<Query>) -> Counts {
        Counts {
            nfiles: 0,
            ntokens: 0,
            nkinds: vec![0; nkinds],
            nkind_patterns: vec![0; nkind_patterns],
            nqueries: Self::nqueries(&queries, HashMap::new(), HashMap::new()),
        }
    }

    fn nqueries(
        queries: &Vec<Query>,
        nmatches: HashMap<&String, u64>,
        ncaptures: HashMap<(&String, &String), u64>,
    ) -> Vec<u64> {
        queries
            .iter()
            .flat_map(|query| match query.kind {
                QueryKind::Match => {
                    vec![nmatches.get(&query.name).unwrap_or(&0)]
                }
                QueryKind::Captures(ref names) => names
                    .iter()
                    .map(|name| ncaptures.get(&(&query.name, name)).unwrap_or(&0))
                    .collect(),
            })
            .map(|n| n.clone())
            .collect()
    }
}

impl AddAssign for Counts {
    fn add_assign(&mut self, other: Self) {
        #[inline(always)]
        // element-wise addition of two equal-sized vectors
        fn add(l: &mut Vec<u64>, r: &Vec<u64>) {
            l.iter_mut().zip(r).for_each(|(a, b)| *a += b);
        }
        self.nfiles += other.nfiles;
        self.ntokens += other.ntokens;
        add(&mut self.nkinds, &other.nkinds);
        add(&mut self.nkind_patterns, &other.nkind_patterns);
        add(&mut self.nqueries, &other.nqueries);
    }
}

impl Counts {
    pub fn from_path(
        path: impl AsRef<Path>,
        lang: &Language,
        kinds: &Vec<String>,
        kind_patterns: &Vec<Regex>,
        queries: &Vec<Query>,
    ) -> Result<Self> {
        let ts_lang = {
            match lang.get_treesitter_language() {
                Ok(ts_lang) => ts_lang,
                Err(_) => {
                    // Unsupported language gets an *empty* Counts struct
                    return Ok(Counts {
                        nfiles: 1,
                        ..Counts::empty(kinds.len(), kind_patterns.len(), queries)
                    });
                }
            }
        };

        let mut ntokens = 0;
        let mut nkinds = vec![0; kinds.len()];
        let mut nkind_patterns = vec![0; kind_patterns.len()];
        let mut nmatch_queries = HashMap::new();
        let mut ncapture_queries = HashMap::new();

        let text = fs::read_to_string(path.as_ref())?;
        let mut parser = Parser::new();
        parser
            .set_language(ts_lang)
            .expect("Unexpected internal error setting parser language");

        let mut qcursor = QueryCursor::new();
        let text_callback = |n: Node| &text[n.byte_range()];
        match parser.parse(&text, None) {
            Some(tree) => {
                queries.iter().for_each(|query| {
                    if let Some(ts_query) = query.langs.get(lang) {
                        match &query.kind {
                            QueryKind::Match => {
                                nmatch_queries.insert(
                                    &query.name,
                                    qcursor
                                        .matches(ts_query, tree.root_node(), text_callback)
                                        .count() as u64,
                                );
                            }
                            QueryKind::Captures(_) => {
                                let capture_names = ts_query.capture_names();
                                qcursor
                                    .captures(ts_query, tree.root_node(), text_callback)
                                    .for_each(|(qmatch, _)| {
                                        qmatch.captures.iter().for_each(|capture| {
                                            *ncapture_queries
                                                .entry((
                                                    &query.name,
                                                    &capture_names[capture.index as usize],
                                                ))
                                                .or_insert(0) += 1;
                                        });
                                    });
                            }
                        }
                    }
                });

                tree::traverse(&tree, |node| {
                    if !node.is_missing() {
                        // count each terminal node which is the closest we can get to counting
                        // tokens. For some tokens this is a bit misleading since they can have
                        // children (e.g. string_literal in rust), but it's the closest we can
                        // acheive with tree-sitter.
                        if node.child_count() == 0 && !node.is_extra() && node.parent().is_some() {
                            ntokens += 1;
                        }

                        // count each --kinds that matche the current nodes kind
                        kinds.iter().enumerate().for_each(|(i, kind)| {
                            if kind == node.kind() {
                                nkinds[i] += 1;
                            }
                        });

                        // count each --kind_patterns that match the current nodes kind
                        kind_patterns.iter().enumerate().for_each(|(i, kind)| {
                            if kind.is_match(node.kind()) {
                                nkind_patterns[i] += 1;
                            }
                        });
                    }
                });
                let nqueries = Counts::nqueries(queries, nmatch_queries, ncapture_queries);
                Ok(Counts {
                    nfiles: 1,
                    ntokens,
                    nkinds,
                    nkind_patterns,
                    nqueries,
                })
            }
            None => Err(Error::Parser(path.as_ref().to_path_buf())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn queries_with_captures() -> Vec<Query> {
        vec![
            Query::from_str("comment").unwrap(),
            Query::from_str("keyword@ifelse,repeat").unwrap(),
            Query::from_str("string_literal").unwrap(),
        ]
    }

    fn queries() -> Vec<Query> {
        vec![
            Query::from_str("comment").unwrap(),
            Query::from_str("string_literal").unwrap(),
        ]
    }

    #[test]
    fn counting_unsupported_language() {
        let queries = Vec::new();
        let got = Counts::from_path(
            "test_data/unsupported.abc",
            &Language::Unsupported,
            &Vec::new(),
            &Vec::new(),
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 0,
            nkinds: Vec::new(),
            nkind_patterns: Vec::new(),
            nqueries: Vec::new(),
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_nothing() {
        let queries = queries();
        let got = Counts::from_path(
            "test_data/empty.rs",
            &Language::Rust,
            &Vec::new(),
            &Vec::new(),
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 0,
            nkinds: Vec::new(),
            nkind_patterns: Vec::new(),
            nqueries: vec![0, 0],
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_tokens() {
        let queries = Vec::new();
        let got = Counts::from_path(
            "test_data/rust1.rs",
            &Language::Rust,
            &Vec::new(),
            &Vec::new(),
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 33,
            nkinds: Vec::new(),
            nkind_patterns: Vec::new(),
            nqueries: Vec::new(),
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_tokens_for_invalid_syntax() {
        let queries = Vec::new();
        let got = Counts::from_path(
            "test_data/invalid.rs",
            &Language::Rust,
            &Vec::new(),
            &Vec::new(),
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 30,
            nkinds: Vec::new(),
            nkind_patterns: Vec::new(),
            nqueries: Vec::new(),
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_node_kinds() {
        let queries = Vec::new();
        let got = Counts::from_path(
            "test_data/rust1.rs",
            &Language::Rust,
            &vec!["identifier".into(), "::".into()],
            &Vec::new(),
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 33,
            nkinds: vec![8, 3],
            nkind_patterns: Vec::new(),
            nqueries: Vec::new(),
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_node_kind_patterns() {
        let queries = Vec::new();
        let got = Counts::from_path(
            "test_data/rust1.rs",
            &Language::Rust,
            &vec!["block_comment".into(), "line_comment".into()],
            &vec![Regex::new(".*comment").unwrap()],
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 33,
            nkinds: vec![1, 3],
            nkind_patterns: vec![4],
            nqueries: Vec::new(),
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_queries() {
        let queries = queries();
        let got = Counts::from_path(
            "test_data/rust1.rs",
            &Language::Rust,
            &Vec::new(),
            &Vec::new(),
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 33,
            nkinds: Vec::new(),
            nkind_patterns: Vec::new(),
            nqueries: vec![4, 2],
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counting_everything() {
        let queries = queries();
        let got = Counts::from_path(
            "test_data/rust1.rs",
            &Language::Rust,
            &vec!["block_comment".into(), "line_comment".into()],
            &vec![Regex::new(".*comment").unwrap()],
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 33,
            nkinds: vec![1, 3],
            nkind_patterns: vec![4],
            nqueries: vec![4, 2],
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn counts_vec_from_query_hashmap_counts() {
        let queries = queries_with_captures();
        let mut nmatches = HashMap::new();
        let comment = String::from("comment");
        let string_literal = String::from("string_literal");
        let keyword = String::from("keyword");
        let ifelse = String::from("ifelse");
        nmatches.insert(&comment, 5);
        nmatches.insert(&string_literal, 7);
        let mut ncaptures = HashMap::new();
        ncaptures.insert((&keyword, &ifelse), 3);
        let got = Counts::nqueries(&queries, nmatches, ncaptures);
        assert_eq!(vec![5, 3, 0, 7], got);
    }

    #[test]
    fn counting_queries_with_captures() {
        let queries = queries_with_captures();
        let got = Counts::from_path(
            "test_data/rust3.rs",
            &Language::Rust,
            &vec![],
            &vec![],
            &queries,
        );
        let expected = Counts {
            nfiles: 1,
            ntokens: 73,
            nkinds: vec![],
            nkind_patterns: vec![],
            nqueries: vec![4, 4, 3, 2],
        };
        assert_eq!(expected, got.unwrap());
    }

    #[test]
    fn add_assign_counts() {
        let mut c1 = Counts {
            nfiles: 30,
            ntokens: 21,
            nkinds: vec![28, 28],
            nkind_patterns: vec![29, 20, 2],
            nqueries: vec![0, 44, 55],
        };
        let c2 = Counts {
            nfiles: 19,
            ntokens: 31,
            nkinds: vec![5, 9],
            nkind_patterns: vec![6, 10, 14],
            nqueries: vec![33, 44],
        };

        c1 += c2;
        let expected = Counts {
            nfiles: 49,
            ntokens: 52,
            nkinds: vec![33, 37],
            nkind_patterns: vec![35, 30, 16],
            nqueries: vec![33, 88, 55],
        };
        assert_eq!(expected, c1);
    }
}
