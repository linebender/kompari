// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use image::{Pixel, Rgba};
use std::fmt::{Debug, Formatter};

use crate::Image;

pub enum ImageDifference {
    None,
    SizeMismatch {
        left_size: (u32, u32),
        right_size: (u32, u32),
    },
    Content {
        n_different_pixels: u64,
        distance_sum: u64,
        diff_image: Image,
    },
}

impl Debug for ImageDifference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Difference::None"),
            Self::SizeMismatch {
                left_size,
                right_size,
            } => write!(
                f,
                "Difference::SizeMismatch({:?}, {:?})",
                left_size, right_size
            ),
            Self::Content {
                n_different_pixels, ..
            } => f
                .debug_struct("Difference::Content")
                .field("n_different_pixels", n_different_pixels)
                .finish(),
        }
    }
}

/// Find differences between two images
pub fn compare_images(left: &Image, right: &Image) -> ImageDifference {
    if left.width() != right.width() || left.height() != right.height() {
        return ImageDifference::SizeMismatch {
            left_size: (left.width(), left.height()),
            right_size: (right.width(), right.height()),
        };
    }

    let n_different_pixels: u64 = left
        .pixels()
        .zip(right.pixels())
        .map(|(pl, pr)| if pl == pr { 0 } else { 1 })
        .sum();

    if n_different_pixels == 0 {
        return ImageDifference::None;
    }

    let mut distance_sum: u64 = 0;

    let diff_image_data: Vec<u8> = left
        .pixels()
        .zip(right.pixels())
        .flat_map(|(&p1, &p2)| {
            let (diff_min, diff_max) = pixel_distance(p1, p2);
            distance_sum += diff_max.max(diff_min) as u64;
            if diff_min > diff_max {
                [diff_min, 0, 0, u8::MAX]
            } else {
                [0, diff_max, 0, u8::MAX]
            }
        })
        .collect();

    let diff_image = Image::from_vec(left.width(), left.height(), diff_image_data)
        .expect("Same number of pixels as left and right, which have the same dimensions");

    ImageDifference::Content {
        n_different_pixels,
        distance_sum,
        diff_image,
    }
}

// fn pixel_distance(left: Rgba<u8>, right: Rgba<u8>) -> u64 {
//     left.channels()
//         .iter()
//         .zip(right.channels())
//         .map(|(left, right)| left.abs_diff(*right).into())
//         .max()
//         .unwrap_or_default()
// }
//

fn pixel_distance(left: Rgba<u8>, right: Rgba<u8>) -> (u8, u8) {
    left.channels()
        .iter()
        .zip(right.channels())
        .fold((0, 0), |(min, max), (c1, c2)| {
            if c2 > c1 {
                (min, max.max(c2 - c1))
            } else {
                (min.max(c1 - c2), max)
            }
        })
}