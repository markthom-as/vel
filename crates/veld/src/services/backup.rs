use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::UNIX_EPOCH,
};

use arrow_array::{ArrayRef, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::ArrowWriter;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use vel_api_types::{
    BackupCoverageData, BackupExportDerivativeData, BackupExportDomainOmissionData,
    BackupExportFileData, BackupExportManifestData, BackupExportRequestData,
    BackupExportResultData, BackupExportStatusData, BackupFreshnessData, BackupFreshnessStateData,
    BackupManifestData, BackupSecretOmissionFlagsData, BackupStatusData, BackupStatusStateData,
    BackupTrustData, BackupTrustLevelData, BackupVerificationData,
};
use vel_storage::{BackupJobRecord, BackupRunRecord, Storage};

use crate::{errors::AppError, state::AppState};

pub(crate) const DEFAULT_BACKUP_ROOT: &str = "var/backups";
const MANIFEST_FILE_NAME: &str = "manifest.json";
const EXPORT_RUNS_DIR_NAME: &str = "runs";
const DATA_DIR_NAME: &str = "data";
const ARTIFACTS_DIR_NAME: &str = "artifacts";
const CONFIG_DIR_NAME: &str = "config";
const SNAPSHOT_FILE_NAME: &str = "vel.sqlite";
const PUBLIC_SETTINGS_FILE_NAME: &str = "public-settings.json";
const RUNTIME_CONFIG_FILE_NAME: &str = "runtime-config.json";
const OMITTED_ARTIFACT_SEGMENTS: &[&str] = &["cache", "tmp"];
const EXPORT_MANIFEST_SCHEMA_VERSION: &str = "backup_export_manifest.v1";
const EXPORT_SOURCE_SNAPSHOT_SCHEMA_VERSION: &str = "local_source_snapshot.v1";
const EXPORT_TASKS_SCHEMA_VERSION: &str = "backup_export_tasks.v1";
const EXPORT_CALENDAR_EVENTS_SCHEMA_VERSION: &str = "backup_export_calendar_events.v1";
const EXPORT_MESSAGING_THREADS_SCHEMA_VERSION: &str = "backup_export_messaging_threads.v1";
const EXPORT_TRANSCRIPT_MESSAGES_SCHEMA_VERSION: &str = "backup_export_transcript_messages.v1";
const EXPORT_HEALTH_SAMPLES_SCHEMA_VERSION: &str = "backup_export_health_samples.v1";
const EXPORT_GIT_EVENTS_SCHEMA_VERSION: &str = "backup_export_git_events.v1";
const EXPORT_REMINDER_ITEMS_SCHEMA_VERSION: &str = "backup_export_reminder_items.v1";
const EXPORT_NOTES_SCHEMA_VERSION: &str = "backup_export_notes.v1";
const EXPORT_ACTIVITY_EVENTS_SCHEMA_VERSION: &str = "backup_export_activity_events.v1";
pub(crate) const BACKUP_STALE_AFTER_SECONDS: i64 = 48 * 60 * 60;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBackupInput {
    #[serde(default)]
    pub output_root: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackupRootInput {
    pub backup_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCreateResultData {
    pub manifest: BackupManifestData,
    pub status: BackupStatusData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInspectResultData {
    pub manifest: BackupManifestData,
    pub status: BackupStatusData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupVerifyResultData {
    pub manifest: BackupManifestData,
    pub status: BackupStatusData,
}

pub async fn create_backup(
    state: &AppState,
    input: CreateBackupInput,
) -> Result<BackupCreateResultData, AppError> {
    let now = OffsetDateTime::now_utc();
    let output_base = prepare_output_base(input.output_root.as_deref())?;
    let backup_id = format!(
        "bkp_{}_{}",
        now.format(
            &time::format_description::parse("[year][month][day]T[hour][minute][second]Z")
                .map_err(|error| AppError::internal(error.to_string()))?
        )
        .map_err(|error| AppError::internal(error.to_string()))?,
        uuid::Uuid::new_v4().simple()
    );
    let backup_root = output_base.join(&backup_id);
    fs::create_dir_all(backup_root.join(DATA_DIR_NAME))
        .map_err(|error| AppError::internal(error.to_string()))?;
    fs::create_dir_all(backup_root.join(ARTIFACTS_DIR_NAME))
        .map_err(|error| AppError::internal(error.to_string()))?;
    fs::create_dir_all(backup_root.join(CONFIG_DIR_NAME))
        .map_err(|error| AppError::internal(error.to_string()))?;

    let snapshot_path = backup_root.join(DATA_DIR_NAME).join(SNAPSHOT_FILE_NAME);
    state
        .storage
        .create_sqlite_snapshot(snapshot_path.to_string_lossy().as_ref())
        .await
        .map_err(AppError::from)?;

    let artifact_coverage = copy_artifacts(Path::new(&state.config.artifact_root), &backup_root)?;
    let (public_settings, omitted_settings) = split_settings(state).await?;
    write_json_file(
        &backup_root
            .join(CONFIG_DIR_NAME)
            .join(PUBLIC_SETTINGS_FILE_NAME),
        &public_settings,
    )?;
    write_json_file(
        &backup_root
            .join(CONFIG_DIR_NAME)
            .join(RUNTIME_CONFIG_FILE_NAME),
        &state.config,
    )?;

    let secret_flags = build_secret_flags(&omitted_settings);
    let mut explicit_omissions = vec![
        "secret settings records".to_string(),
        "integration API tokens".to_string(),
        "local private key material".to_string(),
        "volatile cache and temp files".to_string(),
    ];
    explicit_omissions.extend(
        omitted_settings
            .iter()
            .map(|key| format!("omitted setting key: {key}")),
    );

    let config_coverage = BackupCoverageData {
        included: vec![
            format!("{CONFIG_DIR_NAME}/{PUBLIC_SETTINGS_FILE_NAME}"),
            format!("{CONFIG_DIR_NAME}/{RUNTIME_CONFIG_FILE_NAME}"),
        ],
        omitted: omitted_settings.clone(),
        notes: vec![
            "Secret-bearing settings are intentionally excluded from the backup pack.".to_string(),
            "Runtime config is included so the operator can inspect the effective local posture."
                .to_string(),
        ],
    };

    let mut manifest = BackupManifestData {
        backup_id: backup_id.clone(),
        created_at: now,
        output_root: backup_root.to_string_lossy().to_string(),
        database_snapshot_path: snapshot_path.to_string_lossy().to_string(),
        artifact_coverage,
        config_coverage,
        explicit_omissions,
        secret_omission_flags: secret_flags,
        verification_summary: BackupVerificationData {
            verified: false,
            checksum_algorithm: "sha256".to_string(),
            checksum: String::new(),
            checked_paths: collect_checked_paths(&backup_root),
            notes: vec![
                "Verification is computed from the SQLite snapshot bytes and manifest metadata."
                    .to_string(),
            ],
        },
    };
    manifest.verification_summary.checksum = compute_manifest_checksum(&manifest, &snapshot_path)?;
    manifest.verification_summary.verified = true;

    let manifest_path = backup_root.join(MANIFEST_FILE_NAME);
    write_json_file(&manifest_path, &manifest)?;

    let manifest_json =
        serde_json::to_value(&manifest).map_err(|error| AppError::internal(error.to_string()))?;
    state
        .storage
        .persist_backup_run(
            &backup_id,
            manifest.output_root.as_str(),
            "verified",
            &manifest_json,
            now,
            Some(now),
            Some(now),
            None,
        )
        .await
        .map_err(AppError::from)?;

    Ok(BackupCreateResultData {
        manifest: manifest.clone(),
        status: status_from_manifest(&manifest, Some(now)),
    })
}

pub async fn inspect_backup(
    state: &AppState,
    input: BackupRootInput,
) -> Result<BackupInspectResultData, AppError> {
    let backup_root = canonicalize_existing_dir(&input.backup_root)?;
    let manifest = load_manifest(&backup_root)?;
    let status = match state
        .storage
        .get_backup_run(&manifest.backup_id)
        .await
        .map_err(AppError::from)?
    {
        Some(record) => status_from_record(&record)?,
        None => status_from_manifest(&manifest, Some(manifest.created_at)),
    };

    Ok(BackupInspectResultData { manifest, status })
}

pub async fn verify_backup(
    state: &AppState,
    input: BackupRootInput,
) -> Result<BackupVerifyResultData, AppError> {
    let backup_root = canonicalize_existing_dir(&input.backup_root)?;
    let mut manifest = load_manifest(&backup_root)?;
    validate_manifest_paths(&backup_root, &manifest)?;

    let snapshot_path = PathBuf::from(&manifest.database_snapshot_path);
    let expected_checksum = compute_manifest_checksum(&manifest, &snapshot_path)?;
    manifest.verification_summary.verified =
        expected_checksum == manifest.verification_summary.checksum;
    if !manifest.verification_summary.verified {
        manifest
            .verification_summary
            .notes
            .push("Checksum mismatch detected during verification.".to_string());
    }

    if manifest.verification_summary.verified {
        let manifest_json = serde_json::to_value(&manifest)
            .map_err(|error| AppError::internal(error.to_string()))?;
        let verified_at = OffsetDateTime::now_utc();
        state
            .storage
            .persist_backup_run(
                &manifest.backup_id,
                manifest.output_root.as_str(),
                "verified",
                &manifest_json,
                manifest.created_at,
                Some(manifest.created_at),
                Some(verified_at),
                None,
            )
            .await
            .map_err(AppError::from)?;
    }

    let status = if manifest.verification_summary.verified {
        status_from_manifest(&manifest, Some(manifest.created_at))
    } else {
        BackupStatusData {
            state: BackupStatusStateData::Degraded,
            last_backup_id: Some(manifest.backup_id.clone()),
            last_backup_at: Some(manifest.created_at),
            output_root: Some(manifest.output_root.clone()),
            artifact_coverage: Some(manifest.artifact_coverage.clone()),
            config_coverage: Some(manifest.config_coverage.clone()),
            verification_summary: Some(manifest.verification_summary.clone()),
            warnings: vec!["backup checksum mismatch".to_string()],
        }
    };

    Ok(BackupVerifyResultData { manifest, status })
}

pub async fn export_backup(
    state: &AppState,
    input: BackupExportRequestData,
) -> Result<BackupExportResultData, AppError> {
    let now = OffsetDateTime::now_utc();
    let include_parquet =
        input.include_parquet || state.config.backup_export.include_parquet_derivatives;
    let target_root = input
        .target_root
        .as_deref()
        .or(state.config.backup_export.target_root.as_deref())
        .ok_or_else(|| {
            AppError::bad_request("backup export target_root is required by request or config")
        })?;
    let target_root = canonicalize_existing_export_target(target_root)?;
    let export_id = format!(
        "bex_{}_{}",
        now.format(
            &time::format_description::parse("[year][month][day]T[hour][minute][second]Z")
                .map_err(|error| AppError::internal(error.to_string()))?
        )
        .map_err(|error| AppError::internal(error.to_string()))?,
        uuid::Uuid::new_v4().simple()
    );
    let export_root = target_root.join(EXPORT_RUNS_DIR_NAME).join(&export_id);
    fs::create_dir_all(&export_root).map_err(|error| {
        AppError::internal(format!(
            "create backup export run root {}: {}",
            export_root.display(),
            error
        ))
    })?;
    let config_domains = if input.domains.is_empty() {
        state.config.backup_export.domains.as_slice()
    } else {
        input.domains.as_slice()
    };
    let requested_domains = requested_export_domains(config_domains);
    let sources = export_sources(state);
    let mut included_domains = Vec::new();
    let mut omitted_domains = Vec::new();
    let mut files = Vec::new();

    for domain in requested_domains {
        let Some(source) = sources.iter().find(|source| source.domain == domain) else {
            omitted_domains.push(BackupExportDomainOmissionData {
                domain,
                reason: "unsupported export domain".to_string(),
            });
            continue;
        };

        let Some(source_path) = source.configured_path.as_deref() else {
            omitted_domains.push(BackupExportDomainOmissionData {
                domain,
                reason: "source path is not configured".to_string(),
            });
            continue;
        };

        let source_path = PathBuf::from(source_path);
        if !source_path.exists() {
            omitted_domains.push(BackupExportDomainOmissionData {
                domain,
                reason: format!("source path is missing: {}", source_path.display()),
            });
            continue;
        }

        let canonical_source = fs::canonicalize(&source_path)
            .map_err(|error| AppError::bad_request(error.to_string()))?;
        let write =
            match write_export_domain_snapshot(source.domain, &canonical_source, &export_root, now)
            {
                Ok(write) => write,
                Err(ExportDomainSnapshotError::Omitted(reason)) => {
                    omitted_domains.push(BackupExportDomainOmissionData { domain, reason });
                    continue;
                }
                Err(ExportDomainSnapshotError::Fatal(error)) => return Err(error),
            };
        let checksum = checksum_file(&write.output_path)?;
        included_domains.push(source.domain.to_string());
        files.push(BackupExportFileData {
            domain: source.domain.to_string(),
            path: write.output_path.to_string_lossy().to_string(),
            schema_version: write.schema_version.to_string(),
            record_count: write.record_count,
            checksum_algorithm: "sha256".to_string(),
            checksum,
            source_path: Some(canonical_source.to_string_lossy().to_string()),
        });
    }

    included_domains.sort();
    omitted_domains.sort_by(|left, right| left.domain.cmp(&right.domain));
    files.sort_by(|left, right| left.domain.cmp(&right.domain));
    let derivatives = if include_parquet {
        write_parquet_derivatives(&export_root, &files)?
    } else {
        Vec::new()
    };

    let manifest_path = export_root.join(MANIFEST_FILE_NAME);
    let latest_manifest_path = target_root.join(MANIFEST_FILE_NAME);
    let checked_paths = std::iter::once(manifest_path.to_string_lossy().to_string())
        .chain(files.iter().map(|file| file.path.clone()))
        .chain(derivatives.iter().map(|derivative| derivative.path.clone()))
        .collect();
    let derivation_note = if include_parquet {
        "Manual export only; scheduling and retention pruning are not enabled in this slice. Parquet derivatives are generated from normalized JSON/NDJSON files."
    } else {
        "Manual export only; scheduling and retention pruning are not enabled in this slice. Parquet derivation was not requested for this export."
    };
    let mut manifest = BackupExportManifestData {
        export_id,
        created_at: now,
        target_root: target_root.to_string_lossy().to_string(),
        export_root: export_root.to_string_lossy().to_string(),
        included_domains,
        omitted_domains,
        files,
        derivatives,
        verification_summary: BackupVerificationData {
            verified: false,
            checksum_algorithm: "sha256".to_string(),
            checksum: String::new(),
            checked_paths,
            notes: vec![
                format!("Manifest schema: {EXPORT_MANIFEST_SCHEMA_VERSION}."),
                derivation_note.to_string(),
            ],
        },
    };
    manifest.verification_summary.checksum = compute_export_manifest_checksum(&manifest)?;
    manifest.verification_summary.verified = true;
    write_json_file(&manifest_path, &manifest)?;
    write_json_file(&latest_manifest_path, &manifest)?;
    let manifest_json =
        serde_json::to_value(&manifest).map_err(|error| AppError::internal(error.to_string()))?;
    state
        .storage
        .persist_backup_export_run(
            &manifest.export_id,
            manifest.target_root.as_str(),
            "verified",
            &manifest_json,
            now,
            Some(now),
            Some(now),
            None,
        )
        .await
        .map_err(AppError::from)?;

    let mut last_error = None;
    if let Some(retention_count) = state.config.backup_export.retention_count {
        match prune_backup_export_retention(
            state,
            &target_root,
            manifest.export_id.as_str(),
            retention_count.max(1),
        )
        .await
        {
            Ok(report) => {
                manifest.verification_summary.notes.push(format!(
                    "Retention pruning kept latest {} export run(s) and pruned {} older run directorie(s).",
                    retention_count.max(1),
                    report.pruned_runs
                ));
            }
            Err(error) => {
                let message = format!("backup export retention pruning failed: {error}");
                manifest.verification_summary.notes.push(message.clone());
                last_error = Some(message);
            }
        }
        manifest.verification_summary.checksum = compute_export_manifest_checksum(&manifest)?;
        write_json_file(&manifest_path, &manifest)?;
        write_json_file(&latest_manifest_path, &manifest)?;
        let manifest_json = serde_json::to_value(&manifest)
            .map_err(|error| AppError::internal(error.to_string()))?;
        state
            .storage
            .persist_backup_export_run(
                &manifest.export_id,
                manifest.target_root.as_str(),
                "verified",
                &manifest_json,
                now,
                Some(now),
                Some(now),
                last_error.as_deref(),
            )
            .await
            .map_err(AppError::from)?;
    }

    Ok(BackupExportResultData { manifest })
}

pub async fn backup_status(state: &AppState) -> Result<BackupStatusData, AppError> {
    backup_status_for_storage(&state.storage).await
}

pub async fn backup_export_status(state: &AppState) -> Result<BackupExportStatusData, AppError> {
    let latest_scheduled_job = state
        .storage
        .get_latest_finished_scheduled_backup_export_job()
        .await
        .map_err(AppError::from)?;
    let Some(record) = state
        .storage
        .get_last_successful_backup_export_run()
        .await
        .map_err(AppError::from)?
    else {
        let scheduled_failure = latest_scheduled_job
            .as_ref()
            .filter(|job| job.status == "failed");
        let mut warnings = vec!["no successful backup export has been recorded yet".to_string()];
        if let Some(job) = scheduled_failure {
            warnings.push(scheduled_export_failure_warning(job));
        }
        return Ok(BackupExportStatusData {
            state: if scheduled_failure.is_some() {
                BackupStatusStateData::Degraded
            } else {
                BackupStatusStateData::Missing
            },
            last_export_id: None,
            last_export_at: None,
            target_root: state.config.backup_export.target_root.clone(),
            included_domains: state.config.backup_export.domains.clone(),
            omitted_domains: Vec::new(),
            verification_summary: None,
            warnings,
        });
    };

    let mut status = export_status_from_record(&record)?;
    if let Some(job) = latest_scheduled_job
        .as_ref()
        .filter(|job| job.status == "failed")
    {
        let last_export_at = status
            .last_export_at
            .unwrap_or(record.verified_at.unwrap_or(record.started_at));
        let job_finished_at = job
            .finished_at
            .or(job.completed_at)
            .or(job.started_at)
            .unwrap_or(job.created_at);
        if job_finished_at >= last_export_at {
            status.state = BackupStatusStateData::Degraded;
            status.warnings.push(scheduled_export_failure_warning(job));
        }
    }
    Ok(status)
}

pub async fn backup_status_for_storage(storage: &Storage) -> Result<BackupStatusData, AppError> {
    let Some(record) = storage
        .get_last_successful_backup_run()
        .await
        .map_err(AppError::from)?
    else {
        return Ok(BackupStatusData {
            state: BackupStatusStateData::Missing,
            last_backup_id: None,
            last_backup_at: None,
            output_root: None,
            artifact_coverage: None,
            config_coverage: None,
            verification_summary: None,
            warnings: vec!["no successful backup has been recorded yet".to_string()],
        });
    };

    status_from_record(&record)
}

pub async fn backup_trust_for_storage(storage: &Storage) -> Result<BackupTrustData, AppError> {
    let status = backup_status_for_storage(storage).await?;
    Ok(classify_backup_status(status, OffsetDateTime::now_utc()))
}

pub fn classify_backup_status(
    mut status: BackupStatusData,
    now: OffsetDateTime,
) -> BackupTrustData {
    let age_seconds = status
        .last_backup_at
        .map(|value| (now - value).whole_seconds().max(0));
    let freshness_state = match age_seconds {
        None => BackupFreshnessStateData::Missing,
        Some(age) if age > BACKUP_STALE_AFTER_SECONDS => BackupFreshnessStateData::Stale,
        Some(_) => BackupFreshnessStateData::Current,
    };

    if matches!(freshness_state, BackupFreshnessStateData::Stale)
        && matches!(status.state, BackupStatusStateData::Ready)
    {
        status.state = BackupStatusStateData::Stale;
        if !status
            .warnings
            .iter()
            .any(|warning| warning.contains("stale"))
        {
            status
                .warnings
                .push("last successful backup is stale".to_string());
        }
    }

    let level = match status.state {
        BackupStatusStateData::Missing | BackupStatusStateData::Degraded => {
            BackupTrustLevelData::Fail
        }
        BackupStatusStateData::Stale => BackupTrustLevelData::Warn,
        BackupStatusStateData::Ready => {
            if matches!(freshness_state, BackupFreshnessStateData::Current)
                && status
                    .verification_summary
                    .as_ref()
                    .map(|summary| summary.verified)
                    .unwrap_or(false)
            {
                BackupTrustLevelData::Ok
            } else {
                BackupTrustLevelData::Warn
            }
        }
    };

    BackupTrustData {
        level,
        status,
        freshness: BackupFreshnessData {
            state: freshness_state,
            age_seconds,
            stale_after_seconds: BACKUP_STALE_AFTER_SECONDS,
        },
        guidance: backup_guidance(level),
    }
}

pub(crate) fn backup_guidance(level: BackupTrustLevelData) -> Vec<String> {
    match level {
        BackupTrustLevelData::Ok => vec![
            "Backup trust is healthy. Keep running verify after important local changes."
                .to_string(),
        ],
        BackupTrustLevelData::Warn => vec![
            "Backup trust is degraded. Create or verify a fresh backup before risky maintenance."
                .to_string(),
            "The explicit `vel backup` create/inspect/verify workflow lands in the next Phase 09 slice."
                .to_string(),
        ],
        BackupTrustLevelData::Fail => vec![
            "No trustworthy backup is currently available. Create a fresh backup before destructive actions."
                .to_string(),
            "Use the authenticated `/v1/backup/*` routes until the CLI backup workflow lands in the next slice."
                .to_string(),
        ],
    }
}

fn prepare_output_base(output_root: Option<&str>) -> Result<PathBuf, AppError> {
    let candidate = output_root
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_BACKUP_ROOT);
    let path = PathBuf::from(candidate);
    fs::create_dir_all(&path).map_err(|error| AppError::bad_request(error.to_string()))?;
    fs::canonicalize(path).map_err(|error| AppError::bad_request(error.to_string()))
}

fn canonicalize_existing_dir(value: &str) -> Result<PathBuf, AppError> {
    let path = Path::new(value);
    let canonical = fs::canonicalize(path)
        .map_err(|error| AppError::bad_request(format!("backup root {value}: {error}")))?;
    if !canonical.is_dir() {
        return Err(AppError::bad_request("backup root must be a directory"));
    }
    Ok(canonical)
}

fn canonicalize_existing_export_target(value: &str) -> Result<PathBuf, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::bad_request(
            "backup export target_root is required",
        ));
    }

    let path = Path::new(trimmed);
    let canonical = fs::canonicalize(path)
        .map_err(|error| AppError::bad_request(format!("backup export target {value}: {error}")))?;
    if !canonical.is_dir() {
        return Err(AppError::bad_request(
            "backup export target_root must be an existing directory",
        ));
    }

    let probe_path = canonical.join(format!(
        ".vel-export-write-test-{}",
        uuid::Uuid::new_v4().simple()
    ));
    fs::write(&probe_path, b"").map_err(|error| {
        AppError::bad_request(format!("backup export target is not writable: {error}"))
    })?;
    fs::remove_file(&probe_path).map_err(|error| {
        AppError::bad_request(format!(
            "backup export target probe cleanup failed: {error}"
        ))
    })?;

    Ok(canonical)
}

struct ExportSource {
    domain: &'static str,
    configured_path: Option<String>,
}

fn export_sources(state: &AppState) -> Vec<ExportSource> {
    vec![
        ExportSource {
            domain: "calendar",
            configured_path: state.config.calendar_ics_path.clone(),
        },
        ExportSource {
            domain: "tasks",
            configured_path: state.config.todoist_snapshot_path.clone(),
        },
        ExportSource {
            domain: "activity",
            configured_path: state.config.activity_snapshot_path.clone(),
        },
        ExportSource {
            domain: "health",
            configured_path: state.config.health_snapshot_path.clone(),
        },
        ExportSource {
            domain: "git",
            configured_path: state.config.git_snapshot_path.clone(),
        },
        ExportSource {
            domain: "messaging",
            configured_path: state.config.messaging_snapshot_path.clone(),
        },
        ExportSource {
            domain: "reminders",
            configured_path: state.config.reminders_snapshot_path.clone(),
        },
        ExportSource {
            domain: "notes",
            configured_path: state.config.notes_path.clone(),
        },
        ExportSource {
            domain: "transcripts",
            configured_path: state.config.transcript_snapshot_path.clone(),
        },
    ]
}

fn requested_export_domains(domains: &[String]) -> Vec<String> {
    let mut normalized: Vec<String> = domains
        .iter()
        .map(|domain| normalize_export_domain(domain))
        .filter(|domain| !domain.is_empty())
        .collect();

    if normalized.is_empty() {
        normalized = vec![
            "activity".to_string(),
            "calendar".to_string(),
            "git".to_string(),
            "health".to_string(),
            "messaging".to_string(),
            "notes".to_string(),
            "reminders".to_string(),
            "tasks".to_string(),
            "transcripts".to_string(),
        ];
    }

    normalized.sort();
    normalized.dedup();
    normalized
}

fn normalize_export_domain(domain: &str) -> String {
    match domain.trim().to_ascii_lowercase().as_str() {
        "todoist" => "tasks".to_string(),
        "transcript" => "transcripts".to_string(),
        value => value.to_string(),
    }
}

fn load_manifest(backup_root: &Path) -> Result<BackupManifestData, AppError> {
    let manifest_path = backup_root.join(MANIFEST_FILE_NAME);
    let bytes = fs::read(&manifest_path)
        .map_err(|_| AppError::bad_request("backup manifest is missing"))?;
    serde_json::from_slice::<BackupManifestData>(&bytes)
        .map_err(|error| AppError::bad_request(format!("backup manifest is invalid: {error}")))
}

fn validate_manifest_paths(
    backup_root: &Path,
    manifest: &BackupManifestData,
) -> Result<(), AppError> {
    let manifest_output_root = canonicalize_existing_dir(&manifest.output_root)?;
    if manifest_output_root != backup_root {
        return Err(AppError::bad_request(
            "backup manifest output_root does not match the requested backup root",
        ));
    }

    validate_path_within_root(
        backup_root,
        Path::new(&manifest.database_snapshot_path),
        true,
    )?;
    for checked_path in &manifest.verification_summary.checked_paths {
        validate_path_within_root(backup_root, Path::new(checked_path), false)?;
    }
    Ok(())
}

fn validate_path_within_root(
    backup_root: &Path,
    path: &Path,
    must_be_file: bool,
) -> Result<(), AppError> {
    let canonical = fs::canonicalize(path).map_err(|error| {
        AppError::bad_request(format!("backup path {}: {}", path.display(), error))
    })?;
    if !canonical.starts_with(backup_root) {
        return Err(AppError::bad_request(format!(
            "backup path {} points outside the backup root",
            path.display()
        )));
    }
    if must_be_file && !canonical.is_file() {
        return Err(AppError::bad_request(format!(
            "backup path {} is not a file",
            path.display()
        )));
    }
    Ok(())
}

fn copy_artifacts(source_root: &Path, backup_root: &Path) -> Result<BackupCoverageData, AppError> {
    fs::create_dir_all(source_root).map_err(|error| AppError::internal(error.to_string()))?;
    let mut included = Vec::new();
    let mut omitted = Vec::new();
    let mut notes =
        vec!["Artifact coverage is limited to durable operator-visible files.".to_string()];

    for entry in fs::read_dir(source_root).map_err(|error| AppError::internal(error.to_string()))? {
        let entry = entry.map_err(|error| AppError::internal(error.to_string()))?;
        let name = entry.file_name().to_string_lossy().to_string();
        let relative = format!("{ARTIFACTS_DIR_NAME}/{name}");
        if OMITTED_ARTIFACT_SEGMENTS.contains(&name.as_str()) {
            omitted.push(relative);
            continue;
        }

        copy_path_recursively(
            &entry.path(),
            &backup_root.join(ARTIFACTS_DIR_NAME).join(&name),
        )?;
        included.push(relative);
    }

    if !omitted.is_empty() {
        notes.push("Transient cache and temp directories are intentionally excluded.".to_string());
    }

    Ok(BackupCoverageData {
        included,
        omitted,
        notes,
    })
}

fn copy_path_recursively(source: &Path, destination: &Path) -> Result<(), AppError> {
    if source.is_dir() {
        fs::create_dir_all(destination).map_err(|error| AppError::internal(error.to_string()))?;
        for entry in fs::read_dir(source).map_err(|error| AppError::internal(error.to_string()))? {
            let entry = entry.map_err(|error| AppError::internal(error.to_string()))?;
            copy_path_recursively(&entry.path(), &destination.join(entry.file_name()))?;
        }
    } else if source.is_file() {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| AppError::internal(error.to_string()))?;
        }
        fs::copy(source, destination).map_err(|error| AppError::internal(error.to_string()))?;
    }

    Ok(())
}

