// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use image::{Pixel, Rgba};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use crate::Image;

#[derive(Debug)]
pub enum DiffImageMethod {
    RedGreen,
    Overlay,
}

impl Display for DiffImageMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::RedGreen => "RedGreen",
                Self::Overlay => "Overlay",
            }
        )
    }
}

#[derive(Debug)]
pub struct DiffImage {
    pub method: DiffImageMethod,
    pub image: Image,
}

pub enum ImageDifference {
    None,
    SizeMismatch {
        left_size: (u32, u32),
        right_size: (u32, u32),
    },
    Content {
        diff_images: Vec<DiffImage>,
        background: Option<Rgba<u8>>,
        // If the background is not detected, then the following values are related
        // to the whole image. Otherwise, they are related only to pixels that are
        // not background on at least one side
        n_pixels: u64,
        n_different_pixels: u64,
        distance_sum: u64,
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

fn compute_rg_diff_image(left: &Image, right: &Image) -> (Image, u64) {
    let mut distance_sum = 0;
    let diff_image_data: Vec<u8> = left
        .pixels()
        .zip(right.pixels())
        .flat_map(|(&p1, &p2)| {
            let (diff_min, diff_max) = pixel_min_max_distance(p1, p2);
            distance_sum += diff_max.max(diff_min) as u64;
            if diff_min > diff_max {
                [diff_min, 0, 0, u8::MAX]
            } else {
                [0, diff_max, 0, u8::MAX]
            }
        })
        .collect();
    let image = Image::from_vec(left.width(), left.height(), diff_image_data)
        .expect("Same number of pixels as left and right, which have the same dimensions");
    (image, distance_sum)
}

fn compute_overlay_diff_image(left: &Image, right: &Image) -> Image {
    let diff_image_data: Vec<u8> = left
        .pixels()
        .zip(right.pixels())
        .flat_map(|(&p1, &p2)| {
            let distance = pixel_distance(p1, p2);
            if distance > 0 {
                p2.0
            } else {
                let [r, g, b, a] = p1.0;
                if a > 128 {
                    [r, g, b, a / 3]
                } else {
                    [r, g, b, 0]
                }
            }
        })
        .collect();
    Image::from_vec(left.width(), left.height(), diff_image_data)
        .expect("Same number of pixels as left and right, which have the same dimensions")
}

fn detect_background(image: &Image) -> Option<Rgba<u8>> {
    let mut counter: HashMap<Rgba<u8>, u32> = HashMap::new();
    for pixel in image.pixels() {
        counter.entry(*pixel).and_modify(|c| *c += 1).or_insert(1);
    }
    counter
        .into_iter()
        .max_by_key(|(_, c)| *c)
        .filter(|(_, c)| *c > (image.height() * image.height()) / 4)
        .map(|(p, _)| p)
}

/// Find differences between two images
pub fn compare_images(left: &Image, right: &Image) -> ImageDifference {
    if left.width() != right.width() || left.height() != right.height() {
        return ImageDifference::SizeMismatch {
            left_size: (left.width(), left.height()),
            right_size: (right.width(), right.height()),
        };
    }

    let mut n_pixels = left.width() as u64 * right.height() as u64;

    let background = detect_background(left)
        .and_then(|bg1| detect_background(right).and_then(|bg2| (bg1 == bg2).then_some(bg1)));

    let n_different_pixels: u64 = if let Some(bg) = background {
        left.pixels()
            .zip(right.pixels())
            .map(|(pl, pr)| {
                if pl == pr {
                    if *pl == bg {
                        n_pixels -= 1;
                    };
                    0
                } else {
                    1
                }
            })
            .sum()
    } else {
        left.pixels()
            .zip(right.pixels())
            .map(|(pl, pr)| if pl == pr { 0 } else { 1 })
            .sum()
    };
    if n_different_pixels == 0 {
        return ImageDifference::None;
    }

    let (rg_diff_image, distance_sum) = compute_rg_diff_image(left, right);
    let overlay_diff_image = compute_overlay_diff_image(left, right);
    ImageDifference::Content {
        n_pixels,
        n_different_pixels,
        distance_sum,
        background,
        diff_images: vec![
            DiffImage {
                method: DiffImageMethod::RedGreen,
                image: rg_diff_image,
            },
            DiffImage {
                method: DiffImageMethod::Overlay,
                image: overlay_diff_image,
            },
        ],
    }
}

fn pixel_distance(left: Rgba<u8>, right: Rgba<u8>) -> u64 {
    left.channels()
        .iter()
        .zip(right.channels())
        .map(|(c_left, c_right)| c_left.abs_diff(*c_right).into())
        .max()
        .unwrap_or_default()
}

fn pixel_min_max_distance(left: Rgba<u8>, right: Rgba<u8>) -> (u8, u8) {
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
