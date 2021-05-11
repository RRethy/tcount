use assert_cmd::Command;

#[test]
fn foo() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .current_dir("tests/fixtures/")
        .arg("--format")
        .arg("csv")
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
