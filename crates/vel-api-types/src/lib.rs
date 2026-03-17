use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{
    ArtifactId, ArtifactStorageKind, CaptureId, CommitmentId, PrivacyClass, RiskFactors,
    RiskSnapshot, RunId, SyncClass,
};

/// Wire-level timestamp for resource DTO fields that use Unix seconds.
pub type UnixSeconds = i64;

/// Wire-level timestamp for envelope-style events that use RFC3339 strings.
pub type Rfc3339Timestamp = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMeta {
    pub request_id: String,
    #[serde(default)]
    pub degraded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiErrorDetail>,
    #[serde(default)]
    pub warnings: Vec<String>,
    pub meta: ApiMeta,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, request_id: impl Into<String>) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
            warnings: Vec::new(),
            meta: ApiMeta {
                request_id: request_id.into(),
                degraded: false,
            },
        }
    }

    pub fn error(
        code: impl Into<String>,
        message: impl Into<String>,
        request_id: impl Into<String>,
    ) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(ApiErrorDetail {
                code: code.into(),
                message: message.into(),
            }),
            warnings: Vec::new(),
            meta: ApiMeta {
                request_id: request_id.into(),
                degraded: false,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthData {
    pub status: String,
    pub db: String,
    pub version: String,
}

/// Status of a single diagnostic check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticStatus {
    Ok,
    Warn,
    Fail,
}

/// A single diagnostic check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    pub name: String,
    pub status: DiagnosticStatus,
    pub message: String,
}

/// Results of diagnostic checks for `vel doctor`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorData {
    pub checks: Vec<DiagnosticCheck>,
    pub schema_version: u32,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureCreateRequest {
    pub content_text: String,
    #[serde(default = "default_capture_type")]
    pub capture_type: String,
    pub source_device: Option<String>,
}

