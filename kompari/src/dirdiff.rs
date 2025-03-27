// Copyright 2025 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::imgdiff::{compare_images, ImageDifference};
use crate::{list_image_dir_names, load_image};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DirDiffConfig {
    left_path: PathBuf,
    right_path: PathBuf,
    ignore_left_missing: bool,
    ignore_right_missing: bool,
    filter_name: Option<String>,
}

impl DirDiffConfig {
    pub fn new(left_path: PathBuf, right_path: PathBuf) -> Self {
        Self {
            left_path,
            right_path,
            ignore_left_missing: false,
            ignore_right_missing: false,
            filter_name: None,
        }
    }

    pub fn left_path(&self) -> &Path {
        &self.left_path
    }

    pub fn right_path(&self) -> &Path {
        &self.right_path
    }

    pub fn create_diff(&self) -> crate::Result<DirDiff> {
        let pairs = pairs_from_paths(
            &self.left_path,
            &self.right_path,
            self.filter_name.as_deref(),
        )?;
        let diffs: Vec<_> = pairs
            .into_par_iter()
            .filter_map(|pair| {
                let image_diff = compute_pair_diff(&pair);
                if matches!(image_diff, Ok(ImageDifference::None)) {
                    return None;
                }
                if self.ignore_left_missing
                    && matches!(image_diff, Err(ref e) if e.is_left_missing())
                {
                    return None;
                }
                if self.ignore_right_missing
                    && matches!(image_diff, Err(ref e) if e.is_right_missing())
                {
                    return None;
                }
                Some(PairResult {
                    title: pair.title,
                    left: pair.left,
                    right: pair.right,
                    image_diff,
                })
            })
            .collect();
        Ok(DirDiff { diffs })
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

#[derive(Debug)]
pub enum LeftRightError {
    Left(crate::Error),
    Right(crate::Error),

    // This case is boxed to silence warning of too big error structure,
    // Moreover, this case should be rare
    Both(Box<(crate::Error, crate::Error)>),
}

impl LeftRightError {
    pub fn left(&self) -> Option<&crate::Error> {
        match self {
            Self::Left(e) => Some(e),
            Self::Both(pair) => Some(&pair.0),
            Self::Right(_) => None,
        }
    }
    pub fn right(&self) -> Option<&crate::Error> {
        match self {
            Self::Right(e) => Some(e),
            Self::Both(pair) => Some(&pair.1),
            Self::Left(_) => None,
        }
    }

    pub fn is_left_missing(&self) -> bool {
        self.left()
            .map(|e| matches!(e, crate::Error::FileNotFound(_)))
            .unwrap_or(false)
    }

    pub fn is_right_missing(&self) -> bool {
        self.right()
            .map(|e| matches!(e, crate::Error::FileNotFound(_)))
            .unwrap_or(false)
    }

    pub fn is_missing_file_error(&self) -> bool {
        matches!(
            self,
            Self::Left(crate::Error::FileNotFound(_)) | Self::Right(crate::Error::FileNotFound(_))
        )
    }
}

#[derive(Debug)]
pub struct PairResult {
    pub title: String,
    pub left: PathBuf,
    pub right: PathBuf,
    pub image_diff: Result<ImageDifference, LeftRightError>,
}

#[derive(Default, Debug)]
pub struct DirDiff {
    diffs: Vec<PairResult>,
}

impl DirDiff {
    pub fn results(&self) -> &[PairResult] {
        &self.diffs
    }
}

pub(crate) struct Pair {
    pub title: String,
    pub left: PathBuf,
    pub right: PathBuf,
}

pub(crate) fn pairs_from_paths(
    left_path: &Path,
    right_path: &Path,
    filter_name: Option<&str>,
) -> crate::Result<Vec<Pair>> {
    if !left_path.is_dir() {
        return Err(crate::Error::NotDirectory(left_path.to_path_buf()));
    }
    if !right_path.is_dir() {
        return Err(crate::Error::NotDirectory(right_path.to_path_buf()));
    }
    let mut names: Vec<_> = list_image_dir_names(left_path)?.collect();
    names.extend(list_image_dir_names(right_path)?);
    names.sort_unstable();
    names.dedup();
    names.retain(|filename| {
        filter_name
            .as_ref()
            .map(|f| filename.to_string_lossy().contains(f))
            .unwrap_or(true)
    });
    Ok(names
        .into_iter()
        .map(|name| {
            let left = left_path.join(&name);
            let right = right_path.join(&name);
            Pair {
                title: name.to_string_lossy().to_string(),
                left,
                right,
            }
        })
        .collect())
}

fn compute_pair_diff(pair: &Pair) -> Result<ImageDifference, LeftRightError> {
    let left = load_image(&pair.left);
    let right = load_image(&pair.right);
    let (left_image, right_image) = match (left, right) {
        (Ok(left_image), Ok(right_image)) => (left_image, right_image),
        (Err(e), Ok(_)) => return Err(LeftRightError::Left(e)),
        (Ok(_), Err(e)) => return Err(LeftRightError::Right(e)),
        (Err(e1), Err(e2)) => return Err(LeftRightError::Both(Box::new((e1, e2)))),
    };
    Ok(compare_images(&left_image, &right_image))
}
