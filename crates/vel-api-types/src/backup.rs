use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupCoverageData {
    #[serde(default)]
    pub included: Vec<String>,
    #[serde(default)]
    pub omitted: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupSecretOmissionFlagsData {
    pub settings_secrets_omitted: bool,
    pub integration_tokens_omitted: bool,
    pub local_key_material_omitted: bool,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupVerificationData {
    pub verified: bool,
    pub checksum_algorithm: String,
    pub checksum: String,
    #[serde(default)]
    pub checked_paths: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifestData {
    pub backup_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub output_root: String,
    pub database_snapshot_path: String,
    pub artifact_coverage: BackupCoverageData,
    pub config_coverage: BackupCoverageData,
    #[serde(default)]
    pub explicit_omissions: Vec<String>,
    pub secret_omission_flags: BackupSecretOmissionFlagsData,
    pub verification_summary: BackupVerificationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupExportRequestData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_root: Option<String>,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub include_parquet: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupExportDomainOmissionData {
    pub domain: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupExportFileData {
    pub domain: String,
    pub path: String,
    pub schema_version: String,
    pub record_count: u64,
    pub checksum_algorithm: String,
    pub checksum: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupExportDerivativeData {
    pub domain: String,
    pub path: String,
    pub source_path: String,
    pub format: String,
    pub record_count: u64,
    pub checksum_algorithm: String,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupExportManifestData {
    pub export_id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub target_root: String,
    pub export_root: String,
    #[serde(default)]
    pub included_domains: Vec<String>,
    #[serde(default)]
    pub omitted_domains: Vec<BackupExportDomainOmissionData>,
    #[serde(default)]
    pub files: Vec<BackupExportFileData>,
    #[serde(default)]
    pub derivatives: Vec<BackupExportDerivativeData>,
    pub verification_summary: BackupVerificationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupExportResultData {
    pub manifest: BackupExportManifestData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupExportStatusData {
    pub state: BackupStatusStateData,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_export_id: Option<String>,
    #[serde(default)]
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_export_at: Option<OffsetDateTime>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_root: Option<String>,
    #[serde(default)]
    pub included_domains: Vec<String>,
    #[serde(default)]
    pub omitted_domains: Vec<BackupExportDomainOmissionData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_summary: Option<BackupVerificationData>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupStatusStateData {
    Ready,
    Stale,
    Missing,
    Degraded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupStatusData {
    pub state: BackupStatusStateData,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_id: Option<String>,
    #[serde(default)]
    #[serde(with = "time::serde::rfc3339::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_backup_at: Option<OffsetDateTime>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_root: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_coverage: Option<BackupCoverageData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_coverage: Option<BackupCoverageData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_summary: Option<BackupVerificationData>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupTrustLevelData {
    Ok,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupFreshnessStateData {
    Current,
    Stale,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupFreshnessData {
    pub state: BackupFreshnessStateData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_seconds: Option<i64>,
    pub stale_after_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupTrustData {
    pub level: BackupTrustLevelData,
    pub status: BackupStatusData,
    pub freshness: BackupFreshnessData,
    #[serde(default)]
    pub guidance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSettingsData {
    pub default_output_root: String,
    pub trust: BackupTrustData,
}
