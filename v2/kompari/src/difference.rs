use image::{Pixel, Rgba};

use crate::Image;

#[derive(Debug)]
pub enum Difference {
    None,
    MissingFile,
    LoadError,
    SizeMismatch,
    Content {
        n_different_pixels: u64,
        distance_sum: u64,
        diff_image: Image,
    },
}

pub fn compare_images(old: &Image, new: &Image) -> Difference {
    if old.width() != new.width() || old.height() != new.height() {
        return Difference::SizeMismatch;
    }

    let n_different_pixels: u64 = old
        .pixels()
        .zip(new.pixels())
        .map(|(pl, pr)| if pl == pr { 0 } else { 1 })
        .sum();

    if n_different_pixels == 0 {
        return Difference::None;
    }

    let mut distance_sum: u64 = 0;

    let diff_image_data: Vec<u8> = old
        .pixels()
        .zip(new.pixels())
        .flat_map(|(&p1, &p2)| {
            let total_distance = pixel_distance(p1, p2);
            distance_sum += total_distance;
            if total_distance > 0 {
                // Highlight the differences atop a semi-transparent view of the shared parts.
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

    let diff_image = Image::from_vec(old.width(), old.height(), diff_image_data)
        .expect("Same number of pixels as left and right, which have the same dimensions");

    Difference::Content {
        n_different_pixels,
        distance_sum,
        diff_image,
    }
}

fn pixel_distance(old: Rgba<u8>, new: Rgba<u8>) -> u64 {
    old.channels()
        .iter()
        .zip(new.channels())
        .map(|(left, right)| left.abs_diff(*right).into())
        .max()
        .unwrap_or_default()
}
