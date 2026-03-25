use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use std::collections::{HashMap, HashSet};
use time::OffsetDateTime;
use vel_config::{AppConfig, NowCountDisplayMode, NowTitleMode};
use vel_core::{
    normalize_risk_level, ActionItem, ActionKind, CheckInCard, Commitment, CommitmentStatus,
    ConflictCaseRecord, CurrentContextReflowStatus, CurrentContextV1, DayPlanProposal, ReflowCard,
    ReflowSeverity, ReviewSnapshot, WritebackOperationKind, WritebackOperationRecord,
    WritebackStatus,
};
use vel_storage::{SignalRecord, Storage};

use crate::{errors::AppError, services::integrations, state::AppState};

const NOW_CALENDAR_OVERRIDES_KEY: &str = "now_calendar_overrides";

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CalendarOverrideSettings {
    #[serde(default)]
    overrides: Vec<CalendarOverrideRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CalendarOverrideRecord {
    event_id: String,
    calendar_id: Option<String>,
    start_ts: i64,
    end_ts: Option<i64>,
    updated_at: i64,
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

#[derive(Debug, Clone)]
struct IntegrationGuidanceOutput {
    title: String,
    detail: String,
}

#[derive(Debug, Clone)]
struct CalendarStatusOutput {
    connected: bool,
    all_calendars_selected: bool,
    any_selected: bool,
    last_sync_at: Option<i64>,
    last_sync_status: Option<String>,
    guidance: Option<IntegrationGuidanceOutput>,
}

#[derive(Debug, Clone)]
struct SyncStatusOutput {
    last_sync_at: Option<i64>,
    last_sync_status: Option<String>,
    guidance: Option<IntegrationGuidanceOutput>,
}

#[derive(Debug, Clone)]
struct IntegrationSnapshotOutput {
    google_calendar: CalendarStatusOutput,
    todoist: SyncStatusOutput,
    activity: SyncStatusOutput,
    messaging: SyncStatusOutput,
}

struct NowTaskBuckets {
    next_commitment: Option<NowTaskOutput>,
    in_play: Vec<NowTaskOutput>,
    pullable: Vec<NowTaskOutput>,
}

pub async fn get_now(storage: &Storage, config: &AppConfig) -> Result<NowOutput, AppError> {
    get_now_internal(storage, config, None).await
}

pub async fn get_now_with_state(state: &AppState) -> Result<NowOutput, AppError> {
    get_now_internal(&state.storage, &state.config, Some(state)).await
}

pub async fn reschedule_calendar_event(
    state: &AppState,
    event_id: &str,
    calendar_id: Option<&str>,
    start_ts: i64,
    end_ts: Option<i64>,
) -> Result<NowOutput, AppError> {
    let event_id = event_id.trim();
    if event_id.is_empty() {
        return Err(AppError::bad_request("event_id must not be empty"));
    }
    if let Some(end_ts) = end_ts {
        if end_ts <= start_ts {
            return Err(AppError::bad_request(
                "end_ts must be greater than start_ts",
            ));
        }
    }

    let mut settings = load_calendar_override_settings(&state.storage).await?;
    let updated_at = OffsetDateTime::now_utc().unix_timestamp();
    let calendar_id = calendar_id
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);

    if let Some(existing) = settings.overrides.iter_mut().find(|record| {
        record.event_id == event_id && record.calendar_id.as_deref() == calendar_id.as_deref()
    }) {
        existing.start_ts = start_ts;
        existing.end_ts = end_ts;
        existing.updated_at = updated_at;
    } else {
        settings.overrides.push(CalendarOverrideRecord {
            event_id: event_id.to_string(),
            calendar_id,
            start_ts,
            end_ts,
            updated_at,
        });
    }

    save_calendar_override_settings(&state.storage, &settings).await?;
    get_now_internal(&state.storage, &state.config, Some(state)).await
}

async fn get_now_internal(
    storage: &Storage,
    config: &AppConfig,
    state: Option<&AppState>,
) -> Result<NowOutput, AppError> {
    let now = OffsetDateTime::now_utc();
    let now_ts = now.unix_timestamp();
    let timezone = crate::services::timezone::resolve_timezone(storage).await?;
    let current_day = crate::services::timezone::current_day_window(&timezone, now)?;
    let check_in = crate::services::check_in::get_current_check_in(storage, &timezone).await?;
    let apple_behavior_summary =
        crate::services::apple_behavior::get_summary(storage, config).await?;
    let action_queue = crate::services::operator_queue::build_action_items(storage, config).await?;
    let mesh_summary = if let Some(state) = state {
        Some(build_mesh_summary(state, action_queue.pending_writebacks.len() as u32).await?)
    } else {
        None
    };
    let planning_profile_summary =
        crate::services::planning_profile::load_planning_profile_proposal_summary(storage).await?;
    let commitment_scheduling_summary =
        crate::services::commitment_scheduling::load_commitment_scheduling_proposal_summary(
            storage,
        )
        .await?;
    let Some((computed_at, context)) = storage.get_current_context().await? else {
        return Ok(empty_now(
            now_ts,
            &timezone.name,
            config,
            check_in,
            &action_queue,
            mesh_summary,
            planning_profile_summary,
            commitment_scheduling_summary,
        ));
    };

    let commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 200)
        .await?;
    let completed_commitments = storage
        .list_commitments(Some(CommitmentStatus::Done), None, None, 64)
        .await?;
    let completed_today = build_completed_today_tasks(
        storage,
        &timezone,
        &current_day,
        now,
        &commitments,
        &completed_commitments,
        &action_queue.pending_writebacks,
    )
    .await?;
    let sorted_commitments = sort_commitments(commitments, &timezone, now);
    let all_open_tasks = sorted_commitments.iter().map(now_task).collect::<Vec<_>>();
    let task_buckets = split_now_tasks(&context, sorted_commitments, &timezone, now, &current_day);

    let signal_ids = context.signals_used.clone();
    let calendar_selection = integrations::google_calendar_selection_filter(storage).await?;
    let calendar_overrides = load_calendar_override_settings(storage).await?;
    let mut events = storage
        .list_signals_by_ids(&signal_ids)
        .await?
        .into_iter()
        .filter(|signal| calendar_selection.includes_visible_signal(signal))
        .filter_map(|signal| calendar_event_from_signal(signal, &calendar_overrides))
        .filter(|event| event_overlaps_current_day(event, &current_day))
        .collect::<Vec<_>>();
    if events.is_empty() {
        events = storage
            .list_signals_in_window(
                Some("calendar_event"),
                current_day.start_ts - (24 * 60 * 60),
                current_day.end_ts,
                128,
            )
            .await?
            .into_iter()
            .filter(|signal| calendar_selection.includes_visible_signal(signal))
            .filter_map(|signal| calendar_event_from_signal(signal, &calendar_overrides))
            .filter(|event| event_overlaps_current_day(event, &current_day))
            .collect();
    }
    sort_now_events(&mut events);
    let next_event = events.iter().find(|event| event.start_ts > now_ts).cloned();
    let upcoming_events: Vec<NowEventOutput> = events
        .into_iter()
        .filter(|event| event.end_ts.unwrap_or(event.start_ts) >= now_ts)
        .take(5)
        .collect();
    let next_day = crate::services::timezone::current_day_window(
        &timezone,
        OffsetDateTime::from_unix_timestamp(current_day.end_ts)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH),
    )?;
    let mut following_day_events = storage
        .list_signals_in_window(
            Some("calendar_event"),
            next_day.start_ts,
            next_day.end_ts,
            128,
        )
        .await?
        .into_iter()
        .filter(|signal| calendar_selection.includes_visible_signal(signal))
        .filter_map(|signal| calendar_event_from_signal(signal, &calendar_overrides))
        .filter(|event| event_overlaps_current_day(event, &next_day))
        .collect::<Vec<_>>();
    sort_now_events(&mut following_day_events);
    following_day_events.truncate(5);

    let integrations = integrations::get_integrations_with_config(storage, config).await?;
    let integrations = IntegrationSnapshotOutput {
        google_calendar: CalendarStatusOutput {
            connected: integrations.google_calendar.connected,
            all_calendars_selected: integrations.google_calendar.all_calendars_selected,
            any_selected: integrations
                .google_calendar
                .calendars
                .iter()
                .any(|calendar| calendar.sync_enabled),
            last_sync_at: integrations.google_calendar.last_sync_at,
            last_sync_status: integrations.google_calendar.last_sync_status.clone(),
            guidance: integrations
                .google_calendar
                .guidance
                .as_ref()
                .map(|guidance| IntegrationGuidanceOutput {
                    title: guidance.title.clone(),
                    detail: guidance.detail.clone(),
                }),
        },
        todoist: SyncStatusOutput {
            last_sync_at: integrations.todoist.last_sync_at,
            last_sync_status: integrations.todoist.last_sync_status.clone(),
            guidance: integrations.todoist.guidance.as_ref().map(|guidance| {
                IntegrationGuidanceOutput {
                    title: guidance.title.clone(),
                    detail: guidance.detail.clone(),
                }
            }),
        },
        activity: SyncStatusOutput {
            last_sync_at: integrations.activity.last_sync_at,
            last_sync_status: integrations.activity.last_sync_status.clone(),
            guidance: integrations.activity.guidance.as_ref().map(|guidance| {
                IntegrationGuidanceOutput {
                    title: guidance.title.clone(),
                    detail: guidance.detail.clone(),
                }
            }),
        },
        messaging: SyncStatusOutput {
            last_sync_at: integrations.messaging.last_sync_at,
            last_sync_status: integrations.messaging.last_sync_status.clone(),
            guidance: integrations.messaging.guidance.as_ref().map(|guidance| {
                IntegrationGuidanceOutput {
                    title: guidance.title.clone(),
                    detail: guidance.detail.clone(),
                }
            }),
        },
    };
    let freshness = build_freshness(now_ts, computed_at, &integrations, &calendar_selection);
    let trust_readiness = build_trust_readiness(storage, &freshness, &action_queue).await?;
    let people = storage.list_people().await?;
    let schedule_empty_message = schedule_empty_message(&integrations, upcoming_events.is_empty());
    let attention_reasons = context.attention_reasons.clone();
    let reasons = build_reasons_typed(&context, &attention_reasons);
    let day_plan =
        crate::services::day_plan::derive_current_day_plan(storage, &context, now_ts).await?;
    let reflow_status = crate::services::reflow::current_status_for_snapshot(&context).cloned();
    let reflow = crate::services::reflow::derive_current_reflow(storage, &context, now_ts).await?;
    let overview = build_overview(
        &context,
        now_ts,
        &schedule_empty_message,
        next_event.as_ref(),
        &task_buckets,
        check_in.as_ref(),
        reflow.as_ref(),
        &action_queue.action_items,
        &freshness,
        &trust_readiness,
    );
    let header = Some(build_header(
        config,
        &action_queue.action_items,
        check_in.as_ref(),
        reflow.as_ref(),
        reflow_status.as_ref(),
        &planning_profile_summary,
        &commitment_scheduling_summary,
    ));
    let status_row = Some(build_status_row(
        now,
        &timezone,
        &task_buckets,
        next_event.as_ref(),
    ));
    let context_line = Some(build_context_line(
        &overview,
        &task_buckets,
        next_event.as_ref(),
        &trust_readiness,
    ));
    let task_lane = Some(build_task_lane(
        &context,
        &task_buckets,
        &all_open_tasks,
        &timezone,
        now,
        &current_day,
        completed_today,
    ));
    let next_up_items =
        build_next_up_items(&upcoming_events, task_lane.as_ref(), &timezone, now_ts);
    let progress = build_progress(task_lane.as_ref());
    let docked_input = Some(build_docked_input());
    let nudge_bars = build_nudge_bars(
        check_in.as_ref(),
        reflow.as_ref(),
        &action_queue.action_items,
        mesh_summary.as_ref(),
        task_lane.as_ref(),
        &all_open_tasks,
        context_line.as_ref(),
        docked_input.as_ref(),
        now,
    );

    Ok(NowOutput {
        computed_at,
        timezone: timezone.name,
        header,
        mesh_summary,
        status_row,
        context_line,
        nudge_bars,
        task_lane,
        next_up_items,
        progress,
        docked_input,
        overview,
        summary: NowSummaryOutput {
            mode: label_for_mode(context.mode.as_str()),
            phase: label_for_phase(context.morning_state.as_str()),
            meds: label_for_meds(context.meds_status.as_str()),
            risk: risk_summary(
                context.global_risk_level.as_str(),
                context.global_risk_score,
            ),
        },
        schedule: NowScheduleOutput {
            empty_message: schedule_empty_message,
            next_event,
            upcoming_events,
            following_day_events,
        },
        tasks: NowTasksOutput {
            todoist: task_buckets.pullable,
            other_open: task_buckets.in_play,
            next_commitment: task_buckets.next_commitment,
        },
        attention: NowAttentionOutput {
            state: label_for_attention(context.attention_state.as_str()),
            drift: label_for_drift(context.drift_type.as_deref().unwrap_or("none")),
            severity: label_for_severity(context.drift_severity.as_deref().unwrap_or("none")),
            confidence: context.attention_confidence,
            reasons: attention_reasons,
        },
        sources: NowSourcesOutput {
            git_activity: context_source_activity_typed(
                &context,
                "git_activity_summary",
                "Git activity",
                context.git_activity_summary.clone(),
            ),
            health: apple_behavior_summary
                .as_ref()
                .map(|summary| NowSourceActivityOutput {
                    label: "Apple behavior".to_string(),
                    timestamp: summary.generated_at,
                    summary: crate::services::apple_behavior::summary_to_source_activity(summary),
                })
                .or_else(|| {
                    context_source_activity_typed(
                        &context,
                        "health_summary",
                        "Health",
                        context.health_summary.clone(),
                    )
                }),
            mood: context_source_activity_typed(
                &context,
                "mood_summary",
                "Mood",
                context.mood_summary.clone(),
            ),
            pain: context_source_activity_typed(
                &context,
                "pain_summary",
                "Pain",
                context.pain_summary.clone(),
            ),
            note_document: context_source_activity_typed(
                &context,
                "note_document_summary",
                "Recent note",
                context.note_document_summary.clone(),
            ),
            assistant_message: context_source_activity_typed(
                &context,
                "assistant_message_summary",
                "Recent transcript",
                context.assistant_message_summary.clone(),
            ),
        },
        freshness,
        trust_readiness,
        planning_profile_summary: (!planning_profile_summary.is_empty())
            .then_some(planning_profile_summary),
        commitment_scheduling_summary: (!commitment_scheduling_summary.is_empty())
            .then_some(commitment_scheduling_summary),
        check_in,
        day_plan,
        reflow,
        reflow_status,
        action_items: select_now_action_items(&action_queue.action_items, 5),
        review_snapshot: action_queue.review_snapshot,
        pending_writebacks: action_queue.pending_writebacks,
        conflicts: action_queue.conflicts,
        people,
        reasons,
        debug: NowDebugOutput {
            raw_context: context.clone().into_json(),
            signals_used: context.signals_used.clone(),
            commitments_used: context.commitments_used.clone(),
            risk_used: context.risk_used.clone(),
        },
    })
}

