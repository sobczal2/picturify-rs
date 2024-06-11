use assert_cmd::Command;
use crate::common::test_files::{TestFiles, WithIo};

#[test]
fn negative_standard_args_should_succeed() {
    let test_files = TestFiles::new_100x100_png();
    let mut cmd = Command::cargo_bin("picturify-cli").unwrap();
    cmd
        .arg("image")
        .arg("negative")
        .with_io(test_files);
    
    let assert = cmd.assert();
    
    assert
        .success();
}