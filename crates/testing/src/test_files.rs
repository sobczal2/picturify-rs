use std::fs::remove_file;
use std::path::{Path, PathBuf};

use crate::helpers::{get_null_path, get_sample_100x100_png_path};
use assert_cmd::Command;
use uuid::Uuid;

pub struct TestFiles {
    input: PathBuf,
    output: PathBuf,
}

impl TestFiles {
    pub fn new_100x100_png() -> Self {
        let input = get_sample_100x100_png_path();
        let uuid = Uuid::new_v4();
        let output = get_null_path().join(format!("{}.png", uuid));
        Self { input, output }
    }

    pub fn input(&self) -> &Path {
        &self.input
    }

    pub fn output(&self) -> &Path {
        &self.output
    }
}

impl Drop for TestFiles {
    fn drop(&mut self) {
        if self.output.exists() {
            remove_file(&self.output).unwrap();
        }
    }
}

pub trait WithIo {
    fn with_io(&mut self, test_files: &TestFiles) -> &mut Self;
}

impl WithIo for Command {
    fn with_io(&mut self, test_files: &TestFiles) -> &mut Self {
        self.arg("--input")
            .arg(test_files.input())
            .arg("--output")
            .arg(test_files.output());
        self
    }
}
