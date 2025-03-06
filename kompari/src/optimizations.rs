use crate::{list_image_dir, optimize_png, SizeOptimizationLevel};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct OptimizationResult {
    pub path: PathBuf,
    pub old_size: usize,
    pub new_size: usize,
}

pub fn check_size_optimizations(
    dir_path: &Path,
    optimize: bool,
) -> crate::Result<Vec<OptimizationResult>> {
    let mut results = Vec::new();
    for path in list_image_dir(dir_path)? {
        let old_data = std::fs::read(&path)?;
        let old_size = old_data.len();
        let new_data = optimize_png(old_data, SizeOptimizationLevel::High);
        let new_size = new_data.len();
        if old_size > new_size {
            if optimize {
                std::fs::write(&path, new_data)?;
            }
            results.push(OptimizationResult {
                path,
                old_size,
                new_size,
            });
        }
    }
    results.sort_unstable_by(|a, b| a.path.cmp(&b.path));
    Ok(results)
}
