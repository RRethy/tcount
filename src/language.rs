use crate::error::{Error, Result};
use crate::output::print_languages;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt;
use std::path::Path;

/// There are many commented out languages, these languages do not have up to date Tree-sitter
/// parsers. To add support for them, the parser needs to be update to use tree-sitter v0.19.3
///
/// List of languages that are matched. Some of the languages are not supported since they either
/// don't currently have a tree-sitter parser, or the tree-sitter parser is out-of-date and depends
/// on an old version of the tree-sitter crate (<0.19.3).
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Language {
    Bash,
    BibTeX,
    C,
    CSharp,
    Clojure,
    Cpp,
    Css,
    Dart,
    Elm,
    Erlang,
    EmbeddedTemplate,
    Fennel,
    GDScript,
    Go,
    GraphQL,
    Haskell,
    Html,
    Java,
    Javascript,
    Json,
    Julia,
    Kotlin,
    LaTeX,
    Lua,
    Markdown,
    Nix,
    OCaml,
    OCamlInterface,
    OCamlLex,
    Php,
    Python,
    Query,
    R,
    Rst,
    Ruby,
    Rust,
    Scala,
    Scss,
    Supercollider,
    Svelte,
    Swift,
    Teal,
    Toml,
    Typescript,
    Tsx,
    Verilog,
    Vue,
    Yaml,
    Zig,
    Unsupported,
}

impl Language {
    /// print all the supported languages, their file extensions, and query directory names
    pub fn print_all() {
        let mut lang_exts: Vec<(&Language, Vec<String>)> = EXT_TO_LANGUAGE
            .into_iter()
            .filter(|(_ext, lang)| lang.get_treesitter_language().is_ok())
            .fold(HashMap::new(), |mut acc, (ext, lang)| {
                acc.entry(lang)
                    .or_insert(Vec::new())
                    .push(format!(".{}", ext));
                acc
            })
            .into_iter()
            .collect();
        lang_exts.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));
        let lang_dirs = DIR_TO_LANGUAGE
            .into_iter()
            .filter(|(_dir, lang)| lang.get_treesitter_language().is_ok())
            .fold(HashMap::new(), |mut acc, (dir, lang)| {
                acc.entry(lang).or_insert(Vec::new()).push(dir.to_string());
                acc
            });
        print_languages(
            lang_exts
                .into_iter()
                .map(|(lang, exts)| (lang, exts, lang_dirs.get(&lang).unwrap()))
                .collect(),
        );
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Bash => "Bash",
                Language::BibTeX => "BibTeX",
                Language::C => "C",
                Language::CSharp => "C#",
                Language::Clojure => "Clojure",
                Language::Cpp => "C++",
                Language::Css => "CSS",
                Language::Dart => "Dart",
                Language::Elm => "Elm",
                Language::Erlang => "Erlang",
                Language::EmbeddedTemplate => "Embedded Template",
                Language::Fennel => "Fennel",
                Language::GDScript => "GDScript",
                Language::Go => "Go",
                Language::GraphQL => "GraphQL",
                Language::Haskell => "Haskell",
                Language::Html => "HTML",
                Language::Java => "Java",
                Language::Javascript => "Javascript",
                Language::Json => "JSON",
                Language::Julia => "Julia",
                Language::Kotlin => "Kotlin",
                Language::LaTeX => "LaTeX",
                Language::Lua => "Lua",
                Language::Markdown => "Markdown",
                Language::Nix => "Nix",
                Language::OCaml => "OCaml",
                Language::OCamlInterface => "OCaml Interface",
                Language::OCamlLex => "OCamlLex",
                Language::Php => "PHP",
                Language::Python => "Python",
                Language::Query => "Tree-sitter Query",
                Language::R => "R",
                Language::Rst => "RST",
                Language::Ruby => "Ruby",
                Language::Rust => "Rust",
                Language::Scala => "Scala",
                Language::Scss => "SCSS",
                Language::Supercollider => "Supercollider",
                Language::Svelte => "Svelte",
                Language::Swift => "Swift",
                Language::Teal => "Teal",
                Language::Toml => "TOML",
                Language::Typescript => "Typescript",
                Language::Tsx => "TSX",
                Language::Verilog => "Verilog",
                Language::Vue => "Vue",
                Language::Yaml => "YAML",
                Language::Zig => "Zig",
                Language::Unsupported => "Unsupported",
            }
        )
    }
}

