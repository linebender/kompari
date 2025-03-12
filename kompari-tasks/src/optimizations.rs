// Copyright 2025 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::args::SizeCheckArgs;
use humansize::{format_size, DECIMAL};
use kompari::{list_image_dir, optimize_png, SizeOptimizationLevel};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::cmp::min;
use std::io::Write;
use std::path::{Path, PathBuf};
use termcolor::{Color, ColorSpec, WriteColor};

#[derive(Debug)]
pub struct OptimizationResult {
    pub path: PathBuf,
    pub old_size: usize,
    pub new_size: usize,
    pub improvement: f32,
    pub size_limit_breached: bool,
    pub improvement_limit_breached: bool,
}

pub fn check_file_optimizations(
    path: PathBuf,
    args: &SizeCheckArgs,
) -> kompari::Result<Option<OptimizationResult>> {
    let old_data = std::fs::read(&path)?;
    let old_size = old_data.len();
    let new_data = optimize_png(old_data, SizeOptimizationLevel::High);
    let new_size = min(new_data.len(), old_size);

    let is_big = args.size_limit.is_some_and(|limit| {
        if args.optimize {
            new_size > limit * 1024 // Limit is given in KiB
        } else {
            old_size > limit * 1024
        }
    });

    Ok(if old_size > new_size || is_big {
        if args.optimize {
            std::fs::write(&path, new_data)?;
        }
        let improvement = (old_size as f32 - new_size as f32) / old_size as f32;
        Some(OptimizationResult {
            path,
            old_size,
            new_size,
            improvement,
            size_limit_breached: is_big,
            improvement_limit_breached: args
                .improvement_limit
                .is_some_and(|limit| limit < improvement),
        })
    } else {
        None
    })
}

pub fn check_size_optimizations(dir_path: &Path, args: &SizeCheckArgs) -> kompari::Result<()> {
    let paths: Vec<_> = list_image_dir(dir_path)?.collect();
    let progressbar = indicatif::ProgressBar::new(paths.len() as u64);
    let results = paths
        .into_par_iter()
        .map(|path| {
            let result = check_file_optimizations(path, args);
            progressbar.inc(1);
            result
        })
        .collect::<kompari::Result<Vec<_>>>()?;
    progressbar.finish_with_message("");
    let mut results: Vec<_> = results.into_iter().flatten().collect();
    results.sort_unstable_by(|a, b| a.path.cmp(&b.path));
    if print_size_optimization_results(&results, args.optimize)? && !args.optimize {
        std::process::exit(1);
    }
    Ok(())
}

pub fn print_size_optimization_results(
    results: &[OptimizationResult],
    optimize: bool,
) -> kompari::Result<bool> {
    if results.is_empty() {
        println!("Nothing to optimize");
        return Ok(false);
    }
    let stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
    let mut stdout = stdout.lock();
    let mut total_size = 0;
    let mut total_diff = 0;
    let mut has_error = false;
    for result in results {
        let diff = result.old_size - result.new_size;
        stdout.set_color(ColorSpec::new().set_fg(None))?;
        write!(stdout, "{}: ", result.path.display(),)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
        write!(stdout, "{} ", format_size(result.new_size, DECIMAL))?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(stdout, "(-{})", format_size(diff, DECIMAL))?;
        total_size += result.new_size;
        total_diff += diff;
        if result.size_limit_breached {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(stdout, "Size limit breached")?;
            has_error = true;
        }
        if result.improvement_limit_breached {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
            writeln!(stdout, "Improvement limit breached")?;
            has_error = true;
        }
    }
    stdout.set_color(ColorSpec::new().set_fg(None))?;
    write!(stdout, "----------------------------\nTotal size: ",)?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    write!(stdout, "{} ", format_size(total_size, DECIMAL))?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(stdout, "(-{})", format_size(total_diff, DECIMAL))?;
    stdout.set_color(ColorSpec::new().set_fg(None))?;
    if !optimize {
        write!(stdout, "Run with ")?;
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        write!(stdout, "--optimize ")?;
        stdout.set_color(ColorSpec::new().set_fg(None))?;
        writeln!(stdout, "to apply the optimizations")?;
    }
    Ok(has_error)
}
