// Copyright 2025 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::MinImage;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum SizeOptimizationLevel {
    None,
    Fast,
    High,
}

pub fn load_image(path: &Path) -> crate::Result<MinImage> {
    log::debug!("Loading image {}", path.display());
    if !path.is_file() {
        return Err(crate::Error::FileNotFound(path.to_path_buf()));
    }
    let file = File::open(path)?;
    MinImage::decode_from_png(file)
}

#[cfg(feature = "oxipng")]
pub fn optimize_png(data: Vec<u8>, opt_level: SizeOptimizationLevel) -> Vec<u8> {
    let preset = match opt_level {
        SizeOptimizationLevel::None => return data,
        SizeOptimizationLevel::Fast => 2,
        SizeOptimizationLevel::High => 5,
    };
    let mut options = oxipng::Options::from_preset(preset);
    options.grayscale_reduction = false;
    oxipng::optimize_from_memory(&data[..], &options)
        .inspect_err(|e| log::warn!("PNG optimization failed: {}", e))
        .unwrap_or(data)
}

#[cfg(not(feature = "oxipng"))]
pub fn optimize_png(data: Vec<u8>, _opt_level: SizeOptimizationLevel) -> Vec<u8> {
    /* Do nothing */
    data
}

pub fn image_to_png(image: &MinImage, opt_level: SizeOptimizationLevel) -> Vec<u8> {
    let mut data = Vec::new();
    image.encode_to_png(&mut data).unwrap();
    optimize_png(data, opt_level)
}

#[cfg(feature = "oxipng")]
pub fn bless_image(source: &Path, target: &Path) -> crate::Result<()> {
    let image = load_image(source)?;
    let data = image_to_png(&image, SizeOptimizationLevel::High);
    std::fs::write(target, data)?;
    Ok(())
}

#[cfg(not(feature = "oxipng"))]
pub fn bless_image(source: &Path, target: &Path) -> crate::Result<()> {
    std::fs::copy(source, target)?;
    Ok(())
}
