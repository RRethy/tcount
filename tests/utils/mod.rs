pub fn tcount() -> assert_cmd::Command {
    assert_cmd::Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}
