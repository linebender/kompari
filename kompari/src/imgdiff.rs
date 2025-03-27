// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use color::Rgba8;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use crate::MinImage;

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
    pub image: MinImage,
}

pub enum ImageDifference {
    None,
    SizeMismatch {
        left_size: (u32, u32),
        right_size: (u32, u32),
    },
    Content {
        diff_images: Vec<DiffImage>,
        background: Option<Rgba8>,
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

fn compute_rg_diff_image(left: &MinImage, right: &MinImage) -> (MinImage, u64) {
    let mut distance_sum = 0;
    let diff_image_data = left
        .data
        .iter()
        .zip(&right.data)
        .map(|(&p1, &p2)| {
            let (diff_min, diff_max) = pixel_min_max_distance(p1, p2);
            distance_sum += diff_max.max(diff_min) as u64;
            if diff_min > diff_max {
                Rgba8 {
                    r: diff_min,
                    g: 0,
                    b: 0,
                    a: u8::MAX,
                }
            } else {
                Rgba8 {
                    r: 0,
                    g: diff_max,
                    b: 0,
                    a: u8::MAX,
                }
            }
        })
        .collect();
    let image = MinImage {
        width: left.width,
        height: left.height,
        data: diff_image_data,
    };
    (image, distance_sum)
}

fn compute_overlay_diff_image(left: &MinImage, right: &MinImage) -> MinImage {
    let diff_image_data = left
        .data
        .iter()
        .zip(&right.data)
        .map(|(&p1, &p2)| {
            let distance = pixel_distance(p1, p2);
            if distance > 0 {
                p2
            } else {
                // Opaque pixels with no difference are made more transparent; we hide sufficiently translucenst ones
                let alpha = if p1.a > 128 { p1.a / 3 } else { 0 };
                Rgba8 { a: alpha, ..p1 }
            }
        })
        .collect();
    MinImage {
        width: left.width,
        height: left.height,
        data: diff_image_data,
    }
}

fn detect_background(image: &MinImage) -> Option<Rgba8> {
    // Color in u32 format to count in image. u32 key because Rgba8 isn't Hash.
    let mut counter: HashMap<u32, u32> = HashMap::new();
    for pixel in &image.data {
        counter
            .entry(pixel.to_u32())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    // If no pixels take up more than a quarter of the image, there is no background
    let threshold = u32::try_from(image.data.len() / 4).unwrap();
    counter
        .into_iter()
        .max_by_key(|(_, c)| *c)
        .filter(|(_, c)| *c > threshold)
        .map(|(packed, _)| Rgba8::from_u32(packed))
}

/// Find differences between two images
pub fn compare_images(left: &MinImage, right: &MinImage) -> ImageDifference {
    if left.width != right.width || left.height != right.height {
        return ImageDifference::SizeMismatch {
            left_size: (left.width, left.height),
            right_size: (right.width, right.height),
        };
    }

    let mut n_pixels = left.width as u64 * right.height as u64;

    let background = detect_background(left)
        .and_then(|bg1| detect_background(right).and_then(|bg2| (bg1 == bg2).then_some(bg1)));

    let n_different_pixels: u64 = if let Some(bg) = background {
        left.data
            .iter()
            .zip(&right.data)
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
        left.data
            .iter()
            .zip(&right.data)
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

fn pixel_distance(left: Rgba8, right: Rgba8) -> u64 {
    left.to_u8_array()
        .iter()
        .zip(&right.to_u8_array())
        .map(|(c_left, c_right)| c_left.abs_diff(*c_right).into())
        .max()
        .unwrap_or_default()
}

fn pixel_min_max_distance(left: Rgba8, right: Rgba8) -> (u8, u8) {
    left.to_u8_array()
        .iter()
        .zip(&right.to_u8_array())
        .fold((0, 0), |(min, max), (c1, c2)| {
            if c2 > c1 {
                (min, max.max(c2 - c1))
            } else {
                (min.max(c1 - c2), max)
            }
        })
}
