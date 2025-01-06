// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::fs::{list_image_dir, list_image_dir_names};
use clap::Parser;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct XtaskArgs {
    #[clap(subcommand)]
    command: XtaskCommand,
}

#[derive(Parser, Debug)]
pub enum XtaskCommand {
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

pub trait XtaskActions {
    fn generate_all_tests(&self) -> crate::Result<()>;
}

impl XtaskArgs {
    pub fn run(
        &self,
        current_path: &Path,
        snapshots_path: &Path,
        actions: impl XtaskActions,
    ) -> crate::Result<()> {
        match &self.command {
            XtaskCommand::Report(report_args) => {
                create_report(current_path, snapshots_path, report_args)?;
            }
            XtaskCommand::Clean => {
                clean_image_dir(current_path)?;
            }
            XtaskCommand::DeadSnapshots(ds_args) => {
                process_dead_snapshots(current_path, snapshots_path, actions, ds_args)?;
            }
        }
        Ok(())
    }
}

fn clean_image_dir(path: &Path) -> crate::Result<()> {
    for path in list_image_dir(path)? {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

fn find_dead_snapshots(
    current_path: &Path,
    snapshot_path: &Path,
    actions: impl XtaskActions,
) -> crate::Result<Vec<PathBuf>> {
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
    actions: impl XtaskActions,
    args: &DeadSnapshotArgs,
) -> crate::Result<()> {
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
) -> Result<(), crate::Error> {
    let mut builder =
        crate::DiffBuilder::new(current_path.to_path_buf(), snapshot_path.to_path_buf());
    builder.set_ignore_left_missing(true);

    let diff = builder.build()?;
    let mut report_config = crate::ReportConfig::default();
    report_config.set_left_title("Current test");
    report_config.set_right_title("Snapshot");
    report_config.set_embed_images(report_config.embed_images);
    diff.create_report(&report_config, &report_args.output, true)?;
    Ok(())
}
