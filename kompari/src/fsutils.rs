// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub fn list_image_dir(dir_path: &Path) -> Result<impl Iterator<Item = PathBuf>, std::io::Error> {
    Ok(std::fs::read_dir(dir_path)?.filter_map(|entry| {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path
                .extension()
                .and_then(OsStr::to_str)
                .map(|ext| ext.eq_ignore_ascii_case("png"))
                .unwrap_or(false)
            {
                Some(path)
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
) -> Result<impl Iterator<Item = OsString>, std::io::Error> {
    Ok(list_image_dir(dir_path)?.filter_map(|p| p.file_name().map(|name| name.to_os_string())))
}

pub(crate) fn load_image(path: &Path) -> crate::Result<crate::Image> {
    if !path.is_file() {
        return Err(crate::Error::FileNotFound(path.to_path_buf()));
    }
    Ok(image::ImageReader::open(path)?.decode()?.into_rgba8())
}
