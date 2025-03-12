// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    Report(ReportArgs),
    Review(ReviewArgs),
    Clean,
    DeadSnapshots(DeadSnapshotArgs),
    SizeCheck(SizeCheckArgs),
}

#[derive(Parser, Debug)]
pub struct ReportArgs {
    /// Output filename
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Embed images into the report
    #[arg(long, default_value_t = false)]
    pub embed_images: bool,
}

#[derive(Parser, Debug)]
pub struct ReviewArgs {
    /// Port for web server
    #[arg(long, default_value_t = 7200)]
    pub port: u16,
}

#[derive(Parser, Debug)]
pub struct DeadSnapshotArgs {
    #[arg(long, default_value_t = false)]
    pub remove_files: bool,
}

#[derive(Parser, Debug)]
pub struct SizeCheckArgs {
    /// If enabled, images on file system are replaced with optimized version
    #[arg(long, default_value_t = false)]
    pub optimize: bool,

    /// Command will fail if at least one image can be optimized by more than given ratio.
    /// E.g. --improvement-limit=0.8 means that error is signaled when an image can be optimized
    /// more than 80% of its original size
    /// (that is, the optimized image's size is 20% or less of the original).
    #[arg(long)]
    pub improvement_limit: Option<f32>,

    /// Command will fail if at least one image has a size larger than the given limit (in KiB).
    /// If --optimize is used then limit is computed from target size, otherwise the limit is applied
    /// on the original size
    #[arg(long)]
    pub size_limit: Option<usize>,
}
