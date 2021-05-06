use crate::error::{Error, Result};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt;
use std::path::Path;

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
    CSS,
    Dart,
    Elm,
    Erlang,
    EmbeddedTemplate,
    Fennel,
    GDScript,
    Go,
    GraphQL,
    Haskell,
    HTML,
    Java,
    Javascript,
    JSON,
    Julia,
    Kotlin,
    LaTeX,
    Lua,
    Markdown,
    Nix,
    OCaml,
    OCamlInterface,
    OCamlLex,
    PHP,
    Python,
    Query,
    R,
    RST,
    Ruby,
    Rust,
    Scala,
    SCSS,
    Supercollider,
    Svelte,
    Swift,
    Teal,
    TOML,
    Typescript,
    TSX,
    Verilog,
    Vue,
    YAML,
    Zig,
    Unsupported,
}

impl Language {
    pub fn list_all() -> String {
        let mut langs: Vec<(&Language, Vec<String>)> = EXT_TO_LANGUAGE
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
        langs.sort_by(|(l1, _), (l2, _)| l1.cmp(&l2));
        langs
            .into_iter()
            .map(|(lang, exts)| format!("{} ({})", lang, exts.join(",")))
            .collect::<Vec<String>>()
            .join("\n")
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
                Language::C => "C =>",
                Language::CSharp => "C#",
                Language::Clojure => "Clojure",
                Language::Cpp => "C++",
                Language::CSS => "CSS",
                Language::Dart => "Dart",
                Language::Elm => "Elm",
                Language::Erlang => "Erlang",
                Language::EmbeddedTemplate => "Embedded Template",
                Language::Fennel => "Fennel",
                Language::GDScript => "GDScript",
                Language::Go => "Go",
                Language::GraphQL => "GraphQL",
                Language::Haskell => "Haskell",
                Language::HTML => "HTML",
                Language::Java => "Java",
                Language::Javascript => "Javascript",
                Language::JSON => "JSON",
                Language::Julia => "Julia",
                Language::Kotlin => "Kotlin",
                Language::LaTeX => "LaTeX",
                Language::Lua => "Lua",
                Language::Markdown => "Markdown",
                Language::Nix => "Nix",
                Language::OCaml => "OCaml",
                Language::OCamlInterface => "OCaml Interface",
                Language::OCamlLex => "OCamlLex",
                Language::PHP => "PHP",
                Language::Python => "Python",
                Language::Query => "Tree-sitter Query",
                Language::R => "R =>",
                Language::RST => "RST",
                Language::Ruby => "Ruby",
                Language::Rust => "Rust",
                Language::Scala => "Scala",
                Language::SCSS => "SCSS",
                Language::Supercollider => "Supercollider",
                Language::Svelte => "Svelte",
                Language::Swift => "Swift",
                Language::Teal => "Teal",
                Language::TOML => "TOML",
                Language::Typescript => "Typescript",
                Language::TSX => "TSX",
                Language::Verilog => "Verilog",
                Language::Vue => "Vue",
                Language::YAML => "YAML",
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
            Language::CSS => Ok(tree_sitter_css::language()),
            Language::Elm => Ok(tree_sitter_elm::language()),
            Language::EmbeddedTemplate => Ok(tree_sitter_embedded_template::language()),
            Language::Erlang => Ok(tree_sitter_erlang::language()),
            Language::Go => Ok(tree_sitter_go::language()),
            Language::HTML => Ok(tree_sitter_html::language()),
            Language::Java => Ok(tree_sitter_java::language()),
            Language::Javascript => Ok(tree_sitter_javascript::language()),
            Language::JSON => Ok(tree_sitter_json::language()),
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
            Language::TSX => Ok(tree_sitter_typescript::language_tsx()),
            _ => return Err(Error::UnsupportedLanguage),
        }
    }
}

