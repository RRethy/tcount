# Queries

`tc` has the ability to count [Tree-sitter queries (skim this first)](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries). These allow you to match patterns in the syntax tree for more fine grained counting.

**Note**: Simple queries can probably be replaced with --kind-pattern. (e.g. `--kind-pattern=".*comment"` usually suffices to count comments.)

To count a query named `foo`, pass the option `--query=foo`. If your query `foo` has capture groups `bar` and `baz` then these capture groups can instead be counted with `--query=foo@bar,baz`.

## Query Directories

All queries are user-defined inside query directories. A query directory is a directory that contains subdirectories named for a specific language which themselves contain query files named `{query}.scm` files for each query. For example, a query directory could have a structure similar to:

```
query_directory/
├── rust/
│   ├── functions_query.scm
│   └── keywords_query.scm
├── ruby/
│   ├── functions_query.scm
│   └── keywords_query.scm
└── go
    └── functions_query.scm
```

In the above query directory, we have a two queries `functions_query` (supports Rust, Ruby, and Go) and `keywords_query` (supports Rust and Ruby). If a language doesn't have a specific query file, then it will be counted as 0.

The language directories must be named according to third column in the output from `--list-languages` (e.g. `C#` queries go under `c_sharp`).

### Query Directory Locations

When `--query=foo` is used, `tc` will begin looking for the `foo` query inside a query directory. When it find a `foo` query for any language, it will stop looking for other query directories and only languages with `foo.scm` in the current query directory will have the query counted.

When looking for a query directory with the query, the following locations will be searched:

1. `$PWD/.tc_queries/` will be considered a query directory
2. `$XDG_CONFIG_HOME/tc/*/` will each (post-expansion) be considered query directories

There is no guarantee about the expansion order for #2 so conflicting queries results in undefined behaviour as to which is used.

## Writing your own queries

The most important resource are the [Tree-sitter Query Docs](https://tree-sitter.github.io/tree-sitter/using-parsers#pattern-matching-with-queries).

[nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter/tree/master/queries) has a lot of queries that can be copy pasted.

Most parsers generated using Tree-sitter have a `queries/` directory which have queries that can be copy pasted. For example, [tree-sitter-go](https://github.com/tree-sitter/tree-sitter-go/tree/master/queries).
