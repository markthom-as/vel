//! Doctor service: runs diagnostic checks and returns structured result.

use std::path::Path;
use vel_api_types::{
    BackupFreshnessData, BackupFreshnessStateData, BackupStatusData, BackupStatusStateData,
    BackupTrustData, BackupTrustLevelData,
};
use vel_config::load_repo_contracts_manifest;

use crate::state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoctorCheckStatus {
    Ok,
    Warn,
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
    pub backup: BackupTrustData,
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
    checks.push(check_contracts_manifest());
    let backup = backup_trust(state)
        .await
        .unwrap_or_else(backup_trust_from_error);
    checks.push(backup_check(&backup));

    DoctorReport {
        checks,
        backup,
        schema_version,
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

pub(crate) async fn backup_trust(
    state: &AppState,
) -> Result<BackupTrustData, crate::errors::AppError> {
    crate::services::backup::backup_trust_for_storage(&state.storage).await
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

fn check_contracts_manifest() -> DoctorCheck {
    match load_repo_contracts_manifest() {
        Ok(manifest) => DoctorCheck {
            name: "contracts_manifest".to_string(),
            status: DoctorCheckStatus::Ok,
            message: format!(
                "ok (version {}; live={}, templates={}, examples={}, schemas={})",
                manifest.version,
                manifest.live_configs.len(),
                manifest.templates.len(),
                manifest.contract_examples.len(),
                manifest.schema_count()
            ),
        },
        Err(error) => DoctorCheck {
            name: "contracts_manifest".to_string(),
            status: DoctorCheckStatus::Fail,
            message: format!("cannot load published contracts manifest: {error}"),
        },
    }
}

fn backup_check(backup: &BackupTrustData) -> DoctorCheck {
    let status = match backup.level {
        BackupTrustLevelData::Ok => DoctorCheckStatus::Ok,
        BackupTrustLevelData::Warn => DoctorCheckStatus::Warn,
        BackupTrustLevelData::Fail => DoctorCheckStatus::Fail,
    };
    let message = if let Some(last_backup_at) = backup.status.last_backup_at {
        format!(
            "{}; last backup at {}; warnings={}; next={}",
            backup_status_label(backup.level),
            last_backup_at,
            backup.status.warnings.len(),
            backup
                .guidance
                .first()
                .map(String::as_str)
                .unwrap_or("inspect backup posture")
        )
    } else {
        format!(
            "{}; no successful backup recorded; next={}",
            backup_status_label(backup.level),
            backup
                .guidance
                .first()
                .map(String::as_str)
                .unwrap_or("inspect backup posture")
        )
    };

    DoctorCheck {
        name: "backup".to_string(),
        status,
        message,
    }
}

fn backup_status_label(level: BackupTrustLevelData) -> &'static str {
    match level {
        BackupTrustLevelData::Ok => "backup ready",
        BackupTrustLevelData::Warn => "backup stale",
        BackupTrustLevelData::Fail => "backup missing",
    }
}

fn backup_trust_from_error(error: crate::errors::AppError) -> BackupTrustData {
    BackupTrustData {
        level: BackupTrustLevelData::Fail,
        status: BackupStatusData {
            state: BackupStatusStateData::Degraded,
            last_backup_id: None,
            last_backup_at: None,
            output_root: None,
            artifact_coverage: None,
            config_coverage: None,
            verification_summary: None,
            warnings: vec![format!("backup status unavailable: {error}")],
        },
        freshness: BackupFreshnessData {
            state: BackupFreshnessStateData::Missing,
            age_seconds: None,
            stale_after_seconds: crate::services::backup::BACKUP_STALE_AFTER_SECONDS,
        },
        guidance: crate::services::backup::backup_guidance(BackupTrustLevelData::Fail),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::PathBuf};

    use serde_json::json;
    use time::{Duration, OffsetDateTime};
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_storage::Storage;

    use crate::{policy_config::PolicyConfig, state::AppState};

    fn unique_dir(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "vel_doctor_backup_{}_{}",
            label,
            uuid::Uuid::new_v4().simple()
        ));
        fs::create_dir_all(&path).expect("temp dir should be created");
        path
    }

    async fn test_state() -> AppState {
        let db_path = unique_dir("db").join("vel.sqlite");
        let artifact_root = unique_dir("artifacts");
        let storage = Storage::connect(db_path.to_string_lossy().as_ref())
            .await
            .expect("storage");
        storage.migrate().await.expect("migrations");
        let (broadcast_tx, _) = broadcast::channel(8);

        AppState::new(
            storage,
            AppConfig {
                db_path: db_path.to_string_lossy().to_string(),
                artifact_root: artifact_root.to_string_lossy().to_string(),
                ..Default::default()
            },
            PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        )
    }

    fn manifest_json(output_root: &str, verified: bool) -> serde_json::Value {
        json!({
            "backup_id": "bkp_test",
            "created_at": "2026-03-19T09:00:00Z",
            "output_root": output_root,
            "database_snapshot_path": format!("{output_root}/data/vel.sqlite"),
            "artifact_coverage": {
                "included": ["artifacts/captures"],
                "omitted": ["artifacts/cache"],
                "notes": []
            },
            "config_coverage": {
                "included": ["config/public-settings.json"],
                "omitted": ["integration_google_calendar_secrets"],
                "notes": []
            },
            "explicit_omissions": ["integration_google_calendar_secrets"],
            "secret_omission_flags": {
                "settings_secrets_omitted": true,
                "integration_tokens_omitted": true,
                "local_key_material_omitted": true,
                "notes": []
            },
            "verification_summary": {
                "verified": verified,
                "checksum_algorithm": "sha256",
                "checksum": "abc123",
                "checked_paths": [format!("{output_root}/manifest.json")],
                "notes": []
            }
        })
    }

    #[tokio::test]
    async fn doctor_reports_warn_for_stale_verified_backup() {
        let state = test_state().await;
        let started_at = OffsetDateTime::now_utc() - Duration::hours(72);
        let output_root = unique_dir("stale-backup");

        state
            .storage
            .persist_backup_run(
                "bkp_test",
                output_root.to_string_lossy().as_ref(),
                "verified",
                &manifest_json(output_root.to_string_lossy().as_ref(), true),
                started_at,
                Some(started_at),
                Some(started_at),
                None,
            )
            .await
            .expect("backup run");

        let report = run_diagnostics(&state).await;
        let backup_check = report
            .checks
            .iter()
            .find(|check| check.name == "backup")
            .expect("backup check should be present");

        assert_eq!(backup_check.status, DoctorCheckStatus::Warn);
        assert_eq!(
            report.backup.level,
            vel_api_types::BackupTrustLevelData::Warn
        );
        assert_eq!(
            report.backup.status.state,
            vel_api_types::BackupStatusStateData::Stale
        );
    }

    #[tokio::test]
    async fn doctor_reports_fail_when_backup_history_is_missing() {
        let state = test_state().await;

        let report = run_diagnostics(&state).await;
        let backup_check = report
            .checks
            .iter()
            .find(|check| check.name == "backup")
            .expect("backup check should be present");

        assert_eq!(backup_check.status, DoctorCheckStatus::Fail);
        assert_eq!(
            report.backup.level,
            vel_api_types::BackupTrustLevelData::Fail
        );
        assert_eq!(
            report.backup.status.state,
            vel_api_types::BackupStatusStateData::Missing
        );
    }
}
