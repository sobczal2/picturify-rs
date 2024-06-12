mod image;
mod common;

use crate::common::helpers::get_picturify_cli_cmd;

#[test]
fn empty_no_args_should_print_missing_command() {
    let mut cmd = get_picturify_cli_cmd();
    let assert = cmd.assert();

    assert
        .failure()
        .stderr(predicates::str::contains("Missing command"));
}