impl Language {
    pub fn get_treesitter_language(&self) -> Result<tree_sitter::Language> {
        match self {
            Language::Bash => Ok(tree_sitter_bash::language()),
            Language::BibTeX => Ok(tree_sitter_bibtex::language()),
            Language::C => Ok(tree_sitter_c::language()),
            Language::CSharp => Ok(tree_sitter_c_sharp::language()),
            Language::Clojure => Ok(tree_sitter_clojure::language()),
            Language::Cpp => Ok(tree_sitter_cpp::language()),
            Language::Css => Ok(tree_sitter_css::language()),
            Language::Elm => Ok(tree_sitter_elm::language()),
            // Language::EmbeddedTemplate => Ok(tree_sitter_embedded_template::language()),
            Language::Erlang => Ok(tree_sitter_erlang::language()),
            Language::Go => Ok(tree_sitter_go::language()),
            Language::Html => Ok(tree_sitter_html::language()),
            Language::Java => Ok(tree_sitter_java::language()),
            Language::Javascript => Ok(tree_sitter_javascript::language()),
            Language::Json => Ok(tree_sitter_json::language()),
            Language::Julia => Ok(tree_sitter_julia::language()),
            Language::LaTeX => Ok(tree_sitter_latex::language()),
            Language::Markdown => Ok(tree_sitter_markdown::language()),
            Language::OCaml => Ok(tree_sitter_ocaml::language_ocaml()),
            Language::OCamlInterface => Ok(tree_sitter_ocaml::language_ocaml_interface()),
            Language::Python => Ok(tree_sitter_python::language()),
            Language::Query => Ok(tree_sitter_query::language()),
            Language::Ruby => Ok(tree_sitter_ruby::language()),
            Language::Rust => Ok(tree_sitter_rust::language()),
            Language::Scala => Ok(tree_sitter_scala::language()),
            Language::Svelte => Ok(tree_sitter_svelte::language()),
            Language::Typescript => Ok(tree_sitter_typescript::language_typescript()),
            // Language::TSX => Ok(tree_sitter_typescript::language_tsx()),
            _ => Err(Error::UnsupportedLanguage),
        }
    }
}

impl From<&Path> for Language {
    fn from(path: &Path) -> Language {
        let (tag, map) = if path.is_dir() {
            // we assign a `Language` to query directories which take the form {query dir}/{language}/{query name}.scm
            // the {query name}.scm has already been stripped since this is a directory
            (path.file_name(), &DIR_TO_LANGUAGE)
        } else {
            // we only check the extension to determine language
            // TODO this could be improved on by looking for she-bangs, Vim modelines, or Emacs modelines,
            // among a few other more complicated heuristics
            (path.extension(), &EXT_TO_LANGUAGE)
        };
        let tag = tag.map(OsString::from).unwrap_or(OsString::new());
        map.get(tag.to_string_lossy().as_ref())
            .unwrap_or(&Language::Unsupported)
            .clone()
    }
}

