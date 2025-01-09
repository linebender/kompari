// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::path::Path;
use crate::dirdiff::DirDiffConfig;

#[test]
pub fn test_compare_dir() {
    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent().unwrap().join("tests");
    let left = test_dir.join("left");
    let right = test_dir.join("right");

    let diff = DirDiffConfig::new(left, right).create_diff().unwrap();
    let res = diff.results();
    assert_eq!(res.len(), 1);

}