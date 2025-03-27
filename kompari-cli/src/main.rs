// Copyright 2024 the Kompari Authors
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

use clap::Parser;
use kompari::DirDiffConfig;
use kompari_html::{render_html_report, start_review_server, ReportConfig};
use kompari_tasks::check_size_optimizations;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CliReportArgs {
    #[clap(flatten)]
    diff_args: DiffArgs,
    #[clap(flatten)]
    args: kompari_tasks::args::ReportArgs,
}

#[derive(Parser, Debug)]
pub struct CliReviewArgs {
    #[clap(flatten)]
    diff_args: DiffArgs,
    #[clap(flatten)]
    args: kompari_tasks::args::ReviewArgs,
}

#[derive(Parser, Debug, Clone)]
struct DiffArgs {
    /// Path to "left" images
    left_path: PathBuf,

    /// Path to "right" images
    right_path: PathBuf,

    /// Left title
    #[arg(long, default_value = "Left image")]
    left_title: String,

    /// Right title
    #[arg(long, default_value = "Right image")]
    right_title: String,

    /// Ignore left missing files
    #[arg(long, default_value_t = false)]
    ignore_left_missing: bool,

    /// Ignore right missing files
    #[arg(long, default_value_t = false)]
    ignore_right_missing: bool,

    /// Ignore match
    #[arg(long, default_value_t = false)]
    ignore_match: bool,

    /// Filter filenames by name
    #[arg(long)]
    filter: Option<String>,
}

#[derive(Parser, Debug)]
pub struct CliSizeCheckArgs {
    path: PathBuf,

    #[clap(flatten)]
    args: kompari_tasks::args::SizeCheckArgs,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Args {
    Report(CliReportArgs),
    Review(CliReviewArgs),
    SizeCheck(CliSizeCheckArgs),
}

fn make_diff_config(args: DiffArgs) -> (DirDiffConfig, ReportConfig) {
    let mut diff_config = DirDiffConfig::new(args.left_path, args.right_path);
    diff_config.set_ignore_left_missing(args.ignore_left_missing);
    diff_config.set_ignore_right_missing(args.ignore_right_missing);
    diff_config.set_filter_name(args.filter);

    let mut report_config = ReportConfig::default();
    report_config.set_left_title(args.left_title);
    report_config.set_right_title(args.right_title);

    (diff_config, report_config)
}

fn main() -> kompari::Result<()> {
    let args = Args::parse();

    match args {
        Args::Report(args) => {
            let (diff_config, mut report_config) = make_diff_config(args.diff_args);
            let diff = diff_config.create_diff()?;
            report_config.set_embed_images(args.args.embed_images);
            report_config.set_size_optimization(args.args.optimize_size.to_level());
            let report = render_html_report(&report_config, diff.results())?;
            let output = args.args.output.unwrap_or("report.html".into());
            std::fs::write(&output, report)?;
            println!("Report written into '{}'", output.display());
        }
        Args::Review(args) => {
            let (diff_config, mut report_config) = make_diff_config(args.diff_args);
            report_config.set_size_optimization(args.args.optimize_size.to_level());
            start_review_server(&diff_config, &report_config, args.args.port)?
        }
        Args::SizeCheck(args) => {
            check_size_optimizations(&args.path, &args.args)?;
        }
    }
    Ok(())
}
