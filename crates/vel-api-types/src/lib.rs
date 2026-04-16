use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{ActionItemId, CommitmentId, ProjectId, RiskFactors, RiskSnapshot};

mod actions;
mod agent_grounding;
mod agent_runtime;
mod apple;
mod artifacts;
mod assistant_entry;
mod backup;
mod batch_import;
mod capture;
mod chat;
mod check_in;
mod client_sync;
mod commands;
mod common;
mod conflicts;
mod connect;
mod context;
mod daily_loop;
mod doctor;
mod execution;
mod explain;
mod health;
mod integrations;
mod linking;
mod loops;
mod nudges;
mod people;
mod planning;
mod planning_profile;
mod projects;
mod provenance;
mod responses;
mod reviews;
mod runs;
mod signals;
mod sync;
mod threads;
mod websocket;
mod writebacks;

pub use actions::*;
pub use agent_grounding::*;
pub use agent_runtime::*;
pub use apple::*;
pub use artifacts::*;
pub use assistant_entry::*;
pub use backup::*;
pub use batch_import::*;
pub use capture::*;
pub use chat::*;
pub use check_in::*;
pub use client_sync::*;
pub use commands::*;
pub use common::*;
pub use conflicts::*;
pub use connect::*;
pub use context::*;
pub use daily_loop::*;
pub use doctor::*;
pub use execution::*;
pub use explain::*;
pub use health::*;
pub use integrations::*;
pub use linking::*;
pub use loops::*;
pub use nudges::*;
pub use people::*;
pub use planning::*;
pub use planning_profile::*;
pub use projects::*;
pub use provenance::*;
pub use responses::*;
pub use reviews::*;
pub use runs::*;
pub use signals::*;
pub use sync::*;
pub use threads::*;
pub use websocket::*;
pub use writebacks::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistantProposalStateData {
    Staged,
    Approved,
    Applied,
    Failed,
    Reversed,
}

