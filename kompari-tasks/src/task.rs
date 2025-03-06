// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::args::{Args, Command, DeadSnapshotArgs};
use crate::output::print_size_optimization_results;
use kompari::{check_size_optimizations, list_image_dir, list_image_dir_names, DirDiffConfig};
use kompari_html::{render_html_report, start_review_server, ReportConfig};
use std::collections::BTreeSet;
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub trait Actions {
    fn generate_all_tests(&self) -> kompari::Result<()>;
}

pub struct Task {
    diff_config: DirDiffConfig,
    report_config: ReportConfig,
    report_output_path: PathBuf,
    actions: Box<dyn Actions>,
}

impl Task {
    pub fn new(diff_config: DirDiffConfig, actions: Box<dyn Actions>) -> Self {
        let mut report_config = ReportConfig::default();
        report_config.set_left_title("Reference");
        report_config.set_right_title("Current");
        Task {
            diff_config,
            report_config,
            report_output_path: "report.html".into(),
            actions,
        }
    }

    pub fn report_config(&mut self) -> &mut ReportConfig {
        &mut self.report_config
    }

    pub fn set_report_output_path(&mut self, path: PathBuf) {
        self.report_output_path = path;
    }

    pub fn run(&mut self, args: &Args) -> kompari::Result<()> {
        match &args.command {
            Command::Report(report_args) => {
                if report_args.embed_images {
                    self.report_config.set_embed_images(true);
                }
                let output: &Path = report_args
                    .output
                    .as_deref()
                    .unwrap_or(self.report_output_path.as_path());
                let diff = self.diff_config.create_diff()?;
                let report = render_html_report(&self.report_config, diff.results())?;
                std::fs::write(output, report)?;
                println!("Report written into '{}'", output.display());
            }
            Command::Review(args) => {
                start_review_server(&self.diff_config, &self.report_config, args.port)?
            }
            Command::Clean => {
                clean_image_dir(self.diff_config.right_path())?;
            }
            Command::DeadSnapshots(ds_args) => {
                process_dead_snapshots(
                    self.diff_config.left_path(),
                    self.diff_config.right_path(),
                    self.actions.deref(),
                    ds_args,
                )?;
            }
            Command::SizeCheck(sc_args) => {
                let results =
                    check_size_optimizations(self.diff_config.left_path(), sc_args.optimize)?;
                print_size_optimization_results(&results)?;
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
    snapshot_path: &Path,
    current_path: &Path,
    actions: &dyn Actions,
) -> kompari::Result<Vec<PathBuf>> {
    clean_image_dir(current_path)?;
    actions.generate_all_tests()?;
    let snapshot_images: BTreeSet<_> = list_image_dir_names(snapshot_path)?.collect();
    let current_images: BTreeSet<_> = list_image_dir_names(current_path)?.collect();
    Ok(snapshot_images
        .difference(&current_images)
        .map(|name| current_path.join(name))
        .collect())
}

fn process_dead_snapshots(
    snapshot_path: &Path,
    current_path: &Path,
    actions: &dyn Actions,
    args: &DeadSnapshotArgs,
) -> kompari::Result<()> {
    let dead_snapshots = find_dead_snapshots(snapshot_path, current_path, actions)?;
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
