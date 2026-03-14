//! Diagnostic endpoint for `vel doctor`. Checks DB, schema version, artifact directory, daemon.

use axum::extract::State;
use axum::Json;
use std::path::Path;
use uuid::Uuid;
use vel_api_types::{ApiResponse, DoctorData};

use crate::{errors::AppError, state::AppState};

pub async fn doctor(State(state): State<AppState>) -> Result<Json<ApiResponse<DoctorData>>, AppError> {
    let db_status = match state.storage.healthcheck().await {
        Ok(()) => "ok".to_string(),
        Err(e) => format!("error: {}", e),
    };

    let schema_version = state.storage.schema_version().await.unwrap_or(0);

    let artifact_dir_status = check_artifact_dir(&state.config.artifact_root);

    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        DoctorData {
            daemon: "ok".to_string(),
            db: db_status,
            schema_version,
            artifact_dir: artifact_dir_status,
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        request_id,
    )))
}

fn check_artifact_dir(root: &str) -> String {
    if root.is_empty() {
        return "missing (empty path)".to_string();
    }
    let path = Path::new(root);
    if !path.exists() {
        return match std::fs::create_dir_all(path) {
            Ok(()) => "ok (created)".to_string(),
            Err(e) => format!("error: cannot create: {}", e),
        };
    }
    if !path.is_dir() {
        return "error: not a directory".to_string();
    }
    let test_file = path.join(".vel_write_test");
    match std::fs::write(&test_file, b"") {
        Ok(()) => {
            let _ = std::fs::remove_file(test_file);
            "ok".to_string()
        }
        Err(e) => format!("error: not writable: {}", e),
    }
}