fn default_capture_type() -> String {
    "quick_note".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureCreateResponse {
    pub capture_id: CaptureId,
    pub accepted_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchQuery {
    pub q: String,
    pub capture_type: Option<String>,
    pub source_device: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub capture_id: CaptureId,
    pub capture_type: String,
    pub snippet: String,
    pub occurred_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub source_device: Option<String>,
}

impl From<vel_core::SearchResult> for SearchResult {
    fn from(s: vel_core::SearchResult) -> Self {
        Self {
            capture_id: s.capture_id,
            capture_type: s.capture_type,
            snippet: s.snippet,
            occurred_at: s.occurred_at,
            created_at: s.created_at,
            source_device: s.source_device,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCapture {
    pub capture_id: CaptureId,
    pub capture_type: String,
    pub content_text: String,
    pub occurred_at: OffsetDateTime,
    pub source_device: Option<String>,
}

impl From<vel_core::ContextCapture> for ContextCapture {
    fn from(c: vel_core::ContextCapture) -> Self {
        Self {
            capture_id: c.capture_id,
            capture_type: c.capture_type,
            content_text: c.content_text,
            occurred_at: c.occurred_at,
            source_device: c.source_device,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodayData {
    pub date: String,
    pub recent_captures: Vec<ContextCapture>,
    pub focus_candidates: Vec<String>,
    pub reminders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorningData {
    pub date: String,
    pub top_active_threads: Vec<String>,
    pub pending_commitments: Vec<String>,
    pub suggested_focus: Option<String>,
    pub key_reminders: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfDayData {
    pub date: String,
    pub what_was_done: Vec<ContextCapture>,
    pub what_remains_open: Vec<String>,
    pub what_may_matter_tomorrow: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationCalendarData {
    pub id: String,
    pub summary: String,
    pub primary: bool,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationGuidanceData {
    pub title: String,
    pub detail: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCalendarIntegrationData {
    pub configured: bool,
    pub connected: bool,
    pub has_client_id: bool,
    pub has_client_secret: bool,
    pub calendars: Vec<IntegrationCalendarData>,
    pub all_calendars_selected: bool,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoistIntegrationData {
    pub configured: bool,
    pub connected: bool,
    pub has_api_token: bool,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalIntegrationData {
    pub configured: bool,
    pub source_path: Option<String>,
    pub last_sync_at: Option<UnixSeconds>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub last_item_count: Option<u32>,
    pub guidance: Option<IntegrationGuidanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationsData {
    pub google_calendar: GoogleCalendarIntegrationData,
    pub todoist: TodoistIntegrationData,
    pub activity: LocalIntegrationData,
    pub git: LocalIntegrationData,
    pub messaging: LocalIntegrationData,
    pub notes: LocalIntegrationData,
    pub transcripts: LocalIntegrationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleCalendarAuthStartData {
    pub auth_url: String,
}

// --- Chat / Web surfaces ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationData {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
    pub created_at: UnixSeconds,
    pub updated_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationCreateRequest {
    pub title: Option<String>,
    #[serde(default = "default_conversation_kind")]
    pub kind: String,
}

fn default_conversation_kind() -> String {
    "general".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationUpdateRequest {
    pub title: Option<String>,
    pub pinned: Option<bool>,
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub kind: String,
    pub content: JsonValue,
    pub status: Option<String>,
    pub importance: Option<String>,
    pub created_at: UnixSeconds,
    pub updated_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageResponse {
    pub user_message: MessageData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_message: Option<MessageData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCreateRequest {
    pub role: String,
    pub kind: String,
    pub content: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxItemData {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: UnixSeconds,
    pub snoozed_until: Option<UnixSeconds>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionActionData {
    pub id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceData {
    pub message_id: String,
    pub events: Vec<ProvenanceEvent>,
    pub signals: Vec<JsonValue>,
    pub policy_decisions: Vec<JsonValue>,
    pub linked_objects: Vec<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEvent {
    pub id: String,
    pub event_name: String,
    pub created_at: UnixSeconds,
    pub payload: JsonValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WsEventType {
    #[serde(rename = "messages:new")]
    MessagesNew,
    #[serde(rename = "interventions:new")]
    InterventionsNew,
    #[serde(rename = "interventions:updated")]
    InterventionsUpdated,
    #[serde(rename = "context:updated")]
    ContextUpdated,
    #[serde(rename = "runs:updated")]
    RunsUpdated,
    #[serde(rename = "components:updated")]
    ComponentsUpdated,
}

impl std::fmt::Display for WsEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::MessagesNew => "messages:new",
            Self::InterventionsNew => "interventions:new",
            Self::InterventionsUpdated => "interventions:updated",
            Self::ContextUpdated => "context:updated",
            Self::RunsUpdated => "runs:updated",
            Self::ComponentsUpdated => "components:updated",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for WsEventType {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "messages:new" => Ok(Self::MessagesNew),
            "interventions:new" => Ok(Self::InterventionsNew),
            "interventions:updated" => Ok(Self::InterventionsUpdated),
            "context:updated" => Ok(Self::ContextUpdated),
            "runs:updated" => Ok(Self::RunsUpdated),
            "components:updated" => Ok(Self::ComponentsUpdated),
            other => Err(format!("unknown websocket event type: {}", other)),
        }
    }
}

impl From<&str> for WsEventType {
    fn from(value: &str) -> Self {
        value
            .parse()
            .unwrap_or_else(|_| panic!("invalid websocket event type: {}", value))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsEnvelope {
    #[serde(rename = "type")]
    pub event_type: WsEventType,
    pub timestamp: Rfc3339Timestamp,
    pub payload: JsonValue,
}

impl WsEnvelope {
    pub fn new(event_type: impl Into<WsEventType>, payload: JsonValue) -> Self {
        Self {
            event_type: event_type.into(),
            timestamp: OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .expect("current timestamp should format as RFC3339"),
            payload,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactCreateRequest {
    pub artifact_type: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub storage_uri: String,
    #[serde(default)]
    pub storage_kind: ArtifactStorageKind,
    #[serde(default)]
    pub privacy_class: PrivacyClass,
    #[serde(default)]
    pub sync_class: SyncClass,
    pub content_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactCreateResponse {
    pub artifact_id: ArtifactId,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactData {
    pub artifact_id: ArtifactId,
    pub artifact_type: String,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub storage_uri: String,
    pub storage_kind: String,
    pub privacy_class: String,
    pub sync_class: String,
    pub content_hash: Option<String>,
    pub size_bytes: Option<i64>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

// --- Runs (spec Section 15) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummaryData {
    pub id: RunId,
    pub kind: String,
    pub status: String,
    pub automatic_retry_supported: bool,
    pub automatic_retry_reason: Option<String>,
    pub unsupported_retry_override: bool,
    pub unsupported_retry_override_reason: Option<String>,
    pub created_at: OffsetDateTime,
    pub started_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
    /// Duration in milliseconds; present when run has started_at and finished_at.
    pub duration_ms: Option<i64>,
    /// Optional retry schedule metadata for operator workflows.
    pub retry_scheduled_at: Option<OffsetDateTime>,
    /// Optional operator reason attached when scheduling a retry.
    pub retry_reason: Option<String>,
    /// Optional operator reason attached when marking a run blocked.
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunEventData {
    pub seq: u32,
    pub event_type: String,
    pub payload: JsonValue,
    pub created_at: OffsetDateTime,
}

/// Summary of an artifact linked to a run (e.g. via refs).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactSummaryData {
    pub artifact_id: ArtifactId,
    pub artifact_type: String,
    pub title: Option<String>,
    pub storage_uri: String,
    pub storage_kind: String,
    pub size_bytes: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunUpdateRequest {
    pub status: String,
    #[serde(default, alias = "retry_scheduled_at")]
    pub retry_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub retry_after_seconds: Option<u32>,
    #[serde(default, alias = "retry_reason")]
    pub reason: Option<String>,
    #[serde(default)]
    pub allow_unsupported_retry: bool,
    #[serde(default)]
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunDetailData {
    pub id: RunId,
    pub kind: String,
    pub status: String,
    pub automatic_retry_supported: bool,
    pub automatic_retry_reason: Option<String>,
    pub unsupported_retry_override: bool,
    pub unsupported_retry_override_reason: Option<String>,
    pub input: JsonValue,
    pub output: Option<JsonValue>,
    pub error: Option<JsonValue>,
    pub created_at: OffsetDateTime,
    pub started_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
    /// Duration in milliseconds; present when run has started_at and finished_at.
    pub duration_ms: Option<i64>,
    /// Optional retry schedule metadata for operator workflows.
    pub retry_scheduled_at: Option<OffsetDateTime>,
    /// Optional operator reason attached when scheduling a retry.
    pub retry_reason: Option<String>,
    /// Optional operator reason attached when marking a run blocked.
    pub blocked_reason: Option<String>,
    pub events: Vec<RunEventData>,
    pub artifacts: Vec<ArtifactSummaryData>,
}

// --- Commitments ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentCreateRequest {
    pub text: String,
    #[serde(default = "default_commitment_source_type")]
    pub source_type: String,
    pub source_id: Option<String>,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    #[serde(default)]
    pub metadata: JsonValue,
}

fn default_commitment_source_type() -> String {
    "manual".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentData {
    pub id: CommitmentId,
    pub text: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub status: String,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    pub created_at: OffsetDateTime,
    pub resolved_at: Option<OffsetDateTime>,
    pub metadata: JsonValue,
}

impl From<vel_core::Commitment> for CommitmentData {
    fn from(c: vel_core::Commitment) -> Self {
        Self {
            id: c.id,
            text: c.text,
            source_type: c.source_type,
            source_id: c.source_id,
            status: c.status.to_string(),
            due_at: c.due_at,
            project: c.project,
            commitment_kind: c.commitment_kind,
            created_at: c.created_at,
            resolved_at: c.resolved_at,
            metadata: c.metadata_json,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommitmentUpdateRequest {
    pub status: Option<String>,
    pub due_at: Option<Option<OffsetDateTime>>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentDependencyData {
    pub id: String,
    pub parent_commitment_id: String,
    pub child_commitment_id: String,
    pub dependency_type: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentDependencyCreateRequest {
    pub child_commitment_id: String,
    #[serde(default = "default_dependency_type")]
    pub dependency_type: String,
}

fn default_dependency_type() -> String {
    "blocks".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskData {
    pub commitment_id: String,
    pub risk_score: f64,
    pub risk_level: String,
    pub factors: RiskFactorsData,
    pub computed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactorsData {
    pub consequence: f64,
    pub proximity: f64,
    pub dependency_pressure: f64,
    pub external_anchor: f64,
    pub stale_open_age: f64,
    pub reasons: Vec<String>,
    pub dependency_ids: Vec<String>,
}

impl From<RiskFactors> for RiskFactorsData {
    fn from(value: RiskFactors) -> Self {
        Self {
            consequence: value.consequence,
            proximity: value.proximity,
            dependency_pressure: value.dependency_pressure,
            external_anchor: value.external_anchor,
            stale_open_age: value.stale_open_age,
            reasons: value.reasons,
            dependency_ids: value.dependency_ids,
        }
    }
}

impl From<RiskSnapshot> for RiskData {
    fn from(snapshot: RiskSnapshot) -> Self {
        Self {
            commitment_id: snapshot.commitment_id,
            risk_score: snapshot.risk_score,
            risk_level: snapshot.risk_level,
            factors: snapshot.factors.into(),
            computed_at: snapshot.computed_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionData {
    pub id: String,
    pub suggestion_type: String,
    pub state: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub priority: i64,
    pub confidence: Option<String>,
    pub evidence_count: u32,
    pub decision_context_summary: Option<String>,
    pub decision_context: Option<JsonValue>,
    pub evidence: Option<Vec<SuggestionEvidenceData>>,
    pub payload: JsonValue,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionEvidenceData {
    pub id: String,
    pub evidence_type: String,
    pub ref_id: String,
    pub evidence: Option<JsonValue>,
    pub weight: Option<f64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionUpdateRequest {
    pub state: Option<String>,
    pub payload: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SuggestionActionRequest {
    pub reason: Option<String>,
}

// --- Signals (Phase B) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalCreateRequest {
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: Option<i64>,
    #[serde(default)]
    pub payload: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalData {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: i64,
    pub payload: JsonValue,
    pub created_at: i64,
}

// --- Nudges (Phase D) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NudgeData {
    pub nudge_id: String,
    pub nudge_type: String,
    pub level: String,
    pub state: String,
    pub related_commitment_id: Option<String>,
    pub message: String,
    pub created_at: i64,
    pub snoozed_until: Option<i64>,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NudgeSnoozeRequest {
    #[serde(default = "default_snooze_minutes")]
    pub minutes: u32,
}

fn default_snooze_minutes() -> u32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResultData {
    pub source: String,
    pub signals_ingested: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopData {
    pub kind: String,
    pub enabled: bool,
    pub interval_seconds: i64,
    pub last_started_at: Option<UnixSeconds>,
    pub last_finished_at: Option<UnixSeconds>,
    pub last_status: Option<String>,
    pub last_error: Option<String>,
    pub next_due_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopUpdateRequest {
    pub enabled: Option<bool>,
    pub interval_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyData {
    pub id: String,
    pub subject_type: String,
    pub subject_id: Option<String>,
    pub decision_kind: String,
    pub confidence_band: String,
    pub confidence_score: Option<f64>,
    pub reasons: JsonValue,
    pub missing_evidence: Option<JsonValue>,
    pub resolution_mode: String,
    pub status: String,
    pub created_at: UnixSeconds,
    pub resolved_at: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub last_restarted_at: Option<i64>,
    pub last_error: Option<String>,
    pub restart_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentLogEventData {
    pub id: String,
    pub component_id: String,
    pub event_name: String,
    pub status: String,
    pub message: String,
    pub payload: JsonValue,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationLogEventData {
    pub id: String,
    pub integration_id: String,
    pub event_name: String,
    pub status: String,
    pub message: String,
    pub payload: JsonValue,
    pub created_at: UnixSeconds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluateResultData {
    pub inferred_states: u32,
    pub nudges_created_or_updated: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisWeekData {
    pub run_id: String,
    pub artifact_id: String,
}

/// Persistent current context singleton (computed by inference engine).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentContextData {
    pub computed_at: UnixSeconds,
    pub context: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowLabelData {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowRiskSummaryData {
    pub level: String,
    pub score: Option<f64>,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowSummaryData {
    pub mode: NowLabelData,
    pub phase: NowLabelData,
    pub meds: NowLabelData,
    pub risk: NowRiskSummaryData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowEventData {
    pub title: String,
    pub start_ts: UnixSeconds,
    pub end_ts: Option<UnixSeconds>,
    pub location: Option<String>,
    pub prep_minutes: Option<i64>,
    pub travel_minutes: Option<i64>,
    pub leave_by_ts: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTaskData {
    pub id: String,
    pub text: String,
    pub source_type: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowScheduleData {
    pub empty_message: Option<String>,
    pub next_event: Option<NowEventData>,
    pub upcoming_events: Vec<NowEventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTasksData {
    pub todoist: Vec<NowTaskData>,
    pub other_open: Vec<NowTaskData>,
    pub next_commitment: Option<NowTaskData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowAttentionData {
    pub state: NowLabelData,
    pub drift: NowLabelData,
    pub severity: NowLabelData,
    pub confidence: Option<f64>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowSourceActivityData {
    pub label: String,
    pub timestamp: UnixSeconds,
    pub summary: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowSourcesData {
    pub git_activity: Option<NowSourceActivityData>,
    pub note_document: Option<NowSourceActivityData>,
    pub assistant_message: Option<NowSourceActivityData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowFreshnessEntryData {
    pub key: String,
    pub label: String,
    pub status: String,
    pub last_sync_at: Option<UnixSeconds>,
    pub age_seconds: Option<UnixSeconds>,
    pub guidance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowFreshnessData {
    pub overall_status: String,
    pub sources: Vec<NowFreshnessEntryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowDebugData {
    pub raw_context: JsonValue,
    pub signals_used: Vec<String>,
    pub commitments_used: Vec<String>,
    pub risk_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowData {
    pub computed_at: UnixSeconds,
    pub timezone: String,
    pub summary: NowSummaryData,
    pub schedule: NowScheduleData,
    pub tasks: NowTasksData,
    pub attention: NowAttentionData,
    pub sources: NowSourcesData,
    pub freshness: NowFreshnessData,
    pub reasons: Vec<String>,
    pub debug: NowDebugData,
}

/// One entry in the context timeline (material context transitions).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTimelineEntry {
    pub id: String,
    pub timestamp: i64,
    pub context: JsonValue,
}

/// Thread summary/list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadData {
    pub id: String,
    pub thread_type: String,
    pub title: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ThreadLinkData>>,
}

/// Thread link (entity linked to a thread).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadLinkData {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadCreateRequest {
    pub thread_type: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_json: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadLinkRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub relation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadUpdateRequest {
    pub status: Option<String>,
}

/// Explain payload for current context (context + reasons + entity ids used).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalExplainSummary {
    pub signal_id: String,
    pub signal_type: String,
    pub source: String,
    pub timestamp: UnixSeconds,
    pub summary: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSourceSummaryData {
    pub timestamp: UnixSeconds,
    pub summary: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSourceSummariesData {
    pub git_activity: Option<ContextSourceSummaryData>,
    pub note_document: Option<ContextSourceSummaryData>,
    pub assistant_message: Option<ContextSourceSummaryData>,
}

/// Explain payload for current context (context + reasons + entity ids used).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextExplainData {
    pub computed_at: UnixSeconds,
    pub mode: Option<String>,
    pub morning_state: Option<String>,
    pub context: JsonValue,
    pub source_summaries: ContextSourceSummariesData,
    pub signals_used: Vec<String>,
    pub signal_summaries: Vec<SignalExplainSummary>,
    pub commitments_used: Vec<String>,
    pub risk_used: Vec<String>,
    pub reasons: Vec<String>,
}

/// Explain payload for a commitment (commitment + risk snapshot + why in context).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentExplainData {
    pub commitment_id: String,
    pub commitment: JsonValue,
    pub risk: Option<JsonValue>,
    pub in_context_reasons: Vec<String>,
}

/// Explain payload for drift (attention/drift state from current context).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftExplainData {
    pub attention_state: Option<String>,
    pub drift_type: Option<String>,
    pub drift_severity: Option<String>,
    pub confidence: Option<f64>,
    pub reasons: Vec<String>,
    pub signals_used: Vec<String>,
    pub signal_summaries: Vec<SignalExplainSummary>,
    pub commitments_used: Vec<String>,
}

/// Explain payload for a nudge (nudge + inference/signals snapshots for explainability).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NudgeExplainData {
    pub nudge_id: String,
    pub nudge_type: String,
    pub level: String,
    pub state: String,
    pub message: String,
    pub inference_snapshot: Option<JsonValue>,
    pub signals_snapshot: Option<JsonValue>,
    pub events: Vec<NudgeEventData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NudgeEventData {
    pub id: String,
    pub event_type: String,
    pub payload: JsonValue,
    pub timestamp: i64,
    pub created_at: i64,
}

#[cfg(test)]
mod tests {
    use super::NowTaskData;
    use time::macros::datetime;

    #[test]
    fn now_task_due_at_serializes_as_rfc3339_string() {
        let task = NowTaskData {
            id: "commit_1".to_string(),
            text: "Reply to Dimitri".to_string(),
            source_type: "todoist".to_string(),
            due_at: Some(datetime!(2026-03-16 19:00:00 UTC)),
            project: None,
            commitment_kind: Some("todo".to_string()),
        };

        let value = serde_json::to_value(task).expect("now task should serialize");
        assert_eq!(value["due_at"], "2026-03-16T19:00:00Z");
    }

    #[test]
    fn now_task_none_due_at_serializes_as_null() {
        let task = NowTaskData {
            id: "commit_2".to_string(),
            text: "Inbox zero".to_string(),
            source_type: "manual".to_string(),
            due_at: None,
            project: None,
            commitment_kind: None,
        };

        let value = serde_json::to_value(task).expect("now task should serialize");
        assert!(value["due_at"].is_null());
    }
}