fn build_reasons_typed(context: &CurrentContextV1, attention_reasons: &[String]) -> Vec<String> {
    let mut reasons = Vec::new();
    if !context.mode.is_empty() {
        reasons.push(format!("Mode: {}", label_for_mode(&context.mode).label));
    }
    if context.prep_window_active {
        reasons.push("Prep window active".to_string());
    }
    if context.commute_window_active {
        reasons.push("Commute window active".to_string());
    }
    if context.meds_status == "pending" {
        reasons.push("Medication task is still pending".to_string());
    }
    reasons.extend(attention_reasons.iter().cloned());
    reasons.truncate(8);
    reasons
}

fn context_source_activity_typed(
    context: &CurrentContextV1,
    key: &str,
    label: &str,
    typed_summary: Option<JsonValue>,
) -> Option<NowSourceActivityOutput> {
    let summary = typed_summary.or_else(|| context.extra.get(key).cloned())?;
    let timestamp = summary
        .get("timestamp")
        .and_then(JsonValue::as_i64)
        .unwrap_or(context.computed_at);
    Some(NowSourceActivityOutput {
        label: label.to_string(),
        timestamp,
        summary,
    })
}

async fn build_trust_readiness(
    storage: &Storage,
    freshness: &NowFreshnessOutput,
    action_queue: &crate::services::operator_queue::ActionQueueSnapshot,
) -> Result<TrustReadinessOutput, AppError> {
    let backup = crate::services::backup::backup_trust_for_storage(storage).await?;
    Ok(build_trust_readiness_from_parts(
        Some(&backup),
        freshness,
        &action_queue.review_snapshot,
        action_queue.pending_writebacks.len() as u32,
        action_queue.conflicts.len() as u32,
        &action_queue.action_items,
    ))
}

fn build_trust_readiness_from_parts(
    backup: Option<&vel_api_types::BackupTrustData>,
    freshness: &NowFreshnessOutput,
    review_snapshot: &ReviewSnapshot,
    pending_writeback_count: u32,
    conflict_count: u32,
    action_items: &[ActionItem],
) -> TrustReadinessOutput {
    let backup_level = backup
        .map(|value| match value.level {
            vel_api_types::BackupTrustLevelData::Ok => "ok",
            vel_api_types::BackupTrustLevelData::Warn => "warn",
            vel_api_types::BackupTrustLevelData::Fail => "fail",
        })
        .unwrap_or("warn");
    let freshness_level = match freshness.overall_status.as_str() {
        "fresh" => "ok",
        "stale" => "warn",
        "missing" | "degraded" => "warn",
        _ => "warn",
    };
    let review_level = if conflict_count > 0 || review_snapshot.pending_execution_reviews > 0 {
        "warn"
    } else {
        "ok"
    };
    let overall_level = fold_levels([backup_level, freshness_level, review_level]);

    let backup_facet = TrustReadinessFacetOutput {
        level: backup_level.to_string(),
        label: "Backup".to_string(),
        detail: backup
            .map(|value| {
                value
                    .guidance
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "Backup trust is available.".to_string())
            })
            .unwrap_or_else(|| "Backup trust status is unavailable.".to_string()),
    };
    let freshness_facet = TrustReadinessFacetOutput {
        level: freshness_level.to_string(),
        label: "Freshness".to_string(),
        detail: match freshness.overall_status.as_str() {
            "fresh" => "Current context and integrations look fresh enough to trust.".to_string(),
            "stale" => "Some context inputs are stale and may need recovery.".to_string(),
            "missing" => "Some context inputs are missing and need recovery.".to_string(),
            _ => "Current context freshness is degraded.".to_string(),
        },
    };
    let review = TrustReadinessReviewOutput {
        open_action_count: review_snapshot.open_action_count,
        pending_execution_reviews: review_snapshot.pending_execution_reviews,
        pending_writeback_count,
        conflict_count,
    };

    let (headline, summary) = if overall_level == "fail" {
        (
            "Trust needs attention".to_string(),
            "Backup trust is not yet strong enough for risky maintenance.".to_string(),
        )
    } else if conflict_count > 0 || review_snapshot.pending_execution_reviews > 0 {
        (
            "Review is pending".to_string(),
            format!(
                "{} conflict(s) and {} supervised review(s) still need operator attention.",
                conflict_count, review_snapshot.pending_execution_reviews
            ),
        )
    } else if freshness_level == "warn" {
        (
            "Readiness is degraded".to_string(),
            "Some context inputs are stale enough that recovery may be needed before trusting the day plan."
                .to_string(),
        )
    } else {
        (
            "Ready".to_string(),
            "Backup, freshness, and review pressure look healthy enough for normal operation."
                .to_string(),
        )
    };

    let mut guidance = vec![backup_facet.detail.clone()];
    if freshness_level != "ok" {
        guidance.push(freshness_facet.detail.clone());
    }
    if review.pending_execution_reviews > 0 || review.conflict_count > 0 {
        guidance.push(
            "Review the remaining conflicts or supervised execution handoffs before risky actions."
                .to_string(),
        );
    }
    guidance.truncate(3);
    let follow_through = trust_follow_through_items(action_items);

    TrustReadinessOutput {
        level: overall_level.to_string(),
        headline,
        summary,
        backup: backup_facet,
        freshness: freshness_facet,
        review,
        guidance,
        follow_through,
    }
}

fn trust_follow_through_items(action_items: &[ActionItem]) -> Vec<ActionItem> {
    action_items
        .iter()
        .filter(|item| {
            matches!(
                item.kind,
                ActionKind::Recovery
                    | ActionKind::Freshness
                    | ActionKind::Review
                    | ActionKind::Conflict
            ) || (item.kind == ActionKind::Intervention
                && item
                    .evidence
                    .iter()
                    .any(|evidence| evidence.source_kind == "assistant_proposal"))
        })
        .take(3)
        .cloned()
        .collect()
}

fn select_now_action_items(action_items: &[ActionItem], limit: usize) -> Vec<ActionItem> {
    let mut selected = Vec::new();

    for item in action_items {
        if item.surface == vel_core::ActionSurface::Now {
            selected.push(item.clone());
            if selected.len() >= limit {
                return selected;
            }
        }
    }

    for item in action_items {
        if selected.iter().any(|existing| existing.id == item.id) {
            continue;
        }
        selected.push(item.clone());
        if selected.len() >= limit {
            break;
        }
    }

    selected
}

fn fold_levels<'a>(levels: impl IntoIterator<Item = &'a str>) -> &'a str {
    let mut current = "ok";
    for level in levels {
        current = match (current, level) {
            ("fail", _) | (_, "fail") => "fail",
            ("warn", _) | (_, "warn") => "warn",
            _ => "ok",
        };
    }
    current
}

fn empty_now(
    now_ts: i64,
    timezone: &str,
    config: &AppConfig,
    check_in: Option<CheckInCard>,
    action_queue: &crate::services::operator_queue::ActionQueueSnapshot,
    mesh_summary: Option<NowMeshSummaryOutput>,
    planning_profile_summary: crate::services::planning_profile::PlanningProfileProposalSummary,
    commitment_scheduling_summary:
        crate::services::commitment_scheduling::CommitmentSchedulingProposalSummary,
) -> NowOutput {
    let freshness = NowFreshnessOutput {
        overall_status: "stale".to_string(),
        sources: vec![NowFreshnessEntryOutput {
            key: "context".to_string(),
            label: "Context".to_string(),
            status: "missing".to_string(),
            last_sync_at: None,
            age_seconds: None,
            guidance: None,
        }],
    };
    let trust_readiness = build_trust_readiness_from_parts(
        None,
        &freshness,
        &action_queue.review_snapshot,
        action_queue.pending_writebacks.len() as u32,
        action_queue.conflicts.len() as u32,
        &action_queue.action_items,
    );
    NowOutput {
        computed_at: now_ts,
        timezone: timezone.to_string(),
        header: Some(build_header(
            config,
            &action_queue.action_items,
            check_in.as_ref(),
            None,
            None,
            &planning_profile_summary,
            &commitment_scheduling_summary,
        )),
        mesh_summary: mesh_summary.clone(),
        status_row: Some(empty_status_row(now_ts, timezone)),
        context_line: Some(NowContextLineOutput {
            text: "No active context yet. Sync integrations or run evaluate.".to_string(),
            thread_id: None,
            fallback_used: true,
        }),
        nudge_bars: build_nudge_bars(
            check_in.as_ref(),
            None,
            &action_queue.action_items,
            mesh_summary.as_ref(),
            None,
            &[],
            None,
            None,
            OffsetDateTime::from_unix_timestamp(now_ts)
                .expect("empty now timestamp should convert to offset datetime"),
        ),
        task_lane: Some(NowTaskLaneOutput {
            active: None,
            pending: Vec::new(),
            active_items: Vec::new(),
            next_up: Vec::new(),
            inbox: Vec::new(),
            if_time_allows: Vec::new(),
            completed: Vec::new(),
            recent_completed: Vec::new(),
            overflow_count: 0,
        }),
        next_up_items: Vec::new(),
        progress: NowProgressOutput {
            base_count: 1,
            completed_count: 0,
            backlog_count: 0,
            completed_ratio: 0.0,
            backlog_ratio: 0.0,
        },
        docked_input: Some(build_docked_input()),
        overview: empty_overview(&freshness, &trust_readiness),
        summary: NowSummaryOutput {
            mode: label("unknown", "Unknown"),
            phase: label("unknown", "Unknown"),
            meds: label("unknown", "Unknown"),
            risk: risk_summary("unknown", None),
        },
        schedule: NowScheduleOutput {
            empty_message: Some(
                "No current context yet. Sync calendar sources or run evaluate.".to_string(),
            ),
            next_event: None,
            upcoming_events: Vec::new(),
            following_day_events: Vec::new(),
        },
        tasks: NowTasksOutput {
            todoist: Vec::new(),
            other_open: Vec::new(),
            next_commitment: None,
        },
        attention: NowAttentionOutput {
            state: label("unknown", "Unknown"),
            drift: label("none", "None"),
            severity: label("none", "None"),
            confidence: None,
            reasons: Vec::new(),
        },
        sources: NowSourcesOutput {
            git_activity: None,
            health: None,
            mood: None,
            pain: None,
            note_document: None,
            assistant_message: None,
        },
        freshness: freshness.clone(),
        trust_readiness,
        planning_profile_summary: (!planning_profile_summary.is_empty())
            .then_some(planning_profile_summary),
        commitment_scheduling_summary: (!commitment_scheduling_summary.is_empty())
            .then_some(commitment_scheduling_summary),
        check_in,
        day_plan: None,
        reflow: None,
        reflow_status: None,
        action_items: select_now_action_items(&action_queue.action_items, 5),
        review_snapshot: action_queue.review_snapshot.clone(),
        pending_writebacks: action_queue.pending_writebacks.clone(),
        conflicts: action_queue.conflicts.clone(),
        people: Vec::new(),
        reasons: vec![
            "No current context yet. Sync integrations or run evaluate.".to_string(),
            "Operator follow-through still surfaces pending review, writeback, and recovery work."
                .to_string(),
        ],
        debug: NowDebugOutput {
            raw_context: json!({}),
            signals_used: Vec::new(),
            commitments_used: Vec::new(),
            risk_used: Vec::new(),
        },
    }
}

fn build_header(
    config: &AppConfig,
    action_items: &[ActionItem],
    check_in: Option<&CheckInCard>,
    reflow: Option<&ReflowCard>,
    reflow_status: Option<&CurrentContextReflowStatus>,
    planning_profile_summary: &crate::services::planning_profile::PlanningProfileProposalSummary,
    commitment_scheduling_summary: &crate::services::commitment_scheduling::CommitmentSchedulingProposalSummary,
) -> NowHeaderOutput {
    let now_items = action_items
        .iter()
        .filter(|item| item.surface == vel_core::ActionSurface::Now)
        .count() as u32;
    let snoozed_count = action_items
        .iter()
        .filter(|item| item.snoozed_until.is_some())
        .count() as u32;
    let review_apply_count = planning_profile_summary.pending_count
        + commitment_scheduling_summary.pending_count
        + action_items
            .iter()
            .filter(|item| matches!(item.kind, ActionKind::Review | ActionKind::Conflict))
            .count() as u32;
    let reflow_count = u32::from(reflow.is_some() || reflow_status.is_some());
    let follow_up_count = action_items
        .iter()
        .filter(|item| item.thread_route.is_some())
        .count() as u32;

    NowHeaderOutput {
        title: resolve_now_title(config),
        buckets: vec![
            header_bucket(config, "threads_by_type", now_items, now_items > 0, None),
            header_bucket(
                config,
                "needs_input",
                u32::from(check_in.map(|card| card.blocking).unwrap_or(false)),
                check_in.map(|card| card.blocking).unwrap_or(false),
                check_in
                    .and_then(|card| card.escalation.as_ref())
                    .and_then(|escalation| escalation.thread_id.clone()),
            ),
            header_bucket(config, "new_nudges", now_items, now_items > 0, None),
            header_bucket(config, "search_filter", 0, false, None),
            header_bucket(config, "snoozed", snoozed_count, false, None),
            header_bucket(
                config,
                "review_apply",
                review_apply_count,
                review_apply_count > 0,
                planning_profile_summary
                    .latest_pending
                    .as_ref()
                    .map(|item| item.thread_id.clone())
                    .or_else(|| {
                        commitment_scheduling_summary
                            .latest_pending
                            .as_ref()
                            .map(|item| item.thread_id.clone())
                    }),
            ),
            header_bucket(
                config,
                "reflow",
                reflow_count,
                reflow_count > 0,
                reflow_status.and_then(|status| status.thread_id.clone()),
            ),
            header_bucket(
                config,
                "follow_up",
                follow_up_count,
                follow_up_count > 0,
                None,
            ),
        ],
    }
}

