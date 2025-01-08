use clap::Parser;
use kompari::DirDiffConfig;
use kompari_html::{render_html_report, start_review_server};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ReportArgs {
    /// Output filename, default 'report.html'
    #[arg(long, default_value = "report.html")]
    output: PathBuf,

    /// Embed images into the report
    #[arg(long, default_value_t = false)]
    embed_images: bool,
}

#[derive(Parser, Debug)]
pub struct ReviewArgs {
    /// Embed images into the report
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
    let diff_config = DirDiffConfig::new(args.left_path, args.right_path);
    let mut report_config = kompari_html::ReportConfig::default();
    report_config.set_left_title(args.left_title);
    report_config.set_right_title(args.right_title);

    match args.command {
        Command::Report(args) => {
            let diff = diff_config.create_diff()?;
            let output = args.output;
            let report = render_html_report(&report_config, diff.results())?;
            std::fs::write(&output, report)?;
            println!("Report written into '{}'", output.display());
        }
        Command::Review(args) => start_review_server(&diff_config, &report_config, args.port)?,
    }
    Ok(())
}
