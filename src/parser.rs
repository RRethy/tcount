use crate::error::Error;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::Path;
use tree_sitter::{Parser, Tree};

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum Language {
    BASH,
    BIBTEX,
    C,
    CSHARP,
    CLOJURE,
    CPP,
    CSS,
    ELM,
    EMBEDDEDTEMPLATE,
    FENNEL,
    GO,
    GRAPHQL,
    HTML,
    JAVA,
    JAVASCRIPT,
    JSON,
    JULIA,
    KOTLIN,
    LATEX,
    MARKDOWN,
    NIX,
    OCAML,
    OCAMLINTERFACE,
    PYTHON,
    QUERY,
    RST,
    RUBY,
    RUST,
    SCALA,
    SCSS,
    SVELTE,
    TYPESCRIPT,
    TSX,
    VERILOG,
    ZIG,
    Unsupported,
}

static LANGUAGES: phf::Map<&'static str, Language> = phf::phf_map! {
    "bash" => Language::BASH,
    "bib" => Language::BIBTEX,
    "c" => Language::C,
    "h" => Language::C,
    "cs" => Language::CSHARP,
    "csx" => Language::CSHARP,
    "clj" => Language::CLOJURE,
    "cc" => Language::CPP,
    "cpp" => Language::CPP,
    "cxx" => Language::CPP,
    "c++" => Language::CPP,
    "hh" => Language::CPP,
    "hpp" => Language::CPP,
    "h++" => Language::CPP,
    "css" => Language::CSS,
    "elm" => Language::ELM,
    "erb" => Language::EMBEDDEDTEMPLATE,
    "ejs" => Language::EMBEDDEDTEMPLATE,
    "fnl" => Language::FENNEL,
    "go" => Language::GO,
    "gql" => Language::GRAPHQL,
    "html" => Language::HTML,
    "java" => Language::JAVA,
    "js" => Language::JAVASCRIPT,
    "mjs" => Language::JAVASCRIPT,
    "json" => Language::JSON,
    "jl" => Language::JULIA,
    "kt" => Language::KOTLIN,
    "kts" => Language::KOTLIN,
    "tex" => Language::LATEX,
    // "lua" => Language::LUA,
    "md" => Language::MARKDOWN,
    "nix" => Language::NIX,
    "ml" => Language::OCAML,
    "mli" => Language::OCAMLINTERFACE,
    // "php" => Language::PHP,
    "py" => Language::PYTHON,
    "pyw" => Language::PYTHON,
    "scm" => Language::QUERY,
    "rst" => Language::RST,
    "rb" => Language::RUBY,
    "rs" => Language::RUST,
    "sc" => Language::SCALA,
    "scala" => Language::SCALA,
    "scss" => Language::SCSS,
    "sass" => Language::SCSS,
    "svelte" => Language::SVELTE,
    // "tl" => Language::TEAL,
    "ts" => Language::TYPESCRIPT,
    "tsx" => Language::TSX,
    "vg" => Language::VERILOG,
    "vh" => Language::VERILOG,
    "zig" => Language::ZIG,
};

fn language(path: impl AsRef<Path>) -> Language {
    let ext = path
        .as_ref()
        .extension()
        .map(OsString::from)
        .unwrap_or(OsString::new());
    LANGUAGES
        .get(ext.to_string_lossy().as_ref())
        .unwrap_or(&Language::Unsupported)
        .clone()
}

pub fn parse(path: impl AsRef<Path>) -> Result<(Tree, Language), Error> {
    let lang = language(path.as_ref());
    let tslang = match lang {
        Language::BASH => tree_sitter_bash::language(),
        Language::BIBTEX => tree_sitter_bibtex::language(),
        Language::C => tree_sitter_c::language(),
        Language::CSHARP => tree_sitter_c_sharp::language(),
        Language::CLOJURE => tree_sitter_clojure::language(),
        Language::CPP => tree_sitter_cpp::language(),
        Language::CSS => tree_sitter_css::language(),
        Language::ELM => tree_sitter_elm::language(),
        Language::EMBEDDEDTEMPLATE => tree_sitter_embedded_template::language(),
        // Language::FENNEL => tree_sitter_fennel::language(),
        Language::GO => tree_sitter_go::language(),
        // Language::GRAPHQL => tree_sitter_graphql::language(),
        Language::HTML => tree_sitter_html::language(),
        Language::JAVA => tree_sitter_java::language(),
        Language::JAVASCRIPT => tree_sitter_javascript::language(),
        Language::JSON => tree_sitter_json::language(),
        Language::JULIA => tree_sitter_julia::language(),
        // Language::KOTLIN => tree_sitter_kotlin::language(),
        Language::LATEX => tree_sitter_latex::language(),
        // Language::LUA => tree_sitter_lua::language(),
        Language::MARKDOWN => tree_sitter_markdown::language(),
        // Language::NIX => tree_sitter_nix::language(),
        Language::OCAML => tree_sitter_ocaml::language_ocaml(),
        Language::OCAMLINTERFACE => tree_sitter_ocaml::language_ocaml_interface(),
        // Language::PHP => tree_sitter_php::language(),
        Language::PYTHON => tree_sitter_python::language(),
        Language::QUERY => tree_sitter_query::language(),
        // Language::RST => tree_sitter_rst::language(),
        Language::RUBY => tree_sitter_ruby::language(),
        Language::RUST => tree_sitter_rust::language(),
        Language::SCALA => tree_sitter_scala::language(),
        // Language::SCSS => tree_sitter_scss::language(),
        Language::SVELTE => tree_sitter_svelte::language(),
        // Language::TEAL => tree_sitter_teal::language(),
        Language::TYPESCRIPT => tree_sitter_typescript::language_typescript(),
        Language::TSX => tree_sitter_typescript::language_tsx(),
        // Language::VERILOG => tree_sitter_verilog::language(),
        // Language::ZIG => tree_sitter_zig::language(),
        Language::Unsupported => return Err(Error::Unsupported(String::new())),
        _ => return Err(Error::Unsupported(String::new())),
    };

    let mut parser = Parser::new();
    parser.set_language(tslang)?;
    match parser.parse(read_to_string(&path)?, None) {
        Some(tree) => Ok((tree, lang.clone())),
        None => Err(Error::Parser),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_language_from_extension() {
        let ext_to_language = vec![("main.rs", Language::RUST)];
        ext_to_language
            .into_iter()
            .for_each(|(ext, lang)| assert_eq!(language(ext), lang));
    }
}
