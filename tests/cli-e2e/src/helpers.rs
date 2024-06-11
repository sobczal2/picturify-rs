use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_picturify_cli_cmd() -> Command {
    let mut cmd = Command::new(env!("CARGO"));
    cmd.arg("run").arg("--bin").arg("picturify-cli").current_dir(workspace_dir());
    cmd
}

fn workspace_dir() -> PathBuf {
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