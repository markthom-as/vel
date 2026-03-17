use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{ArtifactId, SyncClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageTargetKind {
    LocalFilesystem,
    Rsync,
    S3,
    IcloudDrive,
    GoogleDrive,
    Dropbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageTargetRole {
    BackupOnly,
    MirrorSync,
    ActiveStorage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactCopyState {
    Pending,
    Copied,
    Verified,
    Missing,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupManifestScope {
    Full,
    Incremental,
    Filtered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupManifestState {
    Pending,
    Running,
    Completed,
    Verified,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationSubjectType {
    LocalArtifact,
    ArtifactCopy,
    Manifest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    Verified,
    Mismatch,
    Missing,
    Unreadable,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorePlanState {
    Pending,
    Prepared,
    Executing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StorageTargetId(String);

impl StorageTargetId {
    pub fn new() -> Self {
        Self(format!("stgt_{}", Uuid::new_v4().simple()))
    }
}

impl Default for StorageTargetId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for StorageTargetId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for StorageTargetId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BackupManifestId(String);

impl BackupManifestId {
    pub fn new() -> Self {
        Self(format!("bman_{}", Uuid::new_v4().simple()))
    }
}

impl Default for BackupManifestId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for BackupManifestId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for BackupManifestId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VerificationRecordId(String);

impl VerificationRecordId {
    pub fn new() -> Self {
        Self(format!("ver_{}", Uuid::new_v4().simple()))
    }
}

impl Default for VerificationRecordId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for VerificationRecordId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for VerificationRecordId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RestorePlanId(String);

impl RestorePlanId {
    pub fn new() -> Self {
        Self(format!("rpl_{}", Uuid::new_v4().simple()))
    }
}

impl Default for RestorePlanId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for RestorePlanId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for RestorePlanId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTarget {
    pub id: StorageTargetId,
    pub kind: StorageTargetKind,
    pub role: StorageTargetRole,
    pub label: String,
    pub root_uri: String,
    pub path_prefix: Option<String>,
    pub provider_ref: Option<String>,
    pub enabled: bool,
    pub metadata_json: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_success_at: Option<OffsetDateTime>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactCopy {
    pub artifact_id: ArtifactId,
    pub storage_target_id: StorageTargetId,
    pub state: ArtifactCopyState,
    pub target_locator: String,
    pub target_version: Option<String>,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub copied_at: Option<OffsetDateTime>,
    pub verified_at: Option<OffsetDateTime>,
    pub last_error: Option<String>,
    pub metadata_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    pub id: BackupManifestId,
    pub storage_target_id: StorageTargetId,
    pub scope: BackupManifestScope,
    pub state: BackupManifestState,
    pub started_at: OffsetDateTime,
    pub completed_at: Option<OffsetDateTime>,
    pub verified_at: Option<OffsetDateTime>,
    pub summary_json: serde_json::Value,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifestEntry {
    pub backup_manifest_id: BackupManifestId,
    pub artifact_id: ArtifactId,
    pub artifact_copy_locator: String,
    pub source_storage_uri: Option<String>,
    pub source_storage_kind: Option<String>,
    pub sync_class: SyncClass,
    pub expected_content_hash: Option<String>,
    pub expected_size_bytes: Option<i64>,
    pub target_version: Option<String>,
    pub entry_state: BackupManifestState,
    pub metadata_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationRecord {
    pub id: VerificationRecordId,
    pub subject_type: VerificationSubjectType,
    pub subject_id: String,
    pub status: VerificationStatus,
    pub observed_content_hash: Option<String>,
    pub observed_size_bytes: Option<i64>,
    pub failure_reason: Option<String>,
    pub checked_at: OffsetDateTime,
    pub metadata_json: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePlan {
    pub id: RestorePlanId,
    pub source_target_id: StorageTargetId,
    pub state: RestorePlanState,
    pub requested_at: OffsetDateTime,
    pub prepared_at: Option<OffsetDateTime>,
    pub executed_at: Option<OffsetDateTime>,
    pub destination_root: String,
    pub summary_json: serde_json::Value,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePlanItem {
    pub restore_plan_id: RestorePlanId,
    pub artifact_id: ArtifactId,
    pub target_locator: String,
    pub target_version: Option<String>,
    pub planned_destination: String,
    pub state: RestorePlanState,
    pub failure_reason: Option<String>,
}