fn header_bucket(
    config: &AppConfig,
    kind: &str,
    count: u32,
    urgent: bool,
    route_thread_id: Option<String>,
) -> NowHeaderBucketOutput {
    NowHeaderBucketOutput {
        kind: kind.to_string(),
        count,
        count_display: now_count_display_mode(config),
        urgent,
        route_thread_id,
    }
}

fn resolve_now_title(config: &AppConfig) -> String {
    match config.now.title_mode {
        NowTitleMode::OperatorNamePossessive => config
            .node_display_name
            .as_ref()
            .filter(|value| !value.trim().is_empty())
            .map(|value| format!("{value}'s Now"))
            .unwrap_or_else(|| "Now".to_string()),
        NowTitleMode::Literal => config
            .now
            .title_literal
            .as_ref()
            .filter(|value| !value.trim().is_empty())
            .cloned()
            .unwrap_or_else(|| "Now".to_string()),
    }
}

fn now_count_display_mode(config: &AppConfig) -> String {
    match config.now.bucket_count_display {
        NowCountDisplayMode::AlwaysShow => "always_show",
        NowCountDisplayMode::ShowNonzero => "show_nonzero",
        NowCountDisplayMode::HiddenUntilActive => "hidden_until_active",
    }
    .to_string()
}

fn build_status_row(
    now: OffsetDateTime,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    task_buckets: &NowTaskBuckets,
    next_event: Option<&NowEventOutput>,
) -> NowStatusRowOutput {
    let local = DateTime::<Utc>::from_timestamp(now.unix_timestamp(), now.nanosecond())
        .expect("current time should convert")
        .with_timezone(&timezone.tz());
    let context_label = if let Some(task) = task_buckets.next_commitment.as_ref() {
        task.text.clone()
    } else if let Some(event) = next_event {
        event.title.clone()
    } else {
        "No active context".to_string()
    };
    let elapsed_label = next_event
        .filter(|event| event.start_ts <= now.unix_timestamp())
        .map(|event| format_elapsed_minutes(now.unix_timestamp() - event.start_ts))
        .unwrap_or_else(|| "No active task".to_string());

    NowStatusRowOutput {
        date_label: crate::services::timezone::local_date_string(timezone, now),
        time_label: format!("{:02}:{:02}", local.hour(), local.minute()),
        context_label,
        elapsed_label,
    }
}

fn empty_status_row(now_ts: i64, timezone: &str) -> NowStatusRowOutput {
    let now = OffsetDateTime::from_unix_timestamp(now_ts).expect("timestamp should convert");
    let timezone = crate::services::timezone::ResolvedTimeZone::parse(timezone)
        .unwrap_or_else(|_| crate::services::timezone::ResolvedTimeZone::utc());
    build_status_row(
        now,
        &timezone,
        &NowTaskBuckets {
            next_commitment: None,
            in_play: Vec::new(),
            pullable: Vec::new(),
        },
        None,
    )
}

fn build_context_line(
    overview: &NowOverviewOutput,
    task_buckets: &NowTaskBuckets,
    next_event: Option<&NowEventOutput>,
    trust_readiness: &TrustReadinessOutput,
) -> NowContextLineOutput {
    let text = if let Some(action) = overview.dominant_action.as_ref() {
        action.summary.clone()
    } else if let Some(task) = task_buckets.next_commitment.as_ref() {
        format!("{} is the next ranked task for the current day.", task.text)
    } else if let Some(event) = next_event {
        format!("{} is the next calendar anchor for the day.", event.title)
    } else {
        trust_readiness.summary.clone()
    };

    NowContextLineOutput {
        text,
        thread_id: None,
        fallback_used: true,
    }
}

async fn build_mesh_summary(
    state: &AppState,
    queued_write_count: u32,
) -> Result<NowMeshSummaryOutput, AppError> {
    let bootstrap = crate::services::client_sync::effective_cluster_bootstrap_data(state).await?;
    let workers = crate::services::client_sync::cluster_workers_data(state).await?;

    let authority_label = if bootstrap.active_authority_node_id == bootstrap.node_id {
        bootstrap.node_display_name.clone()
    } else {
        bootstrap
            .linked_nodes
            .iter()
            .find(|node| node.node_id == bootstrap.active_authority_node_id)
            .map(|node| node.node_display_name.clone())
            .unwrap_or_else(|| bootstrap.active_authority_node_id.clone())
    };

    let linked_node_count = bootstrap.linked_nodes.len() as u32;
    let any_ready = workers
        .workers
        .iter()
        .any(|worker| worker.status == "ready" || worker.sync_status == "ready");
    let any_offline = workers
        .workers
        .iter()
        .any(|worker| worker.status == "offline" || worker.sync_status == "offline");
    let any_stale = workers.workers.iter().any(|worker| {
        matches!(
            worker.sync_status.as_str(),
            "degraded" | "discovered_via_tailscale" | "discovered_via_lan" | "unknown"
        ) || worker.last_sync_error.is_some()
    });

    let sync_state = if linked_node_count == 0 && workers.workers.is_empty() {
        "local_only"
    } else if any_ready {
        "synced"
    } else if any_offline {
        "offline"
    } else {
        "stale"
    };

    let last_sync_at = workers
        .workers
        .iter()
        .filter_map(|worker| {
            worker
                .last_upstream_sync_at
                .max(worker.last_downstream_sync_at)
        })
        .max()
        .or_else(|| {
            bootstrap
                .linked_nodes
                .iter()
                .filter_map(|node| node.last_seen_at.map(|value| value.unix_timestamp()))
                .max()
        });

    let linking_needs_review = bootstrap
        .linked_nodes
        .iter()
        .any(|node| !matches!(node.status, vel_core::LinkStatus::Linked))
        || workers
            .workers
            .iter()
            .any(|worker| worker.incoming_linking_prompt.is_some());

    let repair_route = if linking_needs_review {
        Some(NowRepairRouteOutput {
            target: "settings_linking".to_string(),
            summary: "Linking or paired-node trust needs review before relying on cross-client continuity."
                .to_string(),
        })
    } else if sync_state == "offline" {
        Some(NowRepairRouteOutput {
            target: "settings_sync".to_string(),
            summary: "This client is offline or disconnected from the shared authority runtime."
                .to_string(),
        })
    } else if sync_state == "stale" || any_stale || queued_write_count > 0 {
        Some(NowRepairRouteOutput {
            target: "settings_recovery".to_string(),
            summary:
                "Sync or queued-write posture needs review before trusting all cross-client state."
                    .to_string(),
        })
    } else {
        None
    };

    let urgent = repair_route.is_some() && (sync_state != "local_only" || queued_write_count > 0);

    Ok(NowMeshSummaryOutput {
        authority_node_id: bootstrap.active_authority_node_id,
        authority_label,
        sync_state: sync_state.to_string(),
        linked_node_count,
        queued_write_count,
        last_sync_at,
        urgent,
        repair_route,
    })
}

fn build_nudge_bars(
    check_in: Option<&CheckInCard>,
    reflow: Option<&ReflowCard>,
    action_items: &[ActionItem],
    mesh_summary: Option<&NowMeshSummaryOutput>,
    task_lane: Option<&NowTaskLaneOutput>,
    all_open_tasks: &[NowTaskOutput],
    context_line: Option<&NowContextLineOutput>,
    docked_input: Option<&NowDockedInputOutput>,
    now: OffsetDateTime,
) -> Vec<NowNudgeBarOutput> {
    let mut bars = Vec::new();

    if let Some(card) = check_in {
        bars.push(NowNudgeBarOutput {
            id: card.id.to_string(),
            kind: "needs_input".to_string(),
            title: card.title.clone(),
            summary: card.summary.clone(),
            urgent: card.blocking,
            primary_thread_id: card
                .escalation
                .as_ref()
                .and_then(|escalation| escalation.thread_id.clone()),
            actions: card
                .transitions
                .iter()
                .map(|transition| NowNudgeActionOutput {
                    kind: transition.kind.to_string(),
                    label: transition.label.clone(),
                })
                .collect(),
        });
    }

    if let Some(card) = reflow {
        bars.push(NowNudgeBarOutput {
            id: card.id.to_string(),
            kind: "reflow_proposal".to_string(),
            title: card.title.clone(),
            summary: card.summary.clone(),
            urgent: matches!(
                card.severity,
                ReflowSeverity::Critical | ReflowSeverity::High
            ),
            primary_thread_id: None,
            actions: card
                .transitions
                .iter()
                .map(|transition| NowNudgeActionOutput {
                    kind: transition.kind.to_string(),
                    label: transition.label.clone(),
                })
                .collect(),
        });
    }

    if let Some(mesh_summary) = mesh_summary.filter(|summary| summary.urgent) {
        bars.push(NowNudgeBarOutput {
            id: "mesh_summary_warning".to_string(),
            kind: "trust_warning".to_string(),
            title: format!("{} needs attention", mesh_summary.authority_label),
            summary: mesh_summary
                .repair_route
                .as_ref()
                .map(|route| route.summary.clone())
                .unwrap_or_else(|| {
                    "Cross-client trust posture needs review before relying on current state."
                        .to_string()
                }),
            urgent: true,
            primary_thread_id: None,
            actions: vec![NowNudgeActionOutput {
                kind: "open_settings".to_string(),
                label: "Open settings".to_string(),
            }],
        });
    }

    if let Some(overdue_bar) =
        build_overdue_nudge_bar(task_lane, all_open_tasks, context_line, docked_input, now)
    {
        bars.push(overdue_bar);
    }

    for item in action_items
        .iter()
        .filter(|item| item.surface == vel_core::ActionSurface::Now)
        .filter(|item| !suppress_action_item_nudge(item))
        .take(3)
    {
        bars.push(NowNudgeBarOutput {
            id: item.id.to_string(),
            kind: map_action_item_to_bar_kind(item.kind),
            title: item.title.clone(),
            summary: item.summary.clone(),
            urgent: item.rank >= 80,
            primary_thread_id: item
                .thread_route
                .as_ref()
                .and_then(|route| route.thread_id.clone()),
            actions: vec![NowNudgeActionOutput {
                kind: "accept".to_string(),
                label: item
                    .thread_route
                    .as_ref()
                    .map(|route| route.label.clone())
                    .unwrap_or_else(|| "Accept".to_string()),
            }],
        });
    }

    bars
}

fn build_overdue_nudge_bar(
    task_lane: Option<&NowTaskLaneOutput>,
    all_open_tasks: &[NowTaskOutput],
    context_line: Option<&NowContextLineOutput>,
    docked_input: Option<&NowDockedInputOutput>,
    now: OffsetDateTime,
) -> Option<NowNudgeBarOutput> {
    let overdue_ids = all_open_tasks
        .iter()
        .filter_map(|task| {
            task.due_at
                .filter(|due_at| due_at.unix_timestamp() < now.unix_timestamp())
                .map(|_| task.id.clone())
        })
        .collect::<Vec<_>>();
    if overdue_ids.is_empty() {
        return None;
    }

    let primary_thread_id = context_line
        .and_then(|line| line.thread_id.clone())
        .or_else(|| docked_input.and_then(|input| input.day_thread_id.clone()))
        .or_else(|| {
            task_lane
                .into_iter()
                .flat_map(|lane| {
                    lane.active_items
                        .iter()
                        .chain(lane.next_up.iter())
                        .chain(lane.inbox.iter())
                        .chain(lane.if_time_allows.iter())
                        .chain(lane.completed.iter())
                })
                .find_map(|item| item.primary_thread_id.clone())
        });
    let overdue_count = overdue_ids.len();

    let mut actions = vec![
        NowNudgeActionOutput {
            kind: format!("reschedule_today:{}", overdue_ids.join(",")),
            label: "Reschedule all to today".to_string(),
        },
        NowNudgeActionOutput {
            kind: "jump_backlog:now-backlog".to_string(),
            label: "Review backlog".to_string(),
        },
    ];
    if primary_thread_id.is_some() {
        actions.push(NowNudgeActionOutput {
            kind: "open_thread".to_string(),
            label: "Open thread".to_string(),
        });
    }

    Some(NowNudgeBarOutput {
        id: "todoist_overdue_backlog".to_string(),
        kind: "nudge".to_string(),
        title: format!(
            "{} overdue {} still unresolved",
            overdue_count,
            if overdue_count == 1 { "item is" } else { "items are" }
        ),
        summary: "Overdue work stays visible until you commit it into the day, keep it in backlog, or reschedule it to today without committing it.".to_string(),
        urgent: true,
        primary_thread_id,
        actions,
    })
}

fn suppress_action_item_nudge(item: &ActionItem) -> bool {
    item.kind == ActionKind::NextStep
        && item.summary == "This commitment is overdue and should be handled or rescheduled."
        && item
            .evidence
            .iter()
            .any(|evidence| evidence.source_kind == "commitment")
}

fn map_action_item_to_bar_kind(kind: ActionKind) -> String {
    match kind {
        ActionKind::Review => "review_request",
        ActionKind::Freshness => "freshness_warning",
        ActionKind::Recovery | ActionKind::Conflict | ActionKind::Linking => "trust_warning",
        _ => "nudge",
    }
    .to_string()
}

