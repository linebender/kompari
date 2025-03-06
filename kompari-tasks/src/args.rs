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
}