async fn split_settings(
    state: &AppState,
) -> Result<(BTreeMap<String, serde_json::Value>, Vec<String>), AppError> {
    let settings = state
        .storage
        .get_all_settings()
        .await
        .map_err(AppError::from)?;
    let mut public_settings = BTreeMap::new();
    let mut omitted = Vec::new();

    for (key, value) in settings {
        if is_secret_setting_key(&key) {
            omitted.push(key);
        } else {
            public_settings.insert(key, value);
        }
    }

    omitted.sort();
    Ok((public_settings, omitted))
}

fn is_secret_setting_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    lower.contains("secret") || lower.contains("token") || lower.contains("private_key")
}

fn build_secret_flags(omitted_settings: &[String]) -> BackupSecretOmissionFlagsData {
    BackupSecretOmissionFlagsData {
        settings_secrets_omitted: omitted_settings.iter().any(|key| key.contains("secret")),
        integration_tokens_omitted: omitted_settings
            .iter()
            .any(|key| key.contains("token") || key.contains("todoist")),
        local_key_material_omitted: true,
        notes: vec![
            "The backup pack is intentionally safe to inspect without exposing secrets."
                .to_string(),
        ],
    }
}

fn collect_checked_paths(backup_root: &Path) -> Vec<String> {
    vec![
        backup_root
            .join(MANIFEST_FILE_NAME)
            .to_string_lossy()
            .to_string(),
        backup_root
            .join(DATA_DIR_NAME)
            .join(SNAPSHOT_FILE_NAME)
            .to_string_lossy()
            .to_string(),
        backup_root
            .join(ARTIFACTS_DIR_NAME)
            .to_string_lossy()
            .to_string(),
        backup_root
            .join(CONFIG_DIR_NAME)
            .to_string_lossy()
            .to_string(),
    ]
}

