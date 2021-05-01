use crate::error::{Error, Result};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ffi::OsString;
use std::path::Path;

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
    Go,
    Html,
    Java,
    Javascript,
    Json,
    Julia,
    Latex,
    Markdown,
    Ocaml,
    OcamlInterface,
    Python,
    Query,
    Ruby,
    Rust,
    Scala,
    Svelte,
    Typescript,
    Tsx,
    Unsupported(String),
}

impl Language {
    pub fn get_treesitter_language(&self) -> Result<tree_sitter::Language> {
        match self {
            Language::Bash => Ok(tree_sitter_bash::language()),
            Language::Bibtex => Ok(tree_sitter_bibtex::language()),
            Language::C => Ok(tree_sitter_c::language()),
            Language::CSharp => Ok(tree_sitter_c_sharp::language()),
            Language::Clojure => Ok(tree_sitter_clojure::language()),
            Language::Cpp => Ok(tree_sitter_cpp::language()),
            Language::Css => Ok(tree_sitter_css::language()),
            Language::Elm => Ok(tree_sitter_elm::language()),
            Language::EmbeddedTemplate => Ok(tree_sitter_embedded_template::language()),
            Language::Go => Ok(tree_sitter_go::language()),
            Language::Html => Ok(tree_sitter_html::language()),
            Language::Java => Ok(tree_sitter_java::language()),
            Language::Javascript => Ok(tree_sitter_javascript::language()),
            Language::Json => Ok(tree_sitter_json::language()),
            Language::Julia => Ok(tree_sitter_julia::language()),
            Language::Latex => Ok(tree_sitter_latex::language()),
            Language::Markdown => Ok(tree_sitter_markdown::language()),
            Language::Ocaml => Ok(tree_sitter_ocaml::language_ocaml()),
            Language::OcamlInterface => Ok(tree_sitter_ocaml::language_ocaml_interface()),
            Language::Python => Ok(tree_sitter_python::language()),
            Language::Query => Ok(tree_sitter_query::language()),
            Language::Ruby => Ok(tree_sitter_ruby::language()),
            Language::Rust => Ok(tree_sitter_rust::language()),
            Language::Scala => Ok(tree_sitter_scala::language()),
            Language::Svelte => Ok(tree_sitter_svelte::language()),
            Language::Typescript => Ok(tree_sitter_typescript::language_typescript()),
            Language::Tsx => Ok(tree_sitter_typescript::language_tsx()),
            Language::Unsupported(ext) => return Err(Error::Unsupported(ext.into())),
        }
    }
}

impl<P: AsRef<Path>> From<P> for Language {
    fn from(path: P) -> Language {
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
    "go"     => Language::Go,
    "html"   => Language::Html,
    "java"   => Language::Java,
    "js"     => Language::Javascript,
    "mjs"    => Language::Javascript,
    "json"   => Language::Json,
    "jl"     => Language::Julia,
    "tex"    => Language::Latex,
    "md"     => Language::Markdown,
    "ml"     => Language::Ocaml,
    "mli"    => Language::OcamlInterface,
    "py"     => Language::Python,
    "pyw"    => Language::Python,
    "scm"    => Language::Query,
    "rb"     => Language::Ruby,
    "rs"     => Language::Rust,
    "sc"     => Language::Scala,
    "scala"  => Language::Scala,
    "svelte" => Language::Svelte,
    "ts"     => Language::Typescript,
    "tsx"    => Language::Tsx,
};
