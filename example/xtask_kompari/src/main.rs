// Copyright 2024 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::Parser;
use kompari::xtask_cli::{XtaskActions, XtaskArgs};
use std::path::Path;
use std::process::Command;

struct XtaskActionsImpl();

impl XtaskActions for XtaskActionsImpl {
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

    let current_path = tests_path.join("current");
    let snapshots_path = tests_path.join("snapshots");

    XtaskArgs::parse().run(&current_path, &snapshots_path, XtaskActionsImpl())
}