fn compute_manifest_checksum(
    manifest: &BackupManifestData,
    snapshot_path: &Path,
) -> Result<String, AppError> {
    let snapshot_bytes =
        fs::read(snapshot_path).map_err(|error| AppError::internal(error.to_string()))?;
    let manifest_input = serde_json::json!({
        "backup_id": manifest.backup_id,
        "created_at": manifest.created_at.format(&Rfc3339).map_err(|error| AppError::internal(error.to_string()))?,
        "output_root": manifest.output_root,
        "database_snapshot_path": manifest.database_snapshot_path,
        "artifact_coverage": manifest.artifact_coverage,
        "config_coverage": manifest.config_coverage,
        "explicit_omissions": manifest.explicit_omissions,
        "secret_omission_flags": manifest.secret_omission_flags
    });
    let serialized = serde_json::to_vec(&manifest_input)
        .map_err(|error| AppError::internal(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(serialized);
    hasher.update(snapshot_bytes);
    Ok(hex::encode(hasher.finalize()))
}

fn compute_export_manifest_checksum(
    manifest: &BackupExportManifestData,
) -> Result<String, AppError> {
    let manifest_input = serde_json::json!({
        "export_id": manifest.export_id,
        "created_at": manifest.created_at.format(&Rfc3339).map_err(|error| AppError::internal(error.to_string()))?,
        "target_root": manifest.target_root,
        "export_root": manifest.export_root,
        "included_domains": manifest.included_domains,
        "omitted_domains": manifest.omitted_domains,
        "files": manifest.files,
        "derivatives": manifest.derivatives
    });
    let serialized = serde_json::to_vec(&manifest_input)
        .map_err(|error| AppError::internal(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(serialized);
    Ok(hex::encode(hasher.finalize()))
}

struct BackupExportRetentionPruneReport {
    pruned_runs: usize,
}

async fn prune_backup_export_retention(
    state: &AppState,
    target_root: &Path,
    current_export_id: &str,
    retention_count: u32,
) -> Result<BackupExportRetentionPruneReport, AppError> {
    let runs_root = target_root.join(EXPORT_RUNS_DIR_NAME);
    let canonical_runs_root = canonicalize_export_runs_root(target_root, &runs_root)?;
    let export_runs = state
        .storage
        .list_backup_export_runs(10_000)
        .await
        .map_err(AppError::from)?;
    let mut retained = 1usize;
    let mut pruned = 0usize;
    let retention_count = retention_count as usize;

    for run in export_runs {
        if run.backup_id == current_export_id {
            continue;
        }
        if !matches!(run.state.as_str(), "completed" | "verified") {
            continue;
        }
        if !backup_export_run_matches_target(&run, target_root) {
            continue;
        }

        if retained < retention_count {
            retained += 1;
            continue;
        }

        let run_root = runs_root.join(&run.backup_id);
        if !run_root.exists() {
            continue;
        }
        let metadata = fs::symlink_metadata(&run_root).map_err(|error| {
            AppError::internal(format!(
                "inspect backup export run root {}: {}",
                run_root.display(),
                error
            ))
        })?;
        if metadata.file_type().is_symlink() {
            return Err(AppError::internal(format!(
                "backup export retention refused to prune symlinked run root: {}",
                run_root.display()
            )));
        }
        if !metadata.is_dir() {
            return Err(AppError::internal(format!(
                "backup export retention refused to prune non-directory run root: {}",
                run_root.display()
            )));
        }
        let canonical_run_root = fs::canonicalize(&run_root).map_err(|error| {
            AppError::internal(format!(
                "canonicalize backup export run root {}: {}",
                run_root.display(),
                error
            ))
        })?;
        if !canonical_run_root.starts_with(&canonical_runs_root)
            || canonical_run_root == canonical_runs_root
        {
            return Err(AppError::internal(format!(
                "backup export retention refused to prune path outside runs root: {}",
                canonical_run_root.display()
            )));
        }
        fs::remove_dir_all(&run_root).map_err(|error| {
            AppError::internal(format!(
                "remove backup export run root {}: {}",
                run_root.display(),
                error
            ))
        })?;
        pruned += 1;
    }

    Ok(BackupExportRetentionPruneReport {
        pruned_runs: pruned,
    })
}

fn canonicalize_export_runs_root(
    target_root: &Path,
    runs_root: &Path,
) -> Result<PathBuf, AppError> {
    let canonical_runs_root = fs::canonicalize(runs_root).map_err(|error| {
        AppError::internal(format!(
            "canonicalize backup export runs root {}: {}",
            runs_root.display(),
            error
        ))
    })?;
    if !canonical_runs_root.starts_with(target_root) {
        return Err(AppError::internal(format!(
            "backup export runs root is outside target root: {}",
            canonical_runs_root.display()
        )));
    }
    Ok(canonical_runs_root)
}

fn backup_export_run_matches_target(run: &BackupRunRecord, target_root: &Path) -> bool {
    fs::canonicalize(&run.output_root)
        .map(|output_root| output_root == target_root)
        .unwrap_or(false)
}

fn checksum_file(path: &Path) -> Result<String, AppError> {
    let bytes = fs::read(path).map_err(|error| AppError::internal(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(hex::encode(hasher.finalize()))
}

fn write_parquet_derivatives(
    target_root: &Path,
    files: &[BackupExportFileData],
) -> Result<Vec<BackupExportDerivativeData>, AppError> {
    let mut derivatives = Vec::new();
    for file in files
        .iter()
        .filter(|file| file.schema_version != EXPORT_SOURCE_SNAPSHOT_SCHEMA_VERSION)
    {
        let source_path = Path::new(&file.path);
        let stem = source_path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("records");
        let output_path = target_root
            .join("cold-tier")
            .join(&file.domain)
            .join(format!("{stem}.parquet"));
        write_json_record_parquet(source_path, &output_path)?;
        let record_count = read_ndjson_records(source_path)?.len() as u64;
        derivatives.push(BackupExportDerivativeData {
            domain: file.domain.clone(),
            path: output_path.to_string_lossy().to_string(),
            source_path: file.path.clone(),
            format: "parquet".to_string(),
            record_count,
            checksum_algorithm: "sha256".to_string(),
            checksum: checksum_file(&output_path)?,
        });
    }
    derivatives.sort_by(|left, right| {
        left.domain
            .cmp(&right.domain)
            .then_with(|| left.path.cmp(&right.path))
    });
    Ok(derivatives)
}

fn write_json_record_parquet(source_path: &Path, output_path: &Path) -> Result<(), AppError> {
    let records = read_ndjson_records(source_path)?;
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| AppError::internal(error.to_string()))?;
    }

    let schema = Arc::new(Schema::new(vec![
        Field::new("schema_version", DataType::Utf8, false),
        Field::new("record_kind", DataType::Utf8, false),
        Field::new("source_family", DataType::Utf8, false),
        Field::new("provider_key", DataType::Utf8, false),
        Field::new("source_mode", DataType::Utf8, false),
        Field::new("source_path", DataType::Utf8, false),
        Field::new("external_id", DataType::Utf8, true),
        Field::new("account_ref", DataType::Utf8, true),
        Field::new("record_timestamp", DataType::Utf8, true),
        Field::new("normalized_at", DataType::Utf8, false),
        Field::new("content_hash", DataType::Utf8, false),
        Field::new("payload_json", DataType::Utf8, false),
    ]));
    let columns: Vec<ArrayRef> = vec![
        Arc::new(StringArray::from(required_json_strings(
            &records,
            "schema_version",
        )?)),
        Arc::new(StringArray::from(required_json_strings(
            &records,
            "record_kind",
        )?)),
        Arc::new(StringArray::from(required_json_strings(
            &records,
            "source_family",
        )?)),
        Arc::new(StringArray::from(required_json_strings(
            &records,
            "provider_key",
        )?)),
        Arc::new(StringArray::from(required_json_strings(
            &records,
            "source_mode",
        )?)),
        Arc::new(StringArray::from(required_json_strings(
            &records,
            "source_path",
        )?)),
        Arc::new(StringArray::from(optional_json_strings(
            &records,
            "external_id",
        ))),
        Arc::new(StringArray::from(optional_json_strings(
            &records,
            "account_ref",
        ))),
        Arc::new(StringArray::from(optional_json_strings(
            &records,
            "record_timestamp",
        ))),
        Arc::new(StringArray::from(required_json_strings(
            &records,
            "normalized_at",
        )?)),
        Arc::new(StringArray::from(required_json_strings(
            &records,
            "content_hash",
        )?)),
        Arc::new(StringArray::from(payload_json_strings(&records)?)),
    ];
    let batch = RecordBatch::try_new(schema.clone(), columns)
        .map_err(|error| AppError::internal(format!("build parquet record batch: {error}")))?;
    let file = fs::File::create(output_path).map_err(|error| {
        AppError::internal(format!(
            "create parquet derivative {}: {}",
            output_path.display(),
            error
        ))
    })?;
    let mut writer = ArrowWriter::try_new(file, schema, None)
        .map_err(|error| AppError::internal(format!("create parquet writer: {error}")))?;
    writer
        .write(&batch)
        .map_err(|error| AppError::internal(format!("write parquet batch: {error}")))?;
    writer
        .close()
        .map_err(|error| AppError::internal(format!("close parquet writer: {error}")))?;
    Ok(())
}

fn read_ndjson_records(source_path: &Path) -> Result<Vec<serde_json::Value>, AppError> {
    let content = fs::read_to_string(source_path).map_err(|error| {
        AppError::internal(format!(
            "read export source {} for parquet derivative: {}",
            source_path.display(),
            error
        ))
    })?;
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            serde_json::from_str::<serde_json::Value>(line).map_err(|error| {
                AppError::internal(format!(
                    "parse export source {} for parquet derivative: {}",
                    source_path.display(),
                    error
                ))
            })
        })
        .collect()
}

fn required_json_strings(
    records: &[serde_json::Value],
    field: &str,
) -> Result<Vec<String>, AppError> {
    records
        .iter()
        .map(|record| {
            record
                .get(field)
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
                .ok_or_else(|| {
                    AppError::internal(format!(
                        "normalized export record missing string field {field}"
                    ))
                })
        })
        .collect()
}

fn optional_json_strings(records: &[serde_json::Value], field: &str) -> Vec<Option<String>> {
    records
        .iter()
        .map(|record| {
            record
                .get(field)
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
        })
        .collect()
}

fn payload_json_strings(records: &[serde_json::Value]) -> Result<Vec<String>, AppError> {
    records
        .iter()
        .map(|record| {
            serde_json::to_string(record.get("payload").unwrap_or(&serde_json::Value::Null))
                .map_err(|error| AppError::internal(format!("serialize payload_json: {error}")))
        })
        .collect()
}

struct ExportDomainSnapshotWrite {
    output_path: PathBuf,
    schema_version: &'static str,
    record_count: u64,
}

enum ExportDomainSnapshotError {
    Omitted(String),
    Fatal(AppError),
}

impl From<AppError> for ExportDomainSnapshotError {
    fn from(error: AppError) -> Self {
        Self::Fatal(error)
    }
}

fn write_export_domain_snapshot(
    domain: &str,
    source_path: &Path,
    target_root: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    let domain_root = target_root.join("domains").join(domain);
    match domain {
        "calendar" => {
            let output_path = domain_root.join("events.ndjson");
            write_calendar_export_snapshot(source_path, &output_path, exported_at)
        }
        "tasks" => {
            let output_path = domain_root.join("tasks.ndjson");
            write_tasks_export_snapshot(source_path, &output_path, exported_at)
        }
        "messaging" => {
            let output_path = domain_root.join("threads.ndjson");
            write_messaging_export_snapshot(source_path, &output_path, exported_at)
        }
        "transcripts" => {
            let output_path = domain_root.join("messages.ndjson");
            write_transcripts_export_snapshot(source_path, &output_path, exported_at)
        }
        "health" => {
            let output_path = domain_root.join("samples.ndjson");
            write_health_export_snapshot(source_path, &output_path, exported_at)
        }
        "git" => {
            let output_path = domain_root.join("events.ndjson");
            write_git_export_snapshot(source_path, &output_path, exported_at)
        }
        "reminders" => {
            let output_path = domain_root.join("items.ndjson");
            write_reminders_export_snapshot(source_path, &output_path, exported_at)
        }
        "notes" => {
            let output_path = domain_root.join("notes.ndjson");
            write_notes_export_snapshot(source_path, &output_path, exported_at)
        }
        "activity" => {
            if should_normalize_activity_source(source_path) {
                let output_path = domain_root.join("events.ndjson");
                write_activity_export_snapshot(source_path, &output_path, exported_at)
            } else {
                let output_path = domain_root.join("source.ndjson");
                let record_count =
                    write_export_source_snapshot(domain, source_path, &output_path, exported_at)?;
                Ok(ExportDomainSnapshotWrite {
                    output_path,
                    schema_version: EXPORT_SOURCE_SNAPSHOT_SCHEMA_VERSION,
                    record_count,
                })
            }
        }
        _ => {
            let output_path = domain_root.join("source.ndjson");
            let record_count =
                write_export_source_snapshot(domain, source_path, &output_path, exported_at)?;
            Ok(ExportDomainSnapshotWrite {
                output_path,
                schema_version: EXPORT_SOURCE_SNAPSHOT_SCHEMA_VERSION,
                record_count,
            })
        }
    }
}

fn write_tasks_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    if !source_path.is_file() {
        return Err(ExportDomainSnapshotError::Omitted(format!(
            "tasks source is not a file: {}",
            source_path.display()
        )));
    }
    let bytes = fs::read(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("tasks source could not be read: {error}"))
    })?;
    let snapshot: TodoistExportSnapshot = serde_json::from_slice(&bytes).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("tasks source is malformed: {error}"))
    })?;

    let mut records = Vec::new();
    for item in snapshot
        .items
        .into_iter()
        .filter(|item| !item.content.trim().is_empty())
    {
        let completed = item.checked.unwrap_or(false);
        let payload = serde_json::json!({
            "task_id": item.id,
            "text": item.content.trim(),
            "completed": completed,
            "due_time": item.due.as_ref().and_then(|due| due.date.clone()),
            "labels": item.labels,
            "project_id": item.project_id,
            "priority": item.priority.unwrap_or(1),
            "updated_at": item.updated_at,
        });
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_TASKS_SCHEMA_VERSION,
            record_kind: "task",
            source_family: "tasks",
            provider_key: "todoist",
            source_mode: "local_snapshot",
            source_path,
            external_id: Some(payload["task_id"].as_str().unwrap_or_default().to_string()),
            account_ref: None,
            record_timestamp: payload["updated_at"].as_str().map(str::to_string),
            normalized_at: exported_at,
            payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_TASKS_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn write_calendar_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    if !source_path.is_file() {
        return Err(ExportDomainSnapshotError::Omitted(format!(
            "calendar source is not a file: {}",
            source_path.display()
        )));
    }
    let content = fs::read_to_string(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("calendar source could not be read: {error}"))
    })?;
    let events = parse_backup_export_ics_events(&content);
    if content.contains("BEGIN:VEVENT") && events.is_empty() {
        return Err(ExportDomainSnapshotError::Omitted(
            "calendar source is malformed or contains no exportable events".to_string(),
        ));
    }

    let mut records = Vec::new();
    for event in events {
        let record_timestamp = unix_timestamp_to_rfc3339(event.start_ts);
        let external_id = event
            .payload
            .get("event_id")
            .and_then(|value| value.as_str())
            .filter(|value| !value.trim().is_empty())
            .map(str::to_string);
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_CALENDAR_EVENTS_SCHEMA_VERSION,
            record_kind: "calendar_event",
            source_family: "calendar",
            provider_key: "ics",
            source_mode: "local_ics",
            source_path,
            external_id,
            account_ref: None,
            record_timestamp,
            normalized_at: exported_at,
            payload: event.payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_CALENDAR_EVENTS_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn write_messaging_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    if !source_path.is_file() {
        return Err(ExportDomainSnapshotError::Omitted(format!(
            "messaging source is not a file: {}",
            source_path.display()
        )));
    }
    let bytes = fs::read(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("messaging source could not be read: {error}"))
    })?;
    let snapshot: MessagingExportSnapshot = serde_json::from_slice(&bytes).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("messaging source is malformed: {error}"))
    })?;
    let source = snapshot.source.unwrap_or_else(|| "messaging".to_string());
    let account_id = snapshot.account_id.unwrap_or_else(|| "default".to_string());

    let mut records = Vec::new();
    for thread in snapshot.threads.into_iter() {
        let thread_id = thread.thread_id.trim().to_string();
        let platform = thread.platform.trim().to_string();
        if thread_id.is_empty() || platform.is_empty() {
            continue;
        }
        let record_timestamp = unix_timestamp_to_rfc3339(thread.latest_timestamp);
        let provider_key = platform.clone();
        let source_ref = source.clone();
        let payload = serde_json::json!({
            "source": source_ref,
            "thread_id": thread_id,
            "platform": platform,
            "account_id": account_id,
            "title": thread.title,
            "participants": thread.participants,
            "participant_ids": thread.participant_ids(),
            "latest_timestamp": thread.latest_timestamp,
            "waiting_state": thread.waiting_state,
            "scheduling_related": thread.scheduling_related,
            "urgent": thread.urgent,
            "summary": thread.summary,
            "snippet": thread.snippet,
        });
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_MESSAGING_THREADS_SCHEMA_VERSION,
            record_kind: "message_thread",
            source_family: "messaging",
            provider_key: provider_key.as_str(),
            source_mode: "local_snapshot",
            source_path,
            external_id: Some(
                payload["thread_id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            ),
            account_ref: Some(account_id.clone()),
            record_timestamp,
            normalized_at: exported_at,
            payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_MESSAGING_THREADS_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn write_transcripts_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    if !source_path.is_file() {
        return Err(ExportDomainSnapshotError::Omitted(format!(
            "transcripts source is not a file: {}",
            source_path.display()
        )));
    }
    let bytes = fs::read(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("transcripts source could not be read: {error}"))
    })?;
    let snapshot: TranscriptExportSnapshot = serde_json::from_slice(&bytes).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("transcripts source is malformed: {error}"))
    })?;
    let snapshot_source = snapshot.source();
    let snapshot_conversation_id = snapshot.conversation_id();

    let mut records = Vec::new();
    for message in snapshot.messages() {
        let conversation_id = message
            .conversation_id
            .clone()
            .or_else(|| snapshot_conversation_id.clone());
        let Some(conversation_id) = conversation_id else {
            continue;
        };
        let role = message.role.trim().to_string();
        let content = message.content.trim().to_string();
        if role.is_empty() || content.is_empty() {
            continue;
        }
        let source = message
            .source
            .clone()
            .or_else(|| snapshot_source.clone())
            .unwrap_or_else(|| "transcript".to_string());
        let transcript_id = message.id.clone().unwrap_or_else(|| {
            stable_transcript_export_id(
                &source,
                &conversation_id,
                message.timestamp,
                &role,
                &content,
            )
        });
        let record_timestamp = unix_timestamp_to_rfc3339(message.timestamp);
        let provider_key = source.clone();
        let account_ref = conversation_id.clone();
        let payload = serde_json::json!({
            "transcript_id": transcript_id,
            "conversation_id": conversation_id,
            "source": source,
            "role": role,
            "content": content,
            "timestamp": message.timestamp,
            "metadata": message.metadata,
        });
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_TRANSCRIPT_MESSAGES_SCHEMA_VERSION,
            record_kind: "transcript_message",
            source_family: "transcripts",
            provider_key: provider_key.as_str(),
            source_mode: "local_snapshot",
            source_path,
            external_id: Some(
                payload["transcript_id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            ),
            account_ref: Some(account_ref),
            record_timestamp,
            normalized_at: exported_at,
            payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_TRANSCRIPT_MESSAGES_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn write_activity_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    let bytes = fs::read(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("activity source could not be read: {error}"))
    })?;
    let snapshot: ActivityExportSnapshot = serde_json::from_slice(&bytes).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("activity source is malformed: {error}"))
    })?;
    let default_source = snapshot.source.unwrap_or_else(|| "activity".to_string());

    let mut records = Vec::new();
    for event in snapshot.events {
        let Some(activity) = normalize_backup_export_activity_signal_type(&event.signal_type)
        else {
            continue;
        };
        let source = event.source.unwrap_or_else(|| default_source.clone());
        let provider_key = source.clone();
        let host = event.host.unwrap_or_else(|| "unknown".to_string());
        let external_id = format!("activity:{activity}:{host}:{}", event.timestamp);
        let record_timestamp = unix_timestamp_to_rfc3339(event.timestamp);
        let payload = serde_json::json!({
            "host": host,
            "activity": activity,
            "timestamp": event.timestamp,
            "source": source,
            "details": event.details.unwrap_or_else(|| serde_json::json!({})),
        });
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_ACTIVITY_EVENTS_SCHEMA_VERSION,
            record_kind: "activity_event",
            source_family: "activity",
            provider_key: provider_key.as_str(),
            source_mode: "local_snapshot",
            source_path,
            external_id: Some(external_id),
            account_ref: None,
            record_timestamp,
            normalized_at: exported_at,
            payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_ACTIVITY_EVENTS_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn should_normalize_activity_source(source_path: &Path) -> bool {
    if !source_path.is_file() {
        return false;
    }
    let Ok(bytes) = fs::read(source_path) else {
        return true;
    };
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return true;
    };
    value.get("events").is_some_and(serde_json::Value::is_array)
}

