use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{
    ActionItem, CheckInCard, ConflictCaseRecord, CurrentContextReflowStatus, DayPlanProposal,
    ReflowCard, ReviewSnapshot, WritebackOperationRecord,
};

#[derive(Debug, Clone)]
pub struct NowOutput {
    pub computed_at: i64,
    pub timezone: String,
    pub header: Option<NowHeaderOutput>,
    pub mesh_summary: Option<NowMeshSummaryOutput>,
    pub status_row: Option<NowStatusRowOutput>,
    pub context_line: Option<NowContextLineOutput>,
    pub nudge_bars: Vec<NowNudgeBarOutput>,
    pub task_lane: Option<NowTaskLaneOutput>,
    pub next_up_items: Vec<NowNextUpItemOutput>,
    pub progress: NowProgressOutput,
    pub docked_input: Option<NowDockedInputOutput>,
    pub overview: NowOverviewOutput,
    pub summary: NowSummaryOutput,
    pub schedule: NowScheduleOutput,
    pub tasks: NowTasksOutput,
    pub attention: NowAttentionOutput,
    pub sources: NowSourcesOutput,
    pub freshness: NowFreshnessOutput,
    pub trust_readiness: TrustReadinessOutput,
    pub planning_profile_summary:
        Option<crate::services::planning_profile::PlanningProfileProposalSummary>,
    pub commitment_scheduling_summary:
        Option<crate::services::commitment_scheduling::CommitmentSchedulingProposalSummary>,
    pub check_in: Option<CheckInCard>,
    pub day_plan: Option<DayPlanProposal>,
    pub reflow: Option<ReflowCard>,
    pub reflow_status: Option<CurrentContextReflowStatus>,
    pub action_items: Vec<ActionItem>,
    pub review_snapshot: ReviewSnapshot,
    pub pending_writebacks: Vec<WritebackOperationRecord>,
    pub conflicts: Vec<ConflictCaseRecord>,
    pub people: Vec<vel_core::PersonRecord>,
    pub reasons: Vec<String>,
    pub debug: NowDebugOutput,
}

