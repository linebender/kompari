// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::Parser;
use kompari::DirDiffConfig;
use kompari_tasks::{Actions, Args, Task};
use std::path::Path;
use std::process::Command;

struct ActionsImpl();

impl Actions for ActionsImpl {
    fn generate_all_tests(&self) -> kompari::Result<()> {
        let cargo = std::env::var("CARGO").unwrap();
        Command::new(&cargo)
            .arg("test")
            .env("DEMOLIB_TEST", "generate-all")
            .status()?;
        Ok(())
    }
}

fn main() -> kompari::Result<()> {
    let tests_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("tests");

    let snapshots_path = tests_path.join("snapshots");
    let current_path = tests_path.join("current");

    let args = Args::parse();
    let diff_config = DirDiffConfig::new(snapshots_path, current_path);
    let actions = ActionsImpl();
    let mut task = Task::new(diff_config, Box::new(actions));
    task.run(&args)?;
    Ok(())
}
