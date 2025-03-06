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
    cmd.arg("report").arg(&left).arg(&right);
    cmd.current_dir(&workdir);
    cmd.assert().success();
    let result: Vec<_> = std::fs::read_dir(&workdir)
        .unwrap()
        .map(|e| e.unwrap().file_name().to_str().unwrap().to_owned())
        .collect();
    assert_eq!(result, vec!["report.html"]);
    let report = std::fs::read_to_string(workdir.path().join("report.html")).unwrap();
    for name in [
        "bright",
        "changetext",
        "right_missing",
        "left_missing",
        "shift",
        "size_error",
    ] {
        assert!(report.contains(&format!("{}.png", name)));
    }
    assert!(!report.contains("same.png"));
}

#[test]
fn test_filter_filenames() {
    let workdir = TempDir::new().unwrap();
    std::env::set_current_dir(workdir.path()).unwrap();

    let test_dir = test_assets_dir();
    let left = test_dir.join("left");
    let right = test_dir.join("right");
    let mut cmd = Command::cargo_bin("kompari-cli").unwrap();
    cmd.arg("report").arg("--filter").arg("change");
    cmd.arg(&left).arg(&right);
    cmd.current_dir(&workdir);
    cmd.assert().success();
    let report = std::fs::read_to_string(workdir.path().join("report.html")).unwrap();
    assert!(report.contains("changetext.png"));
    for name in [
        "bright",
        "right_missing",
        "left_missing",
        "shift",
        "size_error",
    ] {
        assert!(!report.contains(&format!("{}.png", name)));
    }
}
