use crate::common::helpers::get_picturify_cli_cmd;

#[test]
fn empty_no_args_should_print_missing_subcommand() {
    let mut cmd = get_picturify_cli_cmd();
    cmd.arg("image");
    let assert = cmd.assert();

    assert
        .failure()
        .stderr(predicates::str::contains("Missing subcommand"));
}
