// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::pageconsts::{CSS_STYLE, ICON, JS_CODE};
use crate::ReportConfig;
use base64::prelude::*;
use chrono::SubsecRound;
use kompari::{ImageDifference, LeftRightError, PairResult};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::cmp::min;
use std::path::Path;

const IMAGE_SIZE_LIMIT: u32 = 400;
const IMAGE_PIXELIZE_LIMIT: u32 = 400;

fn embed_png_url(data: &[u8]) -> String {
    let mut url = "data:image/png;base64,".to_string();
    url.push_str(&base64::engine::general_purpose::STANDARD.encode(data));
    url
}

fn render_image(
    config: &ReportConfig,
    path: &Path,
    error: Option<&kompari::Error>,
) -> kompari::Result<Markup> {
    Ok(match error {
        None => {
            let (path, size) = if config.embed_images {
                let image_data =
                    kompari::optimize_png(std::fs::read(path)?, config.size_optimization);
                (
                    embed_png_url(&image_data),
                    imagesize::blob_size(&image_data)
                        .map_err(|e| kompari::Error::GenericError(e.to_string()))?,
                )
            } else {
                (
                    path.display().to_string(),
                    imagesize::size(path)
                        .map_err(|e| kompari::Error::GenericError(e.to_string()))?,
                )
            };
            let (w, h) = html_size(size.width as u32, size.height as u32, IMAGE_SIZE_LIMIT);
            html! {
                img class="zoom" src=(path)
                    width=[w] height=[h]
                    onclick=(open_image_dialog(size.width as u32, size.height as u32));
            }
        }
        Some(kompari::Error::FileNotFound(_)) => {
            html! { "File is missing" }
        }
        Some(err) => {
            html! { "Error: " (err) }
        }
    })
}

pub fn html_size(width: u32, height: u32, size_limit: u32) -> (Option<u32>, Option<u32>) {
    if width > height {
        (Some(width.min(size_limit)), None)
    } else {
        (None, Some(height.min(size_limit)))
    }
}

fn open_image_dialog(width: u32, height: u32) -> &'static str {
    if min(width, height) < IMAGE_PIXELIZE_LIMIT {
        "openImageDialog(this, true)"
    } else {
        "openImageDialog(this, false)"
    }
}

fn render_difference_image(
    config: &ReportConfig,
    id: usize,
    difference: &Result<ImageDifference, LeftRightError>,
) -> Markup {
    match difference {
        Ok(ImageDifference::Content { diff_images, .. }) => {
            html! {
                @for (idx, di) in diff_images.iter().enumerate() {
                    @let (w, h, data) = {
                        let (w, h) = html_size(
                            di.image.width(),
                            di.image.height(),
                            IMAGE_SIZE_LIMIT,
                        );
                        let data = kompari::image_to_png(&di.image, config.size_optimization);
                        (w, h, data)
                   };
                   @let style = if idx == 0 { None } else { Some("display: none") };
                   img id=(format!("img-diff-{id}-{idx}"))
                       style=[style]
                       class="zoom"
                       src=(embed_png_url(&data))
                       width=[w] height=[h]
                       onclick=(open_image_dialog(di.image.width(), di.image.height()));
                }
                div class="tabs" {
                    @for (idx, img) in diff_images.iter().enumerate() {
                        @let class = if idx == 0 { "tab active" } else { "tab" };
                        div id=(format!("tab-diff-{id}-{idx}")) class=(class) {(img.method.to_string())};
                    }
                }
                script {
                    @for idx in 0..diff_images.len() {
                        (PreEscaped(format!("document.getElementById('tab-diff-{id}-{idx}').addEventListener('click', () => switchDiffTab({id}, {idx}, {}));", diff_images.len())))
                    }
                }
            }
        }
        _ => html!("N/A"),
    }
}

fn render_stat_item(label: &str, value_type: &str, value: &str) -> Markup {
    html! {
        div .stat-item {
            div .stat-label {
                (label)
            }
            @let value_class = format!("stat-value {}", value_type);
            div class=(value_class) {
                (value)
            }
        }
    }
}

