use crate::{
    infra::sqlite_connect_options,
    mapping::{parse_json_value, timestamp_to_datetime},
    runtime_loops,
};
use serde_json::{json, Value as JsonValue};
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, QueryBuilder, Row, Sqlite, SqlitePool};
use std::{fs, path::Path};
use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{
    ArtifactId, ArtifactStorageKind, CaptureId, Commitment, CommitmentId, CommitmentStatus,
    ContextCapture, ConversationId, EventId, IntegrationConnection, IntegrationConnectionEvent,
    IntegrationConnectionEventType, IntegrationConnectionId, IntegrationConnectionSettingRef,
    IntegrationConnectionStatus, IntegrationFamily, IntegrationProvider, InterventionId, JobId,
    JobStatus, MessageId, OrientationSnapshot, PrivacyClass, Ref, Run, RunEvent, RunEventType,
    RunId, RunKind, RunStatus, SearchResult, SyncClass,
};
static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

#[derive(Debug, Clone)]
pub struct Storage {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct CaptureInsert {
    pub content_text: String,
    pub capture_type: String,
    pub source_device: Option<String>,
    pub privacy_class: PrivacyClass,
}

#[derive(Debug, Clone)]
pub struct SignalInsert {
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: i64,
    pub payload_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct SignalRecord {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: i64,
    pub payload_json: JsonValue,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct AssistantTranscriptInsert {
    pub id: String,
    pub source: String,
    pub conversation_id: String,
    pub timestamp: i64,
    pub role: String,
    pub content: String,
    pub metadata_json: JsonValue,
}

#[derive(Debug, Clone)]
pub struct AssistantTranscriptRecord {
    pub id: String,
    pub source: String,
    pub conversation_id: String,
    pub timestamp: i64,
    pub role: String,
    pub content: String,
    pub metadata_json: JsonValue,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct InferredStateInsert {
    pub state_name: String,
    pub confidence: Option<String>,
    pub timestamp: i64,
    pub context_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct InferredStateRecord {
    pub state_id: String,
    pub state_name: String,
    pub confidence: Option<String>,
    pub timestamp: i64,
    pub context_json: JsonValue,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct NudgeInsert {
    pub nudge_type: String,
    pub level: String,
    pub state: String,
    pub related_commitment_id: Option<String>,
    pub message: String,
    pub snoozed_until: Option<i64>,
    pub resolved_at: Option<i64>,
    pub signals_snapshot_json: Option<String>,
    pub inference_snapshot_json: Option<String>,
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct NudgeRecord {
    pub nudge_id: String,
    pub nudge_type: String,
    pub level: String,
    pub state: String,
    pub related_commitment_id: Option<String>,
    pub message: String,
    pub created_at: i64,
    pub snoozed_until: Option<i64>,
    pub resolved_at: Option<i64>,
    pub signals_snapshot_json: Option<String>,
    pub inference_snapshot_json: Option<String>,
    pub metadata_json: JsonValue,
}

#[derive(Debug, Clone)]
pub struct NudgeEventRecord {
    pub id: String,
    pub nudge_id: String,
    pub event_type: String,
    pub payload_json: JsonValue,
    pub timestamp: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct SuggestionInsertV2 {
    pub suggestion_type: String,
    pub state: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub priority: i64,
    pub confidence: Option<String>,
    pub dedupe_key: Option<String>,
    pub payload_json: JsonValue,
    pub decision_context_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct SuggestionRecord {
    pub id: String,
    pub suggestion_type: String,
    pub state: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub priority: i64,
    pub confidence: Option<String>,
    pub dedupe_key: Option<String>,
    pub payload_json: JsonValue,
    pub decision_context_json: Option<JsonValue>,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
    pub evidence_count: u32,
}

#[derive(Debug, Clone)]
pub struct SuggestionEvidenceInsert {
    pub suggestion_id: String,
    pub evidence_type: String,
    pub ref_id: String,
    pub evidence_json: Option<JsonValue>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct SuggestionEvidenceRecord {
    pub id: String,
    pub suggestion_id: String,
    pub evidence_type: String,
    pub ref_id: String,
    pub evidence_json: Option<JsonValue>,
    pub weight: Option<f64>,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct SuggestionFeedbackInsert {
    pub suggestion_id: String,
    pub outcome_type: String,
    pub notes: Option<String>,
    pub observed_at: i64,
    pub payload_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct SuggestionFeedbackRecord {
    pub id: String,
    pub suggestion_id: String,
    pub outcome_type: String,
    pub notes: Option<String>,
    pub observed_at: i64,
    pub payload_json: Option<JsonValue>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Default)]
pub struct SuggestionFeedbackSummary {
    pub accepted_and_policy_changed: u32,
    pub rejected_not_useful: u32,
    pub rejected_incorrect: u32,
}

#[derive(Debug, Clone)]
pub struct UncertaintyRecordInsert {
    pub subject_type: String,
    pub subject_id: Option<String>,
    pub decision_kind: String,
    pub confidence_band: String,
    pub confidence_score: Option<f64>,
    pub reasons_json: JsonValue,
    pub missing_evidence_json: Option<JsonValue>,
    pub resolution_mode: String,
}

#[derive(Debug, Clone)]
pub struct UncertaintyRecord {
    pub id: String,
    pub subject_type: String,
    pub subject_id: Option<String>,
    pub decision_kind: String,
    pub confidence_band: String,
    pub confidence_score: Option<f64>,
    pub reasons_json: JsonValue,
    pub missing_evidence_json: Option<JsonValue>,
    pub resolution_mode: String,
    pub status: String,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ConversationInsert {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
}

#[derive(Debug, Clone)]
pub struct ConversationRecord {
    pub id: ConversationId,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct MessageInsert {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub kind: String,
    pub content_json: String,
    pub status: Option<String>,
    pub importance: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MessageRecord {
    pub id: MessageId,
    pub conversation_id: ConversationId,
    pub role: String,
    pub kind: String,
    pub content_json: String,
    pub status: Option<String>,
    pub importance: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct InterventionInsert {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: i64,
    pub resolved_at: Option<i64>,
    pub snoozed_until: Option<i64>,
    pub confidence: Option<f64>,
    pub source_json: Option<String>,
    pub provenance_json: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InterventionRecord {
    pub id: InterventionId,
    pub message_id: MessageId,
    pub kind: String,
    pub state: String,
    pub surfaced_at: i64,
    pub resolved_at: Option<i64>,
    pub snoozed_until: Option<i64>,
    pub confidence: Option<f64>,
    pub source_json: Option<String>,
    pub provenance_json: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EventLogInsert {
    pub id: Option<String>,
    pub event_name: String,
    pub aggregate_type: Option<String>,
    pub aggregate_id: Option<String>,
    pub payload_json: String,
}

#[derive(Debug, Clone)]
pub struct EventLogRecord {
    pub id: EventId,
    pub event_name: String,
    pub aggregate_type: Option<String>,
    pub aggregate_id: Option<String>,
    pub payload_json: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct CommitmentInsert {
    pub text: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub status: CommitmentStatus,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Default)]
pub struct SearchFilters {
    pub capture_type: Option<String>,
    pub source_device: Option<String>,
    pub limit: Option<u32>,
}

/// A job claimed for processing. Caller must eventually call `mark_job_succeeded` or `mark_job_failed`.
#[derive(Debug, Clone)]
pub struct PendingJob {
    pub job_id: JobId,
    pub job_type: String,
    pub payload_json: String,
}

#[derive(Debug, Clone)]
pub struct ArtifactInsert {
    pub artifact_type: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub storage_uri: String,
    pub storage_kind: ArtifactStorageKind,
    pub privacy_class: PrivacyClass,
    pub sync_class: SyncClass,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    /// Optional JSON metadata (e.g. generator, context_kind). Stored as TEXT.
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct ArtifactRecord {
    pub artifact_id: ArtifactId,
    pub artifact_type: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub storage_uri: String,
    pub storage_kind: ArtifactStorageKind,
    pub privacy_class: String,
    pub sync_class: String,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone)]
pub struct RetryReadyRun {
    pub run: Run,
    pub retry_at: i64,
    pub retry_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuntimeLoopRecord {
    pub loop_kind: String,
    pub enabled: bool,
    pub interval_seconds: i64,
    pub last_started_at: Option<i64>,
    pub last_finished_at: Option<i64>,
    pub last_status: Option<String>,
    pub last_error: Option<String>,
    pub next_due_at: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkAssignmentStatus {
    Assigned,
    Started,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for WorkAssignmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            WorkAssignmentStatus::Assigned => "assigned",
            WorkAssignmentStatus::Started => "started",
            WorkAssignmentStatus::Completed => "completed",
            WorkAssignmentStatus::Failed => "failed",
            WorkAssignmentStatus::Cancelled => "cancelled",
        };
        write!(f, "{value}")
    }
}

impl std::str::FromStr for WorkAssignmentStatus {
    type Err = vel_core::VelCoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "assigned" => Ok(WorkAssignmentStatus::Assigned),
            "started" => Ok(WorkAssignmentStatus::Started),
            "completed" => Ok(WorkAssignmentStatus::Completed),
            "failed" => Ok(WorkAssignmentStatus::Failed),
            "cancelled" => Ok(WorkAssignmentStatus::Cancelled),
            _ => Err(vel_core::VelCoreError::Validation(format!(
                "unknown status {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkAssignmentInsert {
    pub receipt_id: Option<String>,
    pub work_request_id: String,
    pub worker_id: String,
    pub worker_class: Option<String>,
    pub capability: Option<String>,
    pub status: WorkAssignmentStatus,
    pub assigned_at: i64,
}

#[derive(Debug, Clone)]
pub struct WorkAssignmentUpdate {
    pub receipt_id: String,
    pub status: WorkAssignmentStatus,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub result: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkAssignmentRecord {
    pub receipt_id: String,
    pub work_request_id: String,
    pub worker_id: String,
    pub worker_class: Option<String>,
    pub capability: Option<String>,
    pub status: WorkAssignmentStatus,
    pub assigned_at: i64,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub result: Option<String>,
    pub error_message: Option<String>,
    pub last_updated: i64,
}

#[derive(Debug, Clone)]
pub struct IntegrationConnectionInsert {
    pub family: IntegrationFamily,
    pub provider: IntegrationProvider,
    pub status: IntegrationConnectionStatus,
    pub display_name: String,
    pub account_ref: Option<String>,
    pub metadata_json: JsonValue,
}

#[derive(Debug, Clone, Default)]
pub struct IntegrationConnectionFilters {
    pub family: Option<IntegrationFamily>,
    pub provider_key: Option<String>,
    pub include_disabled: bool,
}

#[derive(Debug, Clone)]
pub struct ClusterWorkerUpsert {
    pub worker_id: String,
    pub node_id: String,
    pub node_display_name: Option<String>,
    pub worker_class: Option<String>,
    pub worker_classes: Vec<String>,
    pub capabilities: Vec<String>,
    pub status: Option<String>,
    pub max_concurrency: Option<u32>,
    pub current_load: Option<u32>,
    pub queue_depth: Option<u32>,
    pub reachability: Option<String>,
    pub latency_class: Option<String>,
    pub compute_class: Option<String>,
    pub power_class: Option<String>,
    pub recent_failure_rate: Option<f64>,
    pub tailscale_preferred: bool,
    pub sync_base_url: Option<String>,
    pub sync_transport: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub last_heartbeat_at: i64,
    pub started_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ClusterWorkerRecord {
    pub worker_id: String,
    pub node_id: String,
    pub node_display_name: Option<String>,
    pub worker_class: Option<String>,
    pub worker_classes: Vec<String>,
    pub capabilities: Vec<String>,
    pub status: Option<String>,
    pub max_concurrency: Option<u32>,
    pub current_load: Option<u32>,
    pub queue_depth: Option<u32>,
    pub reachability: Option<String>,
    pub latency_class: Option<String>,
    pub compute_class: Option<String>,
    pub power_class: Option<String>,
    pub recent_failure_rate: Option<f64>,
    pub tailscale_preferred: bool,
    pub sync_base_url: Option<String>,
    pub sync_transport: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub last_heartbeat_at: i64,
    pub started_at: Option<i64>,
    pub updated_at: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("filesystem error: {0}")]
    Filesystem(#[from] std::io::Error),
    #[error("invalid timestamp in storage: {0}")]
    InvalidTimestamp(String),
    #[error("validation error: {0}")]
    Validation(String),
}

impl Storage {
    pub async fn connect(db_path: &str) -> Result<Self, StorageError> {
        if db_path != ":memory:" {
            if let Some(parent) = Path::new(db_path).parent() {
                fs::create_dir_all(parent)?;
            }
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(sqlite_connect_options(db_path)?)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), StorageError> {
        MIGRATOR.run(&self.pool).await?;
        let version = self.schema_version().await?;
        let payload = serde_json::json!({ "schema_version": version }).to_string();
        if let Err(e) = self
            .emit_event("SCHEMA_MIGRATION_COMPLETE", "schema", None, &payload)
            .await
        {
            tracing::warn!(error = %e, "failed to emit SCHEMA_MIGRATION_COMPLETE event");
        }
        Ok(())
    }

    pub async fn healthcheck(&self) -> Result<(), StorageError> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    /// Returns the number of applied migrations (schema version). Used by doctor/diagnostics.
    pub async fn schema_version(&self) -> Result<u32, StorageError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0 as u32)
    }

    /// Appends a runtime event to the events table. Idempotent callers should generate event_id themselves if needed.
    pub async fn emit_event(
        &self,
        event_type: &str,
        subject_type: &str,
        subject_id: Option<&str>,
        payload_json: &str,
    ) -> Result<(), StorageError> {
        let event_id = format!("evt_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            INSERT INTO events (event_id, event_type, subject_type, subject_id, payload_json, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&event_id)
        .bind(event_type)
        .bind(subject_type)
        .bind(subject_id)
        .bind(payload_json)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_capture(&self, input: CaptureInsert) -> Result<CaptureId, StorageError> {
        let capture_id = CaptureId::new();
        self.insert_capture_with_id(capture_id.clone(), input)
            .await?;
        Ok(capture_id)
    }

    pub async fn insert_capture_with_id(
        &self,
        capture_id: CaptureId,
        input: CaptureInsert,
    ) -> Result<bool, StorageError> {
        let job_id = JobId::new();
        let now = OffsetDateTime::now_utc();
        let metadata = json!({});

        let result = sqlx::query(
            r#"
            INSERT OR IGNORE INTO captures (
                capture_id,
                capture_type,
                content_text,
                occurred_at,
                created_at,
                source_device,
                privacy_class,
                metadata_json
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(capture_id.to_string())
        .bind(&input.capture_type)
        .bind(&input.content_text)
        .bind(now.unix_timestamp())
        .bind(now.unix_timestamp())
        .bind(input.source_device)
        .bind(input.privacy_class.to_string())
        .bind(metadata.to_string())
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Ok(false);
        }

        sqlx::query(
            r#"
            INSERT INTO processing_jobs (
                job_id,
                job_type,
                status,
                created_at,
                started_at,
                finished_at,
                payload_json,
                error_text
            ) VALUES (?, ?, ?, ?, NULL, NULL, ?, NULL)
            "#,
        )
        .bind(job_id.to_string())
        .bind("capture_ingest")
        .bind(JobStatus::Pending.to_string())
        .bind(now.unix_timestamp())
        .bind(json!({ "capture_id": capture_id.to_string() }).to_string())
        .execute(&self.pool)
        .await?;

        Ok(true)
    }

    pub async fn capture_count(&self) -> Result<i64, StorageError> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM captures")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    pub async fn get_capture_by_id(
        &self,
        capture_id: &str,
    ) -> Result<Option<ContextCapture>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT capture_id, capture_type, content_text, occurred_at, source_device
            FROM captures
            WHERE capture_id = ?
            "#,
        )
        .bind(capture_id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };
        Ok(Some(map_context_capture_row(row)?))
    }

    /// List captures most recent first. If today_only, restrict to since start of current day (UTC).
    pub async fn list_captures_recent(
        &self,
        limit: u32,
        today_only: bool,
    ) -> Result<Vec<ContextCapture>, StorageError> {
        let limit = limit.min(500) as i64;
        let rows = if today_only {
            let now = OffsetDateTime::now_utc();
            let start_of_day = now
                .date()
                .with_hms(0, 0, 0)
                .map_err(|e| StorageError::InvalidTimestamp(e.to_string()))?
                .assume_utc()
                .unix_timestamp();
            sqlx::query(
                r#"
                SELECT capture_id, capture_type, content_text, occurred_at, source_device
                FROM captures
                WHERE created_at >= ?
                ORDER BY created_at DESC
                LIMIT ?
                "#,
            )
            .bind(start_of_day)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT capture_id, capture_type, content_text, occurred_at, source_device
                FROM captures
                ORDER BY created_at DESC
                LIMIT ?
                "#,
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };
        rows.into_iter()
            .map(|row| map_context_capture_row(row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn insert_commitment(
        &self,
        input: CommitmentInsert,
    ) -> Result<CommitmentId, StorageError> {
        let id = CommitmentId::new();
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let metadata_str =
            serde_json::to_string(input.metadata_json.as_ref().unwrap_or(&json!({})))
                .map_err(|e| StorageError::Validation(e.to_string()))?;
        let due_ts = input.due_at.map(|t| t.unix_timestamp());
        sqlx::query(
            r#"
            INSERT INTO commitments (id, text, source_type, source_id, status, due_at, project, commitment_kind, created_at, resolved_at, metadata_json)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, ?)
            "#,
        )
        .bind(id.as_ref())
        .bind(&input.text)
        .bind(&input.source_type)
        .bind(&input.source_id)
        .bind(input.status.to_string())
        .bind(due_ts)
        .bind(&input.project)
        .bind(&input.commitment_kind)
        .bind(now)
        .bind(&metadata_str)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn get_commitment_by_id(&self, id: &str) -> Result<Option<Commitment>, StorageError> {
        let row = sqlx::query(
            r#"SELECT id, text, source_type, source_id, status, due_at, project, commitment_kind, created_at, resolved_at, metadata_json FROM commitments WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        let Some(row) = row else {
            return Ok(None);
        };
        Ok(Some(map_commitment_row(&row)?))
    }

    pub async fn list_commitments(
        &self,
        status_filter: Option<CommitmentStatus>,
        project: Option<&str>,
        kind: Option<&str>,
        limit: u32,
    ) -> Result<Vec<Commitment>, StorageError> {
        let limit = limit.min(200) as i64;
        let rows = sqlx::query(
            r#"
            SELECT id, text, source_type, source_id, status, due_at, project, commitment_kind, created_at, resolved_at, metadata_json
            FROM commitments
            WHERE (? IS NULL OR status = ?)
              AND (? IS NULL OR project = ?)
              AND (? IS NULL OR commitment_kind = ?)
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(status_filter.as_ref().map(|s| s.to_string()))
        .bind(status_filter.as_ref().map(|s| s.to_string()))
        .bind(project)
        .bind(project)
        .bind(kind)
        .bind(kind)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_commitment_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn update_commitment(
        &self,
        id: &str,
        text: Option<&str>,
        status: Option<CommitmentStatus>,
        due_at: Option<Option<OffsetDateTime>>,
        project: Option<&str>,
        commitment_kind: Option<&str>,
        metadata_json: Option<&JsonValue>,
    ) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let resolved = status.and_then(|s| {
            (s == CommitmentStatus::Done || s == CommitmentStatus::Cancelled).then_some(now)
        });
        let current: Option<Commitment> = self.get_commitment_by_id(id).await?;
        let Some(c) = current else {
            return Err(StorageError::Validation("commitment not found".to_string()));
        };
        let new_text = text.map(String::from).unwrap_or(c.text);
        let new_status = status.unwrap_or(c.status);
        let new_due = due_at.unwrap_or(c.due_at).map(|t| t.unix_timestamp());
        let new_project = project.map(String::from).or(c.project);
        let new_kind = commitment_kind.map(String::from).or(c.commitment_kind);
        let new_resolved = match status {
            Some(CommitmentStatus::Open) => None,
            Some(_) => resolved,
            None => c.resolved_at.map(|t| t.unix_timestamp()),
        };
        let meta = metadata_json
            .map(|v| serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()))
            .unwrap_or_else(|| c.metadata_json.to_string());
        sqlx::query(
            r#"
            UPDATE commitments SET text = ?, status = ?, due_at = ?, project = ?, commitment_kind = ?, resolved_at = ?, metadata_json = ?
            WHERE id = ?
            "#,
        )
        .bind(&new_text)
        .bind(new_status.to_string())
        .bind(new_due)
        .bind(&new_project)
        .bind(&new_kind)
        .bind(new_resolved)
        .bind(&meta)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // --- Commitment dependencies ---

    pub async fn insert_commitment_dependency(
        &self,
        parent_commitment_id: &str,
        child_commitment_id: &str,
        dependency_type: &str,
    ) -> Result<String, StorageError> {
        let existing = sqlx::query_as::<_, (String,)>(
            r#"SELECT id FROM commitment_dependencies WHERE parent_commitment_id = ? AND child_commitment_id = ? AND dependency_type = ?"#,
        )
        .bind(parent_commitment_id)
        .bind(child_commitment_id)
        .bind(dependency_type)
        .fetch_optional(&self.pool)
        .await?;
        if let Some((id,)) = existing {
            return Ok(id);
        }
        let id = format!("cdep_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO commitment_dependencies (id, parent_commitment_id, child_commitment_id, dependency_type, created_at) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(parent_commitment_id)
        .bind(child_commitment_id)
        .bind(dependency_type)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_commitment_dependencies_by_parent(
        &self,
        parent_commitment_id: &str,
    ) -> Result<Vec<(String, String, String, i64)>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String, String, i64)>(
            r#"SELECT id, child_commitment_id, dependency_type, created_at FROM commitment_dependencies WHERE parent_commitment_id = ? ORDER BY created_at ASC"#,
        )
        .bind(parent_commitment_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn list_commitment_dependencies_by_child(
        &self,
        child_commitment_id: &str,
    ) -> Result<Vec<(String, String, String, i64)>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String, String, i64)>(
            r#"SELECT id, parent_commitment_id, dependency_type, created_at FROM commitment_dependencies WHERE child_commitment_id = ? ORDER BY created_at ASC"#,
        )
        .bind(child_commitment_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    // --- Signals (Phase B) ---

    pub async fn insert_signal(&self, input: SignalInsert) -> Result<String, StorageError> {
        if let Some(source_ref) = input.source_ref.as_deref() {
            if let Some(existing_id) = sqlx::query_scalar::<_, String>(
                r#"SELECT signal_id FROM signals WHERE source = ? AND source_ref = ? LIMIT 1"#,
            )
            .bind(&input.source)
            .bind(source_ref)
            .fetch_optional(&self.pool)
            .await?
            {
                return Ok(existing_id);
            }
        }

        let signal_id = format!("sig_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let payload_str = serde_json::to_string(input.payload_json.as_ref().unwrap_or(&json!({})))
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query(
            r#"INSERT INTO signals (signal_id, signal_type, source, source_ref, timestamp, payload_json, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&signal_id)
        .bind(&input.signal_type)
        .bind(&input.source)
        .bind(&input.source_ref)
        .bind(input.timestamp)
        .bind(&payload_str)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(signal_id)
    }

    pub async fn list_signals(
        &self,
        signal_type: Option<&str>,
        since_ts: Option<i64>,
        limit: u32,
    ) -> Result<Vec<SignalRecord>, StorageError> {
        let limit = limit.min(500) as i64;
        let rows = sqlx::query(
            r#"
            SELECT signal_id, signal_type, source, source_ref, timestamp, payload_json, created_at
            FROM signals
            WHERE (? IS NULL OR signal_type = ?) AND (? IS NULL OR timestamp >= ?)
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(signal_type)
        .bind(signal_type)
        .bind(since_ts)
        .bind(since_ts)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(|row| map_signal_row(&row)).collect()
    }

    pub async fn list_signals_by_ids(
        &self,
        signal_ids: &[String],
    ) -> Result<Vec<SignalRecord>, StorageError> {
        if signal_ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = QueryBuilder::<Sqlite>::new(
            "SELECT signal_id, signal_type, source, source_ref, timestamp, payload_json, created_at FROM signals WHERE signal_id IN (",
        );
        let mut separated = query.separated(", ");
        for signal_id in signal_ids {
            separated.push_bind(signal_id);
        }
        query.push(") ORDER BY timestamp DESC");

        let rows = query.build().fetch_all(&self.pool).await?;
        rows.into_iter().map(|row| map_signal_row(&row)).collect()
    }

    // --- Assistant transcripts ---

    pub async fn insert_assistant_transcript(
        &self,
        input: AssistantTranscriptInsert,
    ) -> Result<bool, StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let metadata_str = serde_json::to_string(&input.metadata_json)
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        let result = sqlx::query(
            r#"INSERT OR IGNORE INTO assistant_transcripts
               (id, source, conversation_id, timestamp, role, content, metadata_json, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&input.id)
        .bind(&input.source)
        .bind(&input.conversation_id)
        .bind(input.timestamp)
        .bind(&input.role)
        .bind(&input.content)
        .bind(&metadata_str)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn list_assistant_transcripts_by_conversation(
        &self,
        conversation_id: &str,
    ) -> Result<Vec<AssistantTranscriptRecord>, StorageError> {
        let rows = sqlx::query(
            r#"SELECT id, source, conversation_id, timestamp, role, content, metadata_json, created_at
               FROM assistant_transcripts
               WHERE conversation_id = ?
               ORDER BY timestamp ASC, created_at ASC"#,
        )
        .bind(conversation_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| {
                let metadata_str: String = row.try_get("metadata_json")?;
                Ok(AssistantTranscriptRecord {
                    id: row.try_get("id")?,
                    source: row.try_get("source")?,
                    conversation_id: row.try_get("conversation_id")?,
                    timestamp: row.try_get("timestamp")?,
                    role: row.try_get("role")?,
                    content: row.try_get("content")?,
                    metadata_json: serde_json::from_str(&metadata_str)
                        .unwrap_or_else(|_| json!({})),
                    created_at: row.try_get("created_at")?,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(StorageError::from)
    }

    // --- Inferred state (Phase C) ---

    pub async fn insert_inferred_state(
        &self,
        input: InferredStateInsert,
    ) -> Result<String, StorageError> {
        let state_id = format!("ist_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let context_str = serde_json::to_string(input.context_json.as_ref().unwrap_or(&json!({})))
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query(
            r#"INSERT INTO inferred_state (state_id, state_name, confidence, timestamp, context_json, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&state_id)
        .bind(&input.state_name)
        .bind(&input.confidence)
        .bind(input.timestamp)
        .bind(&context_str)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(state_id)
    }

    pub async fn list_inferred_state_recent(
        &self,
        state_name: Option<&str>,
        limit: u32,
    ) -> Result<Vec<InferredStateRecord>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT state_id, state_name, confidence, timestamp, context_json, created_at
            FROM inferred_state
            WHERE (? IS NULL OR state_name = ?)
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(state_name)
        .bind(state_name)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_inferred_state_row(&row))
            .collect()
    }

    // --- Nudges (Phase D) ---

    pub async fn insert_nudge(&self, input: NudgeInsert) -> Result<String, StorageError> {
        let nudge_id = format!("nud_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let meta_str = serde_json::to_string(input.metadata_json.as_ref().unwrap_or(&json!({})))
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO nudges (nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&nudge_id)
        .bind(&input.nudge_type)
        .bind(&input.level)
        .bind(&input.state)
        .bind(&input.related_commitment_id)
        .bind(&input.message)
        .bind(now)
        .bind(input.snoozed_until)
        .bind(input.resolved_at)
        .bind(&input.signals_snapshot_json)
        .bind(&input.inference_snapshot_json)
        .bind(&meta_str)
        .execute(&self.pool)
        .await?;
        Ok(nudge_id)
    }

    pub async fn get_nudge(&self, id: &str) -> Result<Option<NudgeRecord>, StorageError> {
        let row = sqlx::query(
            r#"SELECT nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json FROM nudges WHERE nudge_id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| map_nudge_row(&r)).transpose()
    }

    pub async fn list_nudges(
        &self,
        state_filter: Option<&str>,
        limit: u32,
    ) -> Result<Vec<NudgeRecord>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT nudge_id, nudge_type, level, state, related_commitment_id, message, created_at, snoozed_until, resolved_at, signals_snapshot_json, inference_snapshot_json, metadata_json
            FROM nudges
            WHERE (? IS NULL OR state = ?)
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(state_filter)
        .bind(state_filter)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(|r| map_nudge_row(&r)).collect()
    }

    pub async fn update_nudge_state(
        &self,
        nudge_id: &str,
        state: &str,
        snoozed_until: Option<i64>,
        resolved_at: Option<i64>,
    ) -> Result<(), StorageError> {
        sqlx::query(
            r#"UPDATE nudges SET state = ?, snoozed_until = ?, resolved_at = ? WHERE nudge_id = ?"#,
        )
        .bind(state)
        .bind(snoozed_until)
        .bind(resolved_at)
        .bind(nudge_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_nudge_lifecycle(
        &self,
        nudge_id: &str,
        level: &str,
        state: &str,
        message: &str,
        snoozed_until: Option<i64>,
        resolved_at: Option<i64>,
        inference_snapshot_json: Option<&str>,
        metadata_json: &JsonValue,
    ) -> Result<(), StorageError> {
        let metadata_json = serde_json::to_string(metadata_json)
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query(
            r#"
            UPDATE nudges
            SET level = ?, state = ?, message = ?, snoozed_until = ?, resolved_at = ?, inference_snapshot_json = ?, metadata_json = ?
            WHERE nudge_id = ?
            "#,
        )
        .bind(level)
        .bind(state)
        .bind(message)
        .bind(snoozed_until)
        .bind(resolved_at)
        .bind(inference_snapshot_json)
        .bind(metadata_json)
        .bind(nudge_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // --- Current context (singleton) ---

    pub async fn get_current_context(&self) -> Result<Option<(i64, String)>, StorageError> {
        let row = sqlx::query_as::<_, (i64, String)>(
            r#"SELECT computed_at, context_json FROM current_context WHERE id = 1"#,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn set_current_context(
        &self,
        computed_at: i64,
        context_json: &str,
    ) -> Result<(), StorageError> {
        sqlx::query(
            r#"INSERT OR REPLACE INTO current_context (id, computed_at, context_json) VALUES (1, ?, ?)"#,
        )
        .bind(computed_at)
        .bind(context_json)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_context_timeline(
        &self,
        timestamp: i64,
        context_json: &str,
        trigger_signal_id: Option<&str>,
    ) -> Result<(), StorageError> {
        let id = format!("ctl_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO context_timeline (id, timestamp, context_json, trigger_signal_id, created_at) VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(timestamp)
        .bind(context_json)
        .bind(trigger_signal_id)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_context_timeline(
        &self,
        limit: u32,
    ) -> Result<Vec<(String, i64, String)>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query_as::<_, (String, i64, String)>(
            r#"SELECT id, timestamp, context_json FROM context_timeline ORDER BY timestamp DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    // --- Integration foundation (INTG-001) ---

    pub async fn insert_integration_connection(
        &self,
        input: IntegrationConnectionInsert,
    ) -> Result<IntegrationConnectionId, StorageError> {
        if input.provider.family != input.family {
            return Err(StorageError::Validation(
                "integration provider family does not match connection family".to_string(),
            ));
        }
        let id = IntegrationConnectionId::new();
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let metadata_json = serde_json::to_string(&input.metadata_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO integration_connections (
                id,
                family,
                provider_key,
                status,
                display_name,
                account_ref,
                metadata_json,
                created_at,
                updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.as_ref())
        .bind(input.family.to_string())
        .bind(&input.provider.key)
        .bind(input.status.to_string())
        .bind(&input.display_name)
        .bind(&input.account_ref)
        .bind(metadata_json)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn get_integration_connection(
        &self,
        id: &str,
    ) -> Result<Option<IntegrationConnection>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT id, family, provider_key, status, display_name, account_ref, metadata_json, created_at, updated_at
            FROM integration_connections
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|row| map_integration_connection_row(&row))
            .transpose()
    }

    pub async fn list_integration_connections(
        &self,
        filters: IntegrationConnectionFilters,
    ) -> Result<Vec<IntegrationConnection>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT id, family, provider_key, status, display_name, account_ref, metadata_json, created_at, updated_at
            FROM integration_connections
            WHERE (? IS NULL OR family = ?)
              AND (? IS NULL OR provider_key = ?)
              AND (? = 1 OR status != 'disabled')
            ORDER BY family ASC, provider_key ASC, created_at ASC
            "#,
        )
        .bind(filters.family.map(|family| family.to_string()))
        .bind(filters.family.map(|family| family.to_string()))
        .bind(filters.provider_key.as_deref())
        .bind(filters.provider_key.as_deref())
        .bind(if filters.include_disabled { 1_i64 } else { 0_i64 })
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_integration_connection_row(&row))
            .collect()
    }

    pub async fn update_integration_connection(
        &self,
        id: &str,
        status: Option<IntegrationConnectionStatus>,
        display_name: Option<&str>,
        account_ref: Option<Option<&str>>,
        metadata_json: Option<&JsonValue>,
    ) -> Result<(), StorageError> {
        let current = self.get_integration_connection(id).await?.ok_or_else(|| {
            StorageError::Validation("integration connection not found".to_string())
        })?;
        let next_status = status.unwrap_or(current.status);
        let next_display_name = display_name.unwrap_or(current.display_name.as_str());
        let next_account_ref = account_ref
            .map(|value| value.map(ToOwned::to_owned))
            .unwrap_or(current.account_ref);
        let next_metadata_json = metadata_json.cloned().unwrap_or(current.metadata_json);
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let metadata_json = serde_json::to_string(&next_metadata_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            UPDATE integration_connections
            SET status = ?, display_name = ?, account_ref = ?, metadata_json = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(next_status.to_string())
        .bind(next_display_name)
        .bind(next_account_ref)
        .bind(metadata_json)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn upsert_integration_connection_setting_ref(
        &self,
        connection_id: &str,
        setting_key: &str,
        setting_value: &str,
    ) -> Result<(), StorageError> {
        let id = format!("icsr_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            INSERT INTO integration_connection_setting_refs (
                id,
                connection_id,
                setting_key,
                setting_value,
                created_at
            )
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(connection_id, setting_key) DO UPDATE SET setting_value = excluded.setting_value
            "#,
        )
        .bind(id)
        .bind(connection_id)
        .bind(setting_key)
        .bind(setting_value)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_integration_connection_setting_refs(
        &self,
        connection_id: &str,
    ) -> Result<Vec<IntegrationConnectionSettingRef>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT connection_id, setting_key, setting_value, created_at
            FROM integration_connection_setting_refs
            WHERE connection_id = ?
            ORDER BY setting_key ASC
            "#,
        )
        .bind(connection_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_integration_connection_setting_ref_row(&row))
            .collect()
    }

    pub async fn insert_integration_connection_event(
        &self,
        connection_id: &str,
        event_type: IntegrationConnectionEventType,
        payload_json: &JsonValue,
        timestamp: i64,
    ) -> Result<String, StorageError> {
        let id = format!("icev_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let payload_json = serde_json::to_string(payload_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO integration_connection_events (
                id,
                connection_id,
                event_type,
                payload_json,
                timestamp,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(connection_id)
        .bind(event_type.to_string())
        .bind(payload_json)
        .bind(timestamp)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_integration_connection_events(
        &self,
        connection_id: &str,
        limit: u32,
    ) -> Result<Vec<IntegrationConnectionEvent>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT id, connection_id, event_type, payload_json, timestamp, created_at
            FROM integration_connection_events
            WHERE connection_id = ?
            ORDER BY timestamp DESC, created_at DESC
            LIMIT ?
            "#,
        )
        .bind(connection_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_integration_connection_event_row(&row))
            .collect()
    }

    // --- Threads (thread graph) ---

    pub async fn insert_thread(
        &self,
        id: &str,
        thread_type: &str,
        title: &str,
        status: &str,
        metadata_json: &str,
    ) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO threads (id, thread_type, title, status, metadata_json, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(id)
        .bind(thread_type)
        .bind(title)
        .bind(status)
        .bind(metadata_json)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_thread_by_id(
        &self,
        id: &str,
    ) -> Result<Option<(String, String, String, String, String, i64, i64)>, StorageError> {
        let row = sqlx::query_as::<_, (String, String, String, String, String, i64, i64)>(
            r#"SELECT id, thread_type, title, status, metadata_json, created_at, updated_at FROM threads WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn list_threads(
        &self,
        status_filter: Option<&str>,
        limit: u32,
    ) -> Result<Vec<(String, String, String, String, i64, i64)>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = if let Some(s) = status_filter {
            sqlx::query_as::<_, (String, String, String, String, i64, i64)>(
                r#"SELECT id, thread_type, title, status, created_at, updated_at FROM threads WHERE status = ? ORDER BY updated_at DESC LIMIT ?"#,
            )
            .bind(s)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, (String, String, String, String, i64, i64)>(
                r#"SELECT id, thread_type, title, status, created_at, updated_at FROM threads ORDER BY updated_at DESC LIMIT ?"#,
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };
        Ok(rows)
    }

    pub async fn update_thread_status(&self, id: &str, status: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(r#"UPDATE threads SET status = ?, updated_at = ? WHERE id = ?"#)
            .bind(status)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn insert_thread_link(
        &self,
        thread_id: &str,
        entity_type: &str,
        entity_id: &str,
        relation_type: &str,
    ) -> Result<String, StorageError> {
        let id = format!("tl_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT OR IGNORE INTO thread_links (id, thread_id, entity_type, entity_id, relation_type, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(thread_id)
        .bind(entity_type)
        .bind(entity_id)
        .bind(relation_type)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_thread_links(
        &self,
        thread_id: &str,
    ) -> Result<Vec<(String, String, String, String)>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String, String, String)>(
            r#"SELECT id, entity_type, entity_id, relation_type FROM thread_links WHERE thread_id = ? ORDER BY created_at ASC"#,
        )
        .bind(thread_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    // --- Suggestions (steering loop) ---

    pub async fn insert_suggestion(
        &self,
        suggestion_type: &str,
        state: &str,
        payload_json: &str,
    ) -> Result<String, StorageError> {
        let payload_json = serde_json::from_str(payload_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        self.insert_suggestion_v2(SuggestionInsertV2 {
            suggestion_type: suggestion_type.to_string(),
            state: state.to_string(),
            title: None,
            summary: None,
            priority: 50,
            confidence: None,
            dedupe_key: None,
            payload_json,
            decision_context_json: None,
        })
        .await
    }

    pub async fn insert_suggestion_v2(
        &self,
        input: SuggestionInsertV2,
    ) -> Result<String, StorageError> {
        let id = format!("sug_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let payload_json = serde_json::to_string(&input.payload_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        let decision_context_json = input
            .decision_context_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO suggestions (
                id,
                suggestion_type,
                state,
                title,
                summary,
                priority,
                confidence,
                dedupe_key,
                payload_json,
                decision_context_json,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.suggestion_type)
        .bind(&input.state)
        .bind(&input.title)
        .bind(&input.summary)
        .bind(input.priority)
        .bind(&input.confidence)
        .bind(&input.dedupe_key)
        .bind(payload_json)
        .bind(decision_context_json)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_suggestions(
        &self,
        state_filter: Option<&str>,
        limit: u32,
    ) -> Result<Vec<SuggestionRecord>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT
                s.id,
                s.suggestion_type,
                s.state,
                s.title,
                s.summary,
                s.priority,
                s.confidence,
                s.dedupe_key,
                s.payload_json,
                s.decision_context_json,
                s.created_at,
                s.resolved_at,
                CAST(COUNT(se.id) AS INTEGER) AS evidence_count
            FROM suggestions s
            LEFT JOIN suggestion_evidence se ON se.suggestion_id = s.id
            WHERE (? IS NULL OR s.state = ?)
            GROUP BY
                s.id,
                s.suggestion_type,
                s.state,
                s.title,
                s.summary,
                s.priority,
                s.confidence,
                s.dedupe_key,
                s.payload_json,
                s.decision_context_json,
                s.created_at,
                s.resolved_at
            ORDER BY s.created_at DESC, s.rowid DESC
            LIMIT ?
            "#,
        )
        .bind(state_filter)
        .bind(state_filter)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_suggestion_row(&row))
            .collect()
    }

    pub async fn get_suggestion_by_id(
        &self,
        id: &str,
    ) -> Result<Option<SuggestionRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT
                s.id,
                s.suggestion_type,
                s.state,
                s.title,
                s.summary,
                s.priority,
                s.confidence,
                s.dedupe_key,
                s.payload_json,
                s.decision_context_json,
                s.created_at,
                s.resolved_at,
                CAST(COUNT(se.id) AS INTEGER) AS evidence_count
            FROM suggestions s
            LEFT JOIN suggestion_evidence se ON se.suggestion_id = s.id
            WHERE s.id = ?
            GROUP BY
                s.id,
                s.suggestion_type,
                s.state,
                s.title,
                s.summary,
                s.priority,
                s.confidence,
                s.dedupe_key,
                s.payload_json,
                s.decision_context_json,
                s.created_at,
                s.resolved_at
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|row| map_suggestion_row(&row)).transpose()
    }

    pub async fn insert_suggestion_evidence(
        &self,
        input: SuggestionEvidenceInsert,
    ) -> Result<String, StorageError> {
        let id = format!("sugev_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let evidence_json = input
            .evidence_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO suggestion_evidence (
                id,
                suggestion_id,
                evidence_type,
                ref_id,
                evidence_json,
                weight,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.suggestion_id)
        .bind(&input.evidence_type)
        .bind(&input.ref_id)
        .bind(evidence_json)
        .bind(input.weight)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_suggestion_evidence(
        &self,
        suggestion_id: &str,
    ) -> Result<Vec<SuggestionEvidenceRecord>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT id, suggestion_id, evidence_type, ref_id, evidence_json, weight, created_at
            FROM suggestion_evidence
            WHERE suggestion_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(suggestion_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_suggestion_evidence_row(&row))
            .collect()
    }

    pub async fn insert_suggestion_feedback(
        &self,
        input: SuggestionFeedbackInsert,
    ) -> Result<String, StorageError> {
        let id = format!("sugfb_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let payload_json = input
            .payload_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO suggestion_feedback (
                id,
                suggestion_id,
                outcome_type,
                notes,
                observed_at,
                payload_json,
                created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&input.suggestion_id)
        .bind(&input.outcome_type)
        .bind(&input.notes)
        .bind(input.observed_at)
        .bind(payload_json)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_suggestion_feedback(
        &self,
        suggestion_id: &str,
    ) -> Result<Vec<SuggestionFeedbackRecord>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT id, suggestion_id, outcome_type, notes, observed_at, payload_json, created_at
            FROM suggestion_feedback
            WHERE suggestion_id = ?
            ORDER BY observed_at DESC, created_at DESC
            "#,
        )
        .bind(suggestion_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_suggestion_feedback_row(&row))
            .collect()
    }

    pub async fn summarize_suggestion_feedback(
        &self,
        suggestion_type: &str,
    ) -> Result<SuggestionFeedbackSummary, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT
                COALESCE(SUM(CASE WHEN sf.outcome_type = 'accepted_and_policy_changed' THEN 1 ELSE 0 END), 0)
                    AS accepted_and_policy_changed,
                COALESCE(SUM(CASE WHEN sf.outcome_type = 'rejected_not_useful' THEN 1 ELSE 0 END), 0)
                    AS rejected_not_useful,
                COALESCE(SUM(CASE WHEN sf.outcome_type = 'rejected_incorrect' THEN 1 ELSE 0 END), 0)
                    AS rejected_incorrect
            FROM suggestion_feedback sf
            INNER JOIN suggestions s ON s.id = sf.suggestion_id
            WHERE s.suggestion_type = ?
            "#,
        )
        .bind(suggestion_type)
        .fetch_one(&self.pool)
        .await?;
        Ok(SuggestionFeedbackSummary {
            accepted_and_policy_changed: row
                .try_get::<i64, _>("accepted_and_policy_changed")?
                .max(0) as u32,
            rejected_not_useful: row.try_get::<i64, _>("rejected_not_useful")?.max(0) as u32,
            rejected_incorrect: row.try_get::<i64, _>("rejected_incorrect")?.max(0) as u32,
        })
    }

    pub async fn insert_uncertainty_record(
        &self,
        input: UncertaintyRecordInsert,
    ) -> Result<String, StorageError> {
        let id = format!("unc_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let reasons_json = serde_json::to_string(&input.reasons_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        let missing_evidence_json = input
            .missing_evidence_json
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO uncertainty_records (
                id,
                subject_type,
                subject_id,
                decision_kind,
                confidence_band,
                confidence_score,
                reasons_json,
                missing_evidence_json,
                resolution_mode,
                status,
                created_at,
                resolved_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'open', ?, NULL)
            "#,
        )
        .bind(&id)
        .bind(&input.subject_type)
        .bind(&input.subject_id)
        .bind(&input.decision_kind)
        .bind(&input.confidence_band)
        .bind(input.confidence_score)
        .bind(reasons_json)
        .bind(missing_evidence_json)
        .bind(&input.resolution_mode)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_uncertainty_records(
        &self,
        status: Option<&str>,
        limit: u32,
    ) -> Result<Vec<UncertaintyRecord>, StorageError> {
        let limit = i64::from(limit.max(1));
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                subject_type,
                subject_id,
                decision_kind,
                confidence_band,
                confidence_score,
                reasons_json,
                missing_evidence_json,
                resolution_mode,
                status,
                created_at,
                resolved_at
            FROM uncertainty_records
            WHERE (? IS NULL OR status = ?)
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(status)
        .bind(status)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_uncertainty_row(&row))
            .collect()
    }

    pub async fn get_uncertainty_record(
        &self,
        id: &str,
    ) -> Result<Option<UncertaintyRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                subject_type,
                subject_id,
                decision_kind,
                confidence_band,
                confidence_score,
                reasons_json,
                missing_evidence_json,
                resolution_mode,
                status,
                created_at,
                resolved_at
            FROM uncertainty_records
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|row| map_uncertainty_row(&row)).transpose()
    }

    pub async fn find_open_uncertainty_record(
        &self,
        subject_type: &str,
        subject_id: Option<&str>,
        decision_kind: &str,
    ) -> Result<Option<UncertaintyRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                subject_type,
                subject_id,
                decision_kind,
                confidence_band,
                confidence_score,
                reasons_json,
                missing_evidence_json,
                resolution_mode,
                status,
                created_at,
                resolved_at
            FROM uncertainty_records
            WHERE subject_type = ?
              AND decision_kind = ?
              AND status = 'open'
              AND ((? IS NULL AND subject_id IS NULL) OR subject_id = ?)
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(subject_type)
        .bind(decision_kind)
        .bind(subject_id)
        .bind(subject_id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|row| map_uncertainty_row(&row)).transpose()
    }

    pub async fn find_recent_uncertainty_record(
        &self,
        subject_type: &str,
        subject_id: Option<&str>,
        decision_kind: &str,
        status: &str,
        since_ts: i64,
    ) -> Result<Option<UncertaintyRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                subject_type,
                subject_id,
                decision_kind,
                confidence_band,
                confidence_score,
                reasons_json,
                missing_evidence_json,
                resolution_mode,
                status,
                created_at,
                resolved_at
            FROM uncertainty_records
            WHERE subject_type = ?
              AND decision_kind = ?
              AND status = ?
              AND ((? IS NULL AND subject_id IS NULL) OR subject_id = ?)
              AND COALESCE(resolved_at, created_at) >= ?
            ORDER BY COALESCE(resolved_at, created_at) DESC
            LIMIT 1
            "#,
        )
        .bind(subject_type)
        .bind(decision_kind)
        .bind(status)
        .bind(subject_id)
        .bind(subject_id)
        .bind(since_ts)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|row| map_uncertainty_row(&row)).transpose()
    }

    pub async fn resolve_uncertainty_record(
        &self,
        id: &str,
    ) -> Result<Option<UncertaintyRecord>, StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let updated = sqlx::query(
            r#"
            UPDATE uncertainty_records
            SET status = 'resolved',
                resolved_at = COALESCE(resolved_at, ?)
            WHERE id = ?
            "#,
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;
        if updated.rows_affected() == 0 {
            return Ok(None);
        }
        self.get_uncertainty_record(id).await
    }

    pub async fn find_recent_suggestion_by_dedupe_key(
        &self,
        dedupe_key: &str,
    ) -> Result<Option<SuggestionRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT
                s.id,
                s.suggestion_type,
                s.state,
                s.title,
                s.summary,
                s.priority,
                s.confidence,
                s.dedupe_key,
                s.payload_json,
                s.decision_context_json,
                s.created_at,
                s.resolved_at,
                CAST(COUNT(se.id) AS INTEGER) AS evidence_count
            FROM suggestions s
            LEFT JOIN suggestion_evidence se ON se.suggestion_id = s.id
            WHERE s.dedupe_key = ?
            GROUP BY
                s.id,
                s.suggestion_type,
                s.state,
                s.title,
                s.summary,
                s.priority,
                s.confidence,
                s.dedupe_key,
                s.payload_json,
                s.decision_context_json,
                s.created_at,
                s.resolved_at
            ORDER BY s.created_at DESC, s.rowid DESC
            LIMIT 1
            "#,
        )
        .bind(dedupe_key)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|row| map_suggestion_row(&row)).transpose()
    }

    pub async fn update_suggestion_state(
        &self,
        id: &str,
        state: &str,
        resolved_at: Option<i64>,
        payload_json: Option<&str>,
    ) -> Result<(), StorageError> {
        if let Some(payload) = payload_json {
            sqlx::query(
                r#"UPDATE suggestions SET state = ?, resolved_at = ?, payload_json = ? WHERE id = ?"#,
            )
            .bind(state)
            .bind(resolved_at)
            .bind(payload)
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else {
            sqlx::query(r#"UPDATE suggestions SET state = ?, resolved_at = ? WHERE id = ?"#)
                .bind(state)
                .bind(resolved_at)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    // --- Commitment risk (risk engine) ---

    pub async fn insert_commitment_risk(
        &self,
        commitment_id: &str,
        risk_score: f64,
        risk_level: &str,
        factors_json: &str,
        computed_at: i64,
    ) -> Result<String, StorageError> {
        let id = format!("risk_{}", Uuid::new_v4().simple());
        sqlx::query(
            r#"INSERT INTO commitment_risk (id, commitment_id, risk_score, risk_level, factors_json, computed_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(commitment_id)
        .bind(risk_score)
        .bind(risk_level)
        .bind(factors_json)
        .bind(computed_at)
        .execute(&self.pool)
        .await?;
        Ok(id)
    }

    pub async fn list_commitment_risk_recent(
        &self,
        commitment_id: &str,
        limit: u32,
    ) -> Result<Vec<(String, f64, String, String, i64)>, StorageError> {
        let limit = limit.min(50) as i64;
        let rows = sqlx::query_as::<_, (String, f64, String, String, i64)>(
            r#"SELECT id, risk_score, risk_level, factors_json, computed_at FROM commitment_risk WHERE commitment_id = ? ORDER BY computed_at DESC LIMIT ?"#,
        )
        .bind(commitment_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Latest risk snapshot per commitment (for listing current risk).
    pub async fn list_commitment_risk_latest_all(
        &self,
    ) -> Result<Vec<(String, String, f64, String, String, i64)>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String, f64, String, String, i64)>(
            r#"SELECT cr.id, cr.commitment_id, cr.risk_score, cr.risk_level, cr.factors_json, cr.computed_at
               FROM commitment_risk cr
               INNER JOIN (
                 SELECT commitment_id, MAX(computed_at) AS max_at FROM commitment_risk GROUP BY commitment_id
               ) latest ON cr.commitment_id = latest.commitment_id AND cr.computed_at = latest.max_at
               ORDER BY cr.risk_score DESC"#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Count commitment_risk rows (for read-boundary tests: explain must not create new rows).
    pub async fn count_commitment_risk(&self) -> Result<i64, StorageError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM commitment_risk")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }

    /// Count inferred_state rows (for read-boundary tests).
    pub async fn count_inferred_state(&self) -> Result<i64, StorageError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM inferred_state")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }

    /// Count context_timeline rows (for read-boundary tests).
    pub async fn count_context_timeline(&self) -> Result<i64, StorageError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM context_timeline")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }

    /// Count nudge_events rows (for read-boundary tests).
    pub async fn count_nudge_events(&self) -> Result<i64, StorageError> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM nudge_events")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }

    // --- Nudge events (append-only log) ---

    pub async fn insert_nudge_event(
        &self,
        nudge_id: &str,
        event_type: &str,
        payload_json: &str,
        timestamp: i64,
    ) -> Result<(), StorageError> {
        let id = format!("nve_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO nudge_events (id, nudge_id, event_type, payload_json, timestamp, created_at) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(nudge_id)
        .bind(event_type)
        .bind(payload_json)
        .bind(timestamp)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_nudge_events(
        &self,
        nudge_id: &str,
        limit: u32,
    ) -> Result<Vec<NudgeEventRecord>, StorageError> {
        let limit = limit.min(100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT id, nudge_id, event_type, payload_json, timestamp, created_at
            FROM nudge_events
            WHERE nudge_id = ?
            ORDER BY rowid ASC
            LIMIT ?
            "#,
        )
        .bind(nudge_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_nudge_event_row(&row))
            .collect()
    }

    pub async fn search_captures(
        &self,
        query: &str,
        filters: SearchFilters,
    ) -> Result<Vec<SearchResult>, StorageError> {
        let mut builder = QueryBuilder::<Sqlite>::new(
            r#"
            SELECT
                c.capture_id,
                c.capture_type,
                snippet(captures_fts, 1, '[', ']', '...', 12) AS snippet,
                c.occurred_at,
                c.created_at,
                c.source_device,
                bm25(captures_fts) AS rank
            FROM captures_fts
            JOIN captures c ON c.capture_id = captures_fts.capture_id
            WHERE captures_fts MATCH
            "#,
        );
        builder.push_bind(query);

        if let Some(capture_type) = filters.capture_type.as_deref() {
            builder.push(" AND c.capture_type = ");
            builder.push_bind(capture_type);
        }

        if let Some(source_device) = filters.source_device.as_deref() {
            builder.push(" AND c.source_device = ");
            builder.push_bind(source_device);
        }

        let limit = i64::from(filters.limit.unwrap_or(10).clamp(1, 50));
        builder.push(" ORDER BY rank ASC, c.occurred_at DESC, c.created_at DESC LIMIT ");
        builder.push_bind(limit);

        let rows = builder.build().fetch_all(&self.pool).await?;
        rows.into_iter().map(map_search_row).collect()
    }

    /// Claims the next pending job of the given type. Returns `None` if no pending job exists.
    /// The job is marked `running` and `started_at` is set. Caller must call `mark_job_succeeded` or `mark_job_failed`.
    pub async fn claim_next_pending_job(
        &self,
        job_type: &str,
    ) -> Result<Option<PendingJob>, StorageError> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query(
            r#"
            SELECT job_id, payload_json
            FROM processing_jobs
            WHERE job_type = ? AND status = ?
            ORDER BY created_at ASC
            LIMIT 1
            "#,
        )
        .bind(job_type)
        .bind(JobStatus::Pending.to_string())
        .fetch_optional(&mut *tx)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let job_id: String = row.try_get("job_id")?;
        let payload_json: String = row.try_get("payload_json")?;

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let updated = sqlx::query(
            r#"
            UPDATE processing_jobs
            SET status = ?, started_at = ?
            WHERE job_id = ? AND status = ?
            "#,
        )
        .bind(JobStatus::Running.to_string())
        .bind(now)
        .bind(&job_id)
        .bind(JobStatus::Pending.to_string())
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        if updated.rows_affected() == 0 {
            return Ok(None);
        }

        Ok(Some(PendingJob {
            job_id: JobId::from(job_id),
            job_type: job_type.to_string(),
            payload_json,
        }))
    }

    pub async fn mark_job_succeeded(&self, job_id: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            UPDATE processing_jobs
            SET status = ?, finished_at = ?, error_text = NULL
            WHERE job_id = ?
            "#,
        )
        .bind(JobStatus::Succeeded.to_string())
        .bind(now)
        .bind(job_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn mark_job_failed(&self, job_id: &str, error: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            UPDATE processing_jobs
            SET status = ?, finished_at = ?, error_text = ?
            WHERE job_id = ?
            "#,
        )
        .bind(JobStatus::Failed.to_string())
        .bind(now)
        .bind(error)
        .bind(job_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn create_artifact(&self, input: ArtifactInsert) -> Result<ArtifactId, StorageError> {
        let artifact_id = ArtifactId::new();
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            INSERT INTO artifacts (
                artifact_id,
                artifact_type,
                title,
                mime_type,
                storage_uri,
                storage_kind,
                privacy_class,
                sync_class,
                content_hash,
                size_bytes,
                created_at,
                updated_at,
                metadata_json
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(artifact_id.to_string())
        .bind(&input.artifact_type)
        .bind(&input.title)
        .bind(&input.mime_type)
        .bind(&input.storage_uri)
        .bind(input.storage_kind.to_string())
        .bind(input.privacy_class.to_string())
        .bind(input.sync_class.to_string())
        .bind(&input.content_hash)
        .bind(input.size_bytes)
        .bind(now)
        .bind(now)
        .bind(
            input
                .metadata_json
                .as_ref()
                .and_then(|v| serde_json::to_string(v).ok())
                .as_deref()
                .unwrap_or("{}"),
        )
        .execute(&self.pool)
        .await?;
        Ok(artifact_id)
    }

    pub async fn get_artifact_by_id(
        &self,
        artifact_id: &str,
    ) -> Result<Option<ArtifactRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT artifact_id, artifact_type, title, mime_type, storage_uri, storage_kind,
                   privacy_class, sync_class, content_hash, size_bytes, created_at, updated_at
            FROM artifacts
            WHERE artifact_id = ?
            "#,
        )
        .bind(artifact_id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let storage_kind_str: String = row.try_get("storage_kind")?;
        let storage_kind = storage_kind_str
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?;

        Ok(Some(ArtifactRecord {
            artifact_id: ArtifactId::from(row.try_get::<String, _>("artifact_id")?),
            artifact_type: row.try_get("artifact_type")?,
            title: row.try_get("title")?,
            mime_type: row.try_get("mime_type")?,
            storage_uri: row.try_get("storage_uri")?,
            storage_kind,
            privacy_class: row.try_get("privacy_class")?,
            sync_class: row.try_get("sync_class")?,
            content_hash: row.try_get("content_hash")?,
            size_bytes: row.try_get("size_bytes")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        }))
    }

    /// Returns the most recently created artifact of the given type, if any.
    pub async fn get_latest_artifact_by_type(
        &self,
        artifact_type: &str,
    ) -> Result<Option<ArtifactRecord>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT artifact_id, artifact_type, title, mime_type, storage_uri, storage_kind,
                   privacy_class, sync_class, content_hash, size_bytes, created_at, updated_at
            FROM artifacts
            WHERE artifact_type = ?
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(artifact_type)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let storage_kind_str: String = row.try_get("storage_kind")?;
        let storage_kind = storage_kind_str
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?;

        Ok(Some(ArtifactRecord {
            artifact_id: ArtifactId::from(row.try_get::<String, _>("artifact_id")?),
            artifact_type: row.try_get("artifact_type")?,
            title: row.try_get("title")?,
            mime_type: row.try_get("mime_type")?,
            storage_uri: row.try_get("storage_uri")?,
            storage_kind,
            privacy_class: row.try_get("privacy_class")?,
            sync_class: row.try_get("sync_class")?,
            content_hash: row.try_get("content_hash")?,
            size_bytes: row.try_get("size_bytes")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        }))
    }

    /// List artifacts by created_at descending, up to limit.
    pub async fn list_artifacts(&self, limit: u32) -> Result<Vec<ArtifactRecord>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT artifact_id, artifact_type, title, mime_type, storage_uri, storage_kind,
                   privacy_class, sync_class, content_hash, size_bytes, created_at, updated_at
            FROM artifacts
            ORDER BY created_at DESC
            LIMIT ?
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let storage_kind_str: String = row.try_get("storage_kind")?;
            let storage_kind = storage_kind_str
                .parse()
                .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?;
            out.push(ArtifactRecord {
                artifact_id: ArtifactId::from(row.try_get::<String, _>("artifact_id")?),
                artifact_type: row.try_get("artifact_type")?,
                title: row.try_get("title")?,
                mime_type: row.try_get("mime_type")?,
                storage_uri: row.try_get("storage_uri")?,
                storage_kind,
                privacy_class: row.try_get("privacy_class")?,
                sync_class: row.try_get("sync_class")?,
                content_hash: row.try_get("content_hash")?,
                size_bytes: row.try_get("size_bytes")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }
        Ok(out)
    }

    pub async fn create_run(
        &self,
        id: &RunId,
        kind: RunKind,
        input_json: &JsonValue,
    ) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let input_str = serde_json::to_string(input_json)
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        let run_created_payload = json!({ "kind": kind.to_string() });
        let payload_str = serde_json::to_string(&run_created_payload)
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        let event_id = format!("evt_{}", Uuid::new_v4().simple());
        let mut tx = self.pool.begin().await?;
        sqlx::query(
            r#"
            INSERT INTO runs (run_id, run_kind, status, created_at, input_json)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.as_ref())
        .bind(kind.to_string())
        .bind(RunStatus::Queued.to_string())
        .bind(now)
        .bind(&input_str)
        .execute(&mut *tx)
        .await?;
        sqlx::query(
            r#"
            INSERT INTO run_events (event_id, run_id, seq, event_type, payload_json, created_at)
            VALUES (?, ?, 1, ?, ?, ?)
            "#,
        )
        .bind(&event_id)
        .bind(id.as_ref())
        .bind(RunEventType::RunCreated.to_string())
        .bind(&payload_str)
        .bind(now)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_run_by_id(&self, run_id: &str) -> Result<Option<Run>, StorageError> {
        let row = sqlx::query(
            r#"
            SELECT run_id, run_kind, status, input_json, output_json, error_json,
                   created_at, started_at, finished_at
            FROM runs WHERE run_id = ?
            "#,
        )
        .bind(run_id)
        .fetch_optional(&self.pool)
        .await?;
        let Some(row) = row else {
            return Ok(None);
        };
        Ok(Some(map_run_row(&row)?))
    }

    pub async fn list_runs(
        &self,
        limit: u32,
        kind_filter: Option<&str>,
        since_ts: Option<i64>,
    ) -> Result<Vec<Run>, StorageError> {
        let limit = limit.clamp(1, 100) as i64;
        let mut sql = r#"
            SELECT run_id, run_kind, status, input_json, output_json, error_json,
                   created_at, started_at, finished_at
            FROM runs
        "#
        .to_string();
        let mut conditions = Vec::new();
        if kind_filter.map(|s| !s.is_empty()).unwrap_or(false) {
            conditions.push("run_kind = ?");
        }
        if since_ts.is_some() {
            conditions.push("created_at >= ?");
        }
        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }
        sql.push_str(" ORDER BY created_at DESC LIMIT ?");

        let mut q = sqlx::query(&sql);
        if let Some(k) = kind_filter.filter(|s| !s.is_empty()) {
            q = q.bind(k);
        }
        if let Some(ts) = since_ts {
            q = q.bind(ts);
        }
        q = q.bind(limit);

        let rows = q.fetch_all(&self.pool).await?;
        rows.into_iter()
            .map(|row| map_run_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn update_run_status(
        &self,
        run_id: &str,
        status: RunStatus,
        started_at: Option<i64>,
        finished_at: Option<i64>,
        output_json: Option<&JsonValue>,
        error_json: Option<&JsonValue>,
    ) -> Result<(), StorageError> {
        let output_str = output_json
            .map(|v| serde_json::to_string(v).map_err(|e| StorageError::Validation(e.to_string())))
            .transpose()?;
        let error_str = error_json
            .map(|v| serde_json::to_string(v).map_err(|e| StorageError::Validation(e.to_string())))
            .transpose()?;
        sqlx::query(
            r#"
            UPDATE runs SET status = ?,
                started_at = COALESCE(?, started_at),
                finished_at = COALESCE(?, finished_at),
                output_json = ?, error_json = ?
            WHERE run_id = ?
            "#,
        )
        .bind(status.to_string())
        .bind(started_at)
        .bind(finished_at)
        .bind(output_str)
        .bind(error_str)
        .bind(run_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn reset_run_for_retry(&self, run_id: &str) -> Result<(), StorageError> {
        sqlx::query(
            r#"
            UPDATE runs
            SET status = ?,
                started_at = NULL,
                finished_at = NULL,
                output_json = NULL,
                error_json = NULL
            WHERE run_id = ?
            "#,
        )
        .bind(RunStatus::Queued.to_string())
        .bind(run_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn append_run_event(
        &self,
        run_id: &str,
        seq: u32,
        event_type: RunEventType,
        payload_json: &JsonValue,
    ) -> Result<(), StorageError> {
        let event_id = format!("evt_{}", Uuid::new_v4().simple());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let payload_str = serde_json::to_string(payload_json)
            .map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query(
            r#"
            INSERT INTO run_events (event_id, run_id, seq, event_type, payload_json, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&event_id)
        .bind(run_id)
        .bind(seq as i64)
        .bind(event_type.to_string())
        .bind(&payload_str)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn next_run_event_seq(&self, run_id: &str) -> Result<u32, StorageError> {
        let (next_seq,): (i64,) =
            sqlx::query_as(r#"SELECT COALESCE(MAX(seq), 0) + 1 FROM run_events WHERE run_id = ?"#)
                .bind(run_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(next_seq as u32)
    }

    pub async fn append_run_event_auto(
        &self,
        run_id: &str,
        event_type: RunEventType,
        payload_json: &JsonValue,
    ) -> Result<u32, StorageError> {
        let seq = self.next_run_event_seq(run_id).await?;
        self.append_run_event(run_id, seq, event_type, payload_json)
            .await?;
        Ok(seq)
    }

    pub async fn list_retry_ready_runs(
        &self,
        now_ts: i64,
        limit: u32,
    ) -> Result<Vec<RetryReadyRun>, StorageError> {
        let limit = limit.clamp(1, 100) as i64;
        let rows = sqlx::query(
            r#"
            SELECT
                r.run_id,
                r.run_kind,
                r.status,
                r.input_json,
                r.output_json,
                r.error_json,
                r.created_at,
                r.started_at,
                r.finished_at,
                CAST(json_extract(e.payload_json, '$.retry_at') AS INTEGER) AS retry_at,
                json_extract(e.payload_json, '$.reason') AS retry_reason
            FROM runs r
            JOIN (
                SELECT run_id, MAX(seq) AS max_seq
                FROM run_events
                WHERE event_type = ?
                GROUP BY run_id
            ) latest
              ON latest.run_id = r.run_id
            JOIN run_events e
              ON e.run_id = latest.run_id AND e.seq = latest.max_seq
            WHERE r.status = ?
              AND CAST(json_extract(e.payload_json, '$.retry_at') AS INTEGER) <= ?
            ORDER BY retry_at ASC, r.created_at ASC
            LIMIT ?
            "#,
        )
        .bind(RunEventType::RunRetryScheduled.to_string())
        .bind(RunStatus::RetryScheduled.to_string())
        .bind(now_ts)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| {
                let retry_at = row.try_get::<i64, _>("retry_at")?;
                let retry_reason = row.try_get::<Option<String>, _>("retry_reason")?;
                let run = map_run_row(&row)?;
                Ok(RetryReadyRun {
                    run,
                    retry_at,
                    retry_reason,
                })
            })
            .collect()
    }

    pub async fn list_run_events(&self, run_id: &str) -> Result<Vec<RunEvent>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT event_id, run_id, seq, event_type, payload_json, created_at
            FROM run_events WHERE run_id = ? ORDER BY seq ASC
            "#,
        )
        .bind(run_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(map_run_event_row)
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn create_ref(&self, ref_: &Ref) -> Result<(), StorageError> {
        let now = ref_.created_at.unix_timestamp();
        sqlx::query(
            r#"
            INSERT INTO refs (ref_id, from_type, from_id, to_type, to_id, relation_type, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&ref_.id)
        .bind(&ref_.from_type)
        .bind(&ref_.from_id)
        .bind(&ref_.to_type)
        .bind(&ref_.to_id)
        .bind(ref_.relation_type.to_string())
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_refs_from(
        &self,
        from_type: &str,
        from_id: &str,
    ) -> Result<Vec<Ref>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT ref_id, from_type, from_id, to_type, to_id, relation_type, created_at
            FROM refs WHERE from_type = ? AND from_id = ? ORDER BY created_at ASC
            "#,
        )
        .bind(from_type)
        .bind(from_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(map_ref_row)
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn list_refs_to(&self, to_type: &str, to_id: &str) -> Result<Vec<Ref>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT ref_id, from_type, from_id, to_type, to_id, relation_type, created_at
            FROM refs WHERE to_type = ? AND to_id = ? ORDER BY created_at ASC
            "#,
        )
        .bind(to_type)
        .bind(to_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(map_ref_row)
            .collect::<Result<Vec<_>, _>>()
    }

    // --- Conversations (chat) ---

    pub async fn create_conversation(
        &self,
        input: ConversationInsert,
    ) -> Result<ConversationId, StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO conversations (id, title, kind, pinned, archived, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&input.id)
        .bind(input.title.as_deref())
        .bind(&input.kind)
        .bind(if input.pinned { 1i32 } else { 0 })
        .bind(if input.archived { 1i32 } else { 0 })
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(ConversationId::from(input.id))
    }

    pub async fn list_conversations(
        &self,
        archived: Option<bool>,
        limit: u32,
    ) -> Result<Vec<ConversationRecord>, StorageError> {
        let limit = limit.min(500) as i64;
        let rows = if let Some(arch) = archived {
            sqlx::query(
                r#"SELECT id, title, kind, pinned, archived, created_at, updated_at
                   FROM conversations WHERE archived = ? ORDER BY updated_at DESC LIMIT ?"#,
            )
            .bind(if arch { 1i32 } else { 0 })
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"SELECT id, title, kind, pinned, archived, created_at, updated_at
                   FROM conversations ORDER BY updated_at DESC LIMIT ?"#,
            )
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
        };
        rows.into_iter()
            .map(|row| map_conversation_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_conversation(
        &self,
        id: &str,
    ) -> Result<Option<ConversationRecord>, StorageError> {
        let row = sqlx::query(
            r#"SELECT id, title, kind, pinned, archived, created_at, updated_at
               FROM conversations WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| map_conversation_row(&r)).transpose()
    }

    pub async fn rename_conversation(&self, id: &str, title: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(r#"UPDATE conversations SET title = ?, updated_at = ? WHERE id = ?"#)
            .bind(title)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn pin_conversation(&self, id: &str, pinned: bool) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(r#"UPDATE conversations SET pinned = ?, updated_at = ? WHERE id = ?"#)
            .bind(if pinned { 1i32 } else { 0 })
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn archive_conversation(&self, id: &str, archived: bool) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(r#"UPDATE conversations SET archived = ?, updated_at = ? WHERE id = ?"#)
            .bind(if archived { 1i32 } else { 0 })
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // --- Messages (chat) ---

    pub async fn create_message(&self, input: MessageInsert) -> Result<MessageId, StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO messages (id, conversation_id, role, kind, content_json, status, importance, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&input.id)
        .bind(&input.conversation_id)
        .bind(&input.role)
        .bind(&input.kind)
        .bind(&input.content_json)
        .bind(input.status.as_deref())
        .bind(input.importance.as_deref())
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(MessageId::from(input.id))
    }

    /// List messages in a conversation, ordered by created_at ASC for stable thread display.
    pub async fn list_messages_by_conversation(
        &self,
        conversation_id: &str,
        limit: u32,
    ) -> Result<Vec<MessageRecord>, StorageError> {
        let limit = limit.min(2000) as i64;
        let rows = sqlx::query(
            r#"SELECT id, conversation_id, role, kind, content_json, status, importance, created_at, updated_at
               FROM messages WHERE conversation_id = ? ORDER BY created_at ASC LIMIT ?"#,
        )
        .bind(conversation_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_message_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_message(&self, id: &str) -> Result<Option<MessageRecord>, StorageError> {
        let row = sqlx::query(
            r#"SELECT id, conversation_id, role, kind, content_json, status, importance, created_at, updated_at
               FROM messages WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| map_message_row(&r)).transpose()
    }

    pub async fn update_message_status(&self, id: &str, status: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(r#"UPDATE messages SET status = ?, updated_at = ? WHERE id = ?"#)
            .bind(status)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // --- Interventions (chat) ---

    pub async fn create_intervention(
        &self,
        input: InterventionInsert,
    ) -> Result<InterventionId, StorageError> {
        sqlx::query(
            r#"INSERT INTO interventions (id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&input.id)
        .bind(&input.message_id)
        .bind(&input.kind)
        .bind(&input.state)
        .bind(input.surfaced_at)
        .bind(input.resolved_at)
        .bind(input.snoozed_until)
        .bind(input.confidence)
        .bind(input.source_json.as_deref())
        .bind(input.provenance_json.as_deref())
        .execute(&self.pool)
        .await?;
        Ok(InterventionId::from(input.id))
    }

    pub async fn list_interventions_active(
        &self,
        limit: u32,
    ) -> Result<Vec<InterventionRecord>, StorageError> {
        let limit = limit.min(500) as i64;
        let now_ts = OffsetDateTime::now_utc().unix_timestamp();
        let rows = sqlx::query(
            r#"SELECT id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json
               FROM interventions WHERE state = 'active' OR (state = 'snoozed' AND (snoozed_until IS NULL OR snoozed_until > ?))
               ORDER BY surfaced_at DESC LIMIT ?"#,
        )
        .bind(now_ts)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_intervention_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_interventions_by_message(
        &self,
        message_id: &str,
    ) -> Result<Vec<InterventionRecord>, StorageError> {
        let rows = sqlx::query(
            r#"SELECT id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json
               FROM interventions WHERE message_id = ? ORDER BY surfaced_at DESC"#,
        )
        .bind(message_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_intervention_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_interventions_by_conversation(
        &self,
        conversation_id: &str,
    ) -> Result<Vec<InterventionRecord>, StorageError> {
        let rows = sqlx::query(
            r#"SELECT i.id, i.message_id, i.kind, i.state, i.surfaced_at, i.resolved_at, i.snoozed_until, i.confidence, i.source_json, i.provenance_json
               FROM interventions i
               JOIN messages m ON m.id = i.message_id
               WHERE m.conversation_id = ?
               ORDER BY i.surfaced_at DESC"#,
        )
        .bind(conversation_id)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_intervention_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_intervention(
        &self,
        id: &str,
    ) -> Result<Option<InterventionRecord>, StorageError> {
        let row = sqlx::query(
            r#"SELECT id, message_id, kind, state, surfaced_at, resolved_at, snoozed_until, confidence, source_json, provenance_json
               FROM interventions WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| map_intervention_row(&r)).transpose()
    }

    pub async fn snooze_intervention(
        &self,
        id: &str,
        snoozed_until_ts: i64,
    ) -> Result<(), StorageError> {
        sqlx::query(
            r#"UPDATE interventions SET state = 'snoozed', snoozed_until = ?, resolved_at = NULL WHERE id = ?"#,
        )
        .bind(snoozed_until_ts)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn resolve_intervention(&self, id: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"UPDATE interventions SET state = 'resolved', resolved_at = ?, snoozed_until = NULL WHERE id = ?"#,
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn dismiss_intervention(&self, id: &str) -> Result<(), StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"UPDATE interventions SET state = 'dismissed', resolved_at = ?, snoozed_until = NULL WHERE id = ?"#,
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // --- Event log (chat) ---

    pub async fn append_event(&self, input: EventLogInsert) -> Result<EventId, StorageError> {
        let id = input
            .id
            .unwrap_or_else(|| format!("evt_{}", Uuid::new_v4().simple()));
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"INSERT INTO event_log (id, event_name, aggregate_type, aggregate_id, payload_json, created_at)
               VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(&input.event_name)
        .bind(input.aggregate_type.as_deref())
        .bind(input.aggregate_id.as_deref())
        .bind(&input.payload_json)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(EventId::from(id))
    }

    pub async fn list_events_recent(
        &self,
        limit: u32,
    ) -> Result<Vec<EventLogRecord>, StorageError> {
        let limit = limit.min(1000) as i64;
        let rows = sqlx::query(
            r#"SELECT id, event_name, aggregate_type, aggregate_id, payload_json, created_at
               FROM event_log ORDER BY created_at DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_event_log_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn list_events_by_aggregate(
        &self,
        aggregate_type: &str,
        aggregate_id: &str,
        limit: u32,
    ) -> Result<Vec<EventLogRecord>, StorageError> {
        let limit = limit.min(500) as i64;
        let rows = sqlx::query(
            r#"SELECT id, event_name, aggregate_type, aggregate_id, payload_json, created_at
               FROM event_log WHERE aggregate_type = ? AND aggregate_id = ? ORDER BY created_at DESC LIMIT ?"#,
        )
        .bind(aggregate_type)
        .bind(aggregate_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| map_event_log_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    // --- Settings (chat/client) ---

    pub async fn get_all_settings(
        &self,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>, StorageError> {
        let rows = sqlx::query_as::<_, (String, String)>("SELECT key, value_json FROM settings")
            .fetch_all(&self.pool)
            .await?;
        let mut out = std::collections::HashMap::new();
        for (k, v) in rows {
            let val: serde_json::Value =
                serde_json::from_str(&v).unwrap_or(serde_json::Value::Null);
            out.insert(k, val);
        }
        Ok(out)
    }

    pub async fn set_setting(
        &self,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<(), StorageError> {
        let json =
            serde_json::to_string(value).map_err(|e| StorageError::Validation(e.to_string()))?;
        sqlx::query("INSERT INTO settings (key, value_json) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json")
            .bind(key)
            .bind(&json)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn claim_due_loop(
        &self,
        loop_kind: &str,
        interval_seconds: i64,
        now_ts: i64,
    ) -> Result<bool, StorageError> {
        runtime_loops::claim_due_loop(&self.pool, loop_kind, interval_seconds, now_ts).await
    }

    pub async fn ensure_runtime_loop(
        &self,
        loop_kind: &str,
        enabled: bool,
        interval_seconds: i64,
        next_due_at: Option<i64>,
    ) -> Result<(), StorageError> {
        runtime_loops::ensure_runtime_loop(
            &self.pool,
            loop_kind,
            enabled,
            interval_seconds,
            next_due_at,
        )
        .await
    }

    pub async fn complete_loop(
        &self,
        loop_kind: &str,
        status: &str,
        error: Option<&str>,
        next_due_at: i64,
    ) -> Result<(), StorageError> {
        runtime_loops::complete_loop(&self.pool, loop_kind, status, error, next_due_at).await
    }

    pub async fn list_runtime_loops(&self) -> Result<Vec<RuntimeLoopRecord>, StorageError> {
        runtime_loops::list_runtime_loops(&self.pool).await
    }

    pub async fn insert_work_assignment(
        &self,
        assignment: WorkAssignmentInsert,
    ) -> Result<String, StorageError> {
        let receipt_id = assignment
            .receipt_id
            .unwrap_or_else(|| Uuid::new_v4().simple().to_string());
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            INSERT INTO work_assignment_receipts (
                receipt_id,
                work_request_id,
                worker_id,
                worker_class,
                capability,
                status,
                assigned_at,
                last_updated
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&receipt_id)
        .bind(&assignment.work_request_id)
        .bind(&assignment.worker_id)
        .bind(&assignment.worker_class)
        .bind(&assignment.capability)
        .bind(assignment.status.to_string())
        .bind(assignment.assigned_at)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(receipt_id)
    }

    pub async fn update_work_assignment(
        &self,
        update: WorkAssignmentUpdate,
    ) -> Result<WorkAssignmentRecord, StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        sqlx::query(
            r#"
            UPDATE work_assignment_receipts
            SET status = ?,
                started_at = ?,
                completed_at = ?,
                result = ?,
                error_message = ?,
                last_updated = ?
            WHERE receipt_id = ?
            "#,
        )
        .bind(update.status.to_string())
        .bind(update.started_at)
        .bind(update.completed_at)
        .bind(update.result)
        .bind(update.error_message)
        .bind(now)
        .bind(&update.receipt_id)
        .execute(&self.pool)
        .await?;

        let row = sqlx::query(
            r#"
            SELECT
                receipt_id,
                work_request_id,
                worker_id,
                worker_class,
                capability,
                status,
                assigned_at,
                started_at,
                completed_at,
                result,
                error_message,
                last_updated
            FROM work_assignment_receipts
            WHERE receipt_id = ?
            "#,
        )
        .bind(&update.receipt_id)
        .fetch_one(&self.pool)
        .await?;

        map_work_assignment_row(&row)
    }

    pub async fn list_work_assignments(
        &self,
        work_request_id: Option<&str>,
        worker_id: Option<&str>,
    ) -> Result<Vec<WorkAssignmentRecord>, StorageError> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT
                receipt_id,
                work_request_id,
                worker_id,
                worker_class,
                capability,
                status,
                assigned_at,
                started_at,
                completed_at,
                result,
                error_message,
                last_updated
            FROM work_assignment_receipts
            "#,
        );
        query.push("WHERE 1=1");
        if work_request_id.is_some() {
            query.push(" AND work_request_id = ");
            query.push_bind(work_request_id);
        }
        if worker_id.is_some() {
            query.push(" AND worker_id = ");
            query.push_bind(worker_id);
        }
        query.push(" ORDER BY last_updated DESC");

        let rows = query.build().fetch_all(&self.pool).await?;
        rows.into_iter()
            .map(|row| map_work_assignment_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn upsert_cluster_worker(
        &self,
        worker: ClusterWorkerUpsert,
    ) -> Result<(), StorageError> {
        let worker_classes_json = serde_json::to_string(&worker.worker_classes)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        let capabilities_json = serde_json::to_string(&worker.capabilities)
            .map_err(|error| StorageError::Validation(error.to_string()))?;
        let now = OffsetDateTime::now_utc().unix_timestamp();

        sqlx::query(
            r#"
            INSERT INTO cluster_workers (
                worker_id,
                node_id,
                node_display_name,
                worker_class,
                worker_classes_json,
                capabilities_json,
                status,
                max_concurrency,
                current_load,
                queue_depth,
                reachability,
                latency_class,
                compute_class,
                power_class,
                recent_failure_rate,
                tailscale_preferred,
                sync_base_url,
                sync_transport,
                tailscale_base_url,
                preferred_tailnet_endpoint,
                tailscale_reachable,
                lan_base_url,
                localhost_base_url,
                last_heartbeat_at,
                started_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(worker_id) DO UPDATE SET
                node_id = excluded.node_id,
                node_display_name = excluded.node_display_name,
                worker_class = excluded.worker_class,
                worker_classes_json = excluded.worker_classes_json,
                capabilities_json = excluded.capabilities_json,
                status = excluded.status,
                max_concurrency = excluded.max_concurrency,
                current_load = excluded.current_load,
                queue_depth = excluded.queue_depth,
                reachability = excluded.reachability,
                latency_class = excluded.latency_class,
                compute_class = excluded.compute_class,
                power_class = excluded.power_class,
                recent_failure_rate = excluded.recent_failure_rate,
                tailscale_preferred = excluded.tailscale_preferred,
                sync_base_url = excluded.sync_base_url,
                sync_transport = excluded.sync_transport,
                tailscale_base_url = excluded.tailscale_base_url,
                preferred_tailnet_endpoint = excluded.preferred_tailnet_endpoint,
                tailscale_reachable = excluded.tailscale_reachable,
                lan_base_url = excluded.lan_base_url,
                localhost_base_url = excluded.localhost_base_url,
                last_heartbeat_at = excluded.last_heartbeat_at,
                started_at = excluded.started_at,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&worker.worker_id)
        .bind(&worker.node_id)
        .bind(&worker.node_display_name)
        .bind(&worker.worker_class)
        .bind(worker_classes_json)
        .bind(capabilities_json)
        .bind(&worker.status)
        .bind(worker.max_concurrency.map(i64::from))
        .bind(worker.current_load.map(i64::from))
        .bind(worker.queue_depth.map(i64::from))
        .bind(&worker.reachability)
        .bind(&worker.latency_class)
        .bind(&worker.compute_class)
        .bind(&worker.power_class)
        .bind(worker.recent_failure_rate)
        .bind(if worker.tailscale_preferred {
            1_i64
        } else {
            0_i64
        })
        .bind(&worker.sync_base_url)
        .bind(&worker.sync_transport)
        .bind(&worker.tailscale_base_url)
        .bind(&worker.preferred_tailnet_endpoint)
        .bind(if worker.tailscale_reachable {
            1_i64
        } else {
            0_i64
        })
        .bind(&worker.lan_base_url)
        .bind(&worker.localhost_base_url)
        .bind(worker.last_heartbeat_at)
        .bind(worker.started_at)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn expire_cluster_workers(&self, stale_before: i64) -> Result<u64, StorageError> {
        let result = sqlx::query("DELETE FROM cluster_workers WHERE last_heartbeat_at < ?")
            .bind(stale_before)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_cluster_workers(&self) -> Result<Vec<ClusterWorkerRecord>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT
                worker_id,
                node_id,
                node_display_name,
                worker_class,
                worker_classes_json,
                capabilities_json,
                status,
                max_concurrency,
                current_load,
                queue_depth,
                reachability,
                latency_class,
                compute_class,
                power_class,
                recent_failure_rate,
                tailscale_preferred,
                sync_base_url,
                sync_transport,
                tailscale_base_url,
                preferred_tailnet_endpoint,
                tailscale_reachable,
                lan_base_url,
                localhost_base_url,
                last_heartbeat_at,
                started_at,
                updated_at
            FROM cluster_workers
            ORDER BY node_id ASC, worker_id ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| map_cluster_worker_row(&row))
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn get_runtime_loop(
        &self,
        loop_kind: &str,
    ) -> Result<Option<RuntimeLoopRecord>, StorageError> {
        runtime_loops::get_runtime_loop(&self.pool, loop_kind).await
    }

    pub async fn update_runtime_loop_config(
        &self,
        loop_kind: &str,
        enabled: Option<bool>,
        interval_seconds: Option<i64>,
    ) -> Result<Option<RuntimeLoopRecord>, StorageError> {
        runtime_loops::update_runtime_loop_config(&self.pool, loop_kind, enabled, interval_seconds)
            .await
    }

    pub async fn orientation_snapshot(&self) -> Result<OrientationSnapshot, StorageError> {
        let now = OffsetDateTime::now_utc();
        let start_of_day = now
            .date()
            .with_hms(0, 0, 0)
            .map_err(|error| StorageError::InvalidTimestamp(error.to_string()))?
            .assume_utc()
            .unix_timestamp();
        let seven_days_ago = now - time::Duration::days(7);

        let recent_today = sqlx::query(
            r#"
            SELECT capture_id, capture_type, content_text, occurred_at, source_device
            FROM captures
            WHERE occurred_at >= ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT 10
            "#,
        )
        .bind(start_of_day)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(map_context_capture_row)
        .collect::<Result<Vec<_>, _>>()?;

        let recent_week = sqlx::query(
            r#"
            SELECT capture_id, capture_type, content_text, occurred_at, source_device
            FROM captures
            WHERE occurred_at >= ?
            ORDER BY occurred_at DESC, created_at DESC
            LIMIT 50
            "#,
        )
        .bind(seven_days_ago.unix_timestamp())
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(map_context_capture_row)
        .collect::<Result<Vec<_>, _>>()?;

        Ok(OrientationSnapshot {
            recent_today,
            recent_week,
        })
    }
}

fn map_search_row(row: sqlx::sqlite::SqliteRow) -> Result<SearchResult, StorageError> {
    let occurred_at = timestamp_to_datetime(row.try_get("occurred_at")?)?;
    let created_at = timestamp_to_datetime(row.try_get("created_at")?)?;

    Ok(SearchResult {
        capture_id: CaptureId::from(row.try_get::<String, _>("capture_id")?),
        capture_type: row.try_get("capture_type")?,
        snippet: row.try_get("snippet")?,
        occurred_at,
        created_at,
        source_device: row.try_get("source_device")?,
    })
}

fn map_context_capture_row(row: sqlx::sqlite::SqliteRow) -> Result<ContextCapture, StorageError> {
    Ok(ContextCapture {
        capture_id: CaptureId::from(row.try_get::<String, _>("capture_id")?),
        capture_type: row.try_get("capture_type")?,
        content_text: row.try_get("content_text")?,
        occurred_at: timestamp_to_datetime(row.try_get("occurred_at")?)?,
        source_device: row.try_get("source_device")?,
    })
}

fn map_conversation_row(row: &sqlx::sqlite::SqliteRow) -> Result<ConversationRecord, StorageError> {
    let pinned: i32 = row.try_get("pinned")?;
    let archived: i32 = row.try_get("archived")?;
    Ok(ConversationRecord {
        id: ConversationId::from(row.try_get::<String, _>("id")?),
        title: row.try_get("title")?,
        kind: row.try_get("kind")?,
        pinned: pinned != 0,
        archived: archived != 0,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_message_row(row: &sqlx::sqlite::SqliteRow) -> Result<MessageRecord, StorageError> {
    Ok(MessageRecord {
        id: MessageId::from(row.try_get::<String, _>("id")?),
        conversation_id: ConversationId::from(row.try_get::<String, _>("conversation_id")?),
        role: row.try_get("role")?,
        kind: row.try_get("kind")?,
        content_json: row.try_get("content_json")?,
        status: row.try_get("status")?,
        importance: row.try_get("importance")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_intervention_row(row: &sqlx::sqlite::SqliteRow) -> Result<InterventionRecord, StorageError> {
    Ok(InterventionRecord {
        id: InterventionId::from(row.try_get::<String, _>("id")?),
        message_id: MessageId::from(row.try_get::<String, _>("message_id")?),
        kind: row.try_get("kind")?,
        state: row.try_get("state")?,
        surfaced_at: row.try_get("surfaced_at")?,
        resolved_at: row.try_get("resolved_at")?,
        snoozed_until: row.try_get("snoozed_until")?,
        confidence: row.try_get("confidence")?,
        source_json: row.try_get("source_json")?,
        provenance_json: row.try_get("provenance_json")?,
    })
}

fn map_event_log_row(row: &sqlx::sqlite::SqliteRow) -> Result<EventLogRecord, StorageError> {
    Ok(EventLogRecord {
        id: EventId::from(row.try_get::<String, _>("id")?),
        event_name: row.try_get("event_name")?,
        aggregate_type: row.try_get("aggregate_type")?,
        aggregate_id: row.try_get("aggregate_id")?,
        payload_json: row.try_get("payload_json")?,
        created_at: row.try_get("created_at")?,
    })
}

fn map_commitment_row(row: &sqlx::sqlite::SqliteRow) -> Result<Commitment, StorageError> {
    let status: String = row.try_get("status")?;
    let created_at: i64 = row.try_get("created_at")?;
    let metadata_str: String = row.try_get("metadata_json")?;
    let metadata_json = serde_json::from_str(&metadata_str).unwrap_or_else(|_| json!({}));
    Ok(Commitment {
        id: CommitmentId::from(row.try_get::<String, _>("id")?),
        text: row.try_get("text")?,
        source_type: row.try_get("source_type")?,
        source_id: row.try_get("source_id")?,
        status: status
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        due_at: row
            .try_get::<Option<i64>, _>("due_at")?
            .and_then(|t| timestamp_to_datetime(t).ok()),
        project: row.try_get("project")?,
        commitment_kind: row.try_get("commitment_kind")?,
        created_at: timestamp_to_datetime(created_at)?,
        resolved_at: row
            .try_get::<Option<i64>, _>("resolved_at")?
            .and_then(|t| timestamp_to_datetime(t).ok()),
        metadata_json,
    })
}

fn map_signal_row(row: &sqlx::sqlite::SqliteRow) -> Result<SignalRecord, StorageError> {
    let payload_str: String = row.try_get("payload_json")?;
    Ok(SignalRecord {
        signal_id: row.try_get("signal_id")?,
        signal_type: row.try_get("signal_type")?,
        source: row.try_get("source")?,
        source_ref: row.try_get("source_ref")?,
        timestamp: row.try_get("timestamp")?,
        payload_json: serde_json::from_str(&payload_str).unwrap_or_else(|_| json!({})),
        created_at: row.try_get("created_at")?,
    })
}

fn map_inferred_state_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<InferredStateRecord, StorageError> {
    let context_str: String = row.try_get("context_json")?;
    Ok(InferredStateRecord {
        state_id: row.try_get("state_id")?,
        state_name: row.try_get("state_name")?,
        confidence: row.try_get("confidence")?,
        timestamp: row.try_get("timestamp")?,
        context_json: serde_json::from_str(&context_str).unwrap_or_else(|_| json!({})),
        created_at: row.try_get("created_at")?,
    })
}

fn map_nudge_row(row: &sqlx::sqlite::SqliteRow) -> Result<NudgeRecord, StorageError> {
    let meta_str: String = row.try_get("metadata_json")?;
    Ok(NudgeRecord {
        nudge_id: row.try_get("nudge_id")?,
        nudge_type: row.try_get("nudge_type")?,
        level: row.try_get("level")?,
        state: row.try_get("state")?,
        related_commitment_id: row.try_get("related_commitment_id")?,
        message: row.try_get("message")?,
        created_at: row.try_get("created_at")?,
        snoozed_until: row.try_get("snoozed_until")?,
        resolved_at: row.try_get("resolved_at")?,
        signals_snapshot_json: row.try_get("signals_snapshot_json")?,
        inference_snapshot_json: row.try_get("inference_snapshot_json")?,
        metadata_json: serde_json::from_str(&meta_str).unwrap_or_else(|_| json!({})),
    })
}

fn map_nudge_event_row(row: &sqlx::sqlite::SqliteRow) -> Result<NudgeEventRecord, StorageError> {
    let payload_json = row.try_get::<String, _>("payload_json")?;
    Ok(NudgeEventRecord {
        id: row.try_get("id")?,
        nudge_id: row.try_get("nudge_id")?,
        event_type: row.try_get("event_type")?,
        payload_json: serde_json::from_str(&payload_json)
            .unwrap_or(JsonValue::Object(Default::default())),
        timestamp: row.try_get("timestamp")?,
        created_at: row.try_get("created_at")?,
    })
}

fn map_suggestion_row(row: &sqlx::sqlite::SqliteRow) -> Result<SuggestionRecord, StorageError> {
    let payload_json = row.try_get::<String, _>("payload_json")?;
    let decision_context_json = row.try_get::<Option<String>, _>("decision_context_json")?;
    let evidence_count = row.try_get::<i64, _>("evidence_count")?;
    Ok(SuggestionRecord {
        id: row.try_get("id")?,
        suggestion_type: row.try_get("suggestion_type")?,
        state: row.try_get("state")?,
        title: row.try_get("title")?,
        summary: row.try_get("summary")?,
        priority: row.try_get("priority")?,
        confidence: row.try_get("confidence")?,
        dedupe_key: row.try_get("dedupe_key")?,
        payload_json: parse_json_value(&payload_json)?,
        decision_context_json: decision_context_json
            .as_deref()
            .map(parse_json_value)
            .transpose()?,
        created_at: row.try_get("created_at")?,
        resolved_at: row.try_get("resolved_at")?,
        evidence_count: evidence_count.max(0) as u32,
    })
}

fn map_suggestion_evidence_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<SuggestionEvidenceRecord, StorageError> {
    let evidence_json = row.try_get::<Option<String>, _>("evidence_json")?;
    Ok(SuggestionEvidenceRecord {
        id: row.try_get("id")?,
        suggestion_id: row.try_get("suggestion_id")?,
        evidence_type: row.try_get("evidence_type")?,
        ref_id: row.try_get("ref_id")?,
        evidence_json: evidence_json.as_deref().map(parse_json_value).transpose()?,
        weight: row.try_get("weight")?,
        created_at: row.try_get("created_at")?,
    })
}

fn map_suggestion_feedback_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<SuggestionFeedbackRecord, StorageError> {
    let payload_json = row.try_get::<Option<String>, _>("payload_json")?;
    Ok(SuggestionFeedbackRecord {
        id: row.try_get("id")?,
        suggestion_id: row.try_get("suggestion_id")?,
        outcome_type: row.try_get("outcome_type")?,
        notes: row.try_get("notes")?,
        observed_at: row.try_get("observed_at")?,
        payload_json: payload_json.as_deref().map(parse_json_value).transpose()?,
        created_at: row.try_get("created_at")?,
    })
}

fn map_integration_connection_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<IntegrationConnection, StorageError> {
    let family = row.try_get::<String, _>("family")?;
    let provider_key = row.try_get::<String, _>("provider_key")?;
    let status = row.try_get::<String, _>("status")?;
    let metadata_json = row.try_get::<String, _>("metadata_json")?;

    Ok(IntegrationConnection {
        id: IntegrationConnectionId::from(row.try_get::<String, _>("id")?),
        provider: IntegrationProvider::new(
            family.parse().map_err(|error: vel_core::VelCoreError| {
                StorageError::Validation(error.to_string())
            })?,
            provider_key,
        )
        .map_err(|error| StorageError::Validation(error.to_string()))?,
        status: status
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        display_name: row.try_get("display_name")?,
        account_ref: row.try_get("account_ref")?,
        metadata_json: parse_json_value(&metadata_json)?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        updated_at: timestamp_to_datetime(row.try_get("updated_at")?)?,
    })
}

fn map_integration_connection_setting_ref_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<IntegrationConnectionSettingRef, StorageError> {
    Ok(IntegrationConnectionSettingRef {
        connection_id: IntegrationConnectionId::from(row.try_get::<String, _>("connection_id")?),
        setting_key: row.try_get("setting_key")?,
        setting_value: row.try_get("setting_value")?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

fn map_integration_connection_event_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<IntegrationConnectionEvent, StorageError> {
    let event_type = row.try_get::<String, _>("event_type")?;
    let payload_json = row.try_get::<String, _>("payload_json")?;

    Ok(IntegrationConnectionEvent {
        id: row.try_get("id")?,
        connection_id: IntegrationConnectionId::from(row.try_get::<String, _>("connection_id")?),
        event_type: event_type
            .parse()
            .map_err(|error: vel_core::VelCoreError| StorageError::Validation(error.to_string()))?,
        payload_json: parse_json_value(&payload_json)?,
        timestamp: timestamp_to_datetime(row.try_get("timestamp")?)?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

fn map_uncertainty_row(row: &sqlx::sqlite::SqliteRow) -> Result<UncertaintyRecord, StorageError> {
    let reasons_json = row.try_get::<String, _>("reasons_json")?;
    let missing_evidence_json = row.try_get::<Option<String>, _>("missing_evidence_json")?;
    Ok(UncertaintyRecord {
        id: row.try_get("id")?,
        subject_type: row.try_get("subject_type")?,
        subject_id: row.try_get("subject_id")?,
        decision_kind: row.try_get("decision_kind")?,
        confidence_band: row.try_get("confidence_band")?,
        confidence_score: row.try_get("confidence_score")?,
        reasons_json: parse_json_value(&reasons_json)?,
        missing_evidence_json: missing_evidence_json
            .as_deref()
            .map(parse_json_value)
            .transpose()?,
        resolution_mode: row.try_get("resolution_mode")?,
        status: row.try_get("status")?,
        created_at: row.try_get("created_at")?,
        resolved_at: row.try_get("resolved_at")?,
    })
}

fn map_cluster_worker_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<ClusterWorkerRecord, StorageError> {
    let worker_classes_json: String = row.try_get("worker_classes_json")?;
    let capabilities_json: String = row.try_get("capabilities_json")?;
    let tailscale_preferred: i64 = row.try_get("tailscale_preferred")?;
    let tailscale_reachable: i64 = row.try_get("tailscale_reachable")?;

    Ok(ClusterWorkerRecord {
        worker_id: row.try_get("worker_id")?,
        node_id: row.try_get("node_id")?,
        node_display_name: row.try_get("node_display_name")?,
        worker_class: row.try_get("worker_class")?,
        worker_classes: serde_json::from_str(&worker_classes_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?,
        capabilities: serde_json::from_str(&capabilities_json)
            .map_err(|error| StorageError::Validation(error.to_string()))?,
        status: row.try_get("status")?,
        max_concurrency: row
            .try_get::<Option<i64>, _>("max_concurrency")?
            .map(|value| value.max(0) as u32),
        current_load: row
            .try_get::<Option<i64>, _>("current_load")?
            .map(|value| value.max(0) as u32),
        queue_depth: row
            .try_get::<Option<i64>, _>("queue_depth")?
            .map(|value| value.max(0) as u32),
        reachability: row.try_get("reachability")?,
        latency_class: row.try_get("latency_class")?,
        compute_class: row.try_get("compute_class")?,
        power_class: row.try_get("power_class")?,
        recent_failure_rate: row.try_get("recent_failure_rate")?,
        tailscale_preferred: tailscale_preferred != 0,
        sync_base_url: row.try_get("sync_base_url")?,
        sync_transport: row.try_get("sync_transport")?,
        tailscale_base_url: row.try_get("tailscale_base_url")?,
        preferred_tailnet_endpoint: row.try_get("preferred_tailnet_endpoint")?,
        tailscale_reachable: tailscale_reachable != 0,
        lan_base_url: row.try_get("lan_base_url")?,
        localhost_base_url: row.try_get("localhost_base_url")?,
        last_heartbeat_at: row.try_get("last_heartbeat_at")?,
        started_at: row.try_get("started_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn map_run_row(row: &sqlx::sqlite::SqliteRow) -> Result<Run, StorageError> {
    let kind: String = row.try_get("run_kind")?;
    let status: String = row.try_get("status")?;
    let input_str: String = row.try_get("input_json")?;
    let output_str: Option<String> = row.try_get("output_json")?;
    let error_str: Option<String> = row.try_get("error_json")?;
    Ok(Run {
        id: RunId::from(row.try_get::<String, _>("run_id")?),
        kind: kind
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        status: status
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        input_json: parse_json_value(&input_str)?,
        output_json: output_str.as_deref().map(parse_json_value).transpose()?,
        error_json: error_str.as_deref().map(parse_json_value).transpose()?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
        started_at: row
            .try_get::<Option<i64>, _>("started_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
        finished_at: row
            .try_get::<Option<i64>, _>("finished_at")?
            .map(timestamp_to_datetime)
            .transpose()?,
    })
}

fn map_work_assignment_row(
    row: &sqlx::sqlite::SqliteRow,
) -> Result<WorkAssignmentRecord, StorageError> {
    let status_str: String = row.try_get("status")?;
    let status = status_str.parse().map_err(|e: vel_core::VelCoreError| {
        StorageError::Validation(format!("invalid work assignment status: {e}"))
    })?;

    Ok(WorkAssignmentRecord {
        receipt_id: row.try_get("receipt_id")?,
        work_request_id: row.try_get("work_request_id")?,
        worker_id: row.try_get("worker_id")?,
        worker_class: row.try_get("worker_class")?,
        capability: row.try_get("capability")?,
        status,
        assigned_at: row.try_get("assigned_at")?,
        started_at: row.try_get("started_at")?,
        completed_at: row.try_get("completed_at")?,
        result: row.try_get("result")?,
        error_message: row.try_get("error_message")?,
        last_updated: row.try_get("last_updated")?,
    })
}

fn map_run_event_row(row: sqlx::sqlite::SqliteRow) -> Result<RunEvent, StorageError> {
    let event_type: String = row.try_get("event_type")?;
    let payload_str: String = row.try_get("payload_json")?;
    Ok(RunEvent {
        id: row.try_get("event_id")?,
        run_id: RunId::from(row.try_get::<String, _>("run_id")?),
        seq: row.try_get::<i64, _>("seq")? as u32,
        event_type: event_type
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        payload_json: parse_json_value(&payload_str)?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

fn map_ref_row(row: sqlx::sqlite::SqliteRow) -> Result<Ref, StorageError> {
    let relation_type: String = row.try_get("relation_type")?;
    Ok(Ref {
        id: row.try_get("ref_id")?,
        from_type: row.try_get("from_type")?,
        from_id: row.try_get("from_id")?,
        to_type: row.try_get("to_type")?,
        to_id: row.try_get("to_id")?,
        relation_type: relation_type
            .parse()
            .map_err(|e: vel_core::VelCoreError| StorageError::Validation(e.to_string()))?,
        created_at: timestamp_to_datetime(row.try_get("created_at")?)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migrations_apply_and_capture_inserts() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_capture(CaptureInsert {
                content_text: "remember lidar budget".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("test".to_string()),
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();

        assert_eq!(storage.capture_count().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn search_returns_matching_captures_in_relevance_then_recency_order() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        storage
            .insert_capture(CaptureInsert {
                content_text: "lidar budget estimate for q3".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("phone".to_string()),
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();
        storage
            .insert_capture(CaptureInsert {
                content_text: "budget notes that mention lidar twice lidar".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("phone".to_string()),
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();
        storage
            .insert_capture(CaptureInsert {
                content_text: "completely unrelated note".to_string(),
                capture_type: "journal".to_string(),
                source_device: Some("desktop".to_string()),
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();

        let results = storage
            .search_captures(
                "lidar",
                SearchFilters {
                    capture_type: Some("quick_note".to_string()),
                    source_device: Some("phone".to_string()),
                    limit: Some(10),
                },
            )
            .await
            .unwrap();

        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|result| result.capture_type == "quick_note"));
        assert!(results
            .iter()
            .all(|result| result.source_device.as_deref() == Some("phone")));
        assert!(results[0].snippet.to_lowercase().contains("lidar"));
        assert!(results[1].snippet.to_lowercase().contains("lidar"));
    }

    #[tokio::test]
    async fn claim_and_complete_capture_ingest_job() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let _capture_id = storage
            .insert_capture(CaptureInsert {
                content_text: "test note".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: None,
                privacy_class: PrivacyClass::Private,
            })
            .await
            .unwrap();

        let job = storage
            .claim_next_pending_job("capture_ingest")
            .await
            .unwrap();
        let job = job.expect("one pending job");
        assert_eq!(job.job_type, "capture_ingest");
        assert!(job.payload_json.contains("capture_id"));

        storage
            .mark_job_succeeded(&job.job_id.to_string())
            .await
            .unwrap();

        let again = storage
            .claim_next_pending_job("capture_ingest")
            .await
            .unwrap();
        assert!(again.is_none());
    }

    #[tokio::test]
    async fn claim_due_loop_prevents_overlap_and_tracks_next_due_time() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        assert!(storage
            .claim_due_loop("retry_due_runs", 30, 100)
            .await
            .unwrap());
        assert!(!storage
            .claim_due_loop("retry_due_runs", 30, 100)
            .await
            .unwrap());

        storage
            .complete_loop("retry_due_runs", "succeeded", None, 130)
            .await
            .unwrap();

        assert!(!storage
            .claim_due_loop("retry_due_runs", 30, 129)
            .await
            .unwrap());
        assert!(storage
            .claim_due_loop("retry_due_runs", 30, 130)
            .await
            .unwrap());

        let loops = storage.list_runtime_loops().await.unwrap();
        assert_eq!(loops.len(), 1);
        let retry_loop = &loops[0];
        assert_eq!(retry_loop.loop_kind, "retry_due_runs");
        assert_eq!(retry_loop.interval_seconds, 30);
        assert_eq!(retry_loop.last_status.as_deref(), Some("running"));
        assert_eq!(retry_loop.next_due_at, Some(130));
    }

    #[tokio::test]
    async fn complete_loop_persists_terminal_status_and_error() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        assert!(storage
            .claim_due_loop("capture_ingest", 5, 200)
            .await
            .unwrap());
        storage
            .complete_loop("capture_ingest", "failed", Some("boom"), 205)
            .await
            .unwrap();

        let loops = storage.list_runtime_loops().await.unwrap();
        assert_eq!(loops.len(), 1);
        let capture_loop = &loops[0];
        assert_eq!(capture_loop.loop_kind, "capture_ingest");
        assert_eq!(capture_loop.last_status.as_deref(), Some("failed"));
        assert_eq!(capture_loop.last_error.as_deref(), Some("boom"));
        assert_eq!(capture_loop.next_due_at, Some(205));
        assert!(capture_loop.last_started_at.is_some());
        assert!(capture_loop.last_finished_at.is_some());
    }

    #[tokio::test]
    async fn upsert_list_and_expire_cluster_workers() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        storage
            .upsert_cluster_worker(ClusterWorkerUpsert {
                worker_id: "worker-1".to_string(),
                node_id: "node-1".to_string(),
                node_display_name: Some("Node One".to_string()),
                worker_class: Some("validation".to_string()),
                worker_classes: vec!["validation".to_string()],
                capabilities: vec!["build_test_profiles".to_string()],
                status: Some("ready".to_string()),
                max_concurrency: Some(4),
                current_load: Some(1),
                queue_depth: Some(2),
                reachability: Some("reachable".to_string()),
                latency_class: Some("low".to_string()),
                compute_class: Some("standard".to_string()),
                power_class: Some("ac_or_unknown".to_string()),
                recent_failure_rate: Some(0.1),
                tailscale_preferred: true,
                sync_base_url: Some("http://node-1.tailnet.ts.net:4130".to_string()),
                sync_transport: Some("tailscale".to_string()),
                tailscale_base_url: Some("http://node-1.tailnet.ts.net:4130".to_string()),
                preferred_tailnet_endpoint: Some("http://node-1.tailnet.ts.net:4130".to_string()),
                tailscale_reachable: true,
                lan_base_url: Some("http://192.168.1.10:4130".to_string()),
                localhost_base_url: None,
                last_heartbeat_at: 100,
                started_at: Some(90),
            })
            .await
            .unwrap();

        let workers = storage.list_cluster_workers().await.unwrap();
        assert_eq!(workers.len(), 1);
        assert_eq!(workers[0].worker_id, "worker-1");
        assert_eq!(workers[0].node_id, "node-1");
        assert_eq!(workers[0].worker_classes, vec!["validation"]);
        assert_eq!(workers[0].capabilities, vec!["build_test_profiles"]);

        let expired = storage.expire_cluster_workers(101).await.unwrap();
        assert_eq!(expired, 1);
        assert!(storage.list_cluster_workers().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn work_assignment_lifecycle_inserts_and_updates() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let receipt_id = storage
            .insert_work_assignment(WorkAssignmentInsert {
                receipt_id: None,
                work_request_id: "wrkreq-1".to_string(),
                worker_id: "worker-1".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
                status: WorkAssignmentStatus::Assigned,
                assigned_at: 100,
            })
            .await
            .unwrap();
        assert!(!receipt_id.is_empty());

        let assignments = storage
            .list_work_assignments(Some("wrkreq-1"), None)
            .await
            .unwrap();
        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments[0].status, WorkAssignmentStatus::Assigned);

        let updated = storage
            .update_work_assignment(WorkAssignmentUpdate {
                receipt_id: receipt_id.clone(),
                status: WorkAssignmentStatus::Started,
                started_at: Some(110),
                completed_at: None,
                result: None,
                error_message: None,
            })
            .await
            .unwrap();
        assert_eq!(updated.status, WorkAssignmentStatus::Started);
        assert_eq!(updated.started_at, Some(110));

        let listed = storage
            .list_work_assignments(None, Some("worker-1"))
            .await
            .unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].receipt_id, receipt_id);
    }

    #[tokio::test]
    async fn create_and_get_artifact() {
        use vel_core::SyncClass;

        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let id = storage
            .create_artifact(ArtifactInsert {
                artifact_type: "transcript".to_string(),
                title: Some("Meeting notes".to_string()),
                mime_type: Some("text/plain".to_string()),
                storage_uri: "file:///var/artifacts/transcripts/abc.txt".to_string(),
                storage_kind: ArtifactStorageKind::External,
                privacy_class: PrivacyClass::Private,
                sync_class: SyncClass::Warm,
                content_hash: Some("sha256:abc".to_string()),
                size_bytes: None,
                metadata_json: None,
            })
            .await
            .unwrap();

        let record = storage.get_artifact_by_id(&id.to_string()).await.unwrap();
        let record = record.expect("artifact should exist");
        assert_eq!(record.artifact_id.to_string(), id.to_string());
        assert_eq!(record.artifact_type, "transcript");
        assert_eq!(record.title.as_deref(), Some("Meeting notes"));
        assert_eq!(
            record.storage_uri,
            "file:///var/artifacts/transcripts/abc.txt"
        );
        assert_eq!(record.content_hash.as_deref(), Some("sha256:abc"));

        let missing = storage.get_artifact_by_id("art_nonexistent").await.unwrap();
        assert!(missing.is_none());
    }

    #[tokio::test]
    async fn create_run_list_runs_get_run_and_events() {
        use vel_core::{RefRelationType, RunKind};

        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                RunKind::ContextGeneration,
                &json!({"context_kind":"today"}),
            )
            .await
            .unwrap();

        let runs = storage.list_runs(10, None, None).await.unwrap();
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].id.to_string(), run_id.to_string());
        assert_eq!(runs[0].status, vel_core::RunStatus::Queued);

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.kind, RunKind::ContextGeneration);

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type.to_string(), "run_created");

        let ref_ = Ref::new(
            "run",
            run_id.as_ref(),
            "artifact",
            "art_1",
            RefRelationType::AttachedTo,
        );
        storage.create_ref(&ref_).await.unwrap();
        let from_refs = storage
            .list_refs_from("run", run_id.as_ref())
            .await
            .unwrap();
        assert_eq!(from_refs.len(), 1);
        assert_eq!(from_refs[0].to_id, "art_1");
    }

    #[tokio::test]
    async fn reset_and_list_retry_ready_runs() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::ContextGeneration,
                &json!({"context_kind":"today"}),
            )
            .await
            .unwrap();

        storage
            .update_run_status(
                run_id.as_ref(),
                vel_core::RunStatus::Failed,
                Some(100),
                Some(120),
                None,
                Some(&json!({"message":"boom"})),
            )
            .await
            .unwrap();
        storage
            .append_run_event_auto(
                run_id.as_ref(),
                vel_core::RunEventType::RunRetryScheduled,
                &json!({"retry_at": 200, "reason": "transient_failure"}),
            )
            .await
            .unwrap();
        storage
            .update_run_status(
                run_id.as_ref(),
                vel_core::RunStatus::RetryScheduled,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        let too_early = storage.list_retry_ready_runs(199, 10).await.unwrap();
        assert!(too_early.is_empty());

        let ready = storage.list_retry_ready_runs(200, 10).await.unwrap();
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].run.id, run_id);
        assert_eq!(ready[0].retry_at, 200);
        assert_eq!(ready[0].retry_reason.as_deref(), Some("transient_failure"));

        storage.reset_run_for_retry(run_id.as_ref()).await.unwrap();
        let reset = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(reset.status, vel_core::RunStatus::Queued);
        assert!(reset.started_at.is_none());
        assert!(reset.finished_at.is_none());
        assert!(reset.output_json.is_none());
        assert!(reset.error_json.is_none());
    }

    #[tokio::test]
    async fn insert_and_list_suggestion_v2_with_evidence() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let suggestion_id = storage
            .insert_suggestion_v2(SuggestionInsertV2 {
                suggestion_type: "increase_commute_buffer".to_string(),
                state: "pending".to_string(),
                title: Some("Increase commute buffer".to_string()),
                summary: Some("Repeated commute danger nudges detected.".to_string()),
                priority: 70,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("increase_commute_buffer".to_string()),
                payload_json: json!({
                    "type": "increase_commute_buffer",
                    "current_minutes": 20,
                    "suggested_minutes": 30
                }),
                decision_context_json: Some(json!({
                    "summary": "Resolved 2 commute danger nudges in the last 7 days.",
                    "count": 2
                })),
            })
            .await
            .unwrap();

        storage
            .insert_suggestion_evidence(SuggestionEvidenceInsert {
                suggestion_id: suggestion_id.clone(),
                evidence_type: "nudge".to_string(),
                ref_id: "nud_123".to_string(),
                evidence_json: Some(json!({ "level": "danger" })),
                weight: Some(1.0),
            })
            .await
            .unwrap();

        let suggestion = storage
            .get_suggestion_by_id(&suggestion_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(suggestion.title.as_deref(), Some("Increase commute buffer"));
        assert_eq!(suggestion.priority, 70);
        assert_eq!(suggestion.confidence.as_deref(), Some("medium"));
        assert_eq!(suggestion.evidence_count, 1);
        assert_eq!(
            suggestion
                .decision_context_json
                .as_ref()
                .and_then(|json| json.get("summary"))
                .and_then(JsonValue::as_str),
            Some("Resolved 2 commute danger nudges in the last 7 days.")
        );

        let evidence = storage
            .list_suggestion_evidence(&suggestion_id)
            .await
            .unwrap();
        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].evidence_type, "nudge");
        assert_eq!(evidence[0].ref_id, "nud_123");
    }

    #[tokio::test]
    async fn find_recent_suggestion_by_dedupe_key_prefers_latest() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let first_id = storage
            .insert_suggestion_v2(SuggestionInsertV2 {
                suggestion_type: "increase_prep_window".to_string(),
                state: "rejected".to_string(),
                title: Some("Increase prep window".to_string()),
                summary: None,
                priority: 55,
                confidence: Some("low".to_string()),
                dedupe_key: Some("increase_prep_window".to_string()),
                payload_json: json!({ "type": "increase_prep_window" }),
                decision_context_json: None,
            })
            .await
            .unwrap();
        let second_id = storage
            .insert_suggestion_v2(SuggestionInsertV2 {
                suggestion_type: "increase_prep_window".to_string(),
                state: "pending".to_string(),
                title: Some("Increase prep window".to_string()),
                summary: None,
                priority: 60,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("increase_prep_window".to_string()),
                payload_json: json!({ "type": "increase_prep_window", "suggested_minutes": 45 }),
                decision_context_json: None,
            })
            .await
            .unwrap();

        let latest = storage
            .find_recent_suggestion_by_dedupe_key("increase_prep_window")
            .await
            .unwrap()
            .unwrap();
        assert_ne!(latest.id, first_id);
        assert_eq!(latest.id, second_id);
        assert_eq!(latest.state, "pending");
    }

    #[tokio::test]
    async fn suggestion_feedback_round_trips_and_summarizes_by_family() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let suggestion_id = storage
            .insert_suggestion_v2(SuggestionInsertV2 {
                suggestion_type: "increase_commute_buffer".to_string(),
                state: "accepted".to_string(),
                title: Some("Increase commute buffer".to_string()),
                summary: None,
                priority: 70,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("increase_commute_buffer".to_string()),
                payload_json: json!({ "type": "increase_commute_buffer" }),
                decision_context_json: None,
            })
            .await
            .unwrap();

        storage
            .insert_suggestion_feedback(SuggestionFeedbackInsert {
                suggestion_id: suggestion_id.clone(),
                outcome_type: "accepted_and_policy_changed".to_string(),
                notes: Some("accepted from API".to_string()),
                observed_at: 100,
                payload_json: Some(json!({ "source": "test" })),
            })
            .await
            .unwrap();
        storage
            .insert_suggestion_feedback(SuggestionFeedbackInsert {
                suggestion_id: suggestion_id.clone(),
                outcome_type: "rejected_not_useful".to_string(),
                notes: Some("not useful".to_string()),
                observed_at: 150,
                payload_json: None,
            })
            .await
            .unwrap();
        storage
            .insert_suggestion_feedback(SuggestionFeedbackInsert {
                suggestion_id: suggestion_id.clone(),
                outcome_type: "rejected_incorrect".to_string(),
                notes: Some("later found incorrect".to_string()),
                observed_at: 200,
                payload_json: None,
            })
            .await
            .unwrap();

        let feedback = storage
            .list_suggestion_feedback(&suggestion_id)
            .await
            .unwrap();
        assert_eq!(feedback.len(), 3);
        assert_eq!(feedback[0].outcome_type, "rejected_incorrect");
        assert_eq!(feedback[1].outcome_type, "rejected_not_useful");
        assert_eq!(feedback[2].outcome_type, "accepted_and_policy_changed");

        let summary = storage
            .summarize_suggestion_feedback("increase_commute_buffer")
            .await
            .unwrap();
        assert_eq!(summary.accepted_and_policy_changed, 1);
        assert_eq!(summary.rejected_not_useful, 1);
        assert_eq!(summary.rejected_incorrect, 1);
    }

    #[tokio::test]
    async fn uncertainty_records_round_trip_and_resolve() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let id = storage
            .insert_uncertainty_record(UncertaintyRecordInsert {
                subject_type: "suggestion_candidate".to_string(),
                subject_id: Some("increase_commute_buffer".to_string()),
                decision_kind: "suggestion_generation".to_string(),
                confidence_band: "low".to_string(),
                confidence_score: Some(0.42),
                reasons_json: json!({
                    "summary": "Barely enough evidence for a commute-buffer change."
                }),
                missing_evidence_json: Some(json!({
                    "current_count": 2,
                    "threshold": 2,
                    "more_events_needed": 1
                })),
                resolution_mode: "defer".to_string(),
            })
            .await
            .unwrap();

        let open = storage
            .list_uncertainty_records(Some("open"), 10)
            .await
            .unwrap();
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].id, id);
        assert_eq!(open[0].resolution_mode, "defer");

        let existing = storage
            .find_open_uncertainty_record(
                "suggestion_candidate",
                Some("increase_commute_buffer"),
                "suggestion_generation",
            )
            .await
            .unwrap()
            .unwrap();
        assert_eq!(existing.id, id);

        let resolved = storage
            .resolve_uncertainty_record(&id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(resolved.status, "resolved");
        assert!(resolved.resolved_at.is_some());
        assert!(storage
            .list_uncertainty_records(Some("open"), 10)
            .await
            .unwrap()
            .is_empty());
        let recent_resolved = storage
            .find_recent_uncertainty_record(
                "suggestion_candidate",
                Some("increase_commute_buffer"),
                "suggestion_generation",
                "resolved",
                resolved.resolved_at.unwrap() - 1,
            )
            .await
            .unwrap()
            .unwrap();
        assert_eq!(recent_resolved.id, id);
    }

    #[tokio::test]
    async fn integration_connections_round_trip_with_setting_refs() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let provider = IntegrationProvider::new(IntegrationFamily::Messaging, "signal").unwrap();
        let connection_id = storage
            .insert_integration_connection(IntegrationConnectionInsert {
                family: IntegrationFamily::Messaging,
                provider: provider.clone(),
                status: IntegrationConnectionStatus::Pending,
                display_name: "Signal personal".to_string(),
                account_ref: Some("+15555550123".to_string()),
                metadata_json: json!({ "scope": "personal" }),
            })
            .await
            .unwrap();

        storage
            .upsert_integration_connection_setting_ref(
                connection_id.as_ref(),
                "messaging_snapshot_path",
                "/tmp/signal.json",
            )
            .await
            .unwrap();
        storage
            .upsert_integration_connection_setting_ref(
                connection_id.as_ref(),
                "messaging_snapshot_path",
                "/tmp/signal-v2.json",
            )
            .await
            .unwrap();

        let fetched = storage
            .get_integration_connection(connection_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(fetched.id, connection_id);
        assert_eq!(fetched.provider, provider);
        assert_eq!(fetched.status, IntegrationConnectionStatus::Pending);
        assert_eq!(fetched.display_name, "Signal personal");
        assert_eq!(fetched.account_ref.as_deref(), Some("+15555550123"));
        assert_eq!(fetched.metadata_json["scope"], "personal");

        storage
            .update_integration_connection(
                connection_id.as_ref(),
                Some(IntegrationConnectionStatus::Connected),
                Some("Signal primary"),
                Some(Some("signal:primary")),
                Some(&json!({ "scope": "primary" })),
            )
            .await
            .unwrap();

        let listed = storage
            .list_integration_connections(IntegrationConnectionFilters {
                family: Some(IntegrationFamily::Messaging),
                provider_key: Some("signal".to_string()),
                include_disabled: false,
            })
            .await
            .unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].status, IntegrationConnectionStatus::Connected);
        assert_eq!(listed[0].display_name, "Signal primary");
        assert_eq!(listed[0].account_ref.as_deref(), Some("signal:primary"));
        assert_eq!(listed[0].metadata_json["scope"], "primary");

        let refs = storage
            .list_integration_connection_setting_refs(connection_id.as_ref())
            .await
            .unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].setting_key, "messaging_snapshot_path");
        assert_eq!(refs[0].setting_value, "/tmp/signal-v2.json");
    }

    #[tokio::test]
    async fn integration_connection_events_append_and_list() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let connection_id = storage
            .insert_integration_connection(IntegrationConnectionInsert {
                family: IntegrationFamily::Calendar,
                provider: IntegrationProvider::new(IntegrationFamily::Calendar, "google").unwrap(),
                status: IntegrationConnectionStatus::Connected,
                display_name: "Google workspace".to_string(),
                account_ref: Some("me@example.com".to_string()),
                metadata_json: json!({}),
            })
            .await
            .unwrap();

        let first_id = storage
            .insert_integration_connection_event(
                connection_id.as_ref(),
                IntegrationConnectionEventType::SyncStarted,
                &json!({ "job": "manual" }),
                1_700_000_100,
            )
            .await
            .unwrap();
        let second_id = storage
            .insert_integration_connection_event(
                connection_id.as_ref(),
                IntegrationConnectionEventType::SyncSucceeded,
                &json!({ "items": 42 }),
                1_700_000_200,
            )
            .await
            .unwrap();

        let events = storage
            .list_integration_connection_events(connection_id.as_ref(), 10)
            .await
            .unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].id, second_id);
        assert_eq!(
            events[0].event_type,
            IntegrationConnectionEventType::SyncSucceeded
        );
        assert_eq!(events[0].payload_json["items"], 42);
        assert_eq!(events[1].id, first_id);
        assert_eq!(
            events[1].event_type,
            IntegrationConnectionEventType::SyncStarted
        );
        assert_eq!(events[1].payload_json["job"], "manual");
    }
}
