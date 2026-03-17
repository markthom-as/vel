use crate::{
    infra,
    repositories::{
        artifacts_repo, assistant_transcripts_repo, captures_repo, chat_repo, cluster_workers_repo,
        commitment_risk_repo, commitments_repo, context_timeline_repo, current_context_repo,
        inferred_state_repo, integration_connections_repo, nudges_repo, processing_jobs_repo,
        run_refs_repo, runs_repo, runtime_loops_repo, settings_repo, signals_repo,
        suggestion_feedback_repo, suggestions_repo, threads_repo, uncertainty_records_repo,
        work_assignments_repo,
    },
};
use serde::Serialize;
use serde_json::Value as JsonValue;
use sqlx::{migrate::Migrator, SqlitePool};
use time::OffsetDateTime;
use vel_core::context::CurrentContextV1;
pub use vel_core::WorkAssignmentStatus;
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
    pub(crate) pool: SqlitePool,
}

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Data corrupted: {0}")]
    DataCorrupted(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<serde_json::Error> for StorageError {
    fn from(error: serde_json::Error) -> Self {
        Self::DataCorrupted(error.to_string())
    }
}

impl From<std::io::Error> for StorageError {
    fn from(error: std::io::Error) -> Self {
        Self::Validation(error.to_string())
    }
}

pub struct CaptureInsert {
    pub content_text: String,
    pub capture_type: String,
    pub source_device: Option<String>,
    pub privacy_class: PrivacyClass,
}

pub struct SearchFilters {
    pub capture_type: Option<String>,
    pub source_device: Option<String>,
    pub limit: Option<u32>,
}

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
    pub privacy_class: PrivacyClass,
    pub sync_class: SyncClass,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub metadata_json: JsonValue,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct SignalInsert {
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: i64,
    pub payload_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SignalRecord {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: i64,
    pub payload_json: JsonValue,
    pub created_at: i64,
}

pub struct AssistantTranscriptInsert {
    pub id: String,
    pub source: String,
    pub conversation_id: String,
    pub message_id: Option<String>,
    pub timestamp: i64,
    pub role: String,
    pub content: String,
    pub metadata_json: JsonValue,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssistantTranscriptRecord {
    pub id: String,
    pub source: String,
    pub conversation_id: String,
    pub message_id: Option<String>,
    pub timestamp: i64,
    pub role: String,
    pub content: String,
    pub metadata_json: JsonValue,
    pub created_at: i64,
}

pub struct InferredStateInsert {
    pub state_name: String,
    pub confidence: Option<String>,
    pub timestamp: i64,
    pub context_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InferredStateRecord {
    pub state_id: String,
    pub state_name: String,
    pub confidence: Option<String>,
    pub timestamp: i64,
    pub context_json: JsonValue,
    pub created_at: i64,
}

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

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct NudgeEventRecord {
    pub id: String,
    pub nudge_id: String,
    pub event_type: String,
    pub payload_json: JsonValue,
    pub timestamp: i64,
    pub created_at: i64,
}

pub struct ConversationInsert {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConversationRecord {
    pub id: ConversationId,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct MessageInsert {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub kind: String,
    pub content_json: String,
    pub status: Option<String>,
    pub importance: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
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
    pub created_at: i64,
}

pub struct EventLogInsert {
    pub id: Option<String>,
    pub event_name: String,
    pub aggregate_type: Option<String>,
    pub aggregate_id: Option<String>,
    pub payload_json: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct EventLogRecord {
    pub id: EventId,
    pub event_name: String,
    pub aggregate_type: Option<String>,
    pub aggregate_id: Option<String>,
    pub payload_json: String,
    pub created_at: i64,
}

pub struct IntegrationConnectionFilters {
    pub family: Option<IntegrationFamily>,
    pub provider_key: Option<String>,
    pub status: Option<IntegrationConnectionStatus>,
    pub include_disabled: bool,
}

pub struct IntegrationConnectionInsert {
    pub family: IntegrationFamily,
    pub provider: IntegrationProvider,
    pub status: IntegrationConnectionStatus,
    pub display_name: String,
    pub account_ref: Option<String>,
    pub metadata_json: JsonValue,
}

pub struct SuggestionInsertV2 {
    pub suggestion_type: String,
    pub state: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub priority: u32,
    pub confidence: Option<String>,
    pub dedupe_key: Option<String>,
    pub payload_json: JsonValue,
    pub decision_context_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SuggestionRecord {
    pub id: String,
    pub suggestion_type: String,
    pub state: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub priority: u32,
    pub confidence: Option<String>,
    pub dedupe_key: Option<String>,
    pub payload_json: JsonValue,
    pub decision_context_json: Option<JsonValue>,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
    pub evidence_count: u32,
}

pub struct SuggestionEvidenceInsert {
    pub suggestion_id: String,
    pub evidence_type: String,
    pub ref_id: String,
    pub evidence_json: Option<JsonValue>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SuggestionEvidenceRecord {
    pub id: String,
    pub suggestion_id: String,
    pub evidence_type: String,
    pub ref_id: String,
    pub evidence_json: Option<JsonValue>,
    pub weight: Option<f64>,
    pub created_at: i64,
}

pub struct SuggestionFeedbackInsert {
    pub suggestion_id: String,
    pub outcome_type: String,
    pub notes: Option<String>,
    pub observed_at: i64,
    pub payload_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SuggestionFeedbackRecord {
    pub id: String,
    pub suggestion_id: String,
    pub outcome_type: String,
    pub notes: Option<String>,
    pub observed_at: i64,
    pub payload_json: Option<JsonValue>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SuggestionFeedbackSummary {
    pub accepted_and_policy_changed: u32,
    pub rejected_not_useful: u32,
    pub rejected_incorrect: u32,
}

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

#[derive(Debug, Clone, Serialize)]
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

pub struct CommitmentInsert {
    pub text: String,
    pub source_type: String,
    pub source_id: String,
    pub status: CommitmentStatus,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone)]
pub struct RetryReadyRun {
    pub run: Run,
    pub retry_at: i64,
    pub retry_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
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

pub struct WorkAssignmentInsert {
    pub receipt_id: Option<String>,
    pub work_request_id: String,
    pub worker_id: String,
    pub worker_class: String,
    pub capability: String,
    pub status: WorkAssignmentStatus,
    pub assigned_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkAssignmentRecord {
    pub receipt_id: String,
    pub work_request_id: String,
    pub worker_id: String,
    pub worker_class: String,
    pub capability: String,
    pub status: WorkAssignmentStatus,
    pub assigned_at: i64,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub result: Option<String>,
    pub error_message: Option<String>,
    pub last_updated: i64,
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
    pub worker_class: String,
    pub worker_classes: Vec<String>,
    pub capabilities: Vec<String>,
    pub status: String,
    pub max_concurrency: Option<u32>,
    pub current_load: Option<u32>,
    pub queue_depth: Option<u32>,
    pub reachability: String,
    pub latency_class: String,
    pub compute_class: String,
    pub power_class: String,
    pub recent_failure_rate: f64,
    pub tailscale_preferred: bool,
    pub sync_base_url: Option<String>,
    pub sync_transport: String,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub ping_ms: Option<u32>,
    pub sync_status: String,
    pub last_upstream_sync_at: Option<i64>,
    pub last_downstream_sync_at: Option<i64>,
    pub last_sync_error: Option<String>,
    pub last_heartbeat_at: i64,
    pub started_at: i64,
    pub updated_at: i64,
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
    pub sync_transport: Option<Option<String>>,
    pub tailscale_base_url: Option<String>,
    pub preferred_tailnet_endpoint: Option<String>,
    pub tailscale_reachable: bool,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub last_heartbeat_at: i64,
    pub started_at: Option<i64>,
}

pub struct WorkAssignmentUpdate {
    pub receipt_id: String,
    pub status: WorkAssignmentStatus,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub result: Option<String>,
    pub error_message: Option<String>,
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

    pub async fn get_current_context(
        &self,
    ) -> Result<Option<(i64, CurrentContextV1)>, StorageError> {
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
        threads_repo::insert_thread_link(
            &self.pool,
            thread_id,
            entity_type,
            entity_id,
            relation_type,
        )
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

    pub async fn list_commitment_risk_latest_all(
        &self,
    ) -> Result<Vec<(String, String, f64, String, String, i64)>, StorageError> {
        commitment_risk_repo::list_commitment_risk_latest_all(&self.pool).await
    }

    pub async fn count_commitment_risk(&self) -> Result<i64, StorageError> {
        commitment_risk_repo::count_commitment_risk(&self.pool).await
    }

    pub async fn count_inferred_state(&self) -> Result<i64, StorageError> {
        inferred_state_repo::count_inferred_state(&self.pool).await
    }

    pub async fn count_context_timeline(&self) -> Result<i64, StorageError> {
        context_timeline_repo::count_context_timeline(&self.pool).await
    }

    pub async fn count_nudge_events(&self) -> Result<i64, StorageError> {
        nudges_repo::count_nudge_events(&self.pool).await
    }

    pub async fn insert_nudge_event(
        &self,
        nudge_id: &str,
        event_type: &str,
        payload_json: &str,
        timestamp: i64,
    ) -> Result<String, StorageError> {
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
        captures_repo::search_captures(&self.pool, query, filters).await
    }

    pub async fn insert_processing_job(
        &self,
        job_id: &JobId,
        job_type: &str,
        status: JobStatus,
        payload_json: &str,
    ) -> Result<(), StorageError> {
        processing_jobs_repo::insert_processing_job(
            &self.pool,
            job_id,
            job_type,
            status,
            payload_json,
        )
        .await
    }

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
        id: &ArtifactId,
    ) -> Result<Option<ArtifactRecord>, StorageError> {
        artifacts_repo::get_artifact_by_id(&self.pool, id.as_ref()).await
    }

    pub async fn get_latest_artifact_by_type(
        &self,
        artifact_type: &str,
    ) -> Result<Option<ArtifactRecord>, StorageError> {
        artifacts_repo::get_latest_artifact_by_type(&self.pool, artifact_type).await
    }

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
        kind: Option<RunKind>,
        status: Option<RunStatus>,
        limit: u32,
    ) -> Result<Vec<Run>, StorageError> {
        let kind_str = kind.map(|k| k.to_string());
        let status_str = status.map(|s| s.to_string());
        runs_repo::list_runs(
            &self.pool,
            limit,
            kind_str.as_deref(),
            status_str.as_deref(),
        )
        .await
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
    ) -> Result<String, StorageError> {
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
    ) -> Result<String, StorageError> {
        runs_repo::append_run_event_auto(&self.pool, run_id, event_type, payload_json).await
    }

    pub async fn list_retry_ready_runs(
        &self,
        max_retries: u32,
        limit: u32,
    ) -> Result<Vec<RetryReadyRun>, StorageError> {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        runs_repo::list_retry_ready_runs(&self.pool, now, max_retries as i64, limit).await
    }

    pub async fn list_run_events(&self, run_id: &str) -> Result<Vec<RunEvent>, StorageError> {
        runs_repo::list_run_events(&self.pool, run_id).await
    }

    pub async fn create_ref(&self, ref_: &Ref) -> Result<(), StorageError> {
        run_refs_repo::create_ref(&self.pool, ref_).await
    }

    pub async fn list_refs_from(
        &self,
        from_type: &str,
        from_id: &str,
    ) -> Result<Vec<Ref>, StorageError> {
        run_refs_repo::list_refs_from(&self.pool, from_type, from_id).await
    }

    pub async fn list_refs_to(&self, to_type: &str, to_id: &str) -> Result<Vec<Ref>, StorageError> {
        run_refs_repo::list_refs_to(&self.pool, to_type, to_id).await
    }

    // --- Conversations (chat) ---

    pub async fn create_conversation(
        &self,
        input: ConversationInsert,
    ) -> Result<ConversationRecord, StorageError> {
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

    pub async fn create_message(&self, input: MessageInsert) -> Result<MessageId, StorageError> {
        chat_repo::create_message(&self.pool, input).await
    }

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
        settings_repo::get_all_settings(&self.pool).await
    }

    pub async fn set_setting(
        &self,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<(), StorageError> {
        settings_repo::set_setting(&self.pool, key, value).await
    }

    pub async fn claim_due_loop(
        &self,
        loop_kind: &str,
        interval_seconds: i64,
        now_ts: i64,
    ) -> Result<bool, StorageError> {
        runtime_loops_repo::claim_due_loop(&self.pool, loop_kind, interval_seconds, now_ts).await
    }

    pub async fn ensure_runtime_loop(
        &self,
        loop_kind: &str,
        enabled: bool,
        interval_seconds: i64,
        next_due_at: Option<i64>,
    ) -> Result<(), StorageError> {
        runtime_loops_repo::ensure_runtime_loop(
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
        runtime_loops_repo::complete_loop(&self.pool, loop_kind, status, error, next_due_at).await
    }

    pub async fn list_runtime_loops(&self) -> Result<Vec<RuntimeLoopRecord>, StorageError> {
        runtime_loops_repo::list_runtime_loops(&self.pool).await
    }

    pub async fn insert_work_assignment(
        &self,
        assignment: WorkAssignmentInsert,
    ) -> Result<String, StorageError> {
        work_assignments_repo::insert_work_assignment(&self.pool, assignment).await
    }

    pub async fn update_work_assignment(
        &self,
        update: WorkAssignmentUpdate,
    ) -> Result<WorkAssignmentRecord, StorageError> {
        work_assignments_repo::update_work_assignment(&self.pool, update).await
    }

    pub async fn set_work_assignment_last_updated(
        &self,
        receipt_id: &str,
        last_updated: i64,
    ) -> Result<(), StorageError> {
        work_assignments_repo::set_work_assignment_last_updated(
            &self.pool,
            receipt_id,
            last_updated,
        )
        .await
    }

    pub async fn list_work_assignments(
        &self,
        work_request_id: Option<&str>,
        worker_id: Option<&str>,
    ) -> Result<Vec<WorkAssignmentRecord>, StorageError> {
        work_assignments_repo::list_work_assignments(&self.pool, work_request_id, worker_id).await
    }

    pub async fn upsert_cluster_worker(
        &self,
        worker: ClusterWorkerUpsert,
    ) -> Result<(), StorageError> {
        cluster_workers_repo::upsert_cluster_worker(&self.pool, worker).await
    }

    pub async fn expire_cluster_workers(&self, stale_before: i64) -> Result<u64, StorageError> {
        cluster_workers_repo::expire_cluster_workers(&self.pool, stale_before).await
    }

    pub async fn list_cluster_workers(&self) -> Result<Vec<ClusterWorkerRecord>, StorageError> {
        cluster_workers_repo::list_cluster_workers(&self.pool).await
    }

    pub async fn get_runtime_loop(
        &self,
        loop_kind: &str,
    ) -> Result<Option<RuntimeLoopRecord>, StorageError> {
        runtime_loops_repo::get_runtime_loop(&self.pool, loop_kind).await
    }

    pub async fn update_runtime_loop_config(
        &self,
        loop_kind: &str,
        enabled: Option<bool>,
        interval_seconds: Option<i64>,
    ) -> Result<Option<RuntimeLoopRecord>, StorageError> {
        runtime_loops_repo::update_runtime_loop_config(
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
