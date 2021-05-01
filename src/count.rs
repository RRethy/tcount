use crate::error::{Error, Result};
use crate::language::Language;
use crate::query::Queries;
use crate::tree;
use std::fs;
use std::path::Path;
use tree_sitter::{Node, Parser, QueryCursor};

/// Counts contains the cumulative totals for the how many files, number of tokens, number of nodes
/// matching each kind specified by --kind, and number of matches for each query specified by
/// --query.
#[derive(Debug)]
pub struct Counts {
    lang: Language,
    nfiles: u64,
    ntokens: u64,
    nkinds: Vec<u64>,
    nqueries: Vec<u64>,
}

impl Counts {
    pub fn from_file(
        path: impl AsRef<Path>,
        kinds: &Vec<String>,
        queries: &Queries,
    ) -> Result<Self> {
        let lang = Language::from(path.as_ref());
        let ts_lang = {
            match lang.get_treesitter_language() {
                Ok(ts_lang) => ts_lang,
                Err(_) => {
                    return Ok(Counts {
                        lang: Language::Unsupported,
                        nfiles: 1,
                        ntokens: 0,
                        nkinds: Vec::new(),
                        nqueries: Vec::new(),
                    });
                }
            }
        };

        let mut counts = Counts {
            lang: lang.clone(),
            nfiles: 1,
            ntokens: 0,
            nkinds: vec![0; kinds.len()],
            nqueries: vec![0; queries.get(&lang).unwrap_or(&Vec::new()).len()],
        };

        let text = fs::read_to_string(path)?;
        let mut parser = Parser::new();
        parser.set_language(ts_lang)?;
        let mut qcursor = QueryCursor::new();
        let text_callback = |n: Node| &text[n.byte_range()];
        match parser.parse(&text, None) {
            Some(tree) => {
                if let Some(queries) = queries.get(&lang) {
                    queries.iter().enumerate().for_each(|(i, query)| {
                        if let Some(query) = query {
                            counts.nqueries[i] += qcursor
                                .matches(query, tree.root_node(), text_callback)
                                .count() as u64;
                        }
                    });
                }

                tree::traverse(&tree, |node| {
                    if !node.is_missing() {
                        // count each terminal node which is the closest we can get to counting
                        // tokens. For some tokens this is a bit misleading since they can have
                        // children (e.g. string_literal in rust), but it's the closest we can
                        // acheive with tree-sitter.
                        if node.child_count() == 0 && !node.is_extra() {
                            counts.ntokens += 1;
                        }

                        // count each --kind that matches the current nodes kind
                        kinds.iter().enumerate().for_each(|(i, kind)| {
                            if kind == node.kind() {
                                counts.nkinds[i] += 1;
                            }
                        });
                    }
                });
                Ok(counts)
            }
            None => Err(Error::Parser),
        }
    }
}