fn render_difference_info(
    config: &ReportConfig,
    difference: &Result<ImageDifference, LeftRightError>,
) -> Markup {
    match difference {
        Ok(ImageDifference::None) => render_stat_item("Status", "ok", "Match"),
        Ok(ImageDifference::SizeMismatch {
            left_size,
            right_size,
        }) => html! {
            (render_stat_item("Status", "error", "Size mismatch"))
            (render_stat_item(&format!("{} size", config.left_title), "", &format!("{}x{}", left_size.0, left_size.1)))
            (render_stat_item(&format!("{} size", config.right_title), "", &format!("{}x{}", right_size.0, right_size.1)))
        },
        Ok(ImageDifference::Content {
            n_pixels,
            n_different_pixels,
            distance_sum,
            ..
        }) => {
            let n_pixels = (*n_pixels) as f32;
            let pct = *n_different_pixels as f32 / n_pixels * 100.0;
            let distance_sum = *distance_sum as f32 / 255.0; // Normalize
            let avg_color_distance = distance_sum / n_pixels;
            html! {
                (render_stat_item("Different pixels", "warning", &format!("{n_different_pixels} ({pct:.1}%)")))
                (render_stat_item("Color distance", "", &format!("{distance_sum:.3}")))
                (render_stat_item("Avg. color distance", "", &format!("{avg_color_distance:.4}")))
            }
        }
        Err(e) if e.is_missing_file_error() => render_stat_item("Status", "error", "Missing file"),
        Err(_) => render_stat_item("Status", "error", "Loading error"),
    }
}

fn render_pair_diff(
    config: &ReportConfig,
    id: usize,
    pair_diff: &PairResult,
) -> kompari::Result<Markup> {
    Ok(html! {
        div class="diff-entry" {
            h2 {
                @if config.is_review {
                    label class="toggle-switch" {
                        input type="checkbox" id=(format!("t{id}"));
                        span class="slider";
                    }
                    script {
                        (format!("document.getElementById('t{id}').addEventListener('change', toggle)"))
                    }
                }
                (pair_diff.title)};
            div class="comparison-container" {
                div class="image-container" {
                    div class="stats-container" {
                        (render_difference_info(config, &pair_diff.image_diff))
                    }
                    div class="image-box" {
                        h3 { (config.left_title) }
                        (render_image(config, &pair_diff.left, if let Err(e) = &pair_diff.image_diff { e.left() } else { None })?)
                    }
                    div class="image-box" {
                        h3 { (config.right_title) }
                        (render_image(config, &pair_diff.right, if let Err(e) = &pair_diff.image_diff { e.right() } else { None })?)
                    }
                    div class="image-box" {
                        h3 { "Difference"}
                        (render_difference_image(config, id, &pair_diff.image_diff))
                    }
                }
            }
        }
    })
}

pub fn render_html_report(config: &ReportConfig, diffs: &[PairResult]) -> kompari::Result<String> {
    let now = chrono::Local::now().round_subsecs(0);
    let rendered_diffs: Vec<Markup> = diffs
        .par_iter()
        .enumerate()
        .map(|(id, pair_diff)| render_pair_diff(config, id, pair_diff))
        .collect::<kompari::Result<Vec<_>>>()?;
    let title = PreEscaped(if config.is_review {
        "Kompari review"
    } else {
        "Kompari report"
    });
    let report = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="generator" content=(format!("Kompari {}", env!("CARGO_PKG_VERSION")));
                title { (title) }
                style { (PreEscaped(CSS_STYLE)) }
                link rel="icon" type="image/png" href=(embed_png_url(&ICON));
            }
            body {
                 div class="header" {
                    h1 { img class="logo" src=(embed_png_url(ICON)) width="32" height="32"; (title) }
                    p { "Generated on " (now) }
                }
                dialog id="imageDialog" {
                    img id="zoomedImage" class="zoomed-image" src="" alt="Zoomed Image";
                }
                @if config.is_review {
                    script { (format!("const nTests = {};", diffs.len())) }
                    button class="accept-button" id="acceptButton" disabled onClick="acceptTests()" {
                        span class="button-text" id="acceptText" { (format!("Accept selected cases (0 / {})", diffs.len())) }
                    }
                    span class="hint" { "Accepting a case copies '" (config.right_title) "' to '" (config.left_title) "'" }
                    span id="errorMsg" {};
                }
                script { (PreEscaped(JS_CODE)) }
                @for chunk in rendered_diffs  {
                   (chunk)
                }
            }
        }
    };
    Ok(report.into_string())
}
