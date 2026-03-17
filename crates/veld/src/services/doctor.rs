//! Doctor service: runs diagnostic checks and returns structured result.

use std::path::Path;

use crate::state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoctorCheckStatus {
    Ok,
    Fail,
}

#[derive(Debug, Clone)]
pub struct DoctorCheck {
    pub name: String,
    pub status: DoctorCheckStatus,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub checks: Vec<DoctorCheck>,
    pub schema_version: u32,
    pub version: String,
}

pub async fn run_diagnostics(state: &AppState) -> DoctorReport {
    let mut checks = Vec::new();

    checks.push(DoctorCheck {
        name: "daemon".to_string(),
        status: DoctorCheckStatus::Ok,
        message: "ok (in-process; not probing remote)".to_string(),
    });

    let db_status = match state.storage.healthcheck().await {
        Ok(()) => DoctorCheck {
            name: "db".to_string(),
            status: DoctorCheckStatus::Ok,
            message: "ok".to_string(),
        },
        Err(e) => DoctorCheck {
            name: "db".to_string(),
            status: DoctorCheckStatus::Fail,
            message: format!("{}", e),
        },
    };
    checks.push(db_status);

    let schema_version = state.storage.schema_version().await.unwrap_or(0);
    checks.push(DoctorCheck {
        name: "schema".to_string(),
        status: DoctorCheckStatus::Ok,
        message: schema_version.to_string(),
    });

    let artifact_check = check_artifact_dir(&state.config.artifact_root);
    checks.push(artifact_check);

    DoctorReport {
        checks,
        schema_version,
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

fn check_artifact_dir(root: &str) -> DoctorCheck {
    if root.is_empty() {
        return DoctorCheck {
            name: "artifact_dir".to_string(),
            status: DoctorCheckStatus::Fail,
            message: "missing (empty path)".to_string(),
        };
    }
    let path = Path::new(root);
    if !path.exists() {
        return match std::fs::create_dir_all(path) {
            Ok(()) => DoctorCheck {
                name: "artifact_dir".to_string(),
                status: DoctorCheckStatus::Ok,
                message: "ok (created)".to_string(),
            },
            Err(e) => DoctorCheck {
                name: "artifact_dir".to_string(),
                status: DoctorCheckStatus::Fail,
                message: format!("cannot create: {}", e),
            },
        };
    }
    if !path.is_dir() {
        return DoctorCheck {
            name: "artifact_dir".to_string(),
            status: DoctorCheckStatus::Fail,
            message: "not a directory".to_string(),
        };
    }
    let test_file = path.join(".vel_write_test");
    match std::fs::write(&test_file, b"") {
        Ok(()) => {
            let _ = std::fs::remove_file(test_file);
            DoctorCheck {
                name: "artifact_dir".to_string(),
                status: DoctorCheckStatus::Ok,
                message: "ok".to_string(),
            }
        }
        Err(e) => DoctorCheck {
            name: "artifact_dir".to_string(),
            status: DoctorCheckStatus::Fail,
            message: format!("not writable: {}", e),
        },
    }
}