impl From<&Path> for Language {
    fn from(path: &Path) -> Language {
        let (tag, map) = if path.is_dir() {
            (path.file_name(), &DIR_TO_LANGUAGE)
        } else {
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
    "css"     => Language::CSS,
    "dart"    => Language::Dart,
    "elm"     => Language::Elm,
    "erl"     => Language::Erlang,
    "hrl"     => Language::Erlang,
    "erb"     => Language::EmbeddedTemplate,
    "ejs"     => Language::EmbeddedTemplate,
    "fnl"     => Language::Fennel,
    "gd"      => Language::GDScript,
    "go"      => Language::Go,
    "graphql" => Language::GraphQL,
    "hs"      => Language::Haskell,
    "html"    => Language::HTML,
    "java"    => Language::Java,
    "js"      => Language::Javascript,
    "mjs"     => Language::Javascript,
    "json"    => Language::JSON,
    "jl"      => Language::Julia,
    "kt"      => Language::Kotlin,
    "kts"     => Language::Kotlin,
    "tex"     => Language::LaTeX,
    "lua"     => Language::Lua,
    "md"      => Language::Markdown,
    "nix"     => Language::Nix,
    "ml"      => Language::OCaml,
    "mli"     => Language::OCamlInterface,
    "mll"     => Language::OCamlLex,
    "php"     => Language::PHP,
    "py"      => Language::Python,
    "pyw"     => Language::Python,
    "scm"     => Language::Query,
    "r"       => Language::R,
    "rst"     => Language::RST,
    "rb"      => Language::Ruby,
    "rs"      => Language::Rust,
    "sc"      => Language::Scala,
    "scala"   => Language::Scala,
    "scss"    => Language::SCSS,
    "svelte"  => Language::Svelte,
    "swift"   => Language::Swift,
    "tl"      => Language::Teal,
    "toml"    => Language::TOML,
    "ts"      => Language::Typescript,
    "tsx"     => Language::TSX,
    "v"       => Language::Verilog,
    "vg"      => Language::Verilog,
    "vh"      => Language::Verilog,
    "vue"     => Language::Vue,
    "yaml"    => Language::YAML,
    "zig"     => Language::Zig,
};

/// These dir names are used when reading query files to know what language they refer to.
/// See tc --help for more information on queries.
static DIR_TO_LANGUAGE: phf::Map<&'static str, Language> = phf::phf_map! {
    "bash"             => Language::Bash,
    "bibtex"           => Language::BibTeX,
    "c"                => Language::C,
    "c_sharp"          => Language::CSharp,
    "clojure"          => Language::Clojure,
    "cpp"              => Language::Cpp,
    "css"              => Language::CSS,
    "dart"             => Language::Dart,
    "elm"              => Language::Elm,
    "erlang"           => Language::Erlang,
    "embeddedtemplate" => Language::EmbeddedTemplate,
    "fennel"           => Language::Fennel,
    "gdscript"         => Language::GDScript,
    "go"               => Language::Go,
    "graphql"          => Language::GraphQL,
    "haskell"          => Language::Haskell,
    "html"             => Language::HTML,
    "java"             => Language::Java,
    "javascript"       => Language::Javascript,
    "json"             => Language::JSON,
    "julia"            => Language::Julia,
    "kotlin"           => Language::Kotlin,
    "latex"            => Language::LaTeX,
    "lua"              => Language::Lua,
    "markdown"         => Language::Markdown,
    "nix"              => Language::Nix,
    "ocaml"            => Language::OCaml,
    "ocaml_interface"  => Language::OCamlInterface,
    "ocamllex"         => Language::OCamlLex,
    "php"              => Language::PHP,
    "python"           => Language::Python,
    "query"            => Language::Query,
    "r"                => Language::R,
    "rst"              => Language::RST,
    "ruby"             => Language::Ruby,
    "rust"             => Language::Rust,
    "scala"            => Language::Scala,
    "scss"             => Language::SCSS,
    "supercollider"    => Language::Supercollider,
    "svelte"           => Language::Svelte,
    "swift"            => Language::Swift,
    "teal"             => Language::Teal,
    "toml"             => Language::TOML,
    "typescript"       => Language::Typescript,
    "tsx"              => Language::TSX,
    "verilog"          => Language::Verilog,
    "vue"              => Language::Vue,
    "yaml"             => Language::YAML,
    "zig"              => Language::Zig,
};