impl From<vel_core::AssistantProposalState> for AssistantProposalStateData {
    fn from(value: vel_core::AssistantProposalState) -> Self {
        match value {
            vel_core::AssistantProposalState::Staged => Self::Staged,
            vel_core::AssistantProposalState::Approved => Self::Approved,
            vel_core::AssistantProposalState::Applied => Self::Applied,
            vel_core::AssistantProposalState::Failed => Self::Failed,
            vel_core::AssistantProposalState::Reversed => Self::Reversed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantActionProposalData {
    pub action_item_id: ActionItemId,
    pub state: AssistantProposalStateData,
    pub kind: ActionKindData,
    pub permission_mode: ActionPermissionModeData,
    pub scope_affinity: ActionScopeAffinityData,
    pub title: String,
    pub summary: String,
    pub project_id: Option<ProjectId>,
    pub project_label: Option<String>,
    pub project_family: Option<ProjectFamilyData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_route: Option<ActionThreadRouteData>,
}

impl From<vel_core::AssistantActionProposal> for AssistantActionProposalData {
    fn from(value: vel_core::AssistantActionProposal) -> Self {
        Self {
            action_item_id: value.action_item_id,
            state: value.state.into(),
            kind: value.kind.into(),
            permission_mode: value.permission_mode.into(),
            scope_affinity: value.scope_affinity.into(),
            title: value.title,
            summary: value.summary,
            project_id: value.project_id,
            project_label: value.project_label,
            project_family: value.project_family.map(Into::into),
            thread_route: value.thread_route.map(Into::into),
        }
    }
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
    #[serde(default)]
    pub conversation_id: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub project_id: Option<ProjectId>,
    #[serde(default)]
    pub project_label: Option<String>,
    #[serde(default)]
    pub available_actions: Vec<AvailableActionData>,
    #[serde(default)]
    pub evidence: Vec<ActionEvidenceRefData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionActionData {
    pub id: String,
    pub state: String,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitmentData {
    pub id: CommitmentId,
    pub text: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub status: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub resolved_at: Option<OffsetDateTime>,
    #[serde(default)]
    pub scheduler_rules: CanonicalScheduleRulesData,
    pub metadata: JsonValue,
}

impl From<vel_core::Commitment> for CommitmentData {
    fn from(c: vel_core::Commitment) -> Self {
        let scheduler_rules = c.scheduler_rules();
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
            scheduler_rules: scheduler_rules.into(),
            metadata: c.metadata_json,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CanonicalScheduleRulesData {
    #[serde(default)]
    pub block_target: Option<String>,
    #[serde(default)]
    pub duration_minutes: Option<i64>,
    #[serde(default)]
    pub calendar_free: bool,
    #[serde(default)]
    pub fixed_start: bool,
    #[serde(default)]
    pub time_window: Option<ScheduleTimeWindowData>,
    #[serde(default)]
    pub local_urgency: bool,
    #[serde(default)]
    pub local_defer: bool,
}

impl From<vel_core::CanonicalScheduleRules> for CanonicalScheduleRulesData {
    fn from(value: vel_core::CanonicalScheduleRules) -> Self {
        Self {
            block_target: value.block_target,
            duration_minutes: value.duration_minutes,
            calendar_free: value.calendar_free,
            fixed_start: value.fixed_start,
            time_window: value.time_window.map(Into::into),
            local_urgency: value.local_urgency,
            local_defer: value.local_defer,
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
        let normalized_level = snapshot.normalized_level().to_string();
        Self {
            commitment_id: snapshot.commitment_id,
            risk_score: snapshot.risk_score,
            risk_level: normalized_level,
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
    #[serde(default)]
    pub latest_feedback_outcome: Option<String>,
    #[serde(default)]
    pub latest_feedback_notes: Option<String>,
    #[serde(default)]
    pub adaptive_policy: Option<SuggestionAdaptivePolicyData>,
    pub payload: JsonValue,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptivePolicyOverrideData {
    pub policy_key: String,
    pub value_minutes: u32,
    pub source_suggestion_id: Option<String>,
    pub source_title: Option<String>,
    pub source_accepted_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionAdaptivePolicyData {
    pub policy_key: String,
    pub suggested_minutes: u32,
    pub current_minutes: Option<u32>,
    pub is_active_source: bool,
    pub active_override: Option<AdaptivePolicyOverrideData>,
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

/// A single freshness entry for a data source tracked by the operator diagnostics endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessEntryData {
    /// Identifier for the data source (e.g. worker ID or source name).
    pub source: String,
    /// Unix timestamp of last successful heartbeat or sync for this source.
    pub last_seen_at: Option<UnixSeconds>,
    /// Freshness status: "fresh" | "stale" | "missing"
    pub status: String,
}

/// Operator diagnostics payload — surfaces currently available sync/capability state.
/// Returned by GET /api/diagnostics (operator-authenticated).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsData {
    /// Node ID of the authority node.
    pub node_id: String,
    /// Human-readable display name for the authority node.
    pub node_display_name: String,
    /// Unix timestamp when this diagnostics snapshot was generated.
    pub generated_at: UnixSeconds,
    /// Overall sync status: "ready" | "degraded" | "offline" | "unknown"
    pub sync_status: String,
    /// Count of currently active (registered) workers.
    pub active_workers: u32,
    /// Unique capability strings advertised across all active workers.
    pub capability_summary: Vec<String>,
    /// Per-source freshness entries derived from active worker heartbeat data.
    pub freshness_entries: Vec<FreshnessEntryData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisWeekData {
    pub run_id: String,
    pub artifact_id: String,
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
pub struct NowOverviewActionData {
    pub kind: String,
    pub title: String,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowOverviewTimelineEntryData {
    pub kind: String,
    pub title: String,
    pub timestamp: UnixSeconds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowOverviewNudgeData {
    pub kind: String,
    pub title: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowOverviewWhyStateData {
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowOverviewSuggestionData {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NowHeaderBucketKindData {
    ThreadsByType,
    NeedsInput,
    NewNudges,
    SearchFilter,
    Snoozed,
    ReviewApply,
    Reflow,
    FollowUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NowCountDisplayModeData {
    AlwaysShow,
    ShowNonzero,
    HiddenUntilActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowThreadFilterTargetData {
    pub bucket: NowHeaderBucketKindData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowHeaderBucketData {
    pub kind: NowHeaderBucketKindData,
    pub count: u32,
    pub count_display: NowCountDisplayModeData,
    pub urgent: bool,
    pub route_target: NowThreadFilterTargetData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowHeaderData {
    pub title: String,
    #[serde(default)]
    pub buckets: Vec<NowHeaderBucketData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NowMeshSyncStateData {
    Synced,
    Stale,
    LocalOnly,
    Offline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NowRepairRouteTargetData {
    SettingsSync,
    SettingsLinking,
    SettingsRecovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowRepairRouteData {
    pub target: NowRepairRouteTargetData,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowMeshSummaryData {
    pub authority_node_id: String,
    pub authority_label: String,
    pub sync_state: NowMeshSyncStateData,
    pub linked_node_count: u32,
    pub queued_write_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_sync_at: Option<UnixSeconds>,
    pub urgent: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_route: Option<NowRepairRouteData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowStatusRowData {
    pub date_label: String,
    pub time_label: String,
    pub context_label: String,
    pub elapsed_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowContextLineData {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    pub fallback_used: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NowNudgeBarKindData {
    Nudge,
    NeedsInput,
    ReviewRequest,
    ReflowProposal,
    ThreadContinuation,
    TrustWarning,
    FreshnessWarning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowNudgeActionData {
    pub kind: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowNudgeBarData {
    pub id: String,
    pub kind: NowNudgeBarKindData,
    pub title: String,
    pub summary: String,
    pub urgent: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_thread_id: Option<String>,
    #[serde(default)]
    pub actions: Vec<NowNudgeActionData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NowTaskKindData {
    Task,
    Commitment,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTaskLaneItemData {
    pub id: String,
    pub task_kind: NowTaskKindData,
    pub text: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lane: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_thread_id: Option<String>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub due_at: Option<OffsetDateTime>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_label: Option<String>,
    #[serde(default)]
    pub is_overdue: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline_label: Option<String>,
    #[serde(default)]
    pub deadline_passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTaskLaneData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<NowTaskLaneItemData>,
    #[serde(default)]
    pub pending: Vec<NowTaskLaneItemData>,
    #[serde(default)]
    pub active_items: Vec<NowTaskLaneItemData>,
    #[serde(default)]
    pub next_up: Vec<NowTaskLaneItemData>,
    #[serde(default)]
    pub inbox: Vec<NowTaskLaneItemData>,
    #[serde(default)]
    pub if_time_allows: Vec<NowTaskLaneItemData>,
    #[serde(default)]
    pub completed: Vec<NowTaskLaneItemData>,
    #[serde(default)]
    pub recent_completed: Vec<NowTaskLaneItemData>,
    pub overflow_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowNextUpItemData {
    pub kind: NowTaskKindData,
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<NowTaskLaneItemData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NowProgressData {
    pub base_count: u32,
    pub completed_count: u32,
    pub backlog_count: u32,
    pub completed_ratio: f64,
    pub backlog_ratio: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebSettingsData {
    #[serde(default = "default_web_settings_dense_rows")]
    pub dense_rows: bool,
    #[serde(default = "default_web_settings_tabular_numbers")]
    pub tabular_numbers: bool,
    #[serde(default)]
    pub reduced_motion: bool,
    #[serde(default = "default_web_settings_strong_focus")]
    pub strong_focus: bool,
    #[serde(default = "default_web_settings_docked_action_bar")]
    pub docked_action_bar: bool,
    #[serde(default)]
    pub semantic_aliases:
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>,
}

impl Default for WebSettingsData {
    fn default() -> Self {
        Self {
            dense_rows: default_web_settings_dense_rows(),
            tabular_numbers: default_web_settings_tabular_numbers(),
            reduced_motion: false,
            strong_focus: default_web_settings_strong_focus(),
            docked_action_bar: default_web_settings_docked_action_bar(),
            semantic_aliases: std::collections::BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmProfileHealthData {
    pub profile_id: String,
    pub healthy: bool,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmProfileHandshakeRequestData {
    pub profile_id: Option<String>,
    pub provider: String,
    pub base_url: String,
    pub model: String,
    pub context_window: Option<u32>,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmOpenAiOauthLaunchRequestData {
    pub profile_id: Option<String>,
    pub base_url: String,
}

fn default_web_settings_dense_rows() -> bool {
    true
}

fn default_web_settings_tabular_numbers() -> bool {
    true
}

fn default_web_settings_strong_focus() -> bool {
    true
}

fn default_web_settings_docked_action_bar() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NowDockedInputIntentData {
    Task,
    Url,
    Question,
    Note,
    Command,
    Continuation,
    Reflection,
    Scheduling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowDockedInputData {
    #[serde(default)]
    pub supported_intents: Vec<NowDockedInputIntentData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_thread_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_capture_thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NowOverviewData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dominant_action: Option<NowOverviewActionData>,
    #[serde(default)]
    pub today_timeline: Vec<NowOverviewTimelineEntryData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_nudge: Option<NowOverviewNudgeData>,
    #[serde(default)]
    pub why_state: Vec<NowOverviewWhyStateData>,
    #[serde(default)]
    pub suggestions: Vec<NowOverviewSuggestionData>,
    #[serde(default)]
    pub decision_options: Vec<String>,
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
    pub event_id: Option<String>,
    pub calendar_id: Option<String>,
    pub calendar_name: Option<String>,
    pub title: String,
    pub start_ts: UnixSeconds,
    pub end_ts: Option<UnixSeconds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default)]
    pub attendees: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_provider: Option<String>,
    pub prep_minutes: Option<i64>,
    pub travel_minutes: Option<i64>,
    pub leave_by_ts: Option<UnixSeconds>,
    #[serde(default)]
    pub rescheduled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowCalendarEventRescheduleRequestData {
    pub event_id: String,
    pub calendar_id: Option<String>,
    pub start_ts: UnixSeconds,
    pub end_ts: Option<UnixSeconds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTaskData {
    pub id: String,
    pub text: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub source_type: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub due_at: Option<OffsetDateTime>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowScheduleData {
    pub empty_message: Option<String>,
    pub next_event: Option<NowEventData>,
    pub upcoming_events: Vec<NowEventData>,
    #[serde(default)]
    pub following_day_events: Vec<NowEventData>,
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
    pub health: Option<NowSourceActivityData>,
    pub mood: Option<NowSourceActivityData>,
    pub pain: Option<NowSourceActivityData>,
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
pub struct TrustReadinessFacetData {
    pub level: String,
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustReadinessReviewData {
    pub open_action_count: u32,
    pub pending_execution_reviews: u32,
    pub pending_writeback_count: u32,
    pub conflict_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustReadinessData {
    pub level: String,
    pub headline: String,
    pub summary: String,
    pub backup: TrustReadinessFacetData,
    pub freshness: TrustReadinessFacetData,
    pub review: TrustReadinessReviewData,
    #[serde(default)]
    pub guidance: Vec<String>,
    #[serde(default)]
    pub follow_through: Vec<ActionItemData>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<NowHeaderData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_summary: Option<NowMeshSummaryData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_row: Option<NowStatusRowData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_line: Option<NowContextLineData>,
    #[serde(default)]
    pub nudge_bars: Vec<NowNudgeBarData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_lane: Option<NowTaskLaneData>,
    #[serde(default)]
    pub next_up_items: Vec<NowNextUpItemData>,
    #[serde(default)]
    pub progress: NowProgressData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docked_input: Option<NowDockedInputData>,
    #[serde(default)]
    pub overview: NowOverviewData,
    pub summary: NowSummaryData,
    pub schedule: NowScheduleData,
    pub tasks: NowTasksData,
    pub attention: NowAttentionData,
    pub sources: NowSourcesData,
    pub freshness: NowFreshnessData,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_readiness: Option<TrustReadinessData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planning_profile_summary: Option<PlanningProfileProposalSummaryData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commitment_scheduling_summary: Option<CommitmentSchedulingProposalSummaryData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in: Option<CheckInCardData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_plan: Option<DayPlanProposalData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflow: Option<ReflowCardData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflow_status: Option<CurrentContextReflowStatusData>,
    #[serde(default)]
    pub action_items: Vec<ActionItemData>,
    #[serde(default)]
    pub review_snapshot: ReviewSnapshotData,
    #[serde(default)]
    pub pending_writebacks: Vec<WritebackOperationData>,
    #[serde(default)]
    pub conflicts: Vec<ConflictCaseData>,
    #[serde(default)]
    pub people: Vec<PersonRecordData>,
    pub reasons: Vec<String>,
    pub debug: NowDebugData,
}

#[cfg(test)]
mod tests {
    use super::{
        AgentBlockerData, AgentCapabilityEntryData, AgentCapabilityGroupKindData, AgentInspectData,
        AgentProfileData, AppleBehaviorMetricData, AppleBehaviorSummaryData,
        AppleBehaviorSummaryScopeData, AppleClientSurfaceData, AppleRequestedOperationData,
        AppleResponseEvidenceData, AppleResponseModeData, AppleScheduleEventData,
        AppleScheduleSnapshotData, AppleTurnProvenanceData, AppleVoiceIntentData,
        AppleVoiceTurnQueuedMutationSummaryData, AppleVoiceTurnRequestData,
        AppleVoiceTurnResponseData, AssistantContextData, DailyCommitmentDraftData,
        DailyDeferredTaskData, DailyFocusBlockProposalData, DailyLoopCheckInResolutionData,
        DailyLoopCheckInResolutionKindData, DailyLoopPhaseData, DailyLoopSessionData,
        DailyLoopSessionOutcomeData, DailyLoopStartMetadataData, DailyLoopStartRequestData,
        DailyLoopStartSourceData, DailyLoopSurfaceData, DailyLoopTurnActionData,
        DailyLoopTurnRequestData, DailyStandupBucketData, DailyStandupOutcomeData,
        DayPlanProposalData, ExecutionHandoffData, ExecutionHandoffReviewStateData,
        ExecutionReviewGateData, ExecutionTaskKindData, LocalRuntimeKindData,
        MorningIntentSignalData, NowTaskData, ProjectExecutionContextData, ProjectFamilyData,
        ProjectProvisionRequestData, ProjectRecordData, ProjectRootRefData, ProjectStatusData,
        RecallContextData, RecallContextHitData, RecallContextSourceCountData,
        TokenBudgetClassData,
    };
    use std::collections::BTreeMap;
    use time::macros::datetime;
    use vel_core::{
        AgentProfile, CapabilityDescriptor, DailyCommitmentDraft, DailyDeferredTask,
        DailyFocusBlockProposal, DailyLoopPhase, DailyLoopPrompt, DailyLoopPromptKind,
        DailyLoopSession, DailyLoopSessionId, DailyLoopSessionOutcome, DailyLoopStartMetadata,
        DailyLoopStartSource, DailyLoopStatus, DailyLoopSurface, DailyLoopTurnState,
        DailyStandupBucket, DailyStandupOutcome, ExecutionHandoff, ExecutionReviewGate,
        ExecutionTaskKind, HandoffEnvelope, LocalAgentManifest, LocalRuntimeKind,
        MorningFrictionCallout, MorningIntentSignal, MorningOverviewState, ProjectExecutionContext,
        ProjectId, ProjectRootRef, RepoWorktreeRef, TokenBudgetClass, TraceId,
    };

    #[test]
    fn now_task_due_at_serializes_as_rfc3339_string() {
        let task = NowTaskData {
            id: "commit_1".to_string(),
            text: "Reply to Dimitri".to_string(),
            title: "Reply to Dimitri".to_string(),
            description: None,
            tags: vec!["follow_up".to_string()],
            source_type: "todoist".to_string(),
            due_at: Some(datetime!(2026-03-16 19:00:00 UTC)),
            deadline: Some(datetime!(2026-03-18 00:00:00 UTC)),
            project: None,
            commitment_kind: Some("todo".to_string()),
        };

        let value = serde_json::to_value(task).expect("now task should serialize");
        assert_eq!(value["due_at"], "2026-03-16T19:00:00Z");
        assert_eq!(value["deadline"], "2026-03-18T00:00:00Z");
    }

    #[test]
    fn now_task_none_due_at_serializes_as_null() {
        let task = NowTaskData {
            id: "commit_2".to_string(),
            text: "Inbox zero".to_string(),
            title: "Inbox zero".to_string(),
            description: None,
            tags: vec![],
            source_type: "manual".to_string(),
            due_at: None,
            deadline: None,
            project: None,
            commitment_kind: None,
        };

        let value = serde_json::to_value(task).expect("now task should serialize");
        assert!(value["due_at"].is_null());
    }

    #[test]
    fn recall_context_round_trips_named_counts_and_scores() {
        let data = RecallContextData {
            query_text: "accountant follow up".to_string(),
            hit_count: 2,
            source_counts: vec![RecallContextSourceCountData {
                source_kind: vel_core::SemanticSourceKind::Note,
                count: 2,
            }],
            hits: vec![RecallContextHitData {
                record_id: vel_core::SemanticRecordId::new("sem_note_1"),
                source_kind: vel_core::SemanticSourceKind::Note,
                source_id: "projects/tax/accountant.md".to_string(),
                snippet: "Need accountant follow up on quarterly estimate.".to_string(),
                lexical_score: 0.4,
                semantic_score: 0.9,
                combined_score: 0.775,
                provenance: vel_core::SemanticProvenance {
                    note_path: Some("projects/tax/accountant.md".to_string()),
                    ..Default::default()
                },
            }],
        };

        let value = serde_json::to_value(&data).expect("recall context should serialize");
        assert_eq!(value["query_text"], "accountant follow up");
        assert_eq!(value["hit_count"], 2);
        assert_eq!(value["source_counts"][0]["source_kind"], "note");
        assert_eq!(value["hits"][0]["combined_score"], 0.775_f32 as f64);
    }

    #[test]
    fn assistant_context_round_trips_summary_and_focus_lines() {
        let data = AssistantContextData {
            query_text: "accountant follow up".to_string(),
            summary: "Found 1 relevant recalled item across note sources.".to_string(),
            focus_lines: vec![
                "note projects/tax/accountant.md: Need accountant follow up on quarterly estimate."
                    .to_string(),
            ],
            commitments: vec![],
            recall: RecallContextData {
                query_text: "accountant follow up".to_string(),
                hit_count: 1,
                source_counts: vec![RecallContextSourceCountData {
                    source_kind: vel_core::SemanticSourceKind::Note,
                    count: 1,
                }],
                hits: vec![RecallContextHitData {
                    record_id: vel_core::SemanticRecordId::new("sem_note_1"),
                    source_kind: vel_core::SemanticSourceKind::Note,
                    source_id: "projects/tax/accountant.md".to_string(),
                    snippet: "Need accountant follow up on quarterly estimate.".to_string(),
                    lexical_score: 0.4,
                    semantic_score: 0.9,
                    combined_score: 0.775,
                    provenance: vel_core::SemanticProvenance {
                        note_path: Some("projects/tax/accountant.md".to_string()),
                        ..Default::default()
                    },
                }],
            },
        };

        let value = serde_json::to_value(&data).expect("assistant context should serialize");
        assert_eq!(
            value["summary"],
            "Found 1 relevant recalled item across note sources."
        );
        assert_eq!(
            value["focus_lines"][0],
            "note projects/tax/accountant.md: Need accountant follow up on quarterly estimate."
        );
        assert_eq!(value["recall"]["source_counts"][0]["source_kind"], "note");
    }

    #[test]
    fn day_plan_proposal_data_serializes_counts_and_routine_blocks() {
        let value = DayPlanProposalData::from(vel_core::DayPlanProposal {
            headline: "Today has a bounded plan".to_string(),
            summary: "Vel shaped the day around current routine blocks and commitments."
                .to_string(),
            scheduled_count: 1,
            deferred_count: 1,
            did_not_fit_count: 1,
            needs_judgment_count: 0,
            changes: vec![vel_core::DayPlanChange {
                kind: vel_core::DayPlanChangeKind::Scheduled,
                commitment_id: None,
                title: "Draft phase contract".to_string(),
                detail: "Placed into the prenoon focus block.".to_string(),
                project_label: Some("Vel".to_string()),
                scheduled_start_ts: Some(1_710_000_000),
                rule_facets: vec![vel_core::ScheduleRuleFacet {
                    kind: vel_core::ScheduleRuleFacetKind::BlockTarget,
                    label: "block:focus".to_string(),
                    detail: None,
                }],
            }],
            routine_blocks: vec![vel_core::RoutineBlock {
                id: "routine_focus_am".to_string(),
                label: "Focus".to_string(),
                source: vel_core::RoutineBlockSourceKind::OperatorDeclared,
                start_ts: 1_710_000_000,
                end_ts: 1_710_003_600,
                protected: true,
            }],
        });

        let json = serde_json::to_value(&value).expect("day-plan data should serialize");
        assert_eq!(json["scheduled_count"], 1);
        assert_eq!(json["did_not_fit_count"], 1);
        assert_eq!(json["changes"][0]["kind"], "scheduled");
        assert_eq!(json["routine_blocks"][0]["source"], "operator_declared");
    }

    #[test]
    fn project_record_timestamps_serialize_as_rfc3339_strings() {
        let project = ProjectRecordData {
            id: ProjectId::from("proj_1".to_string()),
            slug: "vel".to_string(),
            name: "Vel".to_string(),
            family: ProjectFamilyData::Work,
            status: ProjectStatusData::Active,
            primary_repo: ProjectRootRefData {
                path: "/tmp/vel".to_string(),
                label: "vel".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRefData {
                path: "/tmp/notes/vel".to_string(),
                label: "vel".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: vec![],
            secondary_notes_roots: vec![],
            upstream_ids: BTreeMap::new(),
            pending_provision: ProjectProvisionRequestData {
                create_repo: false,
                create_notes_root: false,
            },
            created_at: datetime!(2026-03-19 02:10:00 UTC),
            updated_at: datetime!(2026-03-19 02:20:00 UTC),
            archived_at: None,
        };

        let value = serde_json::to_value(project).expect("project should serialize");
        assert_eq!(value["created_at"], "2026-03-19T02:10:00Z");
        assert_eq!(value["updated_at"], "2026-03-19T02:20:00Z");
        assert!(value["archived_at"].is_null());
    }

    #[test]
    fn project_execution_context_converts_from_core() {
        let context = ProjectExecutionContext {
            project_id: ProjectId::from("proj_velruntime".to_string()),
            repo: RepoWorktreeRef {
                path: "/home/jove/code/vel".to_string(),
                label: "vel".to_string(),
                branch: Some("main".to_string()),
                head_rev: Some("abc1234".to_string()),
            },
            notes_root: ProjectRootRef {
                path: "/home/jove/notes/vel".to_string(),
                label: "Vel Notes".to_string(),
                kind: "notes_root".to_string(),
            },
            gsd_artifact_dir: ".planning/vel".to_string(),
            default_task_kind: ExecutionTaskKind::Implementation,
            default_agent_profile: AgentProfile::Balanced,
            default_token_budget: TokenBudgetClass::Large,
            review_gate: ExecutionReviewGate::OperatorPreview,
            read_roots: vec!["/home/jove/code/vel".to_string()],
            write_roots: vec!["/home/jove/code/vel/.planning/vel".to_string()],
            local_manifests: vec![LocalAgentManifest {
                manifest_id: "manifest_local_cli".to_string(),
                runtime_kind: LocalRuntimeKind::LocalCli,
                entrypoint: "cargo".to_string(),
                working_directory: "/home/jove/code/vel".to_string(),
                args: vec!["run".to_string(), "-p".to_string(), "vel-cli".to_string()],
                env_keys: vec!["VEL_OPERATOR_TOKEN".to_string()],
                read_roots: vec!["/home/jove/code/vel".to_string()],
                write_roots: vec!["/home/jove/code/vel/.planning/vel".to_string()],
                allowed_tools: vec!["rg".to_string(), "cargo".to_string()],
                capabilities: vec![CapabilityDescriptor {
                    scope: "repo.read".to_string(),
                    resource: Some("/home/jove/code/vel".to_string()),
                    action: "read".to_string(),
                }],
                review_gate: ExecutionReviewGate::OperatorPreview,
            }],
            metadata: BTreeMap::from([("phase".to_string(), "08".to_string())]),
            created_at: datetime!(2026-03-19 10:00:00 UTC),
            updated_at: datetime!(2026-03-19 10:05:00 UTC),
        };

        let data = ProjectExecutionContextData::from(context);
        assert_eq!(
            data.project_id,
            ProjectId::from("proj_velruntime".to_string())
        );
        assert_eq!(
            data.default_task_kind,
            ExecutionTaskKindData::Implementation
        );
        assert_eq!(data.default_agent_profile, AgentProfileData::Balanced);
        assert_eq!(data.default_token_budget, TokenBudgetClassData::Large);
        assert_eq!(data.review_gate, ExecutionReviewGateData::OperatorPreview);
        assert_eq!(
            data.local_manifests[0].runtime_kind,
            LocalRuntimeKindData::LocalCli
        );
    }

    #[test]
    fn execution_handoff_converts_from_core() {
        let handoff = ExecutionHandoff {
            handoff: HandoffEnvelope {
                task_id: "task_1".to_string(),
                trace_id: TraceId::from("trace_1".to_string()),
                from_agent: "planner".to_string(),
                to_agent: "executor".to_string(),
                objective: "Implement Phase 08 contracts".to_string(),
                inputs: serde_json::json!({ "ticket": "08-01" }),
                constraints: vec!["stay within write scope".to_string()],
                read_scopes: vec!["docs/".to_string(), "crates/".to_string()],
                write_scopes: vec!["crates/vel-core/".to_string()],
                project_id: Some(ProjectId::from("proj_velruntime".to_string())),
                task_kind: Some(ExecutionTaskKind::Implementation),
                agent_profile: Some(AgentProfile::Balanced),
                token_budget: Some(TokenBudgetClass::Large),
                review_gate: Some(ExecutionReviewGate::OperatorPreview),
                repo_root: Some(RepoWorktreeRef {
                    path: "/home/jove/code/vel".to_string(),
                    label: "vel".to_string(),
                    branch: Some("main".to_string()),
                    head_rev: Some("abc1234".to_string()),
                }),
                allowed_tools: vec!["rg".to_string(), "cargo".to_string()],
                capability_scope: serde_json::json!({ "mode": "scoped" }),
                deadline: Some(datetime!(2026-03-19 12:00:00 UTC)),
                expected_output_schema: serde_json::json!({ "type": "object" }),
            },
            project_id: ProjectId::from("proj_velruntime".to_string()),
            task_kind: ExecutionTaskKind::Implementation,
            agent_profile: AgentProfile::Balanced,
            token_budget: TokenBudgetClass::Large,
            review_gate: ExecutionReviewGate::OperatorPreview,
            repo: RepoWorktreeRef {
                path: "/home/jove/code/vel".to_string(),
                label: "vel".to_string(),
                branch: Some("main".to_string()),
                head_rev: Some("abc1234".to_string()),
            },
            notes_root: ProjectRootRef {
                path: "/home/jove/notes/vel".to_string(),
                label: "Vel Notes".to_string(),
                kind: "notes_root".to_string(),
            },
            manifest_id: Some("manifest_local_cli".to_string()),
        };

        let data = ExecutionHandoffData::from(handoff);
        assert_eq!(data.task_kind, ExecutionTaskKindData::Implementation);
        assert_eq!(data.agent_profile, AgentProfileData::Balanced);
        assert_eq!(data.token_budget, TokenBudgetClassData::Large);
        assert_eq!(data.review_gate, ExecutionReviewGateData::OperatorPreview);
        assert_eq!(
            data.handoff.task_kind,
            Some(ExecutionTaskKindData::Implementation)
        );
        assert_eq!(data.handoff.repo_root.unwrap().label, "vel");
    }

    #[test]
    fn apple_voice_turn_request_round_trips_between_wire_and_core_types() {
        let request = AppleVoiceTurnRequestData {
            transcript: "what matters now".to_string(),
            surface: AppleClientSurfaceData::IosVoice,
            operation: AppleRequestedOperationData::QueryOnly,
            intents: vec![
                AppleVoiceIntentData::CurrentSchedule,
                AppleVoiceIntentData::ExplainWhy,
            ],
            provenance: Some(AppleTurnProvenanceData {
                source_device: Some("iphone".to_string()),
                locale: Some("en-US".to_string()),
                transcript_origin: Some("speech".to_string()),
                recorded_at: Some(datetime!(2026-03-19 07:10:00 UTC)),
                offline_captured_at: None,
                queued_at: None,
            }),
        };

        let core: vel_core::AppleVoiceTurnRequest = request.clone().into();
        let round_trip = AppleVoiceTurnRequestData::from(core);
        let value = serde_json::to_value(round_trip).expect("apple request should serialize");

        assert_eq!(value["surface"], "ios_voice");
        assert_eq!(value["operation"], "query_only");
        assert_eq!(value["provenance"]["recorded_at"], "2026-03-19T07:10:00Z");
    }

    #[test]
    fn apple_voice_turn_response_serializes_nested_schedule_and_behavior_summary() {
        let response = AppleVoiceTurnResponseData {
            operation: AppleRequestedOperationData::CaptureAndQuery,
            mode: AppleResponseModeData::SpokenSummary,
            summary: "You have standup in 20 minutes.".to_string(),
            capture_id: Some("cap_voice_1".to_string().into()),
            thread_id: Some("conv_voice_1".to_string()),
            reasons: vec!["Standup starts at 09:00.".to_string()],
            evidence: vec![AppleResponseEvidenceData {
                kind: "event".to_string(),
                label: "Standup".to_string(),
                detail: "Starts at 09:00".to_string(),
                source_id: Some("evt_1".to_string()),
            }],
            queued_mutation: Some(AppleVoiceTurnQueuedMutationSummaryData {
                mutation_kind: "capture_create".to_string(),
                queued: false,
                summary: "Transcript stored as a voice note.".to_string(),
                action_reference_id: Some("act_1".to_string()),
            }),
            schedule: Some(AppleScheduleSnapshotData {
                generated_at: 1_763_661_000,
                timezone: "America/Denver".to_string(),
                focus_summary: Some("Morning execution block".to_string()),
                next_event: Some(AppleScheduleEventData {
                    title: "Standup".to_string(),
                    start_ts: 1_763_661_600,
                    end_ts: Some(1_763_662_200),
                    location: Some("Desk".to_string()),
                    leave_by_ts: Some(1_763_661_300),
                }),
                upcoming_events: vec![],
                reasons: vec!["Calendar synced 2 minutes ago.".to_string()],
            }),
            behavior_summary: Some(AppleBehaviorSummaryData {
                generated_at: 1_763_661_000,
                timezone: "America/Denver".to_string(),
                scope: AppleBehaviorSummaryScopeData::Daily,
                headline: "You are on track for movement today.".to_string(),
                metrics: vec![AppleBehaviorMetricData {
                    metric_key: "step_count".to_string(),
                    display_label: "Steps".to_string(),
                    value: 4200.0,
                    unit: "count".to_string(),
                    recorded_at: 1_763_660_900,
                    reasons: vec!["Above your same-time baseline.".to_string()],
                }],
                reasons: vec!["Health snapshot is fresh.".to_string()],
                freshness_seconds: Some(120),
            }),
        };

        let value = serde_json::to_value(response).expect("apple response should serialize");
        assert_eq!(value["mode"], "spoken_summary");
        assert_eq!(value["schedule"]["next_event"]["title"], "Standup");
        assert_eq!(
            value["behavior_summary"]["metrics"][0]["metric_key"],
            "step_count"
        );
    }

    #[test]
    fn daily_loop_session_data_round_trips_morning_and_standup_payloads() {
        let morning_session = DailyLoopSession {
            id: DailyLoopSessionId::from("dls_1".to_string()),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhase::MorningOverview,
            status: DailyLoopStatus::WaitingForInput,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::AppleVoice,
            },
            turn_state: DailyLoopTurnState::WaitingForInput,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "prompt_morning_1".to_string(),
                kind: DailyLoopPromptKind::IntentQuestion,
                text: "What most needs to happen before noon?".to_string(),
                ordinal: 1,
                allow_skip: true,
            }),
            state: MorningOverviewState {
                snapshot: "You have two meetings before lunch.".to_string(),
                friction_callouts: vec![MorningFrictionCallout {
                    label: "Prep debt".to_string(),
                    detail: "Design review starts in 45 minutes.".to_string(),
                }],
                signals: vec![MorningIntentSignal::MustDoHint {
                    text: "Finish review notes".to_string(),
                }],
                check_in_history: vec![vel_core::DailyLoopCheckInResolution {
                    prompt_id: "prompt_morning_1".to_string(),
                    ordinal: 1,
                    kind: vel_core::DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some("Finish review notes".to_string()),
                    note_text: None,
                }],
            }
            .into(),
            outcome: Some(DailyLoopSessionOutcome::MorningOverview {
                signals: vec![MorningIntentSignal::FocusIntent {
                    text: "Protect a deep-work block".to_string(),
                }],
                check_in_history: vec![vel_core::DailyLoopCheckInResolution {
                    prompt_id: "prompt_morning_1".to_string(),
                    ordinal: 1,
                    kind: vel_core::DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some("Protect a deep-work block".to_string()),
                    note_text: None,
                }],
            }),
        };

        let standup_session = DailyLoopSession {
            id: DailyLoopSessionId::from("dls_2".to_string()),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhase::Standup,
            status: DailyLoopStatus::Completed,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Cli,
            },
            turn_state: DailyLoopTurnState::Completed,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "prompt_standup_1".to_string(),
                kind: DailyLoopPromptKind::CommitmentReduction,
                text: "Reduce this to three commitments.".to_string(),
                ordinal: 2,
                allow_skip: false,
            }),
            state: DailyStandupOutcome {
                commitments: vec![DailyCommitmentDraft {
                    title: "Ship Phase 10 contract slice".to_string(),
                    bucket: DailyStandupBucket::Must,
                    source_ref: Some("ticket:10-01".to_string()),
                }],
                deferred_tasks: vec![DailyDeferredTask {
                    title: "Triage lower-priority inbox items".to_string(),
                    source_ref: Some("todoist:42".to_string()),
                    reason: "Not part of the top three".to_string(),
                }],
                confirmed_calendar: vec!["Design review at 10:00 remains on".to_string()],
                focus_blocks: vec![DailyFocusBlockProposal {
                    label: "Contract implementation".to_string(),
                    start_at: datetime!(2026-03-19 15:00:00 UTC),
                    end_at: datetime!(2026-03-19 16:00:00 UTC),
                    reason: "Best uninterrupted slot before review".to_string(),
                }],
                check_in_history: vec![vel_core::DailyLoopCheckInResolution {
                    prompt_id: "prompt_standup_1".to_string(),
                    ordinal: 1,
                    kind: vel_core::DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some("Ship Phase 10 contract slice".to_string()),
                    note_text: None,
                }],
            }
            .into(),
            outcome: Some(DailyLoopSessionOutcome::Standup(DailyStandupOutcome {
                commitments: vec![DailyCommitmentDraft {
                    title: "Ship Phase 10 contract slice".to_string(),
                    bucket: DailyStandupBucket::Must,
                    source_ref: Some("ticket:10-01".to_string()),
                }],
                deferred_tasks: vec![DailyDeferredTask {
                    title: "Triage lower-priority inbox items".to_string(),
                    source_ref: Some("todoist:42".to_string()),
                    reason: "Not part of the top three".to_string(),
                }],
                confirmed_calendar: vec!["Design review at 10:00 remains on".to_string()],
                focus_blocks: vec![DailyFocusBlockProposal {
                    label: "Contract implementation".to_string(),
                    start_at: datetime!(2026-03-19 15:00:00 UTC),
                    end_at: datetime!(2026-03-19 16:00:00 UTC),
                    reason: "Best uninterrupted slot before review".to_string(),
                }],
                check_in_history: vec![vel_core::DailyLoopCheckInResolution {
                    prompt_id: "prompt_standup_1".to_string(),
                    ordinal: 1,
                    kind: vel_core::DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some("Ship Phase 10 contract slice".to_string()),
                    note_text: None,
                }],
            })),
        };

        let morning_data = DailyLoopSessionData::from(morning_session.clone());
        let morning_json =
            serde_json::to_value(&morning_data).expect("morning session should serialize");
        assert_eq!(morning_json["phase"], "morning_overview");
        assert_eq!(morning_json["status"], "waiting_for_input");
        assert_eq!(morning_json["current_prompt"]["kind"], "intent_question");
        assert_eq!(morning_json["allowed_actions"][0], "accept");
        assert_eq!(morning_json["allowed_actions"][1], "defer");
        assert_eq!(
            morning_json["continuity_summary"],
            "Morning overview is waiting on question 1 of 3 with 1 captured signal(s)."
        );
        assert_eq!(morning_json["outcome"]["phase"], "morning_overview");

        let round_trip_morning: DailyLoopSession =
            DailyLoopSessionData::from(morning_session).into();
        assert_eq!(round_trip_morning.phase, DailyLoopPhase::MorningOverview);

        let standup_json = serde_json::to_value(DailyLoopSessionData::from(standup_session))
            .expect("standup session should serialize");
        assert_eq!(standup_json["phase"], "standup");
        assert_eq!(standup_json["allowed_actions"][0], "accept");
        assert_eq!(standup_json["allowed_actions"][1], "choose");
        assert_eq!(
            standup_json["continuity_summary"],
            "Standup is waiting on question 2 with 1 commitment draft(s) and 1 deferred item(s)."
        );
        assert_eq!(standup_json["outcome"]["phase"], "standup");
        assert_eq!(standup_json["outcome"]["commitments"][0]["bucket"], "must");
    }

    #[test]
    fn daily_loop_morning_signals_stay_distinct_from_standup_commitments() {
        let morning = DailyLoopSessionOutcomeData::MorningOverview {
            signals: vec![MorningIntentSignalData::MustDoHint {
                text: "Handle payroll first".to_string(),
            }],
            check_in_history: vec![DailyLoopCheckInResolutionData {
                prompt_id: "prompt_morning_1".to_string(),
                ordinal: 1,
                kind: DailyLoopCheckInResolutionKindData::Submitted,
                response_text: Some("Handle payroll first".to_string()),
                note_text: None,
            }],
        };
        let standup = DailyLoopSessionOutcomeData::Standup(DailyStandupOutcomeData {
            commitments: vec![DailyCommitmentDraftData {
                title: "Close payroll".to_string(),
                bucket: DailyStandupBucketData::Must,
                source_ref: Some("todoist:payroll".to_string()),
            }],
            deferred_tasks: vec![DailyDeferredTaskData {
                title: "Draft roadmap notes".to_string(),
                source_ref: None,
                reason: "Deferred until after payroll".to_string(),
            }],
            confirmed_calendar: vec!["11:00 payroll check-in".to_string()],
            focus_blocks: vec![DailyFocusBlockProposalData {
                label: "Payroll close".to_string(),
                start_at: datetime!(2026-03-19 16:00:00 UTC),
                end_at: datetime!(2026-03-19 16:30:00 UTC),
                reason: "Smallest uninterrupted slot".to_string(),
            }],
            check_in_history: vec![DailyLoopCheckInResolutionData {
                prompt_id: "prompt_standup_1".to_string(),
                ordinal: 1,
                kind: DailyLoopCheckInResolutionKindData::Submitted,
                response_text: Some("Close payroll".to_string()),
                note_text: None,
            }],
        });

        let morning_json = serde_json::to_value(morning).expect("morning outcome should serialize");
        let standup_json = serde_json::to_value(standup).expect("standup outcome should serialize");

        assert!(morning_json.get("commitments").is_none());
        assert_eq!(morning_json["phase"], "morning_overview");
        assert_eq!(morning_json["signals"][0]["kind"], "must_do_hint");
        assert_eq!(standup_json["phase"], "standup");
        assert_eq!(standup_json["commitments"][0]["title"], "Close payroll");
    }

    #[test]
    fn daily_loop_start_metadata_keeps_source_and_surface_for_manual_and_future_auto_starts() {
        let start = DailyLoopStartRequestData {
            phase: DailyLoopPhaseData::MorningOverview,
            session_date: "2026-03-19".to_string(),
            start: DailyLoopStartMetadataData {
                source: DailyLoopStartSourceData::Automatic,
                surface: DailyLoopSurfaceData::Web,
            },
        };
        let turn = DailyLoopTurnRequestData {
            session_id: "dls_3".to_string(),
            action: DailyLoopTurnActionData::Resume,
            response_text: None,
        };

        let core_start: vel_core::DailyLoopStartRequest = start.clone().into();
        let round_trip = DailyLoopStartRequestData::from(core_start);
        let start_json = serde_json::to_value(round_trip).expect("start request should serialize");
        let turn_json = serde_json::to_value(turn).expect("turn request should serialize");

        assert_eq!(start_json["start"]["source"], "automatic");
        assert_eq!(start_json["start"]["surface"], "web");
        assert_eq!(turn_json["action"], "resume");
    }

    #[test]
    fn agent_grounding_round_trips_typed_sections() {
        let data: AgentInspectData = serde_json::from_str(include_str!(
            "../../../config/examples/agent-inspect.example.json"
        ))
        .expect("agent inspect example should parse");

        assert_eq!(data.grounding.projects.len(), 1);
        assert_eq!(data.grounding.people.len(), 1);
        assert_eq!(data.grounding.commitments.len(), 1);
        assert_eq!(
            data.grounding.review.pending_execution_handoffs[0].review_state,
            ExecutionHandoffReviewStateData::PendingReview
        );
        assert_eq!(
            data.capabilities.groups[1].kind,
            AgentCapabilityGroupKindData::ReviewActions
        );
        assert!(data.explainability.raw_context_json_supporting_only);

        let value = serde_json::to_value(&data).expect("agent inspect should serialize");
        assert_eq!(
            value["grounding"]["review"]["pending_execution_handoffs"][0]["routing"]["task_kind"],
            "implementation"
        );
        assert_eq!(
            value["capabilities"]["groups"][2]["kind"],
            "mutation_actions"
        );
        assert_eq!(
            value["capabilities"]["groups"][2]["entries"][0]["blocked_reason"]["code"],
            "safe_mode_enabled"
        );
    }

    #[test]
    fn agent_grounding_capability_entries_preserve_explicit_blockers() {
        let entry = AgentCapabilityEntryData {
            key: "integration_writeback".to_string(),
            label: "Request integration writeback".to_string(),
            summary: "Can request bounded upstream mutations when writeback is enabled."
                .to_string(),
            available: false,
            blocked_reason: Some(AgentBlockerData {
                code: "safe_mode_enabled".to_string(),
                message: "SAFE MODE keeps writeback disabled.".to_string(),
                escalation_hint: Some("Enable writeback in Settings before retrying.".to_string()),
            }),
            requires_review_gate: Some(ExecutionReviewGateData::OperatorPreview),
            requires_writeback_enabled: true,
        };

        let value = serde_json::to_value(entry).expect("capability entry should serialize");
        assert_eq!(value["available"], false);
        assert_eq!(value["blocked_reason"]["code"], "safe_mode_enabled");
        assert_eq!(value["requires_review_gate"], "operator_preview");
        assert_eq!(value["requires_writeback_enabled"], true);
    }

    #[test]
    fn agent_grounding_contract_assets_parse_and_register() {
        let pack: super::AgentGroundingPackData = serde_json::from_str(include_str!(
            "../../../config/examples/agent-grounding-pack.example.json"
        ))
        .expect("grounding pack example should parse");
        assert_eq!(pack.review.pending_execution_handoffs.len(), 1);

        let inspect: AgentInspectData = serde_json::from_str(include_str!(
            "../../../config/examples/agent-inspect.example.json"
        ))
        .expect("inspect example should parse");
        assert_eq!(inspect.blockers.len(), 1);

        let grounding_schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../../config/schemas/agent-grounding-pack.schema.json"
        ))
        .expect("grounding schema should parse");
        assert_eq!(grounding_schema["title"], "AgentGroundingPack");

        let inspect_schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../../config/schemas/agent-inspect.schema.json"
        ))
        .expect("inspect schema should parse");
        assert_eq!(inspect_schema["title"], "AgentInspect");

        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../../../config/contracts-manifest.json"))
                .expect("contracts manifest should parse");
        let examples = manifest["contract_examples"]
            .as_array()
            .expect("contract examples should be an array");
        assert!(examples.iter().any(|entry| {
            entry["path"] == "config/examples/agent-grounding-pack.example.json"
                && entry["schema"] == "config/schemas/agent-grounding-pack.schema.json"
        }));
        assert!(examples.iter().any(|entry| {
            entry["path"] == "config/examples/agent-inspect.example.json"
                && entry["schema"] == "config/schemas/agent-inspect.schema.json"
        }));

        let owner_doc = include_str!(
            "../../../docs/cognitive-agent-architecture/agents/agent-grounding-contracts.md"
        );
        assert!(owner_doc.contains("AgentInspectData"));
        assert!(owner_doc.contains("raw context JSON is supporting evidence"));
    }

    #[test]
    fn planning_profile_management_contract_assets_parse_and_register() {
        let profile: crate::RoutinePlanningProfileData = serde_json::from_str(include_str!(
            "../../../config/examples/routine-planning-profile.example.json"
        ))
        .expect("routine planning profile example should parse");
        assert_eq!(profile.routine_blocks.len(), 2);

        let mutation_request: crate::PlanningProfileMutationRequestData = serde_json::from_str(
            include_str!("../../../config/examples/planning-profile-mutation.example.json"),
        )
        .expect("planning profile mutation example should parse");
        match mutation_request.mutation {
            crate::PlanningProfileMutationData::UpsertRoutineBlock(block) => {
                assert_eq!(block.id, "routine_lunch");
            }
            other => panic!("unexpected planning profile mutation example: {other:?}"),
        }

        let mutation_schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../../config/schemas/planning-profile-mutation.schema.json"
        ))
        .expect("planning profile mutation schema should parse");
        assert_eq!(mutation_schema["title"], "PlanningProfileMutationRequest");

        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../../../config/contracts-manifest.json"))
                .expect("contracts manifest should parse");
        let examples = manifest["contract_examples"]
            .as_array()
            .expect("contract examples should be an array");
        assert!(examples.iter().any(|entry| {
            entry["path"] == "config/examples/planning-profile-mutation.example.json"
                && entry["schema"] == "config/schemas/planning-profile-mutation.schema.json"
        }));

        let owner_doc = include_str!(
            "../../../docs/cognitive-agent-architecture/architecture/planning-profile-management-contract.md"
        );
        assert!(owner_doc.contains("PlanningProfileMutation"));
        assert!(owner_doc.contains("upsert_routine_block"));
    }

    #[test]
    fn planning_profile_edit_proposal_contract_assets_parse_and_register() {
        let proposal: crate::PlanningProfileEditProposalData = serde_json::from_str(include_str!(
            "../../../config/examples/planning-profile-edit-proposal.example.json"
        ))
        .expect("planning profile edit proposal example should parse");
        assert_eq!(
            proposal.source_surface,
            crate::PlanningProfileSurfaceData::Assistant
        );
        assert_eq!(proposal.state, crate::AssistantProposalStateData::Staged);
        assert_eq!(
            proposal.continuity,
            crate::PlanningProfileContinuityData::Thread
        );

        let proposal_schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../../config/schemas/planning-profile-edit-proposal.schema.json"
        ))
        .expect("planning profile edit proposal schema should parse");
        assert_eq!(proposal_schema["title"], "PlanningProfileEditProposal");

        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../../../config/contracts-manifest.json"))
                .expect("contracts manifest should parse");
        let examples = manifest["contract_examples"]
            .as_array()
            .expect("contract examples should be an array");
        assert!(examples.iter().any(|entry| {
            entry["path"] == "config/examples/planning-profile-edit-proposal.example.json"
                && entry["schema"] == "config/schemas/planning-profile-edit-proposal.schema.json"
        }));

        let owner_doc = include_str!(
            "../../../docs/cognitive-agent-architecture/architecture/planning-profile-application-contract.md"
        );
        assert!(owner_doc.contains("PlanningProfileEditProposal"));
        assert!(owner_doc.contains("AssistantProposalState"));
    }

    #[test]
    fn commitment_scheduling_proposal_contract_assets_parse_and_register() {
        let proposal: crate::CommitmentSchedulingProposalData = serde_json::from_str(include_str!(
            "../../../config/examples/commitment-scheduling-proposal.example.json"
        ))
        .expect("commitment scheduling proposal example should parse");
        assert_eq!(
            proposal.source_kind,
            crate::CommitmentSchedulingSourceKindData::Reflow
        );
        assert_eq!(proposal.state, crate::AssistantProposalStateData::Staged);
        assert_eq!(
            proposal.continuity,
            crate::CommitmentSchedulingContinuityData::Thread
        );
        assert_eq!(proposal.mutations.len(), 2);

        let proposal_schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../../config/schemas/commitment-scheduling-proposal.schema.json"
        ))
        .expect("commitment scheduling proposal schema should parse");
        assert_eq!(proposal_schema["title"], "CommitmentSchedulingProposal");

        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../../../config/contracts-manifest.json"))
                .expect("contracts manifest should parse");
        let examples = manifest["contract_examples"]
            .as_array()
            .expect("contract examples should be an array");
        assert!(examples.iter().any(|entry| {
            entry["path"] == "config/examples/commitment-scheduling-proposal.example.json"
                && entry["schema"] == "config/schemas/commitment-scheduling-proposal.schema.json"
        }));

        let owner_doc = include_str!(
            "../../../docs/cognitive-agent-architecture/architecture/day-plan-application-contract.md"
        );
        assert!(owner_doc.contains("CommitmentSchedulingProposal"));
        assert!(owner_doc.contains("AssistantProposalState"));
    }
}
