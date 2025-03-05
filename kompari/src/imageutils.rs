// Copyright 2025 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::Image;
use std::io::Cursor;
use std::path::Path;

pub fn load_image(path: &Path) -> crate::Result<Image> {
    if !path.is_file() {
        return Err(crate::Error::FileNotFound(path.to_path_buf()));
    }
    Ok(image::ImageReader::open(path)?.decode()?.into_rgba8())
}

#[cfg(feature = "oxipng")]
pub fn optimize_png(data: Vec<u8>) -> Vec<u8> {
    oxipng::optimize_from_memory(&data[..], &oxipng::Options::default()).unwrap_or(data)
}

#[cfg(not(feature = "oxipng"))]
pub fn optimize_png(data: Vec<u8>) -> Vec<u8> {
    /* Do nothing */
    data
}

pub fn image_to_png(image: &Image) -> Vec<u8> {
    let mut data = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut data), image::ImageFormat::Png)
        .unwrap();
    optimize_png(data)
}

#[cfg(feature = "oxipng")]
pub fn bless_image(source: &Path, target: &Path) -> crate::Result<()> {
    let image = load_image(source)?;
    let data = image_to_png(&image);
    std::fs::write(target, data)?;
    Ok(())
}

#[cfg(not(feature = "oxipng"))]
pub fn bless_image(source: &Path, target: &Path) -> crate::Result<()> {
    std::fs::copy(source, target)?;
    Ok(())
}
