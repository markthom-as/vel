use crate::{
    infra,
    mapping::{parse_json_value, timestamp_to_datetime},
    repositories::{
        artifacts_repo, assistant_transcripts_repo, captures_repo, chat_repo, commitment_risk_repo,
        commitments_repo, context_timeline_repo, current_context_repo, inferred_state_repo,
        integration_connections_repo, nudges_repo, processing_jobs_repo, runs_repo,
        signals_repo, suggestion_feedback_repo, suggestions_repo, threads_repo,
        uncertainty_records_repo,
    },
    runtime_cluster, runtime_loops,
};
use serde_json::Value as JsonValue;
use sqlx::{migrate::Migrator, QueryBuilder, Row, Sqlite, SqlitePool};
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
    pub client_kind: Option<String>,
    pub client_version: Option<String>,
    pub protocol_version: Option<String>,
    pub build_id: Option<String>,
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
    pub ping_ms: Option<u32>,
    pub sync_status: Option<String>,
    pub last_upstream_sync_at: Option<i64>,
    pub last_downstream_sync_at: Option<i64>,
    pub last_sync_error: Option<String>,
    pub last_heartbeat_at: i64,
    pub started_at: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ClusterWorkerRecord {
    pub worker_id: String,
    pub node_id: String,
    pub node_display_name: Option<String>,
    pub client_kind: Option<String>,
    pub client_version: Option<String>,
    pub protocol_version: Option<String>,
    pub build_id: Option<String>,
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
    pub ping_ms: Option<u32>,
    pub sync_status: Option<String>,
    pub last_upstream_sync_at: Option<i64>,
    pub last_downstream_sync_at: Option<i64>,
    pub last_sync_error: Option<String>,
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
        let pool = infra::connect_pool(db_path).await?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), StorageError> {
        infra::run_migrations(&self.pool, &MIGRATOR).await?;
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
        infra::healthcheck(&self.pool).await
    }

    /// Returns the number of applied migrations (schema version). Used by doctor/diagnostics.
    pub async fn schema_version(&self) -> Result<u32, StorageError> {
        infra::schema_version(&self.pool).await
    }

    /// Appends a runtime event to the events table. Idempotent callers should generate event_id themselves if needed.
    pub async fn emit_event(
        &self,
        event_type: &str,
        subject_type: &str,
        subject_id: Option<&str>,
        payload_json: &str,
    ) -> Result<(), StorageError> {
        infra::emit_event(
            &self.pool,
            event_type,
            subject_type,
            subject_id,
            payload_json,
        )
        .await
    }

    pub async fn insert_capture(&self, input: CaptureInsert) -> Result<CaptureId, StorageError> {
        captures_repo::insert_capture(&self.pool, input).await
    }

    pub async fn insert_capture_with_id(
        &self,
        capture_id: CaptureId,
        input: CaptureInsert,
    ) -> Result<bool, StorageError> {
        captures_repo::insert_capture_with_id(&self.pool, capture_id, input).await
    }

    pub async fn capture_count(&self) -> Result<i64, StorageError> {
        captures_repo::capture_count(&self.pool).await
    }

    pub async fn get_capture_by_id(
        &self,
        capture_id: &str,
    ) -> Result<Option<ContextCapture>, StorageError> {
        captures_repo::get_capture_by_id(&self.pool, capture_id).await
    }

    /// List captures most recent first. If today_only, restrict to since start of current day (UTC).
    pub async fn list_captures_recent(
        &self,
        limit: u32,
        today_only: bool,
    ) -> Result<Vec<ContextCapture>, StorageError> {
        captures_repo::list_captures_recent(&self.pool, limit, today_only).await
    }

    pub async fn insert_commitment(
        &self,
        input: CommitmentInsert,
    ) -> Result<CommitmentId, StorageError> {
        commitments_repo::insert_commitment(&self.pool, input).await
    }

    pub async fn get_commitment_by_id(&self, id: &str) -> Result<Option<Commitment>, StorageError> {
        commitments_repo::get_commitment_by_id(&self.pool, id).await
    }

    pub async fn list_commitments(
        &self,
        status_filter: Option<CommitmentStatus>,
        project: Option<&str>,
        kind: Option<&str>,
        limit: u32,
    ) -> Result<Vec<Commitment>, StorageError> {
        commitments_repo::list_commitments(&self.pool, status_filter, project, kind, limit).await
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
        commitments_repo::update_commitment(
            &self.pool,
            id,
            text,
            status,
            due_at,
            project,
            commitment_kind,
            metadata_json,
        )
        .await
    }

    // --- Commitment dependencies ---

    pub async fn insert_commitment_dependency(
        &self,
        parent_commitment_id: &str,
        child_commitment_id: &str,
        dependency_type: &str,
    ) -> Result<String, StorageError> {
        commitments_repo::insert_commitment_dependency(
            &self.pool,
            parent_commitment_id,
            child_commitment_id,
            dependency_type,
        )
        .await
    }

    pub async fn list_commitment_dependencies_by_parent(
        &self,
        parent_commitment_id: &str,
    ) -> Result<Vec<(String, String, String, i64)>, StorageError> {
        commitments_repo::list_commitment_dependencies_by_parent(&self.pool, parent_commitment_id)
            .await
    }

    pub async fn list_commitment_dependencies_by_child(
        &self,
        child_commitment_id: &str,
    ) -> Result<Vec<(String, String, String, i64)>, StorageError> {
        commitments_repo::list_commitment_dependencies_by_child(&self.pool, child_commitment_id)
            .await
    }

    // --- Signals (Phase B) ---

    pub async fn insert_signal(&self, input: SignalInsert) -> Result<String, StorageError> {
        signals_repo::insert_signal(&self.pool, input).await
    }

    pub async fn list_signals(
        &self,
        signal_type: Option<&str>,
        since_ts: Option<i64>,
        limit: u32,
    ) -> Result<Vec<SignalRecord>, StorageError> {
        signals_repo::list_signals(&self.pool, signal_type, since_ts, limit).await
    }

    pub async fn list_signals_by_ids(
        &self,
        signal_ids: &[String],
    ) -> Result<Vec<SignalRecord>, StorageError> {
        signals_repo::list_signals_by_ids(&self.pool, signal_ids).await
    }

    // --- Assistant transcripts ---

    pub async fn insert_assistant_transcript(
        &self,
        input: AssistantTranscriptInsert,
    ) -> Result<bool, StorageError> {
        assistant_transcripts_repo::insert_assistant_transcript(&self.pool, input).await
    }

    pub async fn list_assistant_transcripts_by_conversation(
        &self,
        conversation_id: &str,
    ) -> Result<Vec<AssistantTranscriptRecord>, StorageError> {
        assistant_transcripts_repo::list_assistant_transcripts_by_conversation(
            &self.pool,
            conversation_id,
        )
        .await
    }

    // --- Inferred state (Phase C) ---

    pub async fn insert_inferred_state(
        &self,
        input: InferredStateInsert,
    ) -> Result<String, StorageError> {
        inferred_state_repo::insert_inferred_state(&self.pool, input).await
    }

    pub async fn list_inferred_state_recent(
        &self,
        state_name: Option<&str>,
        limit: u32,
    ) -> Result<Vec<InferredStateRecord>, StorageError> {
        inferred_state_repo::list_inferred_state_recent(&self.pool, state_name, limit).await
    }

    // --- Nudges (Phase D) ---

    pub async fn insert_nudge(&self, input: NudgeInsert) -> Result<String, StorageError> {
        nudges_repo::insert_nudge(&self.pool, input).await
    }

    pub async fn get_nudge(&self, id: &str) -> Result<Option<NudgeRecord>, StorageError> {
        nudges_repo::get_nudge(&self.pool, id).await
    }

    pub async fn list_nudges(
        &self,
        state_filter: Option<&str>,
        limit: u32,
    ) -> Result<Vec<NudgeRecord>, StorageError> {
        nudges_repo::list_nudges(&self.pool, state_filter, limit).await
    }

    pub async fn update_nudge_state(
        &self,
        nudge_id: &str,
        state: &str,
        snoozed_until: Option<i64>,
        resolved_at: Option<i64>,
    ) -> Result<(), StorageError> {
        nudges_repo::update_nudge_state(&self.pool, nudge_id, state, snoozed_until, resolved_at)
            .await
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
        nudges_repo::update_nudge_lifecycle(
            &self.pool,
            nudge_id,
            level,
            state,
            message,
            snoozed_until,
            resolved_at,
            inference_snapshot_json,
            metadata_json,
        )
        .await
    }

    // --- Current context (singleton) ---

    pub async fn get_current_context(&self) -> Result<Option<(i64, vel_core::context::CurrentContextV1)>, StorageError> {
        current_context_repo::get_current_context(&self.pool).await
    }

    pub async fn set_current_context(
        &self,
        computed_at: i64,
        context_json: &str,
    ) -> Result<(), StorageError> {
        current_context_repo::set_current_context(&self.pool, computed_at, context_json).await
    }

    pub async fn insert_context_timeline(
        &self,
        timestamp: i64,
        context_json: &str,
        trigger_signal_id: Option<&str>,
    ) -> Result<(), StorageError> {
        context_timeline_repo::insert_context_timeline(
            &self.pool,
            timestamp,
            context_json,
            trigger_signal_id,
        )
        .await
    }

    pub async fn list_context_timeline(
        &self,
        limit: u32,
    ) -> Result<Vec<(String, i64, String)>, StorageError> {
        context_timeline_repo::list_context_timeline(&self.pool, limit).await
    }

    // --- Integration foundation (INTG-001) ---

    pub async fn insert_integration_connection(
        &self,
        input: IntegrationConnectionInsert,
    ) -> Result<IntegrationConnectionId, StorageError> {
        integration_connections_repo::insert_integration_connection(&self.pool, input).await
    }

    pub async fn get_integration_connection(
        &self,
        id: &str,
    ) -> Result<Option<IntegrationConnection>, StorageError> {
        integration_connections_repo::get_integration_connection(&self.pool, id).await
    }

    pub async fn list_integration_connections(
        &self,
        filters: IntegrationConnectionFilters,
    ) -> Result<Vec<IntegrationConnection>, StorageError> {
        integration_connections_repo::list_integration_connections(&self.pool, filters).await
    }

    pub async fn update_integration_connection(
        &self,
        id: &str,
        status: Option<IntegrationConnectionStatus>,
        display_name: Option<&str>,
        account_ref: Option<Option<&str>>,
        metadata_json: Option<&JsonValue>,
    ) -> Result<(), StorageError> {
        integration_connections_repo::update_integration_connection(
            &self.pool,
            id,
            status,
            display_name,
            account_ref,
            metadata_json,
        )
        .await
    }

    pub async fn upsert_integration_connection_setting_ref(
        &self,
        connection_id: &str,
        setting_key: &str,
        setting_value: &str,
    ) -> Result<(), StorageError> {
        integration_connections_repo::upsert_integration_connection_setting_ref(
            &self.pool,
            connection_id,
            setting_key,
            setting_value,
        )
        .await
    }

    pub async fn list_integration_connection_setting_refs(
        &self,
        connection_id: &str,
    ) -> Result<Vec<IntegrationConnectionSettingRef>, StorageError> {
        integration_connections_repo::list_integration_connection_setting_refs(
            &self.pool,
            connection_id,
        )
        .await
    }

    pub async fn insert_integration_connection_event(
        &self,
        connection_id: &str,
        event_type: IntegrationConnectionEventType,
        payload_json: &JsonValue,
        timestamp: i64,
    ) -> Result<String, StorageError> {
        integration_connections_repo::insert_integration_connection_event(
            &self.pool,
            connection_id,
            event_type,
            payload_json,
            timestamp,
        )
        .await
    }

    pub async fn list_integration_connection_events(
        &self,
        connection_id: &str,
        limit: u32,
    ) -> Result<Vec<IntegrationConnectionEvent>, StorageError> {
        integration_connections_repo::list_integration_connection_events(
            &self.pool,
            connection_id,
            limit,
        )
        .await
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
        threads_repo::insert_thread(&self.pool, id, thread_type, title, status, metadata_json).await
    }

    pub async fn get_thread_by_id(
        &self,
        id: &str,
    ) -> Result<Option<(String, String, String, String, String, i64, i64)>, StorageError> {
        threads_repo::get_thread_by_id(&self.pool, id).await
    }

    pub async fn list_threads(
        &self,
        status_filter: Option<&str>,
        limit: u32,
    ) -> Result<Vec<(String, String, String, String, i64, i64)>, StorageError> {
        threads_repo::list_threads(&self.pool, status_filter, limit).await
    }

    pub async fn update_thread_status(&self, id: &str, status: &str) -> Result<(), StorageError> {
        threads_repo::update_thread_status(&self.pool, id, status).await
    }

    pub async fn insert_thread_link(
        &self,
        thread_id: &str,
        entity_type: &str,
        entity_id: &str,
        relation_type: &str,
    ) -> Result<String, StorageError> {
        threads_repo::insert_thread_link(&self.pool, thread_id, entity_type, entity_id, relation_type)
            .await
    }

    pub async fn list_thread_links(
        &self,
        thread_id: &str,
    ) -> Result<Vec<(String, String, String, String)>, StorageError> {
        threads_repo::list_thread_links(&self.pool, thread_id).await
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
        suggestions_repo::insert_suggestion_v2(&self.pool, input).await
    }

    pub async fn list_suggestions(
        &self,
        state_filter: Option<&str>,
        limit: u32,
    ) -> Result<Vec<SuggestionRecord>, StorageError> {
        suggestions_repo::list_suggestions(&self.pool, state_filter, limit).await
    }

    pub async fn get_suggestion_by_id(
        &self,
        id: &str,
    ) -> Result<Option<SuggestionRecord>, StorageError> {
        suggestions_repo::get_suggestion_by_id(&self.pool, id).await
    }

    pub async fn insert_suggestion_evidence(
        &self,
        input: SuggestionEvidenceInsert,
    ) -> Result<String, StorageError> {
        suggestions_repo::insert_suggestion_evidence(&self.pool, input).await
    }

    pub async fn list_suggestion_evidence(
        &self,
        suggestion_id: &str,
    ) -> Result<Vec<SuggestionEvidenceRecord>, StorageError> {
        suggestions_repo::list_suggestion_evidence(&self.pool, suggestion_id).await
    }

    pub async fn insert_suggestion_feedback(
        &self,
        input: SuggestionFeedbackInsert,
    ) -> Result<String, StorageError> {
        suggestion_feedback_repo::insert_suggestion_feedback(&self.pool, input).await
    }

    pub async fn list_suggestion_feedback(
        &self,
        suggestion_id: &str,
    ) -> Result<Vec<SuggestionFeedbackRecord>, StorageError> {
        suggestion_feedback_repo::list_suggestion_feedback(&self.pool, suggestion_id).await
    }

    pub async fn summarize_suggestion_feedback(
        &self,
        suggestion_type: &str,
    ) -> Result<SuggestionFeedbackSummary, StorageError> {
        suggestion_feedback_repo::summarize_suggestion_feedback(&self.pool, suggestion_type).await
    }

    pub async fn insert_uncertainty_record(
        &self,
        input: UncertaintyRecordInsert,
    ) -> Result<String, StorageError> {
        uncertainty_records_repo::insert_uncertainty_record(&self.pool, input).await
    }

    pub async fn list_uncertainty_records(
        &self,
        status: Option<&str>,
        limit: u32,
    ) -> Result<Vec<UncertaintyRecord>, StorageError> {
        uncertainty_records_repo::list_uncertainty_records(&self.pool, status, limit).await
    }

    pub async fn get_uncertainty_record(
        &self,
        id: &str,
    ) -> Result<Option<UncertaintyRecord>, StorageError> {
        uncertainty_records_repo::get_uncertainty_record(&self.pool, id).await
    }

    pub async fn find_open_uncertainty_record(
        &self,
        subject_type: &str,
        subject_id: Option<&str>,
        decision_kind: &str,
    ) -> Result<Option<UncertaintyRecord>, StorageError> {
        uncertainty_records_repo::find_open_uncertainty_record(
            &self.pool,
            subject_type,
            subject_id,
            decision_kind,
        )
        .await
    }

    pub async fn find_recent_uncertainty_record(
        &self,
        subject_type: &str,
        subject_id: Option<&str>,
        decision_kind: &str,
        status: &str,
        since_ts: i64,
    ) -> Result<Option<UncertaintyRecord>, StorageError> {
        uncertainty_records_repo::find_recent_uncertainty_record(
            &self.pool,
            subject_type,
            subject_id,
            decision_kind,
            status,
            since_ts,
        )
        .await
    }

    pub async fn resolve_uncertainty_record(
        &self,
        id: &str,
    ) -> Result<Option<UncertaintyRecord>, StorageError> {
        uncertainty_records_repo::resolve_uncertainty_record(&self.pool, id).await
    }

    pub async fn find_recent_suggestion_by_dedupe_key(
        &self,
        dedupe_key: &str,
    ) -> Result<Option<SuggestionRecord>, StorageError> {
        suggestions_repo::find_recent_suggestion_by_dedupe_key(&self.pool, dedupe_key).await
    }

    pub async fn update_suggestion_state(
        &self,
        id: &str,
        state: &str,
        resolved_at: Option<i64>,
        payload_json: Option<&str>,
    ) -> Result<(), StorageError> {
        suggestions_repo::update_suggestion_state(&self.pool, id, state, resolved_at, payload_json)
            .await
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
        commitment_risk_repo::insert_commitment_risk(
            &self.pool,
            commitment_id,
            risk_score,
            risk_level,
            factors_json,
            computed_at,
        )
        .await
    }

    pub async fn list_commitment_risk_recent(
        &self,
        commitment_id: &str,
        limit: u32,
    ) -> Result<Vec<(String, f64, String, String, i64)>, StorageError> {
        commitment_risk_repo::list_commitment_risk_recent(&self.pool, commitment_id, limit).await
    }

    /// Latest risk snapshot per commitment (for listing current risk).
    pub async fn list_commitment_risk_latest_all(
        &self,
    ) -> Result<Vec<(String, String, f64, String, String, i64)>, StorageError> {
        commitment_risk_repo::list_commitment_risk_latest_all(&self.pool).await
    }

    /// Count commitment_risk rows (for read-boundary tests: explain must not create new rows).
    pub async fn count_commitment_risk(&self) -> Result<i64, StorageError> {
        commitment_risk_repo::count_commitment_risk(&self.pool).await
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
        nudges_repo::count_nudge_events(&self.pool).await
    }

    // --- Nudge events (append-only log) ---

    pub async fn insert_nudge_event(
        &self,
        nudge_id: &str,
        event_type: &str,
        payload_json: &str,
        timestamp: i64,
    ) -> Result<(), StorageError> {
        nudges_repo::insert_nudge_event(&self.pool, nudge_id, event_type, payload_json, timestamp)
            .await
    }

    pub async fn list_nudge_events(
        &self,
        nudge_id: &str,
        limit: u32,
    ) -> Result<Vec<NudgeEventRecord>, StorageError> {
        nudges_repo::list_nudge_events(&self.pool, nudge_id, limit).await
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

    pub async fn insert_processing_job(
        &self,
        job_id: &JobId,
        job_type: &str,
        status: JobStatus,
        payload_json: &str,
    ) -> Result<(), StorageError> {
        processing_jobs_repo::insert_processing_job(&self.pool, job_id, job_type, status, payload_json)
            .await
    }

    /// Claims the next pending job of the given type. Returns `None` if no pending job exists.
    /// The job is marked `running` and `started_at` is set. Caller must call `mark_job_succeeded` or `mark_job_failed`.
    pub async fn claim_next_pending_job(
        &self,
        job_type: &str,
    ) -> Result<Option<PendingJob>, StorageError> {
        processing_jobs_repo::claim_next_pending_job(&self.pool, job_type).await
    }

    pub async fn mark_job_succeeded(&self, job_id: &str) -> Result<(), StorageError> {
        processing_jobs_repo::mark_job_succeeded(&self.pool, job_id).await
    }

    pub async fn mark_job_failed(&self, job_id: &str, error: &str) -> Result<(), StorageError> {
        processing_jobs_repo::mark_job_failed(&self.pool, job_id, error).await
    }

    pub async fn create_artifact(&self, input: ArtifactInsert) -> Result<ArtifactId, StorageError> {
        artifacts_repo::create_artifact(&self.pool, input).await
    }

    pub async fn get_artifact_by_id(
        &self,
        artifact_id: &str,
    ) -> Result<Option<ArtifactRecord>, StorageError> {
        artifacts_repo::get_artifact_by_id(&self.pool, artifact_id).await
    }

    /// Returns the most recently created artifact of the given type, if any.
    pub async fn get_latest_artifact_by_type(
        &self,
        artifact_type: &str,
    ) -> Result<Option<ArtifactRecord>, StorageError> {
        artifacts_repo::get_latest_artifact_by_type(&self.pool, artifact_type).await
    }

    /// List artifacts by created_at descending, up to limit.
    pub async fn list_artifacts(&self, limit: u32) -> Result<Vec<ArtifactRecord>, StorageError> {
        artifacts_repo::list_artifacts(&self.pool, limit).await
    }

    pub async fn create_run(
        &self,
        id: &RunId,
        kind: RunKind,
        input_json: &JsonValue,
    ) -> Result<(), StorageError> {
        runs_repo::create_run(&self.pool, id, kind, input_json).await
    }

    pub async fn get_run_by_id(&self, run_id: &str) -> Result<Option<Run>, StorageError> {
        runs_repo::get_run_by_id(&self.pool, run_id).await
    }

    pub async fn list_runs(
        &self,
        limit: u32,
        kind_filter: Option<&str>,
        since_ts: Option<i64>,
    ) -> Result<Vec<Run>, StorageError> {
        runs_repo::list_runs(&self.pool, limit, kind_filter, since_ts).await
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
        runs_repo::update_run_status(
            &self.pool,
            run_id,
            status,
            started_at,
            finished_at,
            output_json,
            error_json,
        )
        .await
    }

    pub async fn reset_run_for_retry(&self, run_id: &str) -> Result<(), StorageError> {
        runs_repo::reset_run_for_retry(&self.pool, run_id).await
    }

    pub async fn append_run_event(
        &self,
        run_id: &str,
        seq: u32,
        event_type: RunEventType,
        payload_json: &JsonValue,
    ) -> Result<(), StorageError> {
        runs_repo::append_run_event(&self.pool, run_id, seq, event_type, payload_json).await
    }

    pub async fn next_run_event_seq(&self, run_id: &str) -> Result<u32, StorageError> {
        runs_repo::next_run_event_seq(&self.pool, run_id).await
    }

    pub async fn append_run_event_auto(
        &self,
        run_id: &str,
        event_type: RunEventType,
        payload_json: &JsonValue,
    ) -> Result<u32, StorageError> {
        runs_repo::append_run_event_auto(&self.pool, run_id, event_type, payload_json).await
    }

    pub async fn list_retry_ready_runs(
        &self,
        now_ts: i64,
        limit: u32,
    ) -> Result<Vec<RetryReadyRun>, StorageError> {
        runs_repo::list_retry_ready_runs(&self.pool, now_ts, limit).await
    }

    pub async fn list_run_events(&self, run_id: &str) -> Result<Vec<RunEvent>, StorageError> {
        runs_repo::list_run_events(&self.pool, run_id).await
    }

    pub async fn create_ref(&self, ref_: &Ref) -> Result<(), StorageError> {
        runs_repo::create_ref(&self.pool, ref_).await
    }

    pub async fn list_refs_from(
        &self,
        from_type: &str,
        from_id: &str,
    ) -> Result<Vec<Ref>, StorageError> {
        runs_repo::list_refs_from(&self.pool, from_type, from_id).await
    }

    pub async fn list_refs_to(&self, to_type: &str, to_id: &str) -> Result<Vec<Ref>, StorageError> {
        runs_repo::list_refs_to(&self.pool, to_type, to_id).await
    }

    // --- Conversations (chat) ---

    pub async fn create_conversation(
        &self,
        input: ConversationInsert,
    ) -> Result<ConversationId, StorageError> {
        chat_repo::create_conversation(&self.pool, input).await
    }

    pub async fn list_conversations(
        &self,
        archived: Option<bool>,
        limit: u32,
    ) -> Result<Vec<ConversationRecord>, StorageError> {
        chat_repo::list_conversations(&self.pool, archived, limit).await
    }

    pub async fn get_conversation(
        &self,
        id: &str,
    ) -> Result<Option<ConversationRecord>, StorageError> {
        chat_repo::get_conversation(&self.pool, id).await
    }

    pub async fn rename_conversation(&self, id: &str, title: &str) -> Result<(), StorageError> {
        chat_repo::rename_conversation(&self.pool, id, title).await
    }

    pub async fn pin_conversation(&self, id: &str, pinned: bool) -> Result<(), StorageError> {
        chat_repo::pin_conversation(&self.pool, id, pinned).await
    }

    pub async fn archive_conversation(&self, id: &str, archived: bool) -> Result<(), StorageError> {
        chat_repo::archive_conversation(&self.pool, id, archived).await
    }

    // --- Messages (chat) ---

    pub async fn create_message(&self, input: MessageInsert) -> Result<MessageId, StorageError> {
        chat_repo::create_message(&self.pool, input).await
    }

    /// List messages in a conversation, ordered by created_at ASC for stable thread display.
    pub async fn list_messages_by_conversation(
        &self,
        conversation_id: &str,
        limit: u32,
    ) -> Result<Vec<MessageRecord>, StorageError> {
        chat_repo::list_messages_by_conversation(&self.pool, conversation_id, limit).await
    }

    pub async fn get_message(&self, id: &str) -> Result<Option<MessageRecord>, StorageError> {
        chat_repo::get_message(&self.pool, id).await
    }

    pub async fn update_message_status(&self, id: &str, status: &str) -> Result<(), StorageError> {
        chat_repo::update_message_status(&self.pool, id, status).await
    }

    // --- Interventions (chat) ---

    pub async fn create_intervention(
        &self,
        input: InterventionInsert,
    ) -> Result<InterventionId, StorageError> {
        chat_repo::create_intervention(&self.pool, input).await
    }

    pub async fn list_interventions_active(
        &self,
        limit: u32,
    ) -> Result<Vec<InterventionRecord>, StorageError> {
        chat_repo::list_interventions_active(&self.pool, limit).await
    }

    pub async fn get_interventions_by_message(
        &self,
        message_id: &str,
    ) -> Result<Vec<InterventionRecord>, StorageError> {
        chat_repo::get_interventions_by_message(&self.pool, message_id).await
    }

    pub async fn get_interventions_by_conversation(
        &self,
        conversation_id: &str,
    ) -> Result<Vec<InterventionRecord>, StorageError> {
        chat_repo::get_interventions_by_conversation(&self.pool, conversation_id).await
    }

    pub async fn get_intervention(
        &self,
        id: &str,
    ) -> Result<Option<InterventionRecord>, StorageError> {
        chat_repo::get_intervention(&self.pool, id).await
    }

    pub async fn snooze_intervention(
        &self,
        id: &str,
        snoozed_until_ts: i64,
    ) -> Result<(), StorageError> {
        chat_repo::snooze_intervention(&self.pool, id, snoozed_until_ts).await
    }

    pub async fn resolve_intervention(&self, id: &str) -> Result<(), StorageError> {
        chat_repo::resolve_intervention(&self.pool, id).await
    }

    pub async fn dismiss_intervention(&self, id: &str) -> Result<(), StorageError> {
        chat_repo::dismiss_intervention(&self.pool, id).await
    }

    // --- Event log (chat) ---

    pub async fn append_event(&self, input: EventLogInsert) -> Result<EventId, StorageError> {
        chat_repo::append_event(&self.pool, input).await
    }

    pub async fn list_events_recent(
        &self,
        limit: u32,
    ) -> Result<Vec<EventLogRecord>, StorageError> {
        chat_repo::list_events_recent(&self.pool, limit).await
    }

    pub async fn list_events_by_aggregate(
        &self,
        aggregate_type: &str,
        aggregate_id: &str,
        limit: u32,
    ) -> Result<Vec<EventLogRecord>, StorageError> {
        chat_repo::list_events_by_aggregate(&self.pool, aggregate_type, aggregate_id, limit).await
    }

    // --- Settings (chat/client) ---

    pub async fn get_all_settings(
        &self,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>, StorageError> {
        runtime_cluster::get_all_settings(&self.pool).await
    }

    pub async fn set_setting(
        &self,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<(), StorageError> {
        runtime_cluster::set_setting(&self.pool, key, value).await
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
        runtime_cluster::insert_work_assignment(&self.pool, assignment).await
    }

    pub async fn update_work_assignment(
        &self,
        update: WorkAssignmentUpdate,
    ) -> Result<WorkAssignmentRecord, StorageError> {
        runtime_cluster::update_work_assignment(&self.pool, update).await
    }

    pub async fn set_work_assignment_last_updated(
        &self,
        receipt_id: &str,
        last_updated: i64,
    ) -> Result<(), StorageError> {
        runtime_cluster::set_work_assignment_last_updated(&self.pool, receipt_id, last_updated)
            .await
    }

    pub async fn list_work_assignments(
        &self,
        work_request_id: Option<&str>,
        worker_id: Option<&str>,
    ) -> Result<Vec<WorkAssignmentRecord>, StorageError> {
        runtime_cluster::list_work_assignments(&self.pool, work_request_id, worker_id).await
    }

    pub async fn upsert_cluster_worker(
        &self,
        worker: ClusterWorkerUpsert,
    ) -> Result<(), StorageError> {
        runtime_cluster::upsert_cluster_worker(&self.pool, worker).await
    }

    pub async fn expire_cluster_workers(&self, stale_before: i64) -> Result<u64, StorageError> {
        runtime_cluster::expire_cluster_workers(&self.pool, stale_before).await
    }

    pub async fn list_cluster_workers(&self) -> Result<Vec<ClusterWorkerRecord>, StorageError> {
        runtime_cluster::list_cluster_workers(&self.pool).await
    }

    pub async fn get_runtime_loop(
        &self,
        loop_kind: &str,
    ) -> Result<Option<RuntimeLoopRecord>, StorageError> {
        runtime_cluster::get_runtime_loop(&self.pool, loop_kind).await
    }

    pub async fn update_runtime_loop_config(
        &self,
        loop_kind: &str,
        enabled: Option<bool>,
        interval_seconds: Option<i64>,
    ) -> Result<Option<RuntimeLoopRecord>, StorageError> {
        runtime_cluster::update_runtime_loop_config(
            &self.pool,
            loop_kind,
            enabled,
            interval_seconds,
        )
        .await
    }

    pub async fn orientation_snapshot(&self) -> Result<OrientationSnapshot, StorageError> {
        captures_repo::orientation_snapshot(&self.pool).await
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
    async fn commitment_risk_round_trip_via_storage_facade() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let older = storage
            .insert_commitment_risk("com_1", 0.25, "low", r#"{"factors":["none"]}"#, 100)
            .await
            .unwrap();
        let newer = storage
            .insert_commitment_risk(
                "com_1",
                0.75,
                "high",
                r#"{"factors":["deadline_pressure"]}"#,
                200,
            )
            .await
            .unwrap();
        let other = storage
            .insert_commitment_risk(
                "com_2",
                0.9,
                "high",
                r#"{"factors":["dependency_chain"]}"#,
                150,
            )
            .await
            .unwrap();

        let recent = storage.list_commitment_risk_recent("com_1", 10).await.unwrap();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].0, newer);
        assert_eq!(recent[1].0, older);

        let latest = storage.list_commitment_risk_latest_all().await.unwrap();
        assert_eq!(latest.len(), 2);
        assert_eq!(latest[0].0, other);
        assert_eq!(latest[1].0, newer);

        let count = storage.count_commitment_risk().await.unwrap();
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn nudge_and_nudge_events_round_trip_via_storage_facade() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let nudge_id = storage
            .insert_nudge(NudgeInsert {
                nudge_type: "focus".to_string(),
                level: "warning".to_string(),
                state: "open".to_string(),
                related_commitment_id: Some("com_1".to_string()),
                message: "Take a short planning pass".to_string(),
                snoozed_until: None,
                resolved_at: None,
                signals_snapshot_json: Some(r#"{"signals":1}"#.to_string()),
                inference_snapshot_json: Some(r#"{"reason":"drift"}"#.to_string()),
                metadata_json: Some(json!({"source":"db-test"})),
            })
            .await
            .unwrap();

        let fetched = storage.get_nudge(&nudge_id).await.unwrap().unwrap();
        assert_eq!(fetched.state, "open");
        assert_eq!(fetched.metadata_json["source"], "db-test");

        let listed_open = storage.list_nudges(Some("open"), 10).await.unwrap();
        assert_eq!(listed_open.len(), 1);
        assert_eq!(listed_open[0].nudge_id, nudge_id);

        storage
            .update_nudge_state(&nudge_id, "snoozed", Some(1_700_000_300), None)
            .await
            .unwrap();
        storage
            .update_nudge_lifecycle(
                &nudge_id,
                "danger",
                "resolved",
                "Handled",
                None,
                Some(1_700_000_400),
                Some(r#"{"reason":"completed"}"#),
                &json!({"source":"db-test","final":"yes"}),
            )
            .await
            .unwrap();

        let resolved = storage.get_nudge(&nudge_id).await.unwrap().unwrap();
        assert_eq!(resolved.level, "danger");
        assert_eq!(resolved.state, "resolved");
        assert_eq!(resolved.message, "Handled");
        assert_eq!(resolved.resolved_at, Some(1_700_000_400));
        assert_eq!(
            resolved.inference_snapshot_json.as_deref(),
            Some(r#"{"reason":"completed"}"#)
        );
        assert_eq!(resolved.metadata_json["final"], "yes");

        storage
            .insert_nudge_event(&nudge_id, "nudge_created", r#"{"channel":"local"}"#, 1_700_000_100)
            .await
            .unwrap();
        storage
            .insert_nudge_event(&nudge_id, "nudge_resolved", r#"{"outcome":"done"}"#, 1_700_000_200)
            .await
            .unwrap();

        let events = storage.list_nudge_events(&nudge_id, 10).await.unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "nudge_created");
        assert_eq!(events[1].event_type, "nudge_resolved");
        assert_eq!(events[1].payload_json["outcome"], "done");

        let event_count = storage.count_nudge_events().await.unwrap();
        assert_eq!(event_count, 2);
    }

    #[tokio::test]
    async fn chat_conversation_message_and_intervention_round_trip() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let conversation_id = storage
            .create_conversation(ConversationInsert {
                id: "conv_test_1".to_string(),
                title: Some("Test Conversation".to_string()),
                kind: "thread".to_string(),
                pinned: false,
                archived: false,
            })
            .await
            .unwrap();
        storage
            .rename_conversation(conversation_id.as_ref(), "Renamed Conversation")
            .await
            .unwrap();
        storage
            .pin_conversation(conversation_id.as_ref(), true)
            .await
            .unwrap();

        let fetched_conversation = storage
            .get_conversation(conversation_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            fetched_conversation.title.as_deref(),
            Some("Renamed Conversation")
        );
        assert!(fetched_conversation.pinned);

        let message_id = storage
            .create_message(MessageInsert {
                id: "msg_test_1".to_string(),
                conversation_id: conversation_id.as_ref().to_string(),
                role: "assistant".to_string(),
                kind: "text".to_string(),
                content_json: r#"{"text":"hello"}"#.to_string(),
                status: Some("pending".to_string()),
                importance: Some("normal".to_string()),
            })
            .await
            .unwrap();
        storage
            .update_message_status(message_id.as_ref(), "done")
            .await
            .unwrap();

        let listed_messages = storage
            .list_messages_by_conversation(conversation_id.as_ref(), 10)
            .await
            .unwrap();
        assert_eq!(listed_messages.len(), 1);
        assert_eq!(listed_messages[0].id, message_id);
        assert_eq!(listed_messages[0].status.as_deref(), Some("done"));

        let intervention_id = storage
            .create_intervention(InterventionInsert {
                id: "intv_test_1".to_string(),
                message_id: message_id.as_ref().to_string(),
                kind: "needs_clarification".to_string(),
                state: "active".to_string(),
                surfaced_at: 1_700_000_001,
                resolved_at: None,
                snoozed_until: None,
                confidence: Some(0.6),
                source_json: None,
                provenance_json: None,
            })
            .await
            .unwrap();
        storage
            .snooze_intervention(intervention_id.as_ref(), 1_900_000_000)
            .await
            .unwrap();
        storage
            .resolve_intervention(intervention_id.as_ref())
            .await
            .unwrap();

        let fetched_intervention = storage
            .get_intervention(intervention_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(fetched_intervention.state, "resolved");
        assert!(fetched_intervention.resolved_at.is_some());
    }

    #[tokio::test]
    async fn chat_event_log_append_and_aggregate_filtering() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let first_id = storage
            .append_event(EventLogInsert {
                id: Some("evt_test_1".to_string()),
                event_name: "conversation.created".to_string(),
                aggregate_type: Some("conversation".to_string()),
                aggregate_id: Some("conv_test_agg".to_string()),
                payload_json: r#"{"kind":"thread"}"#.to_string(),
            })
            .await
            .unwrap();
        let second_id = storage
            .append_event(EventLogInsert {
                id: Some("evt_test_2".to_string()),
                event_name: "conversation.updated".to_string(),
                aggregate_type: Some("conversation".to_string()),
                aggregate_id: Some("conv_test_agg".to_string()),
                payload_json: r#"{"field":"title"}"#.to_string(),
            })
            .await
            .unwrap();

        let aggregate_events = storage
            .list_events_by_aggregate("conversation", "conv_test_agg", 10)
            .await
            .unwrap();
        assert_eq!(aggregate_events.len(), 2);
        assert!(aggregate_events.iter().any(|event| event.id == first_id));
        assert!(aggregate_events.iter().any(|event| event.id == second_id));

        let recent_events = storage.list_events_recent(1).await.unwrap();
        assert_eq!(recent_events.len(), 1);
        assert!(recent_events[0].id == first_id || recent_events[0].id == second_id);
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
                client_kind: Some("velios".to_string()),
                client_version: Some("1.2.3".to_string()),
                protocol_version: Some("v1".to_string()),
                build_id: Some("100".to_string()),
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
                ping_ms: Some(42),
                sync_status: Some("fresh".to_string()),
                last_upstream_sync_at: Some(95),
                last_downstream_sync_at: Some(96),
                last_sync_error: None,
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