fn write_health_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    if !source_path.is_file() {
        return Err(ExportDomainSnapshotError::Omitted(format!(
            "health source is not a file: {}",
            source_path.display()
        )));
    }
    let bytes = fs::read(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("health source could not be read: {error}"))
    })?;
    let snapshot: HealthExportSnapshot = serde_json::from_slice(&bytes).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("health source is malformed: {error}"))
    })?;
    let default_source = snapshot.source.unwrap_or_else(|| "health".to_string());

    let mut records = Vec::new();
    for sample in snapshot.samples {
        let metric_type = sample.metric_type.trim().to_string();
        if metric_type.is_empty()
            || !crate::services::apple_behavior::is_supported_metric(&metric_type)
        {
            continue;
        }
        let source = sample.source.unwrap_or_else(|| default_source.clone());
        let provider_key = source.clone();
        let source_ref = sample.source_ref.clone().unwrap_or_else(|| {
            let source_app = sample.source_app.as_deref().unwrap_or("unknown");
            let unit = sample.unit.as_deref().unwrap_or("-");
            format!(
                "health:{}:{}:{}:{}:{}",
                metric_type,
                sample.timestamp,
                sample.value,
                source_app.replace([':', '/'], "_"),
                unit
            )
        });
        let record_timestamp = unix_timestamp_to_rfc3339(sample.timestamp);
        let payload = serde_json::json!({
            "metric_type": metric_type,
            "value": sample.value,
            "unit": sample.unit,
            "source": source,
            "source_app": sample.source_app,
            "device": sample.device,
            "source_ref": source_ref.clone(),
            "timestamp": sample.timestamp,
            "metadata": sample.metadata.unwrap_or_else(|| serde_json::json!({})),
        });
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_HEALTH_SAMPLES_SCHEMA_VERSION,
            record_kind: "health_sample",
            source_family: "health",
            provider_key: provider_key.as_str(),
            source_mode: "local_snapshot",
            source_path,
            external_id: Some(source_ref),
            account_ref: None,
            record_timestamp,
            normalized_at: exported_at,
            payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_HEALTH_SAMPLES_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn write_git_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    if !source_path.is_file() {
        return Err(ExportDomainSnapshotError::Omitted(format!(
            "git source is not a file: {}",
            source_path.display()
        )));
    }
    let bytes = fs::read(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("git source could not be read: {error}"))
    })?;
    let snapshot: GitExportSnapshot = serde_json::from_slice(&bytes).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("git source is malformed: {error}"))
    })?;
    let default_source = snapshot.source.unwrap_or_else(|| "git".to_string());

    let mut records = Vec::new();
    for event in snapshot.events {
        let source = event
            .source
            .clone()
            .unwrap_or_else(|| default_source.clone());
        let provider_key = source.clone();
        let dedupe_key = git_export_dedupe_key(&event);
        let source_ref = format!("git:{dedupe_key}");
        let external_id = source_ref.clone();
        let account_ref = event.repo.clone();
        let record_timestamp = unix_timestamp_to_rfc3339(event.timestamp);
        let payload = serde_json::json!({
            "dedupe_key": dedupe_key,
            "repo": event.repo,
            "repo_name": event.repo_name,
            "branch": event.branch,
            "operation": event.operation,
            "commit_oid": event.commit_oid,
            "head_oid": event.head_oid,
            "author": event.author,
            "message": event.message,
            "files_changed": event.files_changed,
            "insertions": event.insertions,
            "deletions": event.deletions,
            "host": event.host,
            "cwd": event.cwd,
            "source": source,
            "source_ref": source_ref,
            "timestamp": event.timestamp,
            "details": event.details.unwrap_or_else(|| serde_json::json!({})),
        });
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_GIT_EVENTS_SCHEMA_VERSION,
            record_kind: "git_event",
            source_family: "git",
            provider_key: provider_key.as_str(),
            source_mode: "local_snapshot",
            source_path,
            external_id: Some(external_id),
            account_ref,
            record_timestamp,
            normalized_at: exported_at,
            payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_GIT_EVENTS_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn write_reminders_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    if !source_path.is_file() {
        return Err(ExportDomainSnapshotError::Omitted(format!(
            "reminders source is not a file: {}",
            source_path.display()
        )));
    }
    let bytes = fs::read(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("reminders source could not be read: {error}"))
    })?;
    let snapshot: RemindersExportSnapshot = serde_json::from_slice(&bytes).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("reminders source is malformed: {error}"))
    })?;
    let default_source = snapshot.source.unwrap_or_else(|| "reminders".to_string());
    let account_id = snapshot
        .account_id
        .unwrap_or_else(|| "local-default".to_string());
    let fallback_ts = snapshot
        .generated_at
        .unwrap_or_else(|| exported_at.unix_timestamp());

    let mut records = Vec::new();
    for reminder in snapshot.reminders {
        let reminder_id = reminder.reminder_id.trim().to_string();
        let title = reminder.title.trim().to_string();
        if reminder_id.is_empty() || title.is_empty() {
            continue;
        }
        let timestamp = reminder
            .updated_at
            .or(reminder.completed_at)
            .or(reminder.due_at)
            .unwrap_or(fallback_ts);
        let source = reminder.source.unwrap_or_else(|| default_source.clone());
        let provider_key = source.clone();
        let source_ref = reminder.source_ref.clone().unwrap_or_else(|| {
            let list_id = reminder.list_id.as_deref().unwrap_or("default");
            let status = if reminder.completed { "done" } else { "open" };
            format!("reminders:{account_id}:{list_id}:{reminder_id}:{timestamp}:{status}")
        });
        let record_timestamp = unix_timestamp_to_rfc3339(timestamp);
        let payload = serde_json::json!({
            "reminder_id": reminder_id,
            "account_id": account_id,
            "list_id": reminder.list_id,
            "list_title": reminder.list_title,
            "title": title,
            "notes": reminder.notes,
            "due_at": reminder.due_at,
            "completed": reminder.completed,
            "completed_at": reminder.completed_at,
            "priority": reminder.priority,
            "tags": reminder.tags.unwrap_or_default(),
            "updated_at": reminder.updated_at,
            "source": source,
            "source_ref": source_ref.clone(),
            "metadata": reminder.metadata.unwrap_or_else(|| serde_json::json!({})),
        });
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_REMINDER_ITEMS_SCHEMA_VERSION,
            record_kind: "reminder_item",
            source_family: "reminders",
            provider_key: provider_key.as_str(),
            source_mode: "local_snapshot",
            source_path,
            external_id: Some(source_ref),
            account_ref: Some(account_id.clone()),
            record_timestamp,
            normalized_at: exported_at,
            payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_REMINDER_ITEMS_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn write_notes_export_snapshot(
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<ExportDomainSnapshotWrite, ExportDomainSnapshotError> {
    let base_dir = if source_path.is_dir() {
        source_path.to_path_buf()
    } else if source_path.is_file() {
        source_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    } else {
        return Err(ExportDomainSnapshotError::Omitted(format!(
            "notes source is neither a file nor a directory: {}",
            source_path.display()
        )));
    };
    let note_files = collect_note_export_files(source_path).map_err(|error| {
        ExportDomainSnapshotError::Omitted(format!("notes source could not be scanned: {error}"))
    })?;

    let mut records = Vec::new();
    for file_path in note_files {
        let content = fs::read_to_string(&file_path).map_err(|error| {
            ExportDomainSnapshotError::Omitted(format!(
                "notes file {} could not be read: {}",
                file_path.display(),
                error
            ))
        })?;
        let content = content.trim().to_string();
        if content.is_empty() {
            continue;
        }
        let metadata = fs::metadata(&file_path).map_err(|error| {
            ExportDomainSnapshotError::Omitted(format!(
                "notes file {} could not be inspected: {}",
                file_path.display(),
                error
            ))
        })?;
        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|value| value.duration_since(UNIX_EPOCH).ok())
            .map(|value| value.as_secs() as i64)
            .unwrap_or(0);
        let relative_path = normalize_backup_export_relative_path(&base_dir, &file_path);
        let title = extract_backup_export_note_title(&content, &file_path);
        let external_id = stable_note_export_id(&relative_path, modified_at, &content);
        let record_timestamp = unix_timestamp_to_rfc3339(modified_at);
        let payload = serde_json::json!({
            "note_id": external_id,
            "path": relative_path,
            "title": title,
            "modified_at": modified_at,
            "content": content,
            "byte_len": metadata.len(),
        });
        records.push(normalized_export_record(NormalizedExportRecordInput {
            schema_version: EXPORT_NOTES_SCHEMA_VERSION,
            record_kind: "note_document",
            source_family: "notes",
            provider_key: "local_notes",
            source_mode: "local_files",
            source_path,
            external_id: Some(external_id),
            account_ref: None,
            record_timestamp,
            normalized_at: exported_at,
            payload,
        })?);
    }

    write_ndjson_records(output_path, &records)?;
    Ok(ExportDomainSnapshotWrite {
        output_path: output_path.to_path_buf(),
        schema_version: EXPORT_NOTES_SCHEMA_VERSION,
        record_count: records.len() as u64,
    })
}

fn write_export_source_snapshot(
    domain: &str,
    source_path: &Path,
    output_path: &Path,
    exported_at: OffsetDateTime,
) -> Result<u64, AppError> {
    let mut record = serde_json::json!({
        "schema_version": EXPORT_SOURCE_SNAPSHOT_SCHEMA_VERSION,
        "domain": domain,
        "exported_at": exported_at.format(&Rfc3339).map_err(|error| AppError::internal(error.to_string()))?,
        "source_path": source_path.to_string_lossy(),
    });

    if source_path.is_file() {
        let bytes =
            fs::read(source_path).map_err(|error| AppError::bad_request(error.to_string()))?;
        record["source_kind"] = serde_json::json!("file");
        match serde_json::from_slice::<serde_json::Value>(&bytes) {
            Ok(value) => {
                record["payload"] = value;
            }
            Err(_) => {
                record["payload_text"] =
                    serde_json::json!(String::from_utf8_lossy(&bytes).to_string());
            }
        }
    } else if source_path.is_dir() {
        record["source_kind"] = serde_json::json!("directory");
        record["file_count"] = serde_json::json!(count_regular_files(source_path)?);
    } else {
        return Err(AppError::bad_request(format!(
            "backup export source {} is neither a file nor a directory",
            source_path.display()
        )));
    }

    write_ndjson_records(output_path, &[record])?;
    Ok(1)
}

struct NormalizedExportRecordInput<'a> {
    schema_version: &'static str,
    record_kind: &'static str,
    source_family: &'static str,
    provider_key: &'a str,
    source_mode: &'static str,
    source_path: &'a Path,
    external_id: Option<String>,
    account_ref: Option<String>,
    record_timestamp: Option<String>,
    normalized_at: OffsetDateTime,
    payload: serde_json::Value,
}

