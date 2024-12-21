// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::difference::{compute_differences, Difference, ImageInfoResult, PairResult};
use image::ImageError;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::pair::pairs_from_paths;
use crate::report::render_html_report;
use thiserror::Error;

mod difference;
mod fs;
mod pair;
mod report;

#[cfg(feature = "review")]
mod review;

#[cfg(feature = "xtask-cli")]
pub mod xtask_cli;

#[cfg(feature = "review")]
pub use review::start_review_server;

#[cfg(not(feature = "review"))]
pub fn start_review_server(_config: &ReportConfig, _port: u16) -> Result<()> {
    Err(crate::Error::GenericError(
        "Kompari is not compiled with review support, compile it with `--features=review`"
            .to_string(),
    ))
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Path is a directory: `{0}`")]
    NotDirectory(PathBuf),

    #[error("Image error")]
    ImageError(#[from] ImageError),

    #[error("Error `{0}`")]
    GenericError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct DiffBuilder {
    left_path: PathBuf,
    right_path: PathBuf,
    ignore_match: bool,
    ignore_left_missing: bool,
    ignore_right_missing: bool,
    filter_name: Option<String>,
}

impl DiffBuilder {
    pub fn new(left_path: PathBuf, right_path: PathBuf) -> Self {
        DiffBuilder {
            left_path,
            right_path,
            ignore_match: false,
            ignore_left_missing: false,
            ignore_right_missing: false,
            filter_name: None,
        }
    }

    pub fn build(&self) -> Result<Diff> {
        let pairs = pairs_from_paths(
            &self.left_path,
            &self.right_path,
            self.filter_name.as_deref(),
        )?;
        let mut diffs = compute_differences(pairs);

        if self.ignore_match {
            diffs.retain(|pair| !matches!(pair.difference, Difference::None));
        }

        if self.ignore_left_missing {
            diffs.retain(|pair| !matches!(pair.left_info, ImageInfoResult::Missing));
        }

        if self.ignore_right_missing {
            diffs.retain(|pair| !matches!(pair.right_info, ImageInfoResult::Missing));
        }
        Ok(Diff { diffs })
    }

    pub fn set_ignore_match(&mut self, value: bool) {
        self.ignore_match = value;
    }

    pub fn set_ignore_left_missing(&mut self, value: bool) {
        self.ignore_left_missing = value;
    }

    pub fn set_ignore_right_missing(&mut self, value: bool) {
        self.ignore_right_missing = value;
    }

    pub fn set_filter_name(&mut self, value: Option<String>) {
        self.filter_name = value;
    }
}

pub struct ReportConfig {
    left_title: String,
    right_title: String,
    embed_images: bool,
    is_review: bool,
}

impl Default for ReportConfig {
    fn default() -> Self {
        ReportConfig {
            left_title: "Left image".to_string(),
            right_title: "Right image".to_string(),
            embed_images: false,
            is_review: false,
        }
    }
}

impl ReportConfig {
    pub fn set_left_title<S: ToString>(&mut self, title: S) {
        self.left_title = title.to_string();
    }

    pub fn set_right_title<S: ToString>(&mut self, title: S) {
        self.right_title = title.to_string();
    }

    pub fn set_embed_images(&mut self, embed_images: bool) {
        self.embed_images = embed_images;
    }

    pub fn set_review(&mut self, is_review: bool) {
        self.is_review = is_review;
    }
}

#[derive(Default)]
pub struct Diff {
    diffs: Vec<PairResult>,
}

impl Diff {
    pub fn render_report(&self, config: &ReportConfig) -> Result<String> {
        render_html_report(config, &self.diffs)
    }

    pub fn create_report(&self, config: &ReportConfig, output: &Path, verbose: bool) -> Result<()> {
        if verbose && self.diffs.is_empty() {
            println!("Nothing to report");
            return Ok(());
        }
        let count = self.diffs.len();
        let report = self.render_report(config)?;
        let mut file = File::create(output)?;
        file.write_all(report.as_bytes())?;
        if verbose {
            println!(
                "Report written into '{}'; found {} images",
                output.display(),
                count,
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // CI will fail unless cargo nextest can execute at least one test per workspace.
    // Delete this dummy test once we have an actual real test.
    #[test]
    fn dummy_test_until_we_have_a_real_test() {}
}
