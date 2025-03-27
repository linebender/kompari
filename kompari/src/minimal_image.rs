// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::io;

use color::Rgba8;
use png::Transformations;

/// A minimally defined Image type on top of the [`color`] crate.
///
/// This is used inside Kompari as the [Image crate](https://github.com/image-rs/image)
/// (which we believe to be the main competitor) has a very significant compile time cost.
pub struct MinImage {
    /// The width of the image, in pixels.
    pub width: u32,
    /// The height of the image, in pixels.
    pub height: u32,
    /// The data of the image, stored in row-major order
    /// (that is, the first `width` elements are the first row)
    pub data: Vec<Rgba8>,
}

impl std::fmt::Debug for MinImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinImage")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("data", &format_args!("{} pixels", self.data.len()))
            .finish()
    }
}

impl MinImage {
    /// Utility to decode from the png data provided by reader into a `MinImage`.
    ///
    /// This method assumes that the image is not grayscale.
    pub fn decode_from_png(mut reader: impl io::Read + io::Seek) -> Result<Self, crate::Error> {
        let start_location = reader.stream_position();
        let error = match Self::png_decode_internal(&mut reader) {
            Ok(ret) => return Ok(ret),
            Err(error) => error,
        };
        // If it wasn't a png file, see if it was actually (probably) an LFS file.
        if let Ok(pos) = start_location {
            if reader.seek(io::SeekFrom::Start(pos)).is_err() {
                return Err(error);
            }
            match try_detect_lfs(reader) {
                // If the file was actually an LFS file, return the more
                // specific error for that case
                Ok(Some(e)) => Err(e),
                // Otherwise, return the error we got from trying to decode as png.
                _ => Err(error),
            }
        } else {
            Err(error)
        }
    }

    pub fn encode_to_png(&self, write: impl io::Write) -> Result<(), crate::Error> {
        let mut encoder = png::Encoder::new(write, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|e| crate::Error::GenericError(format!("Error encoding PNG: {e}")))?;
        writer
            .write_image_data(bytemuck::cast_slice(&self.data))
            .map_err(|e| crate::Error::GenericError(format!("Error encoding PNG: {e}")))?;
        Ok(())
    }

    fn png_decode_internal(source: impl io::Read + io::Seek) -> Result<Self, crate::Error> {
        let mut decoder = png::Decoder::new(source);
        decoder
            // We treat all images as 8 bit per channel, for simplicity.
            .set_transformations(Transformations::normalize_to_color8() | Transformations::ALPHA);
        let mut reader = decoder.read_info().unwrap_or_else(|e| todo!("Handle {e}"));
        let (png::ColorType::Rgba, png::BitDepth::Eight) = reader.output_color_type() else {
            todo!("Give a proper error type for image not being RGBA8");
        };

        let mut buf = Vec::<Rgba8>::with_capacity(reader.output_buffer_size() / 4);
        let data = bytemuck::cast_slice_mut(&mut buf);
        let (width, height) = reader.info().size();
        reader
            .next_frame(data)
            .unwrap_or_else(|e| todo!("Handle {e}"));
        Ok(Self {
            width,
            height,
            data: buf,
        })
    }
}

const LFS_HEADER: &[u8] = b"version https://git-lfs.github.com/spec/v1\n";

fn try_detect_lfs(
    mut reader: impl io::Read + io::Seek,
) -> Result<Option<crate::Error>, crate::Error> {
    let mut buf = vec![0; LFS_HEADER.len()];
    reader.read_exact(&mut buf)?;
    if buf == LFS_HEADER {
        // TODO: More advanced formatting.
        Ok(Some(crate::Error::GenericError(
            "Image is unresolved LFS file. Maybe you need to install lfs - https://git-lfs.com/?"
                .into(),
        )))
    } else {
        Ok(None)
    }
}
