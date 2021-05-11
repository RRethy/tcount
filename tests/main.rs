mod utils;

use utils::tc;

#[test]
fn test_whitelist() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--whitelist", "Rust", "Ruby"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Rust,5,156
Ruby,2,43
TOTALS,7,199
",
        )
        .success();
}

#[test]
fn test_blacklist() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--blacklist", "Rust", "Ruby"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Tree-sitter Query,21,378
Go,1,52
Unsupported,1,0
TOTALS,23,430
",
        )
        .success();
}

#[test]
fn test_groupby_language() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--group-by", "language"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Tree-sitter Query,21,378
Rust,5,156
Go,1,52
Ruby,2,43
Unsupported,1,0
TOTALS,30,629
",
        )
        .success();
}

#[test]
fn test_groupby_file() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--group-by", "file", "--whitelist", "Go"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
./go1.go,1,52
TOTALS,1,52
",
        )
        .success();
}

#[test]
fn test_sortby_group() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--sort-by", "group"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Go,1,52
Ruby,2,43
Rust,5,156
Tree-sitter Query,21,378
Unsupported,1,0
TOTALS,30,629
",
        )
        .success();
}

#[test]
fn test_sortby_numfiles() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--sort-by", "numfiles"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Tree-sitter Query,21,378
Rust,5,156
Ruby,2,43
Go,1,52
Unsupported,1,0
TOTALS,30,629
",
        )
        .success();
}

#[test]
fn test_sortby_tokens() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--sort-by", "tokens"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Tree-sitter Query,21,378
Rust,5,156
Go,1,52
Ruby,2,43
Unsupported,1,0
TOTALS,30,629
",
        )
        .success();
}

#[test]
fn test_hide_totals() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--hide-totals"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Tree-sitter Query,21,378
Rust,5,156
Go,1,52
Ruby,2,43
Unsupported,1,0
",
        )
        .success();
}