fn normalized_export_record(
    input: NormalizedExportRecordInput<'_>,
) -> Result<serde_json::Value, AppError> {
    Ok(serde_json::json!({
        "schema_version": input.schema_version,
        "record_kind": input.record_kind,
        "source_family": input.source_family,
        "provider_key": input.provider_key,
        "source_mode": input.source_mode,
        "source_path": input.source_path.to_string_lossy(),
        "external_id": input.external_id,
        "account_ref": input.account_ref,
        "record_timestamp": input.record_timestamp,
        "normalized_at": input.normalized_at.format(&Rfc3339).map_err(|error| AppError::internal(error.to_string()))?,
        "content_hash": content_hash(&input.payload)?,
        "payload": input.payload,
    }))
}

fn content_hash(value: &serde_json::Value) -> Result<String, AppError> {
    let bytes = serde_json::to_vec(value).map_err(|error| AppError::internal(error.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

fn write_ndjson_records(path: &Path, records: &[serde_json::Value]) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| AppError::internal(error.to_string()))?;
    }

    let mut bytes = Vec::new();
    for record in records {
        serde_json::to_writer(&mut bytes, record)
            .map_err(|error| AppError::internal(error.to_string()))?;
        bytes.push(b'\n');
    }
    fs::write(path, bytes).map_err(|error| AppError::internal(error.to_string()))
}

