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
