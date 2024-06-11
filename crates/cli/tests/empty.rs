mod image;
mod common;

use assert_cmd::Command;

#[test]
fn empty_no_args_should_print_missing_command() {
    let mut cmd = Command::cargo_bin("picturify-cli").unwrap();
    let assert = cmd.assert();

    assert
        .failure()
        .stderr(predicates::str::contains("Missing command"));
}