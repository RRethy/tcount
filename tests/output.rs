mod utils;

use utils::tc;

#[test]
fn test_format_csv() {
    let expected = r"Group,Files,Tokens
Tree-sitter Query,21,378
Rust,5,156
Go,1,52
Ruby,2,43
Unsupported,1,0
TOTALS,30,629
";

    tc()
        .current_dir("tests/fixtures")
        .args(["--format", "csv"].iter())
        .assert()
        .stdout(expected)
        .success();
}

#[test]
fn test_format_table() {
let expected = r"╭──────────────────────────────────╮
│ Group              Files  Tokens │
│──────────────────────────────────│
│ Tree-sitter Query     21     378 │
│ Rust                   5     156 │
│ Go                     1      52 │
│ Ruby                   2      43 │
│ Unsupported            1       0 │
│ TOTALS                30     629 │
╰──────────────────────────────────╯
";
    tc()
        .current_dir("tests/fixtures")
        .args(["--format", "table"].iter())
        .assert()
        .stdout(expected)
        .success();
}
