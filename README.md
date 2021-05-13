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

## Supported languages

```bash
tc --list-languages
```
```
──────────────────────────────────────────────────────────────────────
 Language           Extensions                        Query Dir Name 
──────────────────────────────────────────────────────────────────────
 Bash               .bash                             bash 
 BibTeX             .bib                              bibtex 
 C                  .h,.c                             c 
 C#                 .csx,.cs                          c_sharp 
 Clojure            .clj                              clojure 
 C++                .cxx,.c++,.h++,.hh,.cc,.cpp,.hpp  cpp 
 CSS                .css                              css 
 Elm                .elm                              elm 
 Erlang             .erl,.hrl                         erlang 
 Go                 .go                               go 
 HTML               .html                             html 
 Java               .java                             java 
 Javascript         .js,.mjs                          javascript 
 JSON               .json                             json 
 Julia              .jl                               julia 
 LaTeX              .tex                              latex 
 Markdown           .md                               markdown 
 OCaml              .ml                               ocaml 
 OCaml Interface    .mli                              ocaml_interface 
 Python             .pyw,.py                          python 
 Tree-sitter Query  .scm                              query 
 Ruby               .rb                               ruby 
 Rust               .rs                               rust 
 Scala              .scala,.sc                        scala 
 Svelte             .svelte                           svelte 
 Typescript         .ts                               typescript 
──────────────────────────────────────────────────────────────────────
```

# Why count tokens instead of lines

1. Counting lines rewards dense programs. For example,

```c
int nums[4] = { 1, 2, 3, 4 };
int mult[4] = {0};
for (int i = 0; i < 4; i++) {
    mult[i] = nums[i] * 2;
}
printf("[%d] [%d] [%d] [%d]", mult[0], mult[1], mult[2], mult[3]);
```

```go
nums := []int{1, 2, 3, 4}
mult := make([]int, 4)
for i, n := range nums {
    mult[i] = n * 2
}
fmt.Println(mult)
```

Are these programs the same size? They are each 6 lines long, but clearly the Go version is considerably smaller than the C version. While this is a contrived example, line counting still rewards dense programs and dense programming languages.

2. Counting lines rewards short variable names. Is `ns` shorter than `namespace`? By bytes it is, when used throughout a project it likely will result in fewer line breaks, but I don't think a program should be considered *smaller* just because it uses cryptic variable names whenever possible.

3. Counting lines penalizes line comments mixed with code. Consider the following contrived example,

```rust
v.iter() // iterate over the vector
    .map(|n| n * 2) // multiply each number by two
    .collect::Vec<u32>(); // collect the iterator into a vector of u32
```

Without the comments, it could be written as `v.iter().map(|n| n * 2).collect::Vec<32>();`.

4. Short syntactical elements in languages are rewarded. For example, the following calculates the mean of a list of numbers in APL:

```ruby
[1, 2, 3, 4].map { |n| n * 2 }
```

Compared with the equivalent

```ruby
[1, 2, 3, 4].map do |n|
    n * 2
end
```

5. Counting lines rewards horizontal programming and penalizes vertical programming

# Usage

```bash
tc -h
```
```
tc 0.1.0
Count your code by tokens, node kinds, and patterns in the syntax tree.

USAGE:
    tc [FLAGS] [OPTIONS] [--] [paths]...

FLAGS:
        --count-hidden        Count hidden files
    -h, --help                Prints help information
        --list-languages      Show a list of supported languages for parsing
        --no-dot-ignore       Don't respect .ignore files
        --no-git              Don't respect gitignore and .git/info/exclude files
        --no-parent-ignore    Don't respect ignore files from parent directories
        --show-totals         Show column totals. This is not affected by --top
    -V, --version             Prints version information

OPTIONS:
        --blacklist <blacklist>...          Blacklist of languages not to parse. This is overriden by --whitelist and
                                            must be an exact match
        --format <format>                   One of table|csv [default: table]
        --groupby <groupby>                 One of language|file|arg. "arg" will group by the `paths` arguments provided
                                            [default: language]
    -k, --kind <kind>...                    kinds of nodes in the syntax tree to count. See node-types.json in the
                                            parser's repo to see the names of nodes or use https://tree-
                                            sitter.github.io/tree-sitter/playground.
    -p, --kind-pattern <kind-pattern>...    Patterns of node kinds to count in the syntax tree (e.g. ".*comment" to
                                            match nodes of type "line_comment", "block_comment", and "comment").
                                            Supports Rust regular expressions
        --query <query>...                  Tree-sitter queries to match and count. Captures can also be counted with
                                            --query=query_name@capture_name,capture_name2. See
                                            https://github.com/RRethy/tc/blob/master/QUERIES.md for more information
        --sort-by <sort-by>                 One of group|numfiles|tokens. "group" will sort based on --groupby value
                                            [default: tokens]
        --top <top>                         How many of the top results to show
        --verbose <verbose>                 Logging level. 0 to not print errors. 1 to print IO and filesystem errors. 2
                                            to print parsing errors. 3 to print everything else. [default: 0]
        --whitelist <whitelist>...          Whitelist of languages to parse. This overrides --blacklist and must be an
                                            exact match

ARGS:
    <paths>...    Files and directories to parse and count. [default: .]
```

# Counting Tree-sitter Queries

See [QUERIES.md](https://github.com/RRethy/tc/blob/master/QUERIES.md)

# Performance

`tc` parses each file using a Tree-sitter parser to create a full syntax tree. This takes more time than only counting lines of code/comments so programs like [tokei](https://github.com/XAMPPRocky/tokei), [scc](https://github.com/boyter/scc), and [cloc](https://github.com/AlDanial/cloc) will typically be faster than `tc`.

Here are some benchmarks using [hyperfine](https://github.com/sharkdp/hyperfine) to give an overview of how much slower it is than line counting programs:

TODO

[**CPython**](https://github.com/python/cpython.git)

[**Redis**](https://github.com/redis/redis)

[**Linux**](https://github.com/torvalds/linux)

# Limitations

- `tc` does not support nested languages like ERB. This may change in the future.
- It's not always clear what is a token, `tc` treats any node in the syntax tree without children as a token. This usually works, but in some cases, like strings in the Rust Tree-sitter parser which can have children (escape codes), it may produce slightly expected results.

# Contributing

To add support for a new language, add it's information to `https://github.com/RRethy/tc/blob/master/src/language.rs` and add the language's Tree-sitter parser crate to `Cargo.toml`.

# Acknowledgements

All parsing is done using [Tree-sitter](https://tree-sitter.github.io/tree-sitter) parsers
