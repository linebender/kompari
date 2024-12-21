// Copyright 2025 the Kompari Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{DiffBuilder, ReportConfig};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::post;
use axum::{routing::get, Json, Router};
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;

struct AppState {
    report_config: ReportConfig,
    diff_builder: DiffBuilder,
}

pub fn start_review_server(
    diff_builder: DiffBuilder,
    report_config: ReportConfig,
    port: u16,
) -> crate::Result<()> {
    let shared_state = Arc::new(AppState {
        report_config,
        diff_builder,
    });
    println!("Running at http://localhost:{port}");
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let app = Router::new()
                .route("/", get(index))
                .route("/update", post(update))
                .with_state(shared_state);
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
                .await
                .unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    Ok(())
}

fn result_to_response(result: crate::Result<String>) -> (StatusCode, Html<String>) {
    match result {
        Ok(s) => (StatusCode::OK, Html::from(s)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Html::from(e.to_string())),
    }
}

async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    result_to_response((|| {
        let diff = state.diff_builder.build()?;
        diff.render_report(&state.report_config)
    })())
}

#[derive(Deserialize, Debug)]
struct UpdateParams {
    accepted_names: Vec<String>,
}

async fn update(
    State(state): State<Arc<AppState>>,
    Json(params): Json<UpdateParams>,
) -> StatusCode {
    let paths: Vec<_> = params
        .accepted_names
        .into_iter()
        .map(PathBuf::from)
        .collect();
    if paths.iter().any(|p| !p.is_relative()) {
        return StatusCode::BAD_REQUEST;
    }
    for path in paths {
        let left = state.diff_builder.left_path.join(&path);
        let right = state.diff_builder.right_path.join(&path);
        println!("Updating {} -> {}", left.display(), right.display());
        if let Err(e) = std::fs::copy(&left, &right) {
            eprintln!("Failed to rename {}: {}", left.display(), e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }
    StatusCode::OK
}
