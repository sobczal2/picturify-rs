use picturify_testing::helpers::get_picturify_cli_cmd;
use picturify_testing::test_files::{TestFiles, WithIo};

#[test]
fn gaussian_blur_standard_args_should_succeed() {
    let test_files = TestFiles::new_100x100_png();
    let mut cmd = get_picturify_cli_cmd();
    cmd.arg("image")
        .arg("gaussian-blur")
        .arg("-s")
        .arg("1.5")
        .with_io(&test_files);

    let assert = cmd.assert();

    assert.success();
}
