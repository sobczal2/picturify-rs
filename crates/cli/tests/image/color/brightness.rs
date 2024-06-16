use picturify_testing::helpers::get_picturify_cli_cmd;
use picturify_testing::test_files::{TestFiles, WithIo};

#[test]
fn brightness_standard_args_should_succeed() {
    let test_files = TestFiles::new_100x100_png();
    let mut cmd = get_picturify_cli_cmd();
    cmd.arg("image")
        .arg("brightness")
        .arg("-f")
        .arg("1.0")
        .with_io(&test_files);

    let assert = cmd.assert();

    assert.success();
}