#[derive(Debug, Clone)]
pub struct NowLabelOutput {
    pub key: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct NowRiskSummaryOutput {
    pub level: String,
    pub score: Option<f64>,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct NowHeaderOutput {
    pub title: String,
    pub buckets: Vec<NowHeaderBucketOutput>,
}

#[derive(Debug, Clone)]
pub struct NowMeshSummaryOutput {
    pub authority_node_id: String,
    pub authority_label: String,
    pub sync_state: String,
    pub linked_node_count: u32,
    pub queued_write_count: u32,
    pub last_sync_at: Option<i64>,
    pub urgent: bool,
    pub repair_route: Option<NowRepairRouteOutput>,
}

#[derive(Debug, Clone)]
pub struct NowRepairRouteOutput {
    pub target: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct NowHeaderBucketOutput {
    pub kind: String,
    pub count: u32,
    pub count_display: String,
    pub urgent: bool,
    pub route_thread_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowStatusRowOutput {
    pub date_label: String,
    pub time_label: String,
    pub context_label: String,
    pub elapsed_label: String,
}

#[derive(Debug, Clone)]
pub struct NowContextLineOutput {
    pub text: String,
    pub thread_id: Option<String>,
    pub fallback_used: bool,
}

#[derive(Debug, Clone)]
pub struct NowNudgeBarOutput {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub summary: String,
    pub urgent: bool,
    pub primary_thread_id: Option<String>,
    pub actions: Vec<NowNudgeActionOutput>,
}

#[derive(Debug, Clone)]
pub struct NowNudgeActionOutput {
    pub kind: String,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct NowTaskLaneOutput {
    pub active: Option<NowTaskLaneItemOutput>,
    pub pending: Vec<NowTaskLaneItemOutput>,
    pub active_items: Vec<NowTaskLaneItemOutput>,
    pub next_up: Vec<NowTaskLaneItemOutput>,
    pub inbox: Vec<NowTaskLaneItemOutput>,
    pub if_time_allows: Vec<NowTaskLaneItemOutput>,
    pub completed: Vec<NowTaskLaneItemOutput>,
    pub recent_completed: Vec<NowTaskLaneItemOutput>,
    pub overflow_count: u32,
}

#[derive(Debug, Clone)]
pub struct NowTaskLaneItemOutput {
    pub id: String,
    pub task_kind: String,
    pub text: String,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub state: String,
    pub lane: Option<String>,
    pub sort_order: Option<u32>,
    pub project: Option<String>,
    pub primary_thread_id: Option<String>,
    pub due_at: Option<OffsetDateTime>,
    pub deadline: Option<OffsetDateTime>,
    pub due_label: Option<String>,
    pub is_overdue: bool,
    pub deadline_label: Option<String>,
    pub deadline_passed: bool,
}

#[derive(Debug, Clone)]
pub struct NowNextUpItemOutput {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub meta: Option<String>,
    pub detail: Option<String>,
    pub task: Option<NowTaskLaneItemOutput>,
}

#[derive(Debug, Clone)]
pub struct NowProgressOutput {
    pub base_count: u32,
    pub completed_count: u32,
    pub backlog_count: u32,
    pub completed_ratio: f64,
    pub backlog_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct NowDockedInputOutput {
    pub supported_intents: Vec<String>,
    pub day_thread_id: Option<String>,
    pub raw_capture_thread_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowOverviewOutput {
    pub dominant_action: Option<NowOverviewActionOutput>,
    pub today_timeline: Vec<NowOverviewTimelineEntryOutput>,
    pub visible_nudge: Option<NowOverviewNudgeOutput>,
    pub why_state: Vec<NowOverviewWhyStateOutput>,
    pub suggestions: Vec<NowOverviewSuggestionOutput>,
    pub decision_options: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NowOverviewActionOutput {
    pub kind: String,
    pub title: String,
    pub summary: String,
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowOverviewTimelineEntryOutput {
    pub kind: String,
    pub title: String,
    pub timestamp: i64,
    pub detail: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowOverviewNudgeOutput {
    pub kind: String,
    pub title: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct NowOverviewWhyStateOutput {
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct NowOverviewSuggestionOutput {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct NowSummaryOutput {
    pub mode: NowLabelOutput,
    pub phase: NowLabelOutput,
    pub meds: NowLabelOutput,
    pub risk: NowRiskSummaryOutput,
}

#[derive(Debug, Clone)]
pub struct NowEventOutput {
    pub event_id: Option<String>,
    pub calendar_id: Option<String>,
    pub calendar_name: Option<String>,
    pub title: String,
    pub start_ts: i64,
    pub end_ts: Option<i64>,
    pub all_day: bool,
    pub event_url: Option<String>,
    pub attachment_url: Option<String>,
    pub location: Option<String>,
    pub notes: Option<String>,
    pub attendees: Vec<String>,
    pub video_url: Option<String>,
    pub video_provider: Option<String>,
    pub status: Option<String>,
    pub transparency: Option<String>,
    pub response_status: Option<String>,
    pub prep_minutes: Option<i64>,
    pub travel_minutes: Option<i64>,
    pub leave_by_ts: Option<i64>,
    pub rescheduled: bool,
}

#[derive(Debug, Clone)]
pub struct NowTaskOutput {
    pub id: String,
    pub text: String,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub source_type: String,
    pub due_at: Option<OffsetDateTime>,
    pub deadline: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub is_inbox_project: bool,
    pub commitment_kind: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowScheduleOutput {
    pub empty_message: Option<String>,
    pub next_event: Option<NowEventOutput>,
    pub upcoming_events: Vec<NowEventOutput>,
    pub following_day_events: Vec<NowEventOutput>,
}

#[derive(Debug, Clone)]
pub struct NowTasksOutput {
    pub todoist: Vec<NowTaskOutput>,
    pub other_open: Vec<NowTaskOutput>,
    pub next_commitment: Option<NowTaskOutput>,
}

#[derive(Debug, Clone)]
pub struct NowAttentionOutput {
    pub state: NowLabelOutput,
    pub drift: NowLabelOutput,
    pub severity: NowLabelOutput,
    pub confidence: Option<f64>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NowSourceActivityOutput {
    pub label: String,
    pub timestamp: i64,
    pub summary: JsonValue,
}

#[derive(Debug, Clone)]
pub struct NowSourcesOutput {
    pub git_activity: Option<NowSourceActivityOutput>,
    pub health: Option<NowSourceActivityOutput>,
    pub mood: Option<NowSourceActivityOutput>,
    pub pain: Option<NowSourceActivityOutput>,
    pub note_document: Option<NowSourceActivityOutput>,
    pub assistant_message: Option<NowSourceActivityOutput>,
}

#[derive(Debug, Clone)]
pub struct NowFreshnessEntryOutput {
    pub key: String,
    pub label: String,
    pub status: String,
    pub last_sync_at: Option<i64>,
    pub age_seconds: Option<i64>,
    pub guidance: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowFreshnessOutput {
    pub overall_status: String,
    pub sources: Vec<NowFreshnessEntryOutput>,
}

#[derive(Debug, Clone)]
pub struct TrustReadinessFacetOutput {
    pub level: String,
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct TrustReadinessReviewOutput {
    pub open_action_count: u32,
    pub pending_execution_reviews: u32,
    pub pending_writeback_count: u32,
    pub conflict_count: u32,
}

#[derive(Debug, Clone)]
pub struct TrustReadinessOutput {
    pub level: String,
    pub headline: String,
    pub summary: String,
    pub backup: TrustReadinessFacetOutput,
    pub freshness: TrustReadinessFacetOutput,
    pub review: TrustReadinessReviewOutput,
    pub guidance: Vec<String>,
    pub follow_through: Vec<ActionItem>,
}

#[derive(Debug, Clone)]
pub struct NowDebugOutput {
    pub raw_context: JsonValue,
    pub signals_used: Vec<String>,
    pub commitments_used: Vec<String>,
    pub risk_used: Vec<String>,
}