#[derive(Debug, Deserialize)]
struct TodoistExportSnapshot {
    #[serde(default, alias = "tasks")]
    items: Vec<TodoistExportItem>,
}

#[derive(Debug, Deserialize)]
struct TodoistExportItem {
    id: String,
    content: String,
    #[serde(default)]
    checked: Option<bool>,
    #[serde(default)]
    priority: Option<u8>,
    #[serde(default)]
    updated_at: Option<String>,
    due: Option<TodoistExportDue>,
    #[serde(default)]
    labels: Vec<String>,
    #[serde(default)]
    project_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TodoistExportDue {
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MessagingExportSnapshot {
    source: Option<String>,
    account_id: Option<String>,
    #[serde(default)]
    threads: Vec<MessagingExportThread>,
}

#[derive(Debug, Deserialize)]
struct MessagingExportThread {
    thread_id: String,
    platform: String,
    title: Option<String>,
    #[serde(default)]
    participants: Vec<MessagingExportParticipant>,
    latest_timestamp: i64,
    waiting_state: String,
    #[serde(default)]
    scheduling_related: bool,
    #[serde(default)]
    urgent: bool,
    summary: Option<String>,
    snippet: Option<String>,
}

impl MessagingExportThread {
    fn participant_ids(&self) -> Vec<String> {
        self.participants
            .iter()
            .map(|participant| participant.id.clone())
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct MessagingExportParticipant {
    id: String,
    name: Option<String>,
    #[serde(default)]
    is_me: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TranscriptExportSnapshot {
    Envelope(TranscriptExportEnvelope),
    Messages(Vec<TranscriptExportMessage>),
}

impl TranscriptExportSnapshot {
    fn source(&self) -> Option<String> {
        match self {
            TranscriptExportSnapshot::Envelope(envelope) => envelope.source.clone(),
            TranscriptExportSnapshot::Messages(_) => None,
        }
    }

    fn conversation_id(&self) -> Option<String> {
        match self {
            TranscriptExportSnapshot::Envelope(envelope) => envelope.conversation_id.clone(),
            TranscriptExportSnapshot::Messages(_) => None,
        }
    }

    fn messages(self) -> Vec<TranscriptExportMessage> {
        match self {
            TranscriptExportSnapshot::Envelope(envelope) => envelope.messages,
            TranscriptExportSnapshot::Messages(messages) => messages,
        }
    }
}

#[derive(Debug, Deserialize)]
struct TranscriptExportEnvelope {
    source: Option<String>,
    conversation_id: Option<String>,
    #[serde(default)]
    messages: Vec<TranscriptExportMessage>,
}

#[derive(Debug, Deserialize)]
struct TranscriptExportMessage {
    id: Option<String>,
    source: Option<String>,
    conversation_id: Option<String>,
    timestamp: i64,
    role: String,
    content: String,
    metadata: Option<serde_json::Value>,
}

fn stable_transcript_export_id(
    source: &str,
    conversation_id: &str,
    timestamp: i64,
    role: &str,
    content: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(source.as_bytes());
    hasher.update(b"|");
    hasher.update(conversation_id.as_bytes());
    hasher.update(b"|");
    hasher.update(timestamp.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(role.as_bytes());
    hasher.update(b"|");
    hasher.update(content.as_bytes());
    let digest = hasher.finalize();
    format!("tr_{}", hex::encode(&digest[..8]))
}

#[derive(Debug, Deserialize)]
struct ActivityExportSnapshot {
    source: Option<String>,
    events: Vec<ActivityExportEvent>,
}

#[derive(Debug, Deserialize)]
struct ActivityExportEvent {
    signal_type: String,
    timestamp: i64,
    source: Option<String>,
    host: Option<String>,
    details: Option<serde_json::Value>,
}

fn normalize_backup_export_activity_signal_type(signal_type: &str) -> Option<&'static str> {
    match signal_type {
        "shell_login" => Some("shell_login"),
        "shell_exit" => Some("shell_exit"),
        "computer_activity" => Some("computer_activity"),
        "idle_state" => Some("idle_state"),
        "git_activity" => Some("computer_activity"),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
struct HealthExportSnapshot {
    source: Option<String>,
    #[serde(default)]
    samples: Vec<HealthExportSample>,
}

#[derive(Debug, Deserialize)]
struct HealthExportSample {
    metric_type: String,
    timestamp: i64,
    value: serde_json::Value,
    unit: Option<String>,
    source: Option<String>,
    source_app: Option<String>,
    device: Option<String>,
    source_ref: Option<String>,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct GitExportSnapshot {
    source: Option<String>,
    #[serde(default)]
    events: Vec<GitExportEvent>,
}

#[derive(Debug, Deserialize)]
struct GitExportEvent {
    timestamp: i64,
    source: Option<String>,
    repo: Option<String>,
    repo_name: Option<String>,
    branch: Option<String>,
    operation: Option<String>,
    commit_oid: Option<String>,
    head_oid: Option<String>,
    author: Option<String>,
    message: Option<String>,
    files_changed: Option<u32>,
    insertions: Option<u32>,
    deletions: Option<u32>,
    host: Option<String>,
    cwd: Option<String>,
    details: Option<serde_json::Value>,
}

fn git_export_dedupe_key(event: &GitExportEvent) -> String {
    let repo = event.repo.as_deref().unwrap_or("-");
    let branch = event.branch.as_deref().unwrap_or("-");
    let operation = event.operation.as_deref().unwrap_or("activity");
    let commit = event
        .commit_oid
        .as_deref()
        .or(event.head_oid.as_deref())
        .unwrap_or("-");
    format!(
        "{}|{}|{}|{}|{}",
        repo, branch, operation, commit, event.timestamp
    )
}

fn collect_note_export_files(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let metadata = fs::metadata(root)?;
    if metadata.is_file() {
        return Ok(if is_supported_note_export_file(root) {
            vec![root.to_path_buf()]
        } else {
            Vec::new()
        });
    }
    if !metadata.is_dir() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                stack.push(path);
            } else if file_type.is_file() && is_supported_note_export_file(&path) {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

fn is_supported_note_export_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| extension.to_ascii_lowercase()),
        Some(extension) if extension == "md" || extension == "markdown" || extension == "txt"
    )
}

fn normalize_backup_export_relative_path(base_dir: &Path, path: &Path) -> String {
    path.strip_prefix(base_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn extract_backup_export_note_title(content: &str, path: &Path) -> String {
    for line in content.lines() {
        let line = line.trim();
        if let Some(title) = line
            .strip_prefix("# ")
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return title.to_string();
        }
    }

    path.file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("note")
        .to_string()
}

fn stable_note_export_id(relative_path: &str, modified_at: i64, content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(relative_path.as_bytes());
    hasher.update(b"|");
    hasher.update(modified_at.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(content.as_bytes());
    let digest = hasher.finalize();
    format!("note_{}", hex::encode(&digest[..8]))
}

#[derive(Debug, Deserialize)]
struct RemindersExportSnapshot {
    source: Option<String>,
    account_id: Option<String>,
    generated_at: Option<i64>,
    #[serde(default)]
    reminders: Vec<ReminderExportItem>,
}

#[derive(Debug, Deserialize)]
struct ReminderExportItem {
    reminder_id: String,
    title: String,
    list_id: Option<String>,
    list_title: Option<String>,
    notes: Option<String>,
    due_at: Option<i64>,
    #[serde(default)]
    completed: bool,
    completed_at: Option<i64>,
    priority: Option<i64>,
    #[serde(default)]
    tags: Option<Vec<String>>,
    metadata: Option<serde_json::Value>,
    updated_at: Option<i64>,
    source: Option<String>,
    source_ref: Option<String>,
}

struct CalendarExportEvent {
    start_ts: i64,
    payload: serde_json::Value,
}

fn parse_backup_export_ics_events(content: &str) -> Vec<CalendarExportEvent> {
    let mut events = Vec::new();
    let mut in_vevent = false;
    let mut uid = String::new();
    let mut summary = String::new();
    let mut start_ts: Option<i64> = None;
    let mut end_ts: Option<i64> = None;
    let mut location = String::new();
    let mut description = String::new();
    let mut status = String::new();
    let mut url = String::new();
    let mut attendees: Vec<String> = Vec::new();
    let mut prep_minutes: Option<i64> = None;
    let mut travel_minutes: Option<i64> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.eq_ignore_ascii_case("BEGIN:VEVENT") {
            in_vevent = true;
            uid.clear();
            summary.clear();
            start_ts = None;
            end_ts = None;
            location.clear();
            description.clear();
            status.clear();
            url.clear();
            attendees.clear();
            prep_minutes = None;
            travel_minutes = None;
            continue;
        }
        if line.eq_ignore_ascii_case("END:VEVENT") {
            in_vevent = false;
            if status.eq_ignore_ascii_case("CANCELLED") {
                continue;
            }
            if let Some(ts) = start_ts {
                events.push(CalendarExportEvent {
                    start_ts: ts,
                    payload: serde_json::json!({
                        "event_id": uid,
                        "title": summary,
                        "start": ts,
                        "end": end_ts,
                        "location": location,
                        "description": description,
                        "status": status,
                        "url": url,
                        "attendees": attendees,
                        "prep_minutes": prep_minutes.unwrap_or(15),
                        "travel_minutes": travel_minutes.unwrap_or(0),
                    }),
                });
            }
            continue;
        }
        if !in_vevent {
            continue;
        }
        if let Some((name, value)) = line.split_once(':') {
            let raw_name = name.trim();
            let base_name = raw_name.split(';').next().unwrap_or(raw_name);
            let value = value.trim();
            match base_name.to_ascii_uppercase().as_str() {
                "UID" => uid = value.to_string(),
                "SUMMARY" => summary = value.to_string(),
                "DTSTART" => start_ts = parse_backup_export_ical_dt(raw_name, value),
                "DTEND" => end_ts = parse_backup_export_ical_dt(raw_name, value),
                "LOCATION" => location = value.to_string(),
                "DESCRIPTION" => description = value.to_string(),
                "STATUS" => status = value.to_string(),
                "URL" => url = value.to_string(),
                "ATTENDEE" => attendees.push(parse_backup_export_attendee(raw_name, value)),
                "X-VEL-PREP-MINUTES" => prep_minutes = value.parse::<i64>().ok(),
                "X-VEL-TRAVEL-MINUTES" => travel_minutes = value.parse::<i64>().ok(),
                _ => {}
            }
        }
    }

    events
}

fn parse_backup_export_ical_dt(raw_name: &str, value: &str) -> Option<i64> {
    let tzid = raw_name.split(';').skip(1).find_map(|param| {
        let (key, value) = param.split_once('=')?;
        key.eq_ignore_ascii_case("TZID").then_some(value.trim())
    });
    let has_utc_suffix = value.trim().ends_with('Z');
    let value = value.trim().trim_end_matches('Z');
    if value.len() == 8 {
        let year: i32 = value.get(0..4)?.parse().ok()?;
        let month: u8 = value.get(4..6)?.parse().ok()?;
        let day: u8 = value.get(6..8)?.parse().ok()?;
        let month = time::Month::try_from(month).ok()?;
        let date = time::Date::from_calendar_date(year, month, day).ok()?;
        return Some(date.midnight().assume_utc().unix_timestamp());
    }
    if value.len() < 15 || value.get(8..9) != Some("T") {
        return None;
    }
    let date_part = value.get(0..8)?;
    let time_part = value.get(9..15)?;
    let year: i32 = date_part.get(0..4)?.parse().ok()?;
    let month: u8 = date_part.get(4..6)?.parse().ok()?;
    let day: u8 = date_part.get(6..8)?.parse().ok()?;
    let hour: u8 = time_part.get(0..2)?.parse().ok()?;
    let minute: u8 = time_part.get(2..4)?.parse().ok()?;
    let second: u8 = time_part.get(4..6)?.parse().ok()?;
    let month = time::Month::try_from(month).ok()?;
    let date = time::Date::from_calendar_date(year, month, day).ok()?;
    let time = time::Time::from_hms(hour, minute, second).ok()?;
    let date_time = time::PrimitiveDateTime::new(date, time);
    let date_time = if has_utc_suffix {
        date_time.assume_utc()
    } else if let Some(tzid) = tzid {
        date_time.assume_offset(backup_export_offset_for_tzid(tzid, date)?)
    } else {
        date_time.assume_utc()
    };
    Some(date_time.unix_timestamp())
}

fn backup_export_offset_for_tzid(tzid: &str, date: time::Date) -> Option<time::UtcOffset> {
    let hours = match tzid {
        "UTC" | "Etc/UTC" => 0,
        "America/Phoenix" => -7,
        "America/Denver" => {
            if backup_export_is_us_dst(date) {
                -6
            } else {
                -7
            }
        }
        "America/Chicago" => {
            if backup_export_is_us_dst(date) {
                -5
            } else {
                -6
            }
        }
        "America/New_York" => {
            if backup_export_is_us_dst(date) {
                -4
            } else {
                -5
            }
        }
        "America/Los_Angeles" => {
            if backup_export_is_us_dst(date) {
                -7
            } else {
                -8
            }
        }
        _ => return None,
    };
    time::UtcOffset::from_hms(hours, 0, 0).ok()
}

fn backup_export_is_us_dst(date: time::Date) -> bool {
    let year = date.year();
    let dst_start =
        backup_export_nth_weekday_of_month(year, time::Month::March, time::Weekday::Sunday, 2);
    let dst_end =
        backup_export_nth_weekday_of_month(year, time::Month::November, time::Weekday::Sunday, 1);
    date >= dst_start && date < dst_end
}

fn backup_export_nth_weekday_of_month(
    year: i32,
    month: time::Month,
    weekday: time::Weekday,
    occurrence: u8,
) -> time::Date {
    let first = time::Date::from_calendar_date(year, month, 1).expect("valid month start");
    let days_until = (weekday.number_days_from_monday() as i16
        - first.weekday().number_days_from_monday() as i16)
        .rem_euclid(7) as u8;
    first + time::Duration::days(i64::from(days_until + (occurrence - 1) * 7))
}

fn parse_backup_export_attendee(name: &str, value: &str) -> String {
    for param in name.split(';').skip(1) {
        if let Some((key, param_value)) = param.split_once('=') {
            if key.eq_ignore_ascii_case("CN") && !param_value.trim().is_empty() {
                return param_value.trim_matches('"').to_string();
            }
        }
    }

    value
        .trim()
        .strip_prefix("mailto:")
        .unwrap_or(value.trim())
        .to_string()
}

fn unix_timestamp_to_rfc3339(timestamp: i64) -> Option<String> {
    OffsetDateTime::from_unix_timestamp(timestamp)
        .ok()
        .and_then(|value| value.format(&Rfc3339).ok())
}

fn count_regular_files(root: &Path) -> Result<u64, AppError> {
    let mut count = 0;
    for entry in fs::read_dir(root).map_err(|error| AppError::bad_request(error.to_string()))? {
        let entry = entry.map_err(|error| AppError::bad_request(error.to_string()))?;
        let path = entry.path();
        if path.is_dir() {
            count += count_regular_files(&path)?;
        } else if path.is_file() {
            count += 1;
        }
    }
    Ok(count)
}

fn status_from_record(record: &BackupRunRecord) -> Result<BackupStatusData, AppError> {
    let manifest: BackupManifestData = serde_json::from_value(record.manifest_json.clone())
        .map_err(|error| AppError::internal(error.to_string()))?;
    let last_backup_at = record
        .verified_at
        .or(record.completed_at)
        .or(Some(record.started_at));

    Ok(BackupStatusData {
        state: match record.state.as_str() {
            "completed" | "verified" => BackupStatusStateData::Ready,
            "failed" => BackupStatusStateData::Degraded,
            _ => BackupStatusStateData::Stale,
        },
        last_backup_id: Some(record.backup_id.clone()),
        last_backup_at,
        output_root: Some(record.output_root.clone()),
        artifact_coverage: Some(manifest.artifact_coverage.clone()),
        config_coverage: Some(manifest.config_coverage.clone()),
        verification_summary: Some(manifest.verification_summary.clone()),
        warnings: record.last_error.clone().into_iter().collect(),
    })
}

fn export_status_from_record(record: &BackupRunRecord) -> Result<BackupExportStatusData, AppError> {
    let manifest: BackupExportManifestData =
        serde_json::from_value(record.manifest_json.clone())
            .map_err(|error| AppError::internal(error.to_string()))?;
    let last_export_at = record
        .verified_at
        .or(record.completed_at)
        .or(Some(record.started_at));

    Ok(BackupExportStatusData {
        state: match record.state.as_str() {
            "completed" | "verified" => BackupStatusStateData::Ready,
            "failed" => BackupStatusStateData::Degraded,
            _ => BackupStatusStateData::Stale,
        },
        last_export_id: Some(manifest.export_id),
        last_export_at,
        target_root: Some(record.output_root.clone()),
        included_domains: manifest.included_domains,
        omitted_domains: manifest.omitted_domains,
        verification_summary: Some(manifest.verification_summary),
        warnings: record.last_error.clone().into_iter().collect(),
    })
}

fn scheduled_export_failure_warning(job: &BackupJobRecord) -> String {
    let detail = job
        .last_error_message
        .as_deref()
        .or(job.last_error_code.as_deref())
        .unwrap_or("unknown error");
    format!(
        "latest scheduled backup export failed for {}: {}",
        job.storage_target_root, detail
    )
}

fn status_from_manifest(
    manifest: &BackupManifestData,
    last_backup_at: Option<OffsetDateTime>,
) -> BackupStatusData {
    BackupStatusData {
        state: if manifest.verification_summary.verified {
            BackupStatusStateData::Ready
        } else {
            BackupStatusStateData::Degraded
        },
        last_backup_id: Some(manifest.backup_id.clone()),
        last_backup_at,
        output_root: Some(manifest.output_root.clone()),
        artifact_coverage: Some(manifest.artifact_coverage.clone()),
        config_coverage: Some(manifest.config_coverage.clone()),
        verification_summary: Some(manifest.verification_summary.clone()),
        warnings: Vec::new(),
    }
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), AppError> {
    let bytes =
        serde_json::to_vec_pretty(value).map_err(|error| AppError::internal(error.to_string()))?;
    fs::write(path, bytes).map_err(|error| AppError::internal(error.to_string()))
}
