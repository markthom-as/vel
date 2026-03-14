//! Doctor service: runs diagnostic checks and returns structured result.

use std::path::Path;
use vel_api_types::{DiagnosticCheck, DiagnosticStatus, DoctorData};

use crate::state::AppState;

pub async fn run_diagnostics(state: &AppState) -> DoctorData {
    let mut checks = Vec::new();

    checks.push(DiagnosticCheck {
        name: "daemon".to_string(),
        status: DiagnosticStatus::Ok,
        message: "ok (in-process; not probing remote)".to_string(),
    });

    let db_status = match state.storage.healthcheck().await {
        Ok(()) => DiagnosticCheck {
            name: "db".to_string(),
            status: DiagnosticStatus::Ok,
            message: "ok".to_string(),
        },
        Err(e) => DiagnosticCheck {
            name: "db".to_string(),
            status: DiagnosticStatus::Fail,
            message: format!("{}", e),
        },
    };
    checks.push(db_status);

    let schema_version = state.storage.schema_version().await.unwrap_or(0);
    checks.push(DiagnosticCheck {
        name: "schema".to_string(),
        status: DiagnosticStatus::Ok,
        message: schema_version.to_string(),
    });

    let artifact_check = check_artifact_dir(&state.config.artifact_root);
    checks.push(artifact_check);

    DoctorData {
        checks,
        schema_version,
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

fn check_artifact_dir(root: &str) -> DiagnosticCheck {
    if root.is_empty() {
        return DiagnosticCheck {
            name: "artifact_dir".to_string(),
            status: DiagnosticStatus::Fail,
            message: "missing (empty path)".to_string(),
        };
    }
    let path = Path::new(root);
    if !path.exists() {
        return match std::fs::create_dir_all(path) {
            Ok(()) => DiagnosticCheck {
                name: "artifact_dir".to_string(),
                status: DiagnosticStatus::Ok,
                message: "ok (created)".to_string(),
            },
            Err(e) => DiagnosticCheck {
                name: "artifact_dir".to_string(),
                status: DiagnosticStatus::Fail,
                message: format!("cannot create: {}", e),
            },
        };
    }
    if !path.is_dir() {
        return DiagnosticCheck {
            name: "artifact_dir".to_string(),
            status: DiagnosticStatus::Fail,
            message: "not a directory".to_string(),
        };
    }
    let test_file = path.join(".vel_write_test");
    match std::fs::write(&test_file, b"") {
        Ok(()) => {
            let _ = std::fs::remove_file(test_file);
            DiagnosticCheck {
                name: "artifact_dir".to_string(),
                status: DiagnosticStatus::Ok,
                message: "ok".to_string(),
            }
        }
        Err(e) => DiagnosticCheck {
            name: "artifact_dir".to_string(),
            status: DiagnosticStatus::Fail,
            message: format!("not writable: {}", e),
        },
    }
}
