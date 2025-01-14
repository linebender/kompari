use crate::Actions;
use clap::Parser;
use kompari::{list_image_dir, list_image_dir_names, DirDiffConfig};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    Report(ReportArgs),
    Clean,
    DeadSnapshots(DeadSnapshotArgs),
}

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
pub struct DeadSnapshotArgs {
    #[arg(long, default_value_t = false)]
    remove_files: bool,
}

impl Args {
    pub fn run(&self, diff_config: DirDiffConfig, actions: impl Actions) -> kompari::Result<()> {
        match &self.command {
            Command::Report(report_args) => {
                create_report(current_path, snapshots_path, report_args)?;
            }
            Command::Clean => {
                clean_image_dir(current_path)?;
            }
            Command::DeadSnapshots(ds_args) => {
                process_dead_snapshots(current_path, snapshots_path, actions, ds_args)?;
            }
        }
        Ok(())
    }
}

fn clean_image_dir(path: &Path) -> kompari::Result<()> {
    for path in list_image_dir(path)? {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

fn find_dead_snapshots(
    current_path: &Path,
    snapshot_path: &Path,
    actions: impl Actions,
) -> kompari::Result<Vec<PathBuf>> {
    clean_image_dir(current_path)?;
    actions.generate_all_tests()?;
    let current_images: BTreeSet<_> = list_image_dir_names(current_path)?.collect();
    let snapshot_images: BTreeSet<_> = list_image_dir_names(snapshot_path)?.collect();
    Ok(snapshot_images
        .difference(&current_images)
        .map(|name| current_path.join(name))
        .collect())
}

fn process_dead_snapshots(
    current_path: &Path,
    snapshot_path: &Path,
    actions: impl Actions,
    args: &DeadSnapshotArgs,
) -> kompari::Result<()> {
    let dead_snapshots = find_dead_snapshots(current_path, snapshot_path, actions)?;
    if dead_snapshots.is_empty() {
        println!("No dead snapshots detected");
    } else {
        println!("========== DEAD SNAPSHOTS ==========");
        for path in &dead_snapshots {
            println!("{}", path.display());
        }
        println!("====================================");
        if args.remove_files {
            for path in &dead_snapshots {
                std::fs::remove_file(path)?;
            }
            println!("Dead snapshots removed")
        } else {
            println!("Run the command with '--remove' to remove the files")
        }
    }
    clean_image_dir(current_path)?;
    Ok(())
}

fn create_report(
    current_path: &Path,
    snapshot_path: &Path,
    report_args: &ReportArgs,
) -> Result<(), kompari::Error> {
    let mut builder =
        kompari::DirDiffConfig::new(current_path.to_path_buf(), snapshot_path.to_path_buf());
    builder.set_ignore_left_missing(true);

    let diff = builder.build()?;
    let mut report_config = crate::ReportConfig::default();
    report_config.set_left_title("Current test");
    report_config.set_right_title("Snapshot");
    report_config.set_embed_images(report_config.embed_images);
    diff.create_report(&report_config, &report_args.output, true)?;
    Ok(())
}
