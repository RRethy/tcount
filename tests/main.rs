mod utils;

use utils::tcount;

#[test]
fn test_whitelist() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--whitelist", "Rust", "Ruby"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Rust,5,156
Ruby,2,43
",
        )
        .success();
}

#[test]
fn test_blacklist() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--blacklist", "Rust", "Ruby"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Go,1,52
Unsupported,1,0
",
        )
        .success();
}

#[test]
fn test_groupby_language() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--groupby", "language"].iter())
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
fn test_groupby_file() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--groupby", "file", "--whitelist", "Go"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
./go1.go,1,52
",
        )
        .success();
}

#[test]
fn test_groupby_arguments() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(
            [
                "--format",
                "csv",
                "--groupby",
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
",
        )
        .success();
}

#[test]
fn test_sortby_group() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--sort-by", "group"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Go,1,52
Ruby,2,43
Rust,5,156
Unsupported,1,0
",
        )
        .success();
}

#[test]
fn test_sortby_numfiles() {
    tcount()
        .current_dir("tests/fixtures/")
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
",
        )
        .success();
}

#[test]
fn test_sortby_tokens() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--sort-by", "tokens"].iter())
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
fn test_show_totals() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--show-totals"].iter())
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
fn test_count_node_kinds() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--kind", "line_comment"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens,Kind(line_comment)
Rust,5,156,9
Go,1,52,0
Ruby,2,43,0
Unsupported,1,0,0
",
        )
        .success();
}

#[test]
fn test_count_node_kind_patterns() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--kind-pattern", ".*comment.*"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens,Pattern(.*comment.*)
Rust,5,156,12
Go,1,52,0
Ruby,2,43,1
Unsupported,1,0,0
",
        )
        .success();
}

#[test]
fn test_top_n() {
    tcount()
        .current_dir("tests/fixtures/")
        .args(["--format", "csv", "--top", "2"].iter())
        .assert()
        .stdout(
            r"Group,Files,Tokens
Rust,5,156
Go,1,52
",
        )
        .success();
}