fn build_task_lane(
    context: &CurrentContextV1,
    task_buckets: &NowTaskBuckets,
    all_open_tasks: &[NowTaskOutput],
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
    current_day: &crate::services::timezone::CurrentDayWindow,
    completed_commitments: Vec<Commitment>,
) -> NowTaskLaneOutput {
    let mut open_by_id = HashMap::new();
    let default_active = Vec::new();
    let (default_inbox, default_next_up): (Vec<_>, Vec<_>) = task_buckets
        .next_commitment
        .iter()
        .cloned()
        .chain(task_buckets.in_play.iter().cloned())
        .partition(is_server_inbox_task);
    let (default_inbox_pullable, default_if_time_allows): (Vec<_>, Vec<_>) = task_buckets
        .pullable
        .clone()
        .into_iter()
        .partition(is_server_inbox_task);
    let default_inbox = default_inbox
        .into_iter()
        .chain(default_inbox_pullable)
        .collect::<Vec<_>>();

    for task in default_active
        .iter()
        .chain(default_next_up.iter())
        .chain(default_inbox.iter())
        .chain(default_if_time_allows.iter())
    {
        open_by_id.insert(task.id.clone(), task.clone());
    }
    for task in all_open_tasks {
        open_by_id
            .entry(task.id.clone())
            .or_insert_with(|| task.clone());
    }

    let completed_by_id = completed_commitments
        .iter()
        .map(|commitment| (commitment.id.as_ref().to_string(), now_task(commitment)))
        .collect::<HashMap<_, _>>();

    let mut consumed = HashSet::new();
    let mut assign_items = |ids: &[String], lane: &str, source: &HashMap<String, NowTaskOutput>| {
        let mut items = Vec::new();
        for (index, id) in ids.iter().enumerate() {
            if let Some(task) = source.get(id) {
                consumed.insert(id.clone());
                items.push(task_output_to_lane_item(
                    task,
                    lane,
                    Some(index as u32),
                    timezone,
                    now,
                    current_day,
                ));
            }
        }
        items
    };

    let mut active_items = assign_items(
        &context.task_lanes.active_commitment_ids,
        "active",
        &open_by_id,
    );
    let mut next_up = assign_items(
        &context.task_lanes.next_up_commitment_ids,
        "next_up",
        &open_by_id,
    );
    let mut inbox = Vec::new();
    let mut if_time_allows = assign_items(
        &context.task_lanes.if_time_allows_commitment_ids,
        "if_time_allows",
        &open_by_id,
    );
    let mut completed = Vec::new();

    for task in default_active {
        if consumed.insert(task.id.clone()) {
            active_items.push(task_output_to_lane_item(
                &task,
                "active",
                None,
                timezone,
                now,
                current_day,
            ));
        }
    }
    for task in default_next_up {
        if consumed.insert(task.id.clone()) {
            next_up.push(task_output_to_lane_item(
                &task,
                "next_up",
                None,
                timezone,
                now,
                current_day,
            ));
        }
    }
    for task in default_inbox {
        if consumed.insert(task.id.clone()) {
            inbox.push(task_output_to_lane_item(
                &task,
                "inbox",
                None,
                timezone,
                now,
                current_day,
            ));
        }
    }
    for task in default_if_time_allows {
        if consumed.insert(task.id.clone()) {
            if_time_allows.push(task_output_to_lane_item(
                &task,
                "if_time_allows",
                None,
                timezone,
                now,
                current_day,
            ));
        }
    }
    for commitment in completed_commitments {
        let id = commitment.id.as_ref().to_string();
        if consumed.insert(id.clone()) {
            completed.push(task_output_to_lane_item(
                &now_task(&commitment),
                "completed",
                Some(completed.len() as u32),
                timezone,
                now,
                current_day,
            ));
        } else if let Some(task) = completed_by_id.get(&id) {
            completed.push(task_output_to_lane_item(
                task,
                "completed",
                Some(completed.len() as u32),
                timezone,
                now,
                current_day,
            ));
        }
    }

    let active = active_items.first().cloned();
    let mut pending = next_up.clone();
    pending.extend(inbox.clone());
    pending.extend(if_time_allows.clone());

    NowTaskLaneOutput {
        active,
        pending,
        active_items,
        next_up,
        inbox,
        if_time_allows,
        completed: completed.clone(),
        recent_completed: completed,
        overflow_count: 0,
    }
}

fn build_next_up_items(
    upcoming_events: &[NowEventOutput],
    task_lane: Option<&NowTaskLaneOutput>,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now_ts: i64,
) -> Vec<NowNextUpItemOutput> {
    let active_event_id = upcoming_events
        .iter()
        .find(|event| {
            let end_ts = event.end_ts.unwrap_or(event.start_ts);
            event.start_ts <= now_ts && end_ts >= now_ts
        })
        .map(now_event_item_id);
    let mut items = upcoming_events
        .iter()
        .filter(|event| Some(now_event_item_id(event)) != active_event_id)
        .map(|event| NowNextUpItemOutput {
            kind: "event".to_string(),
            id: now_event_item_id(event),
            title: event.title.clone(),
            meta: Some(format_event_window_label(event, timezone)),
            detail: Some(
                event
                    .location
                    .clone()
                    .unwrap_or_else(|| "Calendar event".to_string()),
            ),
            task: None,
        })
        .collect::<Vec<_>>();
    items.extend(
        task_lane
            .into_iter()
            .flat_map(|lane| lane.next_up.iter().cloned())
            .map(|task| NowNextUpItemOutput {
                kind: "task".to_string(),
                id: task.id.clone(),
                title: task.title.clone(),
                meta: None,
                detail: None,
                task: Some(task),
            }),
    );
    items
}

fn build_progress(task_lane: Option<&NowTaskLaneOutput>) -> NowProgressOutput {
    let Some(task_lane) = task_lane else {
        return NowProgressOutput {
            base_count: 1,
            completed_count: 0,
            backlog_count: 0,
            completed_ratio: 0.0,
            backlog_ratio: 0.0,
        };
    };
    let completed_count = task_lane.completed.len() as u32;
    let active_count = task_lane.active_items.len() as u32;
    let next_up_count = task_lane.next_up.len() as u32;
    let backlog_count = task_lane.if_time_allows.len() as u32;
    let base_count = (active_count + next_up_count + completed_count).max(1);
    NowProgressOutput {
        base_count,
        completed_count,
        backlog_count,
        completed_ratio: completed_count as f64 / base_count as f64,
        backlog_ratio: if backlog_count > 0 {
            backlog_count as f64 / base_count as f64
        } else {
            0.0
        },
    }
}

fn is_server_inbox_task(task: &NowTaskOutput) -> bool {
    task.source_type.eq_ignore_ascii_case("todoist")
        && task
            .project
            .as_deref()
            .map(|value| value.trim().eq_ignore_ascii_case("inbox"))
            .unwrap_or(true)
}

async fn build_completed_today_tasks(
    storage: &Storage,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    current_day: &crate::services::timezone::CurrentDayWindow,
    now: OffsetDateTime,
    open_commitments: &[Commitment],
    done_commitments: &[Commitment],
    pending_writebacks: &[WritebackOperationRecord],
) -> Result<Vec<Commitment>, AppError> {
    let provider_completed =
        crate::services::integrations_todoist::list_completed_todoist_tasks_for_window(
            storage,
            current_day.start_ts,
            current_day.end_ts,
        )
        .await?;

    Ok(merge_completed_today_commitments(
        open_commitments,
        done_commitments,
        &provider_completed,
        pending_writebacks,
        timezone,
        current_day,
        now,
    ))
}

fn merge_completed_today_commitments(
    open_commitments: &[Commitment],
    done_commitments: &[Commitment],
    provider_completed: &[crate::services::integrations_todoist::TodoistCompletedTaskSnapshot],
    pending_writebacks: &[WritebackOperationRecord],
    timezone: &crate::services::timezone::ResolvedTimeZone,
    current_day: &crate::services::timezone::CurrentDayWindow,
    now: OffsetDateTime,
) -> Vec<Commitment> {
    let mut by_local_id = HashMap::new();
    let mut by_source_id = HashMap::new();

    for commitment in open_commitments.iter().chain(done_commitments.iter()) {
        by_local_id.insert(commitment.id.as_ref().to_string(), commitment.clone());
        if let Some(source_id) = commitment.source_id.as_ref() {
            by_source_id.insert(source_id.clone(), commitment.clone());
        }
    }

    let mut merged = Vec::new();
    let mut seen = HashSet::new();

    for commitment in done_commitments.iter().cloned() {
        if resolved_within_current_day(&commitment, current_day)
            && seen.insert(commitment.id.as_ref().to_string())
        {
            merged.push(commitment);
        }
    }

    for item in provider_completed {
        let source_id = format!("todoist_{}", item.id);
        let Some(base) = by_source_id.get(&source_id) else {
            continue;
        };
        let resolved_at = item
            .completed_at
            .as_deref()
            .and_then(parse_rfc3339_timestamp)
            .or_else(|| item.updated_at.as_deref().and_then(parse_rfc3339_timestamp))
            .unwrap_or(now);
        let local_date = crate::services::timezone::local_date_string(timezone, resolved_at);
        if local_date != current_day.session_date {
            continue;
        }
        let mut synthetic = base.clone();
        synthetic.text = item.content.clone();
        synthetic.status = CommitmentStatus::Done;
        synthetic.due_at = item
            .due
            .as_ref()
            .and_then(|due| due.datetime.as_deref().or(due.date.as_deref()))
            .and_then(parse_iso_timestamp);
        synthetic.resolved_at = Some(resolved_at);
        if let Some(deadline) = item.deadline.as_ref() {
            let deadline_at = deadline
                .datetime
                .as_deref()
                .or(deadline.date.as_deref())
                .map(str::to_string);
            if let Some(deadline_at) = deadline_at {
                synthetic.metadata_json["deadline_at"] = JsonValue::String(deadline_at);
            }
        }
        synthetic.metadata_json["labels"] = JsonValue::Array(
            item.labels
                .iter()
                .cloned()
                .map(JsonValue::String)
                .collect::<Vec<_>>(),
        );
        if seen.insert(synthetic.id.as_ref().to_string()) {
            merged.push(synthetic);
        }
    }

    for writeback in pending_writebacks {
        if writeback.kind != WritebackOperationKind::TodoistCompleteTask {
            continue;
        }
        if !matches!(
            writeback.status,
            WritebackStatus::Queued | WritebackStatus::InProgress | WritebackStatus::Conflicted
        ) {
            continue;
        }

        let local_id = writeback
            .requested_payload
            .get("commitment_id")
            .and_then(JsonValue::as_str)
            .map(str::to_string)
            .or_else(|| {
                writeback
                    .target
                    .external_id
                    .as_ref()
                    .and_then(|external_id| by_source_id.get(&format!("todoist_{external_id}")))
                    .map(|commitment| commitment.id.as_ref().to_string())
            });
        let Some(local_id) = local_id else {
            continue;
        };
        let Some(base) = by_local_id.get(&local_id) else {
            continue;
        };

        let mut synthetic = base.clone();
        synthetic.status = CommitmentStatus::Done;
        synthetic.resolved_at = Some(writeback.requested_at);
        if resolved_within_current_day(&synthetic, current_day)
            && seen.insert(synthetic.id.as_ref().to_string())
        {
            merged.push(synthetic);
        }
    }

    merged.sort_by(|left, right| {
        right
            .resolved_at
            .cmp(&left.resolved_at)
            .then_with(|| right.created_at.cmp(&left.created_at))
            .then_with(|| left.id.as_ref().cmp(right.id.as_ref()))
    });
    merged
}

fn resolved_within_current_day(
    commitment: &Commitment,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> bool {
    commitment
        .resolved_at
        .map(|value| {
            let ts = value.unix_timestamp();
            ts >= current_day.start_ts && ts < current_day.end_ts
        })
        .unwrap_or(false)
}

fn parse_rfc3339_timestamp(value: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339).ok()
}

fn parse_iso_timestamp(value: &str) -> Option<OffsetDateTime> {
    let trimmed = value.trim();
    parse_rfc3339_timestamp(trimmed)
        .or_else(|| {
            let normalized = if trimmed.ends_with('Z') {
                trimmed.to_string()
            } else {
                format!("{trimmed}Z")
            };
            parse_rfc3339_timestamp(&normalized)
        })
        .or_else(|| {
            (trimmed.len() == 10)
                .then(|| format!("{trimmed}T00:00:00Z"))
                .and_then(|normalized| parse_rfc3339_timestamp(&normalized))
        })
}

