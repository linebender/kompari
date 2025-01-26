// Copyright 2025 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// LINEBENDER LINT SET - lib.rs - v3
// See https://linebender.org/wiki/canonical-lints/
// These lints shouldn't apply to examples or tests.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
// These lints shouldn't apply to examples.
// #![warn(clippy::print_stdout, clippy::print_stderr)]
// Targeting e.g. 32-bit means structs containing usize can give false positives for 64-bit.
#![cfg_attr(target_pointer_width = "64", warn(clippy::trivially_copy_pass_by_ref))]
// END LINEBENDER LINT SET
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod pageconsts;
mod report;
mod review;

#[derive(Debug, Clone)]
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
    pub fn set_left_title(&mut self, value: impl ToString) {
        self.left_title = value.to_string()
    }
    pub fn set_right_title(&mut self, value: impl ToString) {
        self.right_title = value.to_string()
    }
    pub fn set_embed_images(&mut self, value: bool) {
        self.embed_images = value
    }
    pub fn set_review(&mut self, value: bool) {
        self.is_review = value
    }
}

pub use report::render_html_report;
pub use review::start_review_server;
