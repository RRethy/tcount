use crate::error::{Error, Result};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ffi::OsString;
use std::path::Path;

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
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
    Unsupported,
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Language::Bash => "Bash".into(),
            Language::Bibtex => "BibTeX".into(),
            Language::C => "C".into(),
            Language::CSharp => "C#".into(),
            Language::Clojure => "Clojure".into(),
            Language::Cpp => "C++".into(),
            Language::Css => "CSS".into(),
            Language::Elm => "Elm".into(),
            Language::EmbeddedTemplate => "Embedded Template".into(),
            Language::Go => "Go".into(),
            Language::Html => "HTML".into(),
            Language::Java => "Java".into(),
            Language::Javascript => "Javascript".into(),
            Language::Json => "JSON".into(),
            Language::Julia => "Julia".into(),
            Language::Latex => "LaTeX".into(),
            Language::Markdown => "Markdown".into(),
            Language::Ocaml => "OCaml".into(),
            Language::OcamlInterface => "OCaml Interface".into(),
            Language::Python => "Python".into(),
            Language::Query => "Tree-sitter Query".into(),
            Language::Ruby => "Ruby".into(),
            Language::Rust => "Rust".into(),
            Language::Scala => "Scala".into(),
            Language::Svelte => "Svelte".into(),
            Language::Typescript => "Typescript".into(),
            Language::Tsx => "TSX".into(),
            Language::Unsupported => "Unsupported".into(),
        }
    }
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
            Language::Unsupported => return Err(Error::UnsupportedLanguage),
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

static DIR_TO_LANGUAGE: phf::Map<&'static str, Language> = phf::phf_map! {
    "bash"             => Language::Bash,
    "bibtex"           => Language::Bibtex,
    "c"                => Language::C,
    "c_sharp"          => Language::CSharp,
    "clojure"          => Language::Clojure,
    "cpp"              => Language::Cpp,
    "css"              => Language::Css,
    "elm"              => Language::Elm,
    "embeddedtemplate" => Language::EmbeddedTemplate,
    "go"               => Language::Go,
    "html"             => Language::Html,
    "java"             => Language::Java,
    "javascript"       => Language::Javascript,
    "json"             => Language::Json,
    "julia"            => Language::Julia,
    "latex"            => Language::Latex,
    "markdown"         => Language::Markdown,
    "ocaml"            => Language::Ocaml,
    "ocaml_interface"  => Language::OcamlInterface,
    "python"           => Language::Python,
    "query"            => Language::Query,
    "ruby"             => Language::Ruby,
    "rust"             => Language::Rust,
    "scala"            => Language::Scala,
    "svelte"           => Language::Svelte,
    "typescript"       => Language::Typescript,
    "tsx"              => Language::Tsx,
};
