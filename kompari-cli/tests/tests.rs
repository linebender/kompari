// Copyright 2025 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use assert_cmd::Command;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn test_assets_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("tests")
}

#[test]
fn test_create_report() {
    let workdir = TempDir::new().unwrap();
    std::env::set_current_dir(workdir.path()).unwrap();

    let test_dir = test_assets_dir();
    let left = test_dir.join("left");
    let right = test_dir.join("right");
    let mut cmd = Command::cargo_bin("kompari-cli").unwrap();
    cmd.arg(&left).arg(&right).arg("report");
    cmd.current_dir(&workdir);
    cmd.assert().success();
    let result: Vec<_> = std::fs::read_dir(&workdir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_str().unwrap().to_owned())
        .collect();
    assert_eq!(result, vec!["report.html"]);
}
