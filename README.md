<h1 align="center">
  <a href="https://github.com/RRethy/tc">tc</a>
</h1>

<p align="center"><em>(pronounced "tee-see")</em></p>

<h4 align="center">Count your code by tokens, types of syntax tree nodes, and patterns in the syntax tree.</h4>

# Quick Start

Simply run `tc` in your project root to count tokens and files and print the results grouped by Language. E.g.,

```bash
tc
```
```txt
────────────────────────────
 Group        Files  Tokens
────────────────────────────
 Rust            18   10309
 Go               8    4539
 Ruby             6    1301
────────────────────────────
```

## Installation

```bash
cargo install --git https://github.com/RRethy/tc.git
```

## Requirements

- Lastest stable [Rust](https://www.rust-lang.org/) compiler.
- Mac or Linux (untested on Windows but most functionality should work, `--query` option likely will not work on Windows)

# tc Cookbook

**Note**: None of these use --query, see [Queries](https://github.com/RRethy/tc#Queries) for information on that option

## Compare size of each language in pwd

```bash
tc
```
```txt
────────────────────────────
 Group        Files  Tokens
────────────────────────────
 Rust            18   10309
 Go               8    4539
 Ruby             6    1301
────────────────────────────
```

## Top 5 files by token count

```bash
tc --groupby=file --top=5
```
```txt
──────────────────────────────────
 Group              Files  Tokens
──────────────────────────────────
 ./src/count.rs         1    2451
 ./src/language.rs      1    1685
 ./src/main.rs          1    1214
 ./src/output.rs        1    1157
 ./src/cli.rs           1     757
──────────────────────────────────
```

## Compare size of two directories

```bash
tc --groupby=arg go/scc/ rust/tokei/
```
```txt
─────────────────────────────────────────────
 Group                         Files  Tokens 
─────────────────────────────────────────────
 go/scc                          170  479544 
 rust/tokei                      152   39797 
─────────────────────────────────────────────
```

## Compare size of a Go file and a Rust file

```bash
tc --groupby=file foo.go foo.rs
```
```txt
────────────────────────────
 Group        Files  Tokens
────────────────────────────
 foo.rs           1    1214
 foo.go           1     757
────────────────────────────
```

## Count comments for each language

```bash
tc --kind-pattern=".*comment"
```
```txt
──────────────────────────────────────────────────
 Group        Files  Tokens  Pattern(.*comment)
──────────────────────────────────────────────────
 Rust            18   10309                    78
 Go               7    1302                    35
 Ruby             4     802                    12
──────────────────────────────────────────────────
```

**Note**: Comment nodes can have different names depending on the parser. For a language, you can look in the node-types.json file in the parser repo to see what names are given to different nodes (e.g. [Go Parser Repo's node-types.json](https://github.com/tree-sitter/tree-sitter-go/blob/master/src/node-types.json))

## Track change in project size over time

```bash
tc --format=csv > tc-$(date +%m-%d-%Y).csv
```

These CSV files can then be read and graphed using your tool of choice.

## Compare size of all Go files vs all Rust files in foo/

```bash
tc --whitelist Go Rust -- foo/
```
```txt
──────────────────────
 Group  Files  Tokens
──────────────────────
 Rust       9    9034
 Go         6    2011
──────────────────────
```

## See supported languages

```bash
tc --list-languages
```

# Why count tokens instead of lines

TODO

# Usage

```bash
tc -h
TODO
```

# Counting Tree-sitter Queries

See [QUERIES.md](https://github.com/RRethy/tc/blob/master/QUERIES.md)

# Performance

`tc` parses each file using a Tree-sitter parser to create a full syntax tree. This takes more time than only counting lines of code/comments so programs like [tokei](https://github.com/XAMPPRocky/tokei), [scc](https://github.com/boyter/scc), and [cloc](https://github.com/AlDanial/cloc) will typically be faster than `tc`.

Here are some benchmarks using [hyperfine](https://github.com/sharkdp/hyperfine) to give an overview of how much slower it is than line counting programs:

[**CPython**](https://github.com/python/cpython.git)

[**Redis**](https://github.com/redis/redis)

[**Linux**](https://github.com/torvalds/linux)

# Limitations

TODO

- determining what is a token
- nested languages

# Acknowledgements

All parsing is done using [Tree-sitter](https://tree-sitter.github.io/tree-sitter) parsers
