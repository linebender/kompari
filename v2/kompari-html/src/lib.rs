mod pageconsts;
mod report;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
