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
    /// Embed images into the report
    #[arg(long, default_value_t = 7200)]
    pub port: u16,
}

#[derive(Parser, Debug)]
pub struct DeadSnapshotArgs {
    #[arg(long, default_value_t = false)]
    pub remove_files: bool,
}