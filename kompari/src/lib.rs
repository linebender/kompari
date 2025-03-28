// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A shared image diffing implementation, to be used in testing and developer tools.
//!
//! This crate also includes utilities for creating image snapshot test suites.

// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
#![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use image;
use std::path::PathBuf;
use thiserror::Error;

mod dirdiff;
mod fsutils;
mod imageutils;
mod imgdiff;

/// The image type used throughout Kompari.
pub type Image = image::RgbaImage;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Path is a directory: `{0}`")]
    NotDirectory(PathBuf),

    #[error("File not found: `{0}`")]
    FileNotFound(PathBuf),

    #[error("Image error")]
    ImageError(#[from] image::ImageError),

    #[error("Error `{0}`")]
    GenericError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub use dirdiff::{DirDiff, DirDiffConfig, LeftRightError, PairResult};
pub use fsutils::{list_image_dir, list_image_dir_names};
pub use imageutils::{bless_image, image_to_png, load_image, optimize_png, SizeOptimizationLevel};
pub use imgdiff::{compare_images, DiffImage, DiffImageMethod, ImageDifference};
