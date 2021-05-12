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
Go,1,52
Unsupported,1,0
TOTALS,2,52
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
Rust,5,156
Go,1,52
Ruby,2,43
Unsupported,1,0
TOTALS,9,251
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
fn test_groupby_arguments() {
    tc().current_dir("tests/fixtures/")
        .args(
            [
                "--format",
                "csv",
                "--group-by",
                "arg",
                "--",
                "go1.go",
                "foo",
                "ruby.rb",
            ]
            .iter(),
        )
        .assert()
        .stdout(
            r"Group,Files,Tokens
foo,7,199
go1.go,1,52
ruby.rb,1,10
TOTALS,9,261
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
Unsupported,1,0
TOTALS,9,251
",
        )
        .success();
}

#[test]
fn test_sortby_numfiles() {
    tc().current_dir("tests/fixtures/")
        .args(
            [
                "--format",
                "csv",
                "--sort-by",
                "numfiles",
                "--whitelist",
                "Rust",
                "Ruby",
                "Go",
            ]
            .iter(),
        )
        .assert()
        .stdout(
            r"Group,Files,Tokens
Rust,5,156
Ruby,2,43
Go,1,52
TOTALS,8,251
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
Rust,5,156
Go,1,52
Ruby,2,43
Unsupported,1,0
TOTALS,9,251
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
Rust,5,156
Go,1,52
Ruby,2,43
Unsupported,1,0
",
        )
        .success();
}

#[test]
fn test_count_node_kinds() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--kind", "line_comment"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens,Kind(line_comment)
Rust,5,156,9
Go,1,52,0
Ruby,2,43,0
Unsupported,1,0,0
TOTALS,9,251,9
",
        )
        .success();
}

#[test]
fn test_count_node_kind_patterns() {
    tc().current_dir("tests/fixtures/")
        .args(["--format", "csv", "--kind-pattern", ".*comment.*"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens,Pattern(.*comment.*)
Rust,5,156,12
Go,1,52,0
Ruby,2,43,1
Unsupported,1,0,0
TOTALS,9,251,13
",
        )
        .success();
}