fn task_output_to_lane_item(
    task: &NowTaskOutput,
    lane: &str,
    sort_order: Option<u32>,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> NowTaskLaneItemOutput {
    let (due_label, is_overdue) =
        due_label_for_lane_item(task.due_at, lane, timezone, now, current_day);
    let (deadline_label, deadline_passed) =
        deadline_label_for_lane_item(task.deadline, timezone, now);
    NowTaskLaneItemOutput {
        id: task.id.clone(),
        task_kind: "commitment".to_string(),
        text: task.text.clone(),
        title: task.title.clone(),
        description: task.description.clone(),
        tags: task.tags.clone(),
        state: if lane == "completed" {
            "done".to_string()
        } else {
            lane.to_string()
        },
        lane: Some(lane.to_string()),
        sort_order,
        project: task.project.clone(),
        primary_thread_id: None,
        due_at: task.due_at,
        deadline: task.deadline,
        due_label,
        is_overdue,
        deadline_label,
        deadline_passed,
    }
}

fn now_event_item_id(event: &NowEventOutput) -> String {
    format!("{}-{}", event.title, event.start_ts)
}

fn format_event_window_label(
    event: &NowEventOutput,
    timezone: &crate::services::timezone::ResolvedTimeZone,
) -> String {
    let start = format_local_time_label(timezone, event.start_ts);
    let Some(end_ts) = event.end_ts else {
        return start;
    };
    format!("{start}–{}", format_local_time_label(timezone, end_ts))
}

fn format_local_time_label(
    timezone: &crate::services::timezone::ResolvedTimeZone,
    unix_ts: i64,
) -> String {
    let local = DateTime::<Utc>::from_timestamp(unix_ts, 0)
        .expect("unix timestamp should convert to chrono datetime")
        .with_timezone(&timezone.tz());
    if local.minute() == 0 {
        local.format("%-I %p").to_string()
    } else {
        local.format("%-I:%M %p").to_string()
    }
}

fn due_label_for_lane_item(
    due_at: Option<OffsetDateTime>,
    lane: &str,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> (Option<String>, bool) {
    let Some(due_at) = due_at else {
        return (
            match lane {
                "active" => Some("Committed".to_string()),
                "completed" => Some("Done".to_string()),
                _ => None,
            },
            false,
        );
    };

    if due_at.unix_timestamp() < now.unix_timestamp() {
        return (Some("Overdue".to_string()), true);
    }

    let due_session_date =
        crate::services::timezone::current_day_date_string(timezone, due_at).ok();
    if due_session_date.as_deref() == Some(current_day.session_date.as_str()) {
        return (Some("Today".to_string()), false);
    }

    if matches!(lane, "if_time_allows" | "inbox" | "completed") {
        return (
            Some(format_local_calendar_label(timezone, due_at, "Due")),
            false,
        );
    }

    (None, false)
}

fn deadline_label_for_lane_item(
    deadline: Option<OffsetDateTime>,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
) -> (Option<String>, bool) {
    let Some(deadline) = deadline else {
        return (None, false);
    };
    (
        Some(format_local_calendar_label(timezone, deadline, "Deadline")),
        deadline.unix_timestamp() < now.unix_timestamp(),
    )
}

fn format_local_calendar_label(
    timezone: &crate::services::timezone::ResolvedTimeZone,
    value: OffsetDateTime,
    prefix: &str,
) -> String {
    let local = DateTime::<Utc>::from_timestamp(value.unix_timestamp(), value.nanosecond())
        .expect("offset datetime should convert to chrono datetime")
        .with_timezone(&timezone.tz());
    format!("{prefix} {}", local.format("%b %-d"))
}

fn build_docked_input() -> NowDockedInputOutput {
    NowDockedInputOutput {
        supported_intents: vec![
            "task".to_string(),
            "url".to_string(),
            "question".to_string(),
            "note".to_string(),
            "command".to_string(),
            "continuation".to_string(),
            "reflection".to_string(),
            "scheduling".to_string(),
        ],
        day_thread_id: None,
        raw_capture_thread_id: None,
    }
}

fn format_elapsed_minutes(seconds: i64) -> String {
    let minutes = (seconds.max(0) / 60).max(1);
    format!("{minutes}m")
}

fn build_overview(
    context: &CurrentContextV1,
    now_ts: i64,
    schedule_empty_message: &Option<String>,
    next_event: Option<&NowEventOutput>,
    task_buckets: &NowTaskBuckets,
    check_in: Option<&CheckInCard>,
    reflow: Option<&ReflowCard>,
    action_items: &[ActionItem],
    freshness: &NowFreshnessOutput,
    trust_readiness: &TrustReadinessOutput,
) -> NowOverviewOutput {
    let dominant_action =
        build_dominant_action(check_in, reflow, action_items, task_buckets, next_event);
    let visible_nudge = build_visible_nudge(context, action_items, dominant_action.as_ref());
    let suggestions = if dominant_action.is_none() {
        build_suggestions(
            schedule_empty_message,
            next_event,
            task_buckets,
            action_items,
        )
    } else {
        Vec::new()
    };

    NowOverviewOutput {
        dominant_action,
        today_timeline: build_today_timeline(now_ts, next_event, task_buckets),
        visible_nudge,
        why_state: build_why_state(context, freshness, trust_readiness),
        suggestions,
        decision_options: vec![
            "accept".to_string(),
            "choose".to_string(),
            "thread".to_string(),
            "close".to_string(),
        ],
    }
}

fn empty_overview(
    freshness: &NowFreshnessOutput,
    trust_readiness: &TrustReadinessOutput,
) -> NowOverviewOutput {
    NowOverviewOutput {
        dominant_action: None,
        today_timeline: Vec::new(),
        visible_nudge: None,
        why_state: vec![
            NowOverviewWhyStateOutput {
                label: "Freshness".to_string(),
                detail: match freshness.overall_status.as_str() {
                    "stale" => {
                        "Current context is stale; sync integrations or run evaluate.".to_string()
                    }
                    "missing" => {
                        "Current context is missing; sync integrations or run evaluate.".to_string()
                    }
                    status => format!("Current context status is {status}."),
                },
            },
            NowOverviewWhyStateOutput {
                label: "Trust".to_string(),
                detail: trust_readiness.summary.clone(),
            },
        ],
        suggestions: vec![NowOverviewSuggestionOutput {
            id: "sync_context".to_string(),
            kind: "recovery".to_string(),
            title: "Recover current context".to_string(),
            summary:
                "Sync integrations or run evaluate so Now can assemble the current-day overview."
                    .to_string(),
        }],
        decision_options: vec![
            "accept".to_string(),
            "choose".to_string(),
            "thread".to_string(),
            "close".to_string(),
        ],
    }
}

fn build_dominant_action(
    check_in: Option<&CheckInCard>,
    reflow: Option<&ReflowCard>,
    action_items: &[ActionItem],
    task_buckets: &NowTaskBuckets,
    next_event: Option<&NowEventOutput>,
) -> Option<NowOverviewActionOutput> {
    if let Some(check_in) = check_in.filter(|item| item.blocking) {
        return Some(NowOverviewActionOutput {
            kind: "check_in".to_string(),
            title: check_in.title.clone(),
            summary: check_in.summary.clone(),
            reference_id: Some(check_in.id.to_string()),
        });
    }

    if let Some(reflow) = reflow.filter(|item| {
        item.severity == ReflowSeverity::Critical || item.severity == ReflowSeverity::High
    }) {
        return Some(NowOverviewActionOutput {
            kind: "reflow".to_string(),
            title: reflow.title.clone(),
            summary: reflow.summary.clone(),
            reference_id: Some(reflow.id.to_string()),
        });
    }

    if let Some(item) = action_items.first() {
        return Some(NowOverviewActionOutput {
            kind: item.kind.to_string(),
            title: item.title.clone(),
            summary: item.summary.clone(),
            reference_id: Some(item.id.to_string()),
        });
    }

    if let Some(task) = task_buckets.next_commitment.as_ref() {
        return Some(NowOverviewActionOutput {
            kind: "commitment".to_string(),
            title: task.text.clone(),
            summary: task
                .project
                .as_ref()
                .map(|project| format!("Next open commitment in {project}."))
                .unwrap_or_else(|| "Next open commitment is ready for review.".to_string()),
            reference_id: Some(task.id.clone()),
        });
    }

    next_event.map(|event| NowOverviewActionOutput {
        kind: "calendar_event".to_string(),
        title: event.title.clone(),
        summary: "Upcoming calendar event anchors the next part of the day.".to_string(),
        reference_id: None,
    })
}

fn build_visible_nudge(
    context: &CurrentContextV1,
    action_items: &[ActionItem],
    dominant_action: Option<&NowOverviewActionOutput>,
) -> Option<NowOverviewNudgeOutput> {
    if let Some(item) = action_items.iter().find(|item| {
        item.surface == vel_core::ActionSurface::Now
            && dominant_action.and_then(|current| current.reference_id.as_ref())
                != Some(&item.id.to_string())
    }) {
        return Some(NowOverviewNudgeOutput {
            kind: item.kind.to_string(),
            title: item.title.clone(),
            summary: item.summary.clone(),
        });
    }

    if context.prep_window_active {
        return Some(NowOverviewNudgeOutput {
            kind: "prep_window".to_string(),
            title: "Prep window is active".to_string(),
            summary: "The current-day context says it is time to prepare for the next commitment."
                .to_string(),
        });
    }

    if context.commute_window_active {
        return Some(NowOverviewNudgeOutput {
            kind: "commute_window".to_string(),
            title: "Commute window is active".to_string(),
            summary: "Travel pressure is active and may change what is safe to start next."
                .to_string(),
        });
    }

    if context.meds_status == "pending" {
        return Some(NowOverviewNudgeOutput {
            kind: "meds".to_string(),
            title: "Medication is still pending".to_string(),
            summary: "The current context still marks medication follow-through as incomplete."
                .to_string(),
        });
    }

    None
}

fn build_suggestions(
    schedule_empty_message: &Option<String>,
    next_event: Option<&NowEventOutput>,
    task_buckets: &NowTaskBuckets,
    action_items: &[ActionItem],
) -> Vec<NowOverviewSuggestionOutput> {
    let mut suggestions = Vec::new();

    if let Some(task) = task_buckets.next_commitment.as_ref() {
        suggestions.push(NowOverviewSuggestionOutput {
            id: task.id.clone(),
            kind: "commitment".to_string(),
            title: task.text.clone(),
            summary: task
                .project
                .as_ref()
                .map(|project| format!("Continue the next open commitment in {project}."))
                .unwrap_or_else(|| "Continue the next open commitment.".to_string()),
        });
    }

    if let Some(event) = next_event {
        suggestions.push(NowOverviewSuggestionOutput {
            id: format!("event:{}", event.start_ts),
            kind: "calendar_event".to_string(),
            title: event.title.clone(),
            summary: "Review the next calendar anchor before committing new work.".to_string(),
        });
    }

    if let Some(item) = action_items.first() {
        suggestions.push(NowOverviewSuggestionOutput {
            id: item.id.to_string(),
            kind: item.kind.to_string(),
            title: item.title.clone(),
            summary: item.summary.clone(),
        });
    }

    if suggestions.is_empty() {
        suggestions.push(NowOverviewSuggestionOutput {
            id: "overview_wait".to_string(),
            kind: "review".to_string(),
            title: "Review current-day state".to_string(),
            summary: schedule_empty_message.clone().unwrap_or_else(|| {
                "No dominant action is available yet; inspect current-day state before choosing."
                    .to_string()
            }),
        });
    }

    suggestions.truncate(3);
    suggestions
}

fn build_today_timeline(
    now_ts: i64,
    next_event: Option<&NowEventOutput>,
    task_buckets: &NowTaskBuckets,
) -> Vec<NowOverviewTimelineEntryOutput> {
    let mut timeline = vec![NowOverviewTimelineEntryOutput {
        kind: "now".to_string(),
        title: "Current time".to_string(),
        timestamp: now_ts,
        detail: None,
    }];

    if let Some(event) = next_event {
        timeline.push(NowOverviewTimelineEntryOutput {
            kind: "calendar_event".to_string(),
            title: event.title.clone(),
            timestamp: event.start_ts,
            detail: event.location.clone(),
        });
    }

    if let Some(task) = task_buckets.next_commitment.as_ref() {
        if let Some(due_at) = task.due_at {
            timeline.push(NowOverviewTimelineEntryOutput {
                kind: "commitment_due".to_string(),
                title: "Next commitment due".to_string(),
                timestamp: due_at.unix_timestamp(),
                detail: Some(task.text.clone()),
            });
        }
    }

    timeline.truncate(3);
    timeline
}

fn build_why_state(
    context: &CurrentContextV1,
    freshness: &NowFreshnessOutput,
    trust_readiness: &TrustReadinessOutput,
) -> Vec<NowOverviewWhyStateOutput> {
    let mut why_state = vec![
        NowOverviewWhyStateOutput {
            label: "Mode".to_string(),
            detail: label_for_mode(context.mode.as_str()).label,
        },
        NowOverviewWhyStateOutput {
            label: "Phase".to_string(),
            detail: label_for_phase(context.morning_state.as_str()).label,
        },
        NowOverviewWhyStateOutput {
            label: "Attention".to_string(),
            detail: label_for_attention(context.attention_state.as_str()).label,
        },
        NowOverviewWhyStateOutput {
            label: "Freshness".to_string(),
            detail: freshness.overall_status.clone(),
        },
        NowOverviewWhyStateOutput {
            label: "Trust".to_string(),
            detail: trust_readiness.headline.clone(),
        },
    ];

    if context.prep_window_active {
        why_state.push(NowOverviewWhyStateOutput {
            label: "Prep".to_string(),
            detail: "Prep window is active.".to_string(),
        });
    }

    if context.commute_window_active {
        why_state.push(NowOverviewWhyStateOutput {
            label: "Commute".to_string(),
            detail: "Commute window is active.".to_string(),
        });
    }

    why_state.truncate(6);
    why_state
}

fn schedule_empty_message(
    integrations: &IntegrationSnapshotOutput,
    no_upcoming_events: bool,
) -> Option<String> {
    if !no_upcoming_events {
        return None;
    }

    let calendar = &integrations.google_calendar;
    if !calendar.connected {
        return Some("Google Calendar is disconnected. Reconnect it in Settings.".to_string());
    }
    if !calendar.all_calendars_selected && !calendar.any_selected {
        return Some("No calendars are selected in Settings.".to_string());
    }

    Some("No upcoming calendar events in the current stream.".to_string())
}

fn build_freshness(
    now_ts: i64,
    computed_at: i64,
    integrations: &IntegrationSnapshotOutput,
    calendar_selection: &integrations::GoogleCalendarSelectionFilter,
) -> NowFreshnessOutput {
    let mut sources = vec![NowFreshnessEntryOutput {
        key: "context".to_string(),
        label: "Context".to_string(),
        status: age_status(now_ts - computed_at).to_string(),
        last_sync_at: Some(computed_at),
        age_seconds: Some(now_ts - computed_at),
        guidance: None,
    }];
    sources.push(sync_freshness(
        now_ts,
        "calendar",
        "Calendar",
        integrations.google_calendar.last_sync_at,
        integrations.google_calendar.last_sync_status.as_deref(),
        integrations
            .google_calendar
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
        (!calendar_selection.has_any_selected()).then_some(
            "No calendars are selected in Settings. Unchecked calendars are excluded from Vel context by default."
                .to_string(),
        ),
    ));
    sources.push(sync_freshness(
        now_ts,
        "todoist",
        "Todoist",
        integrations.todoist.last_sync_at,
        integrations.todoist.last_sync_status.as_deref(),
        integrations
            .todoist
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
        None,
    ));
    sources.push(sync_freshness(
        now_ts,
        "activity",
        "Activity",
        integrations.activity.last_sync_at,
        integrations.activity.last_sync_status.as_deref(),
        integrations
            .activity
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
        None,
    ));
    sources.push(sync_freshness(
        now_ts,
        "messaging",
        "Messaging",
        integrations.messaging.last_sync_at,
        integrations.messaging.last_sync_status.as_deref(),
        integrations
            .messaging
            .guidance
            .as_ref()
            .map(|guidance| format!("{}: {}", guidance.title, guidance.detail)),
        None,
    ));
    let overall_status = if sources
        .iter()
        .any(|entry| matches!(entry.status.as_str(), "error" | "stale" | "missing"))
    {
        "stale"
    } else if sources.iter().any(|entry| entry.status == "aging") {
        "aging"
    } else {
        "fresh"
    };
    NowFreshnessOutput {
        overall_status: overall_status.to_string(),
        sources,
    }
}

fn sync_freshness(
    now_ts: i64,
    key: &str,
    label_text: &str,
    last_sync_at: Option<i64>,
    last_sync_status: Option<&str>,
    guidance: Option<String>,
    override_guidance: Option<String>,
) -> NowFreshnessEntryOutput {
    let age_seconds = last_sync_at.map(|timestamp| now_ts - timestamp);
    let status = if override_guidance.is_some() {
        "unchecked"
    } else {
        match last_sync_status {
            Some("error") => "error",
            Some("disconnected") => "disconnected",
            _ => age_seconds.map(age_status).unwrap_or("missing"),
        }
    };
    NowFreshnessEntryOutput {
        key: key.to_string(),
        label: label_text.to_string(),
        status: status.to_string(),
        last_sync_at,
        age_seconds,
        guidance: override_guidance.or(guidance),
    }
}

fn age_status(age_seconds: i64) -> &'static str {
    if age_seconds <= 120 {
        "fresh"
    } else if age_seconds <= 600 {
        "aging"
    } else {
        "stale"
    }
}

fn sort_commitments(
    mut commitments: Vec<Commitment>,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
) -> Vec<Commitment> {
    commitments.sort_by(|left, right| compare_commitments_in_timezone(left, right, timezone, now));
    commitments
}

fn split_now_tasks(
    context: &CurrentContextV1,
    commitments: Vec<Commitment>,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> NowTaskBuckets {
    let mut in_play_commitments = Vec::new();
    let mut pullable_tasks = Vec::new();

    for commitment in commitments {
        if commitment_is_in_play(context, &commitment, timezone, now, current_day) {
            in_play_commitments.push(commitment);
        } else {
            pullable_tasks.push(commitment);
        }
    }

    let next_commitment = in_play_commitments.first().map(now_task);
    let in_play = in_play_commitments
        .into_iter()
        .skip(1)
        .map(|commitment| now_task(&commitment))
        .collect::<Vec<_>>();
    let pullable = pullable_tasks
        .into_iter()
        .map(|commitment| now_task(&commitment))
        .collect::<Vec<_>>();

    NowTaskBuckets {
        next_commitment,
        in_play,
        pullable,
    }
}

fn compare_commitments(
    left: &Commitment,
    right: &Commitment,
    now: OffsetDateTime,
) -> std::cmp::Ordering {
    compare_commitments_in_timezone(
        left,
        right,
        &crate::services::timezone::ResolvedTimeZone::utc(),
        now,
    )
}

fn compare_commitments_in_timezone(
    left: &Commitment,
    right: &Commitment,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
) -> std::cmp::Ordering {
    let current_day = crate::services::timezone::current_day_window(timezone, now)
        .expect("current day window should resolve for known timezone");
    let left_priority = commitment_priority_key(left, timezone, now, &current_day);
    let right_priority = commitment_priority_key(right, timezone, now, &current_day);
    left_priority
        .cmp(&right_priority)
        .then_with(|| {
            left.due_at
                .map(|value| value.unix_timestamp())
                .unwrap_or(i64::MAX)
                .cmp(
                    &right
                        .due_at
                        .map(|value| value.unix_timestamp())
                        .unwrap_or(i64::MAX),
                )
        })
        .then_with(|| todoist_priority(right).cmp(&todoist_priority(left)))
        .then_with(|| todoist_updated_at(right).cmp(&todoist_updated_at(left)))
        .then_with(|| right.created_at.cmp(&left.created_at))
}

fn commitment_is_in_play(
    context: &CurrentContextV1,
    commitment: &Commitment,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> bool {
    let matches_due_context =
        commitment_matches_next_up_due_context(commitment, timezone, now, current_day);
    if context
        .next_commitment_id
        .as_deref()
        .is_some_and(|id| id == commitment.id.as_ref())
    {
        return matches_due_context;
    }
    if context
        .commitments_used
        .iter()
        .any(|id| id == commitment.id.as_ref())
    {
        return matches_due_context;
    }

    matches_due_context
}

fn commitment_matches_next_up_due_context(
    commitment: &Commitment,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> bool {
    let due_ts = commitment.due_at.map(|value| value.unix_timestamp());
    let due_session_date = commitment
        .due_at
        .and_then(|value| crate::services::timezone::current_day_date_string(timezone, value).ok());
    due_ts.is_some_and(|value| value < now.unix_timestamp())
        || due_session_date.as_deref() == Some(current_day.session_date.as_str())
}

fn commitment_priority_key(
    commitment: &Commitment,
    timezone: &crate::services::timezone::ResolvedTimeZone,
    now: OffsetDateTime,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> (u8, i64) {
    let due_ts = commitment.due_at.map(|value| value.unix_timestamp());
    let due_at = commitment.due_at;
    let due_session_date = due_at
        .and_then(|value| crate::services::timezone::current_day_date_string(timezone, value).ok());
    let has_due_time = commitment
        .metadata_json
        .get("has_due_time")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let priority_bucket = if commitment.commitment_kind.as_deref() == Some("medication")
        && due_session_date.as_deref() == Some(current_day.session_date.as_str())
    {
        0
    } else if due_ts.is_some_and(|value| value < now.unix_timestamp()) {
        1
    } else if due_ts.is_some_and(|value| {
        has_due_time && value >= current_day.start_ts && value < current_day.end_ts
    }) {
        2
    } else if due_session_date.as_deref() == Some(current_day.session_date.as_str()) {
        3
    } else if todoist_priority(commitment) >= 3 || was_recently_updated(commitment, now) {
        4
    } else {
        5
    };
    (priority_bucket, due_ts.unwrap_or(i64::MAX))
}

fn todoist_priority(commitment: &Commitment) -> i64 {
    commitment
        .metadata_json
        .get("priority")
        .and_then(|value| value.as_i64())
        .unwrap_or(1)
}

fn todoist_updated_at(commitment: &Commitment) -> i64 {
    commitment
        .metadata_json
        .get("updated_at")
        .and_then(|value| value.as_i64())
        .unwrap_or(i64::MIN)
}

fn was_recently_updated(commitment: &Commitment, now: OffsetDateTime) -> bool {
    todoist_updated_at(commitment) >= now.unix_timestamp() - (24 * 60 * 60)
}

fn now_task(commitment: &Commitment) -> NowTaskOutput {
    let description = commitment
        .metadata_json
        .get("description")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let tags = commitment
        .metadata_json
        .get("labels")
        .and_then(|value| value.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let deadline = commitment
        .metadata_json
        .get("deadline_at")
        .and_then(|value| value.as_str())
        .and_then(|value| {
            OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339).ok()
        })
        .or_else(|| {
            commitment
                .metadata_json
                .get("deadline")
                .and_then(|value| value.get("date"))
                .and_then(|value| value.as_str())
                .and_then(|value| {
                    OffsetDateTime::parse(
                        &format!("{value}T00:00:00Z"),
                        &time::format_description::well_known::Rfc3339,
                    )
                    .ok()
                })
        });
    NowTaskOutput {
        id: commitment.id.as_ref().to_string(),
        text: commitment.text.clone(),
        title: commitment.text.clone(),
        description,
        tags,
        source_type: commitment.source_type.clone(),
        due_at: commitment.due_at,
        deadline,
        project: normalized_now_task_project(commitment),
        commitment_kind: commitment.commitment_kind.clone(),
    }
}

fn normalized_now_task_project(commitment: &Commitment) -> Option<String> {
    let is_todoist = commitment.source_type.eq_ignore_ascii_case("todoist");
    let is_inbox_project = commitment
        .metadata_json
        .get("is_inbox_project")
        .and_then(JsonValue::as_bool)
        .unwrap_or(false);
    let literal_inbox = commitment
        .project
        .as_deref()
        .map(|value| value.trim().eq_ignore_ascii_case("inbox"))
        .unwrap_or(false);
    if is_todoist && (is_inbox_project || literal_inbox) {
        None
    } else {
        commitment.project.clone()
    }
}

fn event_overlaps_current_day(
    event: &NowEventOutput,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> bool {
    event.start_ts < current_day.end_ts
        && event.end_ts.unwrap_or(event.start_ts) >= current_day.start_ts
        && event_is_relevant(event)
}

fn sort_now_events(events: &mut [NowEventOutput]) {
    events.sort_by(|left, right| {
        left.all_day
            .cmp(&right.all_day)
            .reverse()
            .then_with(|| left.start_ts.cmp(&right.start_ts))
            .then_with(|| left.title.cmp(&right.title))
    });
}

fn event_is_relevant(event: &NowEventOutput) -> bool {
    let transparency = event.transparency.as_deref().unwrap_or_default();
    let response_status = event.response_status.as_deref().unwrap_or_default();
    let status = event.status.as_deref().unwrap_or_default();
    !transparency.eq_ignore_ascii_case("transparent")
        && !transparency.eq_ignore_ascii_case("free")
        && !response_status.eq_ignore_ascii_case("declined")
        && !status.eq_ignore_ascii_case("cancelled")
}

fn calendar_event_from_signal(
    signal: SignalRecord,
    override_settings: &CalendarOverrideSettings,
) -> Option<NowEventOutput> {
    if signal.signal_type != "calendar_event" {
        return None;
    }
    let payload = signal.payload_json;
    let original_start_ts = payload
        .get("start")?
        .as_i64()
        .or_else(|| payload.get("start_ts")?.as_i64())?;
    let event_id = payload
        .get("event_id")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let calendar_id = payload
        .get("calendar_id")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    let event_override = event_id.as_deref().and_then(|event_id| {
        find_calendar_override(override_settings, event_id, calendar_id.as_deref())
    });
    let start_ts = event_override
        .map(|record| record.start_ts)
        .unwrap_or(original_start_ts);
    let title = payload
        .get("title")
        .and_then(|value| value.as_str())
        .unwrap_or("Untitled event")
        .to_string();
    let payload_end_ts = payload.get("end").and_then(|value| value.as_i64());
    let end_ts = event_override.and_then(|record| record.end_ts).or_else(|| {
        payload_end_ts.map(|original_end_ts| {
            let duration = original_end_ts - original_start_ts;
            start_ts + duration.max(0)
        })
    });
    let notes = payload
        .get("description")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let attendees = payload
        .get("attendees")
        .and_then(|value| value.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let travel_minutes = payload
        .get("travel_minutes")
        .and_then(|value| value.as_i64());
    Some(NowEventOutput {
        event_id,
        calendar_id,
        calendar_name: payload
            .get("calendar_name")
            .and_then(|value| value.as_str())
            .map(str::to_string),
        title,
        start_ts,
        end_ts,
        all_day: payload
            .get("all_day")
            .and_then(|value| value.as_bool())
            .unwrap_or(false),
        event_url: payload
            .get("url")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
        attachment_url: payload
            .get("attachment_url")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
        location: payload
            .get("location")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
        notes,
        attendees,
        video_url: payload
            .get("video_url")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
        video_provider: payload
            .get("video_provider")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
        status: payload
            .get("status")
            .and_then(|value| value.as_str())
            .map(str::to_string),
        transparency: payload
            .get("transparency")
            .and_then(|value| value.as_str())
            .map(str::to_string),
        response_status: payload
            .get("response_status")
            .and_then(|value| value.as_str())
            .map(str::to_string),
        prep_minutes: payload.get("prep_minutes").and_then(|value| value.as_i64()),
        travel_minutes,
        leave_by_ts: travel_minutes.map(|minutes| start_ts - (minutes * 60)),
        rescheduled: event_override.is_some(),
    })
}

fn find_calendar_override<'a>(
    settings: &'a CalendarOverrideSettings,
    event_id: &str,
    calendar_id: Option<&str>,
) -> Option<&'a CalendarOverrideRecord> {
    settings
        .overrides
        .iter()
        .find(|record| record.event_id == event_id && record.calendar_id.as_deref() == calendar_id)
}

async fn load_calendar_override_settings(
    storage: &Storage,
) -> Result<CalendarOverrideSettings, AppError> {
    let all = storage.get_all_settings().await?;
    match all.get(NOW_CALENDAR_OVERRIDES_KEY) {
        Some(value) => serde_json::from_value(value.clone()).map_err(|error| {
            AppError::internal(format!("deserialize calendar override settings: {error}"))
        }),
        None => Ok(CalendarOverrideSettings::default()),
    }
}

async fn save_calendar_override_settings(
    storage: &Storage,
    settings: &CalendarOverrideSettings,
) -> Result<(), AppError> {
    let value = serde_json::to_value(settings).map_err(|error| {
        AppError::internal(format!("serialize calendar override settings: {error}"))
    })?;
    storage
        .set_setting(NOW_CALENDAR_OVERRIDES_KEY, &value)
        .await?;
    Ok(())
}

fn label(key: &str, text: &str) -> NowLabelOutput {
    NowLabelOutput {
        key: key.to_string(),
        label: text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Duration, Month};
    use vel_core::{
        ActionEvidenceRef, ActionPermissionMode, ActionScopeAffinity, ActionState, ActionSurface,
        CommitmentId, CommitmentStatus, ContextMigrator, IntegrationFamily, WritebackOperationId,
        WritebackOperationKind, WritebackRisk, WritebackStatus, WritebackTargetRef,
    };

    #[test]
    fn pending_medication_today_outranks_ordinary_task() {
        let now = fixed_now();
        let med = commitment_fixture(
            "Take meds",
            Some(now.replace_hour(9).unwrap()),
            Some("medication"),
            json!({ "has_due_time": true }),
        );
        let ordinary = commitment_fixture("Reply to email", None, Some("todo"), json!({}));

        assert_eq!(
            compare_commitments(&med, &ordinary, now),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn overdue_task_outranks_no_due_task() {
        let now = fixed_now();
        let overdue = commitment_fixture(
            "Ship report",
            Some(now - Duration::hours(2)),
            Some("todo"),
            json!({ "has_due_time": true }),
        );
        let no_due = commitment_fixture("Clean inbox", None, Some("todo"), json!({}));

        assert_eq!(
            compare_commitments(&overdue, &no_due, now),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn due_today_with_time_outranks_no_due_task() {
        let now = fixed_now();
        let due_soon = commitment_fixture(
            "Prepare notes",
            Some(now + Duration::hours(3)),
            Some("todo"),
            json!({ "has_due_time": true }),
        );
        let no_due = commitment_fixture("Someday idea", None, Some("todo"), json!({}));

        assert_eq!(
            compare_commitments(&due_soon, &no_due, now),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn calendar_event_from_signal_applies_persisted_override() {
        let signal = vel_storage::SignalRecord {
            signal_id: "sig_calendar_1".to_string(),
            signal_type: "calendar_event".to_string(),
            source: "google_calendar".to_string(),
            source_ref: Some("google_calendar:cal_1:evt_1".to_string()),
            timestamp: 1_710_003_600,
            payload_json: json!({
                "event_id": "evt_1",
                "calendar_id": "cal_1",
                "calendar_name": "Primary",
                "title": "Design review",
                "start": 1_710_003_600,
                "end": 1_710_007_200,
                "location": "Studio",
                "prep_minutes": 15,
                "travel_minutes": 0
            }),
            created_at: 1_710_000_000,
        };
        let overrides = CalendarOverrideSettings {
            overrides: vec![CalendarOverrideRecord {
                event_id: "evt_1".to_string(),
                calendar_id: Some("cal_1".to_string()),
                start_ts: 1_710_005_400,
                end_ts: Some(1_710_009_000),
                updated_at: 1_710_000_100,
            }],
        };

        let event = calendar_event_from_signal(signal, &overrides).expect("calendar event");

        assert_eq!(event.event_id.as_deref(), Some("evt_1"));
        assert_eq!(event.calendar_name.as_deref(), Some("Primary"));
        assert_eq!(event.start_ts, 1_710_005_400);
        assert_eq!(event.end_ts, Some(1_710_009_000));
        assert!(event.rescheduled);
    }

    #[tokio::test]
    async fn get_now_fallback_keeps_today_events_when_future_events_exceed_limit() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting("timezone", &serde_json::json!("America/Denver"))
            .await
            .unwrap();
        storage
            .set_setting(
                "integration_google_calendar",
                &json!({
                    "all_calendars_selected": false,
                    "calendars": [
                        {
                            "id": "routine",
                            "summary": "Routine",
                            "primary": false,
                            "sync_enabled": true,
                            "display_enabled": true
                        }
                    ]
                }),
            )
            .await
            .unwrap();

        let timezone =
            crate::services::timezone::ResolvedTimeZone::parse("America/Denver").unwrap();
        let now = time::macros::datetime!(2026-03-24 18:30:00 UTC);
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        storage
            .set_current_context(
                now.unix_timestamp(),
                &serde_json::to_string(&CurrentContextV1 {
                    computed_at: now.unix_timestamp(),
                    ..CurrentContextV1::default()
                })
                .unwrap(),
            )
            .await
            .unwrap();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: Some("today-event".to_string()),
                timestamp: current_day.start_ts + (18 * 60 * 60),
                payload_json: Some(json!({
                    "event_id": "evt_today",
                    "calendar_id": "routine",
                    "calendar_name": "Routine",
                    "title": "block:workout",
                    "start": current_day.start_ts + (18 * 60 * 60),
                    "end": current_day.start_ts + (21 * 60 * 60),
                    "status": "confirmed"
                })),
            })
            .await
            .unwrap();

        for index in 0..200 {
            let start_ts = current_day.end_ts + ((index as i64 + 1) * 3600);
            storage
                .insert_signal(vel_storage::SignalInsert {
                    signal_type: "calendar_event".to_string(),
                    source: "google_calendar".to_string(),
                    source_ref: Some(format!("future-now-event-{index}")),
                    timestamp: start_ts,
                    payload_json: Some(json!({
                        "event_id": format!("evt_future_{index}"),
                        "calendar_id": "routine",
                        "calendar_name": "Routine",
                        "title": format!("Future {index}"),
                        "start": start_ts,
                        "end": start_ts + 1800,
                        "status": "confirmed"
                    })),
                })
                .await
                .unwrap();
        }

        let now_output = get_now(&storage, &vel_config::AppConfig::default())
            .await
            .unwrap();

        assert_eq!(now_output.schedule.upcoming_events.len(), 1);
        assert_eq!(
            now_output.schedule.upcoming_events[0].title,
            "block:workout"
        );
    }

    #[tokio::test]
    async fn get_now_filters_out_hidden_display_google_calendar_events() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting("timezone", &serde_json::json!("America/Denver"))
            .await
            .unwrap();
        storage
            .set_setting(
                "integration_google_calendar",
                &json!({
                    "all_calendars_selected": true,
                    "calendars": [
                        {
                            "id": "cal_visible",
                            "summary": "Visible",
                            "primary": false,
                            "sync_enabled": true,
                            "display_enabled": true
                        },
                        {
                            "id": "cal_hidden",
                            "summary": "Hidden",
                            "primary": false,
                            "sync_enabled": true,
                            "display_enabled": false
                        }
                    ]
                }),
            )
            .await
            .unwrap();

        let timezone =
            crate::services::timezone::ResolvedTimeZone::parse("America/Denver").unwrap();
        let now = time::macros::datetime!(2026-03-24 14:30:00 UTC);
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        storage
            .set_current_context(
                now.unix_timestamp(),
                &serde_json::to_string(&CurrentContextV1 {
                    computed_at: now.unix_timestamp(),
                    ..CurrentContextV1::default()
                })
                .unwrap(),
            )
            .await
            .unwrap();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: Some("visible-event".to_string()),
                timestamp: current_day.start_ts + (10 * 60 * 60),
                payload_json: Some(json!({
                    "event_id": "evt_visible",
                    "calendar_id": "cal_visible",
                    "calendar_name": "Visible",
                    "title": "Visible event",
                    "start": current_day.start_ts + (10 * 60 * 60),
                    "end": current_day.start_ts + (10 * 60 * 60 + 15 * 60),
                    "status": "confirmed"
                })),
            })
            .await
            .unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: Some("hidden-event".to_string()),
                timestamp: current_day.start_ts + (11 * 60 * 60),
                payload_json: Some(json!({
                    "event_id": "evt_hidden",
                    "calendar_id": "cal_hidden",
                    "calendar_name": "Hidden",
                    "title": "Hidden event",
                    "start": current_day.start_ts + (11 * 60 * 60),
                    "end": current_day.start_ts + (11 * 60 * 60 + 15 * 60),
                    "status": "confirmed"
                })),
            })
            .await
            .unwrap();

        let now_output = get_now(&storage, &vel_config::AppConfig::default())
            .await
            .unwrap();

        assert_eq!(now_output.schedule.upcoming_events.len(), 1);
        assert_eq!(now_output.schedule.upcoming_events[0].title, "Visible event");
        assert!(!now_output.next_up_items.iter().any(|item| item.title == "Hidden event"));
    }

    #[tokio::test]
    async fn get_now_filters_out_hidden_display_google_calendar_events_from_following_day_events() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting("timezone", &serde_json::json!("America/Denver"))
            .await
            .unwrap();
        storage
            .set_setting(
                "integration_google_calendar",
                &json!({
                    "all_calendars_selected": true,
                    "calendars": [
                        {
                            "id": "cal_visible",
                            "summary": "Visible",
                            "primary": false,
                            "sync_enabled": true,
                            "display_enabled": true
                        },
                        {
                            "id": "cal_hidden",
                            "summary": "Hidden",
                            "primary": false,
                            "sync_enabled": true,
                            "display_enabled": false
                        }
                    ]
                }),
            )
            .await
            .unwrap();

        let timezone =
            crate::services::timezone::ResolvedTimeZone::parse("America/Denver").unwrap();
        let now = time::macros::datetime!(2026-03-24 14:30:00 UTC);
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        let next_day = crate::services::timezone::current_day_window(
            &timezone,
            OffsetDateTime::from_unix_timestamp(current_day.end_ts + 1).unwrap(),
        )
        .unwrap();
        storage
            .set_current_context(
                now.unix_timestamp(),
                &serde_json::to_string(&CurrentContextV1 {
                    computed_at: now.unix_timestamp(),
                    ..CurrentContextV1::default()
                })
                .unwrap(),
            )
            .await
            .unwrap();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: Some("visible-following-event".to_string()),
                timestamp: next_day.start_ts + (10 * 60 * 60),
                payload_json: Some(json!({
                    "event_id": "evt_visible_next",
                    "calendar_id": "cal_visible",
                    "calendar_name": "Visible",
                    "title": "Visible tomorrow event",
                    "start": next_day.start_ts + (10 * 60 * 60),
                    "end": next_day.start_ts + (10 * 60 * 60 + 15 * 60),
                    "status": "confirmed"
                })),
            })
            .await
            .unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: Some("hidden-following-event".to_string()),
                timestamp: next_day.start_ts + (11 * 60 * 60),
                payload_json: Some(json!({
                    "event_id": "evt_hidden_next",
                    "calendar_id": "cal_hidden",
                    "calendar_name": "Hidden",
                    "title": "Hidden tomorrow event",
                    "start": next_day.start_ts + (11 * 60 * 60),
                    "end": next_day.start_ts + (11 * 60 * 60 + 15 * 60),
                    "status": "confirmed"
                })),
            })
            .await
            .unwrap();

        let now_output = get_now(&storage, &vel_config::AppConfig::default())
            .await
            .unwrap();

        assert_eq!(now_output.schedule.following_day_events.len(), 1);
        assert_eq!(
            now_output.schedule.following_day_events[0].title,
            "Visible tomorrow event"
        );
        assert!(
            !now_output
                .schedule
                .following_day_events
                .iter()
                .any(|event| event.title == "Hidden tomorrow event")
        );
    }

    #[test]
    fn high_priority_beats_plain_backlog_task() {
        let now = fixed_now();
        let high_priority = commitment_fixture(
            "Urgent follow-up",
            None,
            Some("todo"),
            json!({ "priority": 4 }),
        );
        let backlog = commitment_fixture(
            "Backlog cleanup",
            None,
            Some("todo"),
            json!({ "priority": 1 }),
        );

        assert_eq!(
            compare_commitments(&high_priority, &backlog, now),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn late_night_current_day_bucket_keeps_commitments_in_play() {
        let timezone =
            crate::services::timezone::ResolvedTimeZone::parse("America/Denver").unwrap();
        let now = time::Date::from_calendar_date(2026, Month::March, 17)
            .unwrap()
            .with_hms(7, 30, 0)
            .unwrap()
            .assume_utc();
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        let context = CurrentContextV1::default();
        let same_session_commitment = commitment_fixture(
            "Finish standup notes",
            Some(
                time::Date::from_calendar_date(2026, Month::March, 17)
                    .unwrap()
                    .with_hms(8, 30, 0)
                    .unwrap()
                    .assume_utc(),
            ),
            Some("todo"),
            json!({ "has_due_time": true }),
        );
        let backlog_task = commitment_fixture("Backlog cleanup", None, Some("todo"), json!({}));

        let buckets = split_now_tasks(
            &context,
            vec![backlog_task, same_session_commitment],
            &timezone,
            now,
            &current_day,
        );

        assert_eq!(current_day.session_date, "2026-03-16");
        assert_eq!(
            buckets
                .next_commitment
                .as_ref()
                .map(|task| task.text.as_str()),
            Some("Finish standup notes")
        );
        assert_eq!(buckets.in_play.len(), 0);
        assert_eq!(buckets.pullable.len(), 1);
        assert_eq!(buckets.pullable[0].text, "Backlog cleanup");
    }

    #[test]
    fn split_now_tasks_preserves_full_in_play_and_pullable_sets() {
        let timezone = crate::services::timezone::ResolvedTimeZone::utc();
        let now = fixed_now();
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        let context = CurrentContextV1::default();
        let mut commitments = Vec::new();

        for index in 0..8 {
            commitments.push(commitment_fixture(
                &format!("Overdue {}", index + 1),
                Some(now - Duration::hours((index + 1) as i64)),
                Some("todo"),
                json!({ "has_due_time": true }),
            ));
        }

        for index in 0..7 {
            commitments.push(commitment_fixture(
                &format!("Backlog {}", index + 1),
                None,
                Some("todo"),
                json!({}),
            ));
        }

        let buckets = split_now_tasks(&context, commitments, &timezone, now, &current_day);

        assert_eq!(
            buckets
                .next_commitment
                .as_ref()
                .map(|task| task.text.as_str()),
            Some("Overdue 1")
        );
        assert_eq!(buckets.in_play.len(), 7);
        assert_eq!(buckets.pullable.len(), 7);
        assert_eq!(buckets.in_play[0].text, "Overdue 2");
        assert_eq!(buckets.pullable[0].text, "Backlog 1");
    }

    #[test]
    fn default_task_lane_keeps_uncommitted_items_in_next_up() {
        let timezone = crate::services::timezone::ResolvedTimeZone::utc();
        let now = fixed_now();
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        let context = CurrentContextV1::default();
        let commitments = sort_commitments(
            vec![
                commitment_fixture("Backlog cleanup", None, Some("todo"), json!({})),
                commitment_fixture(
                    "Today task",
                    Some(now + Duration::hours(2)),
                    Some("todo"),
                    json!({ "has_due_time": true }),
                ),
                commitment_fixture(
                    "Overdue task",
                    Some(now - Duration::hours(1)),
                    Some("todo"),
                    json!({ "has_due_time": true }),
                ),
            ],
            &timezone,
            now,
        );
        let all_open_tasks = commitments.iter().map(now_task).collect::<Vec<_>>();
        let buckets = split_now_tasks(&context, commitments, &timezone, now, &current_day);

        let lane = build_task_lane(
            &context,
            &buckets,
            &all_open_tasks,
            &timezone,
            now,
            &current_day,
            Vec::new(),
        );

        assert!(lane.active.is_none());
        assert!(lane.active_items.is_empty());
        assert_eq!(
            lane.next_up
                .iter()
                .map(|task| task.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Overdue task", "Today task"]
        );
        assert_eq!(
            lane.if_time_allows
                .iter()
                .map(|task| task.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Backlog cleanup"]
        );
    }

    #[test]
    fn default_task_lane_reserves_todoist_inbox_items_into_inbox() {
        let timezone = crate::services::timezone::ResolvedTimeZone::utc();
        let now = fixed_now();
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        let context = CurrentContextV1::default();

        let due_inbox = commitment_fixture(
            "Inbox due today",
            Some(now + Duration::hours(2)),
            Some("todo"),
            json!({ "has_due_time": true, "is_inbox_project": true }),
        );
        let unscheduled_inbox = commitment_fixture(
            "Inbox unscheduled",
            None,
            Some("todo"),
            json!({ "is_inbox_project": true }),
        );
        let mut projected_task = commitment_fixture(
            "Projected task",
            Some(now + Duration::hours(3)),
            Some("todo"),
            json!({ "has_due_time": true }),
        );
        projected_task.project = Some("ops".to_string());

        let commitments = sort_commitments(
            vec![due_inbox, unscheduled_inbox, projected_task],
            &timezone,
            now,
        );
        let all_open_tasks = commitments.iter().map(now_task).collect::<Vec<_>>();
        let buckets = split_now_tasks(&context, commitments, &timezone, now, &current_day);

        let lane = build_task_lane(
            &context,
            &buckets,
            &all_open_tasks,
            &timezone,
            now,
            &current_day,
            Vec::new(),
        );

        assert_eq!(
            lane.next_up
                .iter()
                .map(|task| task.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Projected task"]
        );
        assert_eq!(
            lane.inbox
                .iter()
                .map(|task| task.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Inbox due today", "Inbox unscheduled"]
        );
        assert!(lane.if_time_allows.is_empty());
    }

    #[test]
    fn commitments_used_does_not_pull_undated_or_future_tasks_into_next_up() {
        let timezone =
            crate::services::timezone::ResolvedTimeZone::parse("America/Denver").unwrap();
        let now = fixed_now();
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        let due_today_at = now + Duration::hours(1);
        let due_tomorrow_at = now + Duration::hours(3);
        let due_today = commitment_fixture(
            "Due today",
            Some(due_today_at),
            Some("todo"),
            json!({ "has_due_time": true }),
        );
        let tomorrow = commitment_fixture(
            "Due tomorrow",
            Some(due_tomorrow_at),
            Some("todo"),
            json!({ "has_due_time": true }),
        );
        let undated = commitment_fixture("Undated task", None, Some("todo"), json!({}));
        let commitments = sort_commitments(vec![undated, tomorrow, due_today], &timezone, now);
        let mut context = CurrentContextV1::default();
        context.commitments_used = commitments
            .iter()
            .map(|commitment| commitment.id.as_ref().to_string())
            .collect();

        let buckets = split_now_tasks(&context, commitments, &timezone, now, &current_day);

        assert_eq!(
            buckets
                .next_commitment
                .as_ref()
                .map(|task| task.text.as_str()),
            Some("Due today")
        );
        assert!(buckets.in_play.is_empty());
        assert_eq!(buckets.pullable.len(), 2);
        assert_eq!(buckets.pullable[0].text, "Due tomorrow");
        assert_eq!(buckets.pullable[1].text, "Undated task");
    }

    #[test]
    fn completed_today_merges_provider_results_and_pending_local_completions() {
        let timezone = crate::services::timezone::ResolvedTimeZone::utc();
        let now = fixed_now();
        let current_day = crate::services::timezone::current_day_window(&timezone, now).unwrap();
        let open_commitment = commitment_fixture(
            "Reply to Dimitri",
            Some(now - Duration::hours(2)),
            Some("todo"),
            json!({ "labels": ["Urgent"] }),
        );
        let mut open_commitment = open_commitment;
        open_commitment.source_type = "todoist".to_string();
        open_commitment.source_id = Some("todoist_task_provider".to_string());

        let local_done = commitment_fixture("Finish weekly review", None, Some("todo"), json!({}));
        let mut local_done = local_done;
        local_done.status = CommitmentStatus::Done;
        local_done.resolved_at = Some(now - Duration::minutes(15));

        let optimistic_open = commitment_fixture("Call Apria", None, Some("todo"), json!({}));
        let mut optimistic_open = optimistic_open;
        optimistic_open.source_type = "todoist".to_string();
        optimistic_open.source_id = Some("todoist_task_optimistic".to_string());

        let provider_completed = vec![
            crate::services::integrations_todoist::TodoistCompletedTaskSnapshot {
                id: "task_provider".to_string(),
                content: "Reply to Dimitri".to_string(),
                labels: vec!["Urgent".to_string()],
                project_id: Some("proj_ops".to_string()),
                due: Some(crate::services::integrations_todoist::TodoistDue {
                    date: Some("2026-03-24".to_string()),
                    datetime: None,
                }),
                deadline: None,
                updated_at: Some("2026-03-24T15:00:00Z".to_string()),
                completed_at: Some("2026-03-24T15:00:00Z".to_string()),
            },
        ];
        let pending_writebacks = vec![WritebackOperationRecord {
            id: WritebackOperationId::new(),
            kind: WritebackOperationKind::TodoistCompleteTask,
            risk: WritebackRisk::Safe,
            status: WritebackStatus::Queued,
            target: WritebackTargetRef {
                family: IntegrationFamily::Tasks,
                provider_key: "todoist".to_string(),
                project_id: None,
                connection_id: None,
                external_id: Some("task_optimistic".to_string()),
            },
            requested_payload: json!({
                "commitment_id": optimistic_open.id.as_ref().to_string(),
            }),
            result_payload: None,
            provenance: Vec::new(),
            conflict_case_id: None,
            requested_by_node_id: "vel-local".to_string(),
            requested_at: now - Duration::minutes(5),
            applied_at: None,
            updated_at: now - Duration::minutes(5),
        }];

        let merged = merge_completed_today_commitments(
            &[open_commitment.clone(), optimistic_open.clone()],
            &[local_done.clone()],
            &provider_completed,
            &pending_writebacks,
            &timezone,
            &current_day,
            now,
        );

        assert_eq!(
            merged
                .iter()
                .map(|item| item.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Call Apria", "Finish weekly review", "Reply to Dimitri"]
        );
        assert_eq!(merged[0].status, CommitmentStatus::Done);
        assert_eq!(
            merged[2].source_id.as_deref(),
            Some("todoist_task_provider")
        );
        assert_eq!(merged[2].metadata_json["labels"], json!(["Urgent"]));
    }

    #[test]
    fn risk_summary_normalizes_unrecognized_levels_to_unknown() {
        let summary = risk_summary("danger", Some(0.4));

        assert_eq!(summary.level, "unknown");
        assert_eq!(summary.label, "unknown · 40%");
    }

    #[test]
    fn trust_readiness_warns_when_review_pressure_exists() {
        let freshness = NowFreshnessOutput {
            overall_status: "fresh".to_string(),
            sources: Vec::new(),
        };
        let review = ReviewSnapshot {
            open_action_count: 2,
            triage_count: 1,
            projects_needing_review: 0,
            pending_execution_reviews: 1,
        };

        let output = build_trust_readiness_from_parts(None, &freshness, &review, 1, 1, &[]);

        assert_eq!(output.level, "warn");
        assert_eq!(output.headline, "Review is pending");
        assert_eq!(output.review.pending_execution_reviews, 1);
        assert_eq!(output.review.conflict_count, 1);
    }

    #[test]
    fn trust_readiness_surfaces_assistant_proposals_in_follow_through() {
        let item = ActionItem {
            id: "act_assistant_proposal_1".to_string().into(),
            surface: ActionSurface::Inbox,
            kind: ActionKind::Intervention,
            permission_mode: ActionPermissionMode::Blocked,
            scope_affinity: ActionScopeAffinity::Global,
            title: "Send the draft reply".to_string(),
            summary: "Gate: SAFE MODE keeps writeback disabled.".to_string(),
            project_id: None,
            project_label: None,
            project_family: None,
            state: ActionState::Active,
            rank: 79,
            surfaced_at: OffsetDateTime::UNIX_EPOCH,
            snoozed_until: None,
            evidence: vec![ActionEvidenceRef {
                source_kind: "assistant_proposal".to_string(),
                source_id: "act_intervention_intv_1".to_string(),
                label: "assistant staged action".to_string(),
                detail: Some("message_id=msg_1 permission_mode=blocked".to_string()),
            }],
            thread_route: None,
        };

        let follow_through = trust_follow_through_items(&[item.clone()]);
        assert_eq!(follow_through.len(), 1);
        assert_eq!(follow_through[0].id, item.id);
    }

    #[test]
    fn build_nudge_bars_suppresses_individual_overdue_commitment_follow_through() {
        let overdue_item = ActionItem {
            id: "act_commitment_overdue_1".to_string().into(),
            surface: ActionSurface::Now,
            kind: ActionKind::NextStep,
            permission_mode: ActionPermissionMode::UserConfirm,
            scope_affinity: ActionScopeAffinity::Global,
            title: "Reply to overdue thread".to_string(),
            summary: "This commitment is overdue and should be handled or rescheduled.".to_string(),
            project_id: None,
            project_label: None,
            project_family: None,
            state: ActionState::Active,
            rank: 90,
            surfaced_at: OffsetDateTime::UNIX_EPOCH,
            snoozed_until: None,
            evidence: vec![ActionEvidenceRef {
                source_kind: "commitment".to_string(),
                source_id: "com_overdue_1".to_string(),
                label: "Reply to overdue thread".to_string(),
                detail: Some("todo".to_string()),
            }],
            thread_route: None,
        };
        let ordinary_item = ActionItem {
            id: "act_follow_up_1".to_string().into(),
            surface: ActionSurface::Now,
            kind: ActionKind::NextStep,
            permission_mode: ActionPermissionMode::UserConfirm,
            scope_affinity: ActionScopeAffinity::Global,
            title: "Review morning plan".to_string(),
            summary: "A review request is ready.".to_string(),
            project_id: None,
            project_label: None,
            project_family: None,
            state: ActionState::Active,
            rank: 70,
            surfaced_at: OffsetDateTime::UNIX_EPOCH,
            snoozed_until: None,
            evidence: vec![ActionEvidenceRef {
                source_kind: "intervention".to_string(),
                source_id: "intv_1".to_string(),
                label: "review".to_string(),
                detail: None,
            }],
            thread_route: None,
        };

        let bars = build_nudge_bars(
            None,
            None,
            &[overdue_item, ordinary_item.clone()],
            None,
            None,
            &[],
            None,
            None,
            fixed_now(),
        );

        assert_eq!(bars.len(), 1);
        assert_eq!(bars[0].title, ordinary_item.title);
    }

    #[test]
    fn now_context_shape_parses_with_typed_context_migrator() {
        let context = json!({
            "computed_at": 1_700_000_000i64,
            "mode": "morning_mode",
            "morning_state": "underway",
            "meds_status": "pending",
            "attention_confidence": 0.9,
            "signals_used": ["sig_1"],
            "commitments_used": ["com_1"],
            "risk_used": ["risk_1"]
        });

        let typed = ContextMigrator::from_json_value(context).expect("context should parse");
        assert_eq!(typed.mode, "morning_mode");
        assert_eq!(typed.attention_confidence, Some(0.9));
    }

    #[test]
    fn now_task_normalizes_todoist_inbox_project_to_none() {
        let mut commitment = commitment_fixture(
            "Message Connie",
            None,
            Some("todo"),
            json!({
                "is_inbox_project": true
            }),
        );
        commitment.project = Some("inbox".to_string());

        let task = now_task(&commitment);

        assert_eq!(task.project, None);
    }

    fn fixed_now() -> OffsetDateTime {
        time::Date::from_calendar_date(2026, Month::March, 16)
            .unwrap()
            .with_hms(8, 0, 0)
            .unwrap()
            .assume_utc()
    }

    fn commitment_fixture(
        text: &str,
        due_at: Option<OffsetDateTime>,
        kind: Option<&str>,
        metadata_json: JsonValue,
    ) -> Commitment {
        Commitment {
            id: CommitmentId::from(format!("com_{}", text.replace(' ', "_").to_lowercase())),
            text: text.to_string(),
            source_type: "todoist".to_string(),
            source_id: None,
            status: CommitmentStatus::Open,
            due_at,
            project: None,
            commitment_kind: kind.map(str::to_string),
            created_at: fixed_now() - Duration::hours(1),
            resolved_at: None,
            metadata_json,
        }
    }
}

fn label_for_mode(value: &str) -> NowLabelOutput {
    match value {
        "meeting_mode" => label(value, "Meeting prep"),
        "commute_mode" => label(value, "Commute"),
        "morning_mode" => label(value, "Morning"),
        "day_mode" => label(value, "Day"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_phase(value: &str) -> NowLabelOutput {
    match value {
        "awake_unstarted" => label(value, "Not started"),
        "underway" => label(value, "Underway"),
        "engaged" => label(value, "Engaged"),
        "at_risk" => label(value, "At risk"),
        "inactive" => label(value, "Inactive"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_meds(value: &str) -> NowLabelOutput {
    match value {
        "pending" => label(value, "Pending"),
        "done" => label(value, "Done"),
        "none" => label(value, "None"),
        _ => label(value, "Unknown"),
    }
}

fn label_for_attention(value: &str) -> NowLabelOutput {
    match value {
        "on_task" => label(value, "On task"),
        "distracted" => label(value, "Distracted"),
        "unknown" => label(value, "Unknown"),
        _ => label(value, value),
    }
}

fn label_for_drift(value: &str) -> NowLabelOutput {
    match value {
        "morning_drift" => label(value, "Morning drift"),
        "prep_drift" => label(value, "Prep drift"),
        "none" => label(value, "None"),
        _ => label(value, value),
    }
}

fn label_for_severity(value: &str) -> NowLabelOutput {
    match value {
        "gentle" => label(value, "Gentle"),
        "warning" => label(value, "Warning"),
        "danger" => label(value, "Danger"),
        "none" => label(value, "None"),
        _ => label(value, value),
    }
}

fn risk_summary(level: &str, score: Option<f64>) -> NowRiskSummaryOutput {
    let normalized_level = normalize_risk_level(level);
    let label = match score {
        Some(score) => format!("{normalized_level} · {}%", (score * 100.0).round() as i64),
        None => normalized_level.to_string(),
    };
    NowRiskSummaryOutput {
        level: normalized_level.to_string(),
        score,
        label,
    }
}
