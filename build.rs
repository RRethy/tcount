use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tsparsers", "tree-sitter-rust", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-rust");
}
