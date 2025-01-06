// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::Parser;
use kompari::{start_review_server, DiffBuilder, ReportConfig};
use std::path::PathBuf;

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
struct ReportArgs {
    /// Output filename, default 'report.html'
    #[arg(long, default_value = "report.html")]
    output: PathBuf,

    /// Embed images into the report
    #[arg(long, default_value_t = false)]
    embed_images: bool,
}

#[derive(Parser, Debug)]
struct ReviewArgs {
    /// Embed images into the report
    #[arg(long, default_value_t = 7200)]
    port: u16,
}

#[derive(Parser, Debug)]
enum Command {
    Report(ReportArgs),
    Review(ReviewArgs),
}

fn process_command(args: Args) -> kompari::Result<()> {
    let mut builder = DiffBuilder::new(args.left_path, args.right_path);
    builder.set_ignore_match(args.ignore_match);
    builder.set_ignore_left_missing(args.ignore_left_missing);
    builder.set_ignore_right_missing(args.ignore_right_missing);
    builder.set_filter_name(args.filter);

    match args.command {
        Command::Report(opts) => {
            let diff = builder.build()?;
            let mut config = ReportConfig::default();
            config.set_left_title(&args.left_title);
            config.set_right_title(&args.right_title);
            config.set_embed_images(opts.embed_images);
            diff.create_report(&config, &opts.output, true)?;
        }
        Command::Review(opts) => {
            let mut config = ReportConfig::default();
            config.set_left_title(&args.left_title);
            config.set_right_title(&args.right_title);
            config.set_review(true);
            config.set_embed_images(true);
            start_review_server(builder, config, opts.port)?;
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = process_command(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
