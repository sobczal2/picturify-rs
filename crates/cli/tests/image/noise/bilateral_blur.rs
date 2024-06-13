use crate::common::helpers::get_picturify_cli_cmd;
use crate::common::test_files::{TestFiles, WithIo};

#[test]
fn bilateral_blur_standard_args_should_succeed() {
    let test_files = TestFiles::new_100x100_png();
    let mut cmd = get_picturify_cli_cmd();
    cmd.arg("core")
        .arg("bilateral-blur")
        .arg("--ss")
        .arg("1.0")
        .arg("--si")
        .arg("1.0")
        .with_io(&test_files);

    let assert = cmd.assert();

    assert.success();
}