static EXT_TO_LANGUAGE: phf::Map<&'static str, Language> = phf::phf_map! {
    "bash"    => Language::Bash,
    "bib"     => Language::BibTeX,
    "c"       => Language::C,
    "h"       => Language::C,
    "cs"      => Language::CSharp,
    "csx"     => Language::CSharp,
    "clj"     => Language::Clojure,
    "cc"      => Language::Cpp,
    "cpp"     => Language::Cpp,
    "cxx"     => Language::Cpp,
    "c++"     => Language::Cpp,
    "hh"      => Language::Cpp,
    "hpp"     => Language::Cpp,
    "h++"     => Language::Cpp,
    "css"     => Language::Css,
    // "dart"    => Language::Dart,
    "elm"     => Language::Elm,
    "erl"     => Language::Erlang,
    "hrl"     => Language::Erlang,
    // "erb"     => Language::EmbeddedTemplate,
    // "ejs"     => Language::EmbeddedTemplate,
    // "fnl"     => Language::Fennel,
    // "gd"      => Language::GDScript,
    "go"      => Language::Go,
    // "graphql" => Language::GraphQL,
    // "hs"      => Language::Haskell,
    "html"    => Language::Html,
    "java"    => Language::Java,
    "js"      => Language::Javascript,
    "mjs"     => Language::Javascript,
    "json"    => Language::Json,
    "jl"      => Language::Julia,
    // "kt"      => Language::Kotlin,
    // "kts"     => Language::Kotlin,
    "tex"     => Language::LaTeX,
    // "lua"     => Language::Lua,
    "md"      => Language::Markdown,
    // "nix"     => Language::Nix,
    "ml"      => Language::OCaml,
    "mli"     => Language::OCamlInterface,
    // "mll"     => Language::OCamlLex,
    // "php"     => Language::PHP,
    "py"      => Language::Python,
    "pyw"     => Language::Python,
    "scm"     => Language::Query,
    // "r"       => Language::R,
    // "rst"     => Language::RST,
    "rb"      => Language::Ruby,
    "rs"      => Language::Rust,
    "sc"      => Language::Scala,
    "scala"   => Language::Scala,
    // "scss"    => Language::SCSS,
    "svelte"  => Language::Svelte,
    // "swift"   => Language::Swift,
    // "tl"      => Language::Teal,
    // "toml"    => Language::TOML,
    "ts"      => Language::Typescript,
    // "tsx"     => Language::TSX,
    // "v"       => Language::Verilog,
    // "vg"      => Language::Verilog,
    // "vh"      => Language::Verilog,
    // "vue"     => Language::Vue,
    // "yaml"    => Language::YAML,
    // "zig"     => Language::Zig,
};

/// These dir names are used when reading query files to know what language they refer to.
/// See tcount --help for more information on queries.
static DIR_TO_LANGUAGE: phf::Map<&'static str, Language> = phf::phf_map! {
    "bash"             => Language::Bash,
    "bibtex"           => Language::BibTeX,
    "c"                => Language::C,
    "c_sharp"          => Language::CSharp,
    "clojure"          => Language::Clojure,
    "cpp"              => Language::Cpp,
    "css"              => Language::Css,
    "dart"             => Language::Dart,
    "elm"              => Language::Elm,
    "erlang"           => Language::Erlang,
    "embeddedtemplate" => Language::EmbeddedTemplate,
    "fennel"           => Language::Fennel,
    "gdscript"         => Language::GDScript,
    "go"               => Language::Go,
    "graphql"          => Language::GraphQL,
    "haskell"          => Language::Haskell,
    "html"             => Language::Html,
    "java"             => Language::Java,
    "javascript"       => Language::Javascript,
    "json"             => Language::Json,
    "julia"            => Language::Julia,
    "kotlin"           => Language::Kotlin,
    "latex"            => Language::LaTeX,
    "lua"              => Language::Lua,
    "markdown"         => Language::Markdown,
    "nix"              => Language::Nix,
    "ocaml"            => Language::OCaml,
    "ocaml_interface"  => Language::OCamlInterface,
    "ocamllex"         => Language::OCamlLex,
    "php"              => Language::Php,
    "python"           => Language::Python,
    "query"            => Language::Query,
    "r"                => Language::R,
    "rst"              => Language::Rst,
    "ruby"             => Language::Ruby,
    "rust"             => Language::Rust,
    "scala"            => Language::Scala,
    "scss"             => Language::Scss,
    "supercollider"    => Language::Supercollider,
    "svelte"           => Language::Svelte,
    "swift"            => Language::Swift,
    "teal"             => Language::Teal,
    "toml"             => Language::Toml,
    "typescript"       => Language::Typescript,
    "tsx"              => Language::Tsx,
    "verilog"          => Language::Verilog,
    "vue"              => Language::Vue,
    "yaml"             => Language::Yaml,
    "zig"              => Language::Zig,
};
