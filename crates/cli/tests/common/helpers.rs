use std::fs::create_dir;
use std::path::{Path, PathBuf};

use assert_cmd::Command;

pub fn workspace_dir() -> PathBuf {
    let output = Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

pub fn get_sample_100x100_png_path() -> PathBuf {
    workspace_dir().join("assets/samples/sample_100x100.png")
}

pub fn get_null_path() -> PathBuf {
    let workspace_dir = workspace_dir();
    if !workspace_dir.join("assets/null").exists() {
        create_dir(workspace_dir.join("assets/null")).unwrap();
    }
    workspace_dir.join("assets/null")
}

pub fn get_picturify_cli_cmd() -> Command {
    Command::cargo_bin("picturify-cli").unwrap()
}
