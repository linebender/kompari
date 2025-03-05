// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn list_image_dir(dir_path: &Path) -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    Ok(WalkDir::new(dir_path).into_iter().filter_map(|entry| {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path
                .extension()
                .and_then(OsStr::to_str)
                .map(|ext| ext.eq_ignore_ascii_case("png"))
                .unwrap_or(false)
            {
                Some(path.to_path_buf())
            } else {
                None
            }
        } else {
            None
        }
    }))
}

pub fn list_image_dir_names(
    dir_path: &Path,
) -> Result<impl Iterator<Item = PathBuf> + '_, std::io::Error> {
    Ok(list_image_dir(dir_path)?.map(move |p| p.strip_prefix(dir_path).unwrap().to_path_buf()))
}
