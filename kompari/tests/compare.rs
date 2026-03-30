// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use kompari::color::Rgba8;
use kompari::{compare_images, DirDiffConfig, ImageDifference, LeftRightError, MinImage};
use std::path::Path;

fn create_test_diff_config() -> DirDiffConfig {
    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("tests");
    let left = test_dir.join("left");
    let right = test_dir.join("right");
    DirDiffConfig::new(left, right)
}

#[test]
pub(crate) fn test_compare_dir() {
    let diff = create_test_diff_config().create_diff().unwrap();
    let res = diff.results();
    let titles: Vec<_> = res.iter().map(|r| r.title.as_str()).collect();
    assert_eq!(
        titles,
        [
            "bright.png",
            "changetext.png",
            "grayscale.png",
            "left_missing.png",
            "right_missing.png",
            "shift.png",
            "size_error.png",
        ]
    );
    assert!(matches!(
        res[0].image_diff,
        Ok(ImageDifference::Content {
            n_different_pixels: 18623,
            ..
        })
    ));
    assert!(matches!(
        res[1].image_diff,
        Ok(ImageDifference::Content {
            n_different_pixels: 275,
            ..
        })
    ));
    assert!(matches!(
        res[2].image_diff,
        Ok(ImageDifference::Content {
            n_different_pixels: 104,
            ..
        })
    ));
    assert!(matches!(
        res[3].image_diff,
        Err(LeftRightError::Left(kompari::Error::FileNotFound(_)))
    ));
    assert!(matches!(
        res[4].image_diff,
        Err(LeftRightError::Right(kompari::Error::FileNotFound(_)))
    ));
    assert!(matches!(
        res[5].image_diff,
        Ok(ImageDifference::Content {
            n_different_pixels: 3858,
            ..
        })
    ));
    assert!(matches!(
        res[6].image_diff,
        Ok(ImageDifference::SizeMismatch {
            left_size: (850, 88),
            right_size: (147, 881)
        })
    ));
}

#[test]
pub(crate) fn test_ignore_left_missing() {
    let mut config = create_test_diff_config();
    config.set_ignore_left_missing(true);
    let diff = config.create_diff().unwrap();
    let res = diff.results();
    let titles: Vec<_> = res.iter().map(|r| r.title.as_str()).collect();
    assert_eq!(
        titles,
        [
            "bright.png",
            "changetext.png",
            "grayscale.png",
            "right_missing.png",
            "shift.png",
            "size_error.png"
        ]
    );
}

#[test]
pub(crate) fn test_ignore_right_missing() {
    let mut config = create_test_diff_config();
    config.set_ignore_right_missing(true);
    let diff = config.create_diff().unwrap();
    let res = diff.results();
    let titles: Vec<_> = res.iter().map(|r| r.title.as_str()).collect();
    assert_eq!(
        titles,
        [
            "bright.png",
            "changetext.png",
            "grayscale.png",
            "left_missing.png",
            "shift.png",
            "size_error.png"
        ]
    );
}

#[test]
fn test_pixel_distance_tolerance() {
    let px = |r| Rgba8 {
        r,
        g: 0,
        b: 0,
        a: 255,
    };
    let left = MinImage {
        width: 2,
        height: 2,
        data: vec![px(100), px(100), px(100), px(100)],
    };
    let right = MinImage {
        width: 2,
        height: 2,
        // distances: 0, 1, 2, 3
        data: vec![px(100), px(101), px(102), px(103)],
    };
    // With tolerance=2, only the pixel with distance 3 is different.
    let diff = compare_images(&left, &right, 2);
    assert!(matches!(
        diff,
        ImageDifference::Content {
            n_different_pixels: 1,
            ..
        }
    ));
}
