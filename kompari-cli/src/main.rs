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
use kompari_html::{render_html_report, start_review_server};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ReportArgs {
    /// Output filename
    #[arg(long, default_value = "report.html")]
    output: PathBuf,

    /// Embed images into the report
    #[arg(long, default_value_t = false)]
    embed_images: bool,
}

#[derive(Parser, Debug)]
pub struct ReviewArgs {
    /// Port for web server
    #[arg(long, default_value_t = 7200)]
    port: u16,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
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

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    Report(ReportArgs),
    Review(ReviewArgs),
}

fn main() -> kompari::Result<()> {
    let args = Args::parse();
    let mut diff_config = DirDiffConfig::new(args.left_path, args.right_path);
    diff_config.set_ignore_left_missing(args.ignore_left_missing);
    diff_config.set_ignore_right_missing(args.ignore_right_missing);
    diff_config.set_filter_name(args.filter);

    let mut report_config = kompari_html::ReportConfig::default();
    report_config.set_left_title(args.left_title);
    report_config.set_right_title(args.right_title);

    match args.command {
        Command::Report(args) => {
            let diff = diff_config.create_diff()?;
            report_config.set_embed_images(args.embed_images);
            let report = render_html_report(&report_config, diff.results())?;
            let output = args.output;
            std::fs::write(&output, report)?;
            println!("Report written into '{}'", output.display());
        }
        Command::Review(args) => start_review_server(&diff_config, &report_config, args.port)?,
    }
    Ok(())
}
