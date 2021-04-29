use crate::error::Error;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::Path;
use tree_sitter::{Parser, Tree};

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum Language {
    Bash,
    Bibtex,
    C,
    CSharp,
    Clojure,
    Cpp,
    Css,
    Elm,
    EmbeddedTemplate,
    Fennel,
    Go,
    GraphQL,
    Html,
    Java,
    Javascript,
    Json,
    Julia,
    Kotlin,
    Latex,
    Markdown,
    Nix,
    Ocaml,
    OcamlInterface,
    Python,
    Query,
    Rst,
    Ruby,
    Rust,
    Scala,
    Scss,
    Svelte,
    Typescript,
    Tsx,
    Verilog,
    Zig,
    Unsupported(String),
}

static LANGUAGES: phf::Map<&'static str, Language> = phf::phf_map! {
    "bash"   => Language::Bash,
    "bib"    => Language::Bibtex,
    "c"      => Language::C,
    "h"      => Language::C,
    "cs"     => Language::CSharp,
    "csx"    => Language::CSharp,
    "clj"    => Language::Clojure,
    "cc"     => Language::Cpp,
    "cpp"    => Language::Cpp,
    "cxx"    => Language::Cpp,
    "c++"    => Language::Cpp,
    "hh"     => Language::Cpp,
    "hpp"    => Language::Cpp,
    "h++"    => Language::Cpp,
    "css"    => Language::Css,
    "elm"    => Language::Elm,
    "erb"    => Language::EmbeddedTemplate,
    "ejs"    => Language::EmbeddedTemplate,
    "fnl"    => Language::Fennel,
    "go"     => Language::Go,
    "gql"    => Language::GraphQL,
    "html"   => Language::Html,
    "java"   => Language::Java,
    "js"     => Language::Javascript,
    "mjs"    => Language::Javascript,
    "json"   => Language::Json,
    "jl"     => Language::Julia,
    "kt"     => Language::Kotlin,
    "kts"    => Language::Kotlin,
    "tex"    => Language::Latex,
    "md"     => Language::Markdown,
    "nix"    => Language::Nix,
    "ml"     => Language::Ocaml,
    "mli"    => Language::OcamlInterface,
    "py"     => Language::Python,
    "pyw"    => Language::Python,
    "scm"    => Language::Query,
    "rst"    => Language::Rst,
    "rb"     => Language::Ruby,
    "rs"     => Language::Rust,
    "sc"     => Language::Scala,
    "scala"  => Language::Scala,
    "scss"   => Language::Scss,
    "sass"   => Language::Scss,
    "svelte" => Language::Svelte,
    "ts"     => Language::Typescript,
    "tsx"    => Language::Tsx,
    "vg"     => Language::Verilog,
    "vh"     => Language::Verilog,
    "zig"    => Language::Zig,
};

fn language(path: impl AsRef<Path>) -> Language {
    let ext = path
        .as_ref()
        .extension()
        .map(OsString::from)
        .unwrap_or(OsString::new());
    LANGUAGES
        .get(ext.to_string_lossy().as_ref())
        .unwrap_or(&Language::Unsupported(ext.to_string_lossy().to_string()))
        .clone()
}

pub fn parse(path: impl AsRef<Path>) -> Result<(Tree, Language), Error> {
    let lang = language(path.as_ref());
    let tslang = match lang {
        Language::Bash => tree_sitter_bash::language(),
        Language::Bibtex => tree_sitter_bibtex::language(),
        Language::C => tree_sitter_c::language(),
        Language::CSharp => tree_sitter_c_sharp::language(),
        Language::Clojure => tree_sitter_clojure::language(),
        Language::Cpp => tree_sitter_cpp::language(),
        Language::Css => tree_sitter_css::language(),
        Language::Elm => tree_sitter_elm::language(),
        Language::EmbeddedTemplate => tree_sitter_embedded_template::language(),
        Language::Go => tree_sitter_go::language(),
        Language::Html => tree_sitter_html::language(),
        Language::Java => tree_sitter_java::language(),
        Language::Javascript => tree_sitter_javascript::language(),
        Language::Json => tree_sitter_json::language(),
        Language::Julia => tree_sitter_julia::language(),
        Language::Latex => tree_sitter_latex::language(),
        Language::Markdown => tree_sitter_markdown::language(),
        Language::Ocaml => tree_sitter_ocaml::language_ocaml(),
        Language::OcamlInterface => tree_sitter_ocaml::language_ocaml_interface(),
        Language::Python => tree_sitter_python::language(),
        Language::Query => tree_sitter_query::language(),
        Language::Ruby => tree_sitter_ruby::language(),
        Language::Rust => tree_sitter_rust::language(),
        Language::Scala => tree_sitter_scala::language(),
        Language::Svelte => tree_sitter_svelte::language(),
        Language::Typescript => tree_sitter_typescript::language_typescript(),
        Language::Tsx => tree_sitter_typescript::language_tsx(),
        Language::Unsupported(ext) => return Err(Error::Unsupported(ext)),
        _ => return Err(Error::Unsupported(String::new())),
    };

    let mut parser = Parser::new();
    parser.set_language(tslang)?;
    match parser.parse(read_to_string(&path)?, None) {
        Some(tree) => Ok((tree, lang.clone())),
        None => Err(Error::Parser),
    }
}
