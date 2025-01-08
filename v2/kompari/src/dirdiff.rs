use std::fs::File;
use std::path::{Path, PathBuf};

pub struct DirDiffConfig {
    left_path: PathBuf,
    right_path: PathBuf,
    ignore_left_missing: bool,
    ignore_right_missing: bool,
    filter_name: Option<String>,
}

impl DirDiffConfig {
    pub fn new(left_path: PathBuf, right_path: PathBuf) -> Self {
        DirDiffConfig {
            left_path,
            right_path,
            ignore_left_missing: false,
            ignore_right_missing: false,
            filter_name: None,
        }
    }

    pub fn create_diff(&self) -> Result<DirDiff> {
        let pairs = pairs_from_paths(
            &self.left_path,
            &self.right_path,
            self.filter_name.as_deref(),
        )?;
        let mut diffs = compute_differences(pairs);

        if self.ignore_left_missing {
            diffs.retain(|pair| !matches!(pair.left_info, ImageInfoResult::Missing));
        }

        if self.ignore_right_missing {
            diffs.retain(|pair| !matches!(pair.right_info, ImageInfoResult::Missing));
        }
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

enum DiffResult {
    LeftMissing,
    RightMissing,
    ImageDifference(ImageDifference)
}

pub struct Pair {
    pub title: String,
    pub left: PathBuf,
    pub right: PathBuf,
}

pub struct PairResult {
    pair: Pair,

}

#[derive(Default)]
pub struct DirDiff {
    diffs: Vec<PairResult>,
}

impl DirDiff {

}
