use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use vel_api_types::{
    BackupCoverageData, BackupFreshnessData, BackupFreshnessStateData, BackupManifestData,
    BackupSecretOmissionFlagsData, BackupStatusData, BackupStatusStateData, BackupTrustData,
    BackupTrustLevelData, BackupVerificationData,
};
use vel_storage::{BackupRunRecord, Storage};

use crate::{errors::AppError, state::AppState};

pub(crate) const DEFAULT_BACKUP_ROOT: &str = "var/backups";
const MANIFEST_FILE_NAME: &str = "manifest.json";
const DATA_DIR_NAME: &str = "data";
const ARTIFACTS_DIR_NAME: &str = "artifacts";
const CONFIG_DIR_NAME: &str = "config";
const SNAPSHOT_FILE_NAME: &str = "vel.sqlite";
const PUBLIC_SETTINGS_FILE_NAME: &str = "public-settings.json";
const RUNTIME_CONFIG_FILE_NAME: &str = "runtime-config.json";
const OMITTED_ARTIFACT_SEGMENTS: &[&str] = &["cache", "tmp"];
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

pub async fn backup_status(state: &AppState) -> Result<BackupStatusData, AppError> {
    backup_status_for_storage(&state.storage).await
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
