use crate::common::helpers::get_picturify_cli_cmd;
use crate::common::test_files::{TestFiles, WithIo};

#[test]
fn rotate_standard_args_should_succeed() {
    let test_files = TestFiles::new_100x100_png();
    let mut cmd = get_picturify_cli_cmd();
    cmd.arg("core").arg("rotate").with_io(&test_files);

    let assert = cmd.assert();

    assert.success();
}
