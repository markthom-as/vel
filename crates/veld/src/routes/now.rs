use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use tracing::warn;
use time::OffsetDateTime;
use vel_api_types::{
    ActionItemData, ApiResponse, CheckInCardData, CommitmentSchedulingProposalSummaryData,
    CommitmentSchedulingProposalSummaryItemData, CurrentContextReflowStatusData,
    DayPlanProposalData, NowAttentionData, NowCalendarEventRescheduleRequestData,
    NowContextLineData, NowCountDisplayModeData, NowData, NowDebugData, NowDockedInputData,
    NowDockedInputIntentData, NowEventData, NowFreshnessData, NowFreshnessEntryData,
    NowHeaderBucketData, NowHeaderBucketKindData, NowHeaderData, NowLabelData, NowMeshSummaryData,
    NowMeshSyncStateData, NowNextUpItemData, NowNudgeActionData, NowNudgeBarData,
    NowNudgeBarKindData, NowOverviewActionData, NowOverviewData, NowOverviewNudgeData,
    NowOverviewSuggestionData, NowOverviewTimelineEntryData, NowOverviewWhyStateData,
    NowProgressData, NowRepairRouteData, NowRepairRouteTargetData, NowRiskSummaryData,
    NowScheduleData, NowSourceActivityData, NowSourcesData, NowStatusRowData, NowSummaryData,
    NowTaskData, NowTaskKindData, NowTaskLaneData, NowTaskLaneItemData, NowTasksData,
    NowThreadFilterTargetData, PlanningProfileProposalSummaryData,
    PlanningProfileProposalSummaryItemData, ReflowCardData, TrustReadinessData,
    TrustReadinessFacetData, TrustReadinessReviewData,
};

use crate::{errors::AppError, routes::response, services, state::AppState};

pub async fn get_now(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<NowData>>, AppError> {
    let data = services::now::get_now_with_state(&state).await?;
    Ok(response::success(data.into()))
}

#[derive(Debug, Deserialize)]
pub struct UpdateNowTaskLaneRequest {
    pub commitment_id: String,
    pub lane: String,
    pub position: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct RescheduleNowTasksToTodayRequest {
    #[serde(default)]
    pub commitment_ids: Vec<String>,
}

pub async fn update_now_task_lane(
    State(state): State<AppState>,
    Json(payload): Json<UpdateNowTaskLaneRequest>,
) -> Result<Json<ApiResponse<NowData>>, AppError> {
    use vel_core::{CommitmentStatus, CurrentContextV1};

    let commitment_id = payload.commitment_id.trim();
    if commitment_id.is_empty() {
        return Err(AppError::bad_request("commitment_id must not be empty"));
    }
    let lane = match payload.lane.trim() {
        "active" | "next_up" | "if_time_allows" | "completed" => payload.lane.trim(),
        _ => return Err(AppError::bad_request("invalid lane")),
    };

    let (_, mut context) = state
        .storage
        .get_current_context()
        .await?
        .unwrap_or((0, CurrentContextV1::default()));

    let commitment = state
        .storage
        .get_commitment_by_id(commitment_id)
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;

    remove_commitment_from_all_lanes(&mut context, commitment_id);

    if lane == "completed" {
        if commitment.status != CommitmentStatus::Done {
            state
                .storage
                .update_commitment(
                    commitment_id,
                    None,
                    Some(CommitmentStatus::Done),
                    None,
                    None,
                    None,
                    None,
                )
                .await?;
        }
    } else if commitment.status == CommitmentStatus::Done {
        state
            .storage
            .update_commitment(
                commitment_id,
                None,
                Some(CommitmentStatus::Open),
                None,
                None,
                None,
                None,
            )
            .await?;
    }

    if lane == "next_up" {
        assign_commitment_to_current_day(&state, &commitment).await?;
    }

    let target = match lane {
        "active" => &mut context.task_lanes.active_commitment_ids,
        "next_up" => &mut context.task_lanes.next_up_commitment_ids,
        "if_time_allows" => &mut context.task_lanes.if_time_allows_commitment_ids,
        _ => &mut context.task_lanes.completed_commitment_ids,
    };
    let position = payload.position.unwrap_or(target.len()).min(target.len());
    target.insert(position, commitment_id.to_string());
    context.next_commitment_id = context
        .task_lanes
        .active_commitment_ids
        .first()
        .cloned()
        .or_else(|| context.task_lanes.next_up_commitment_ids.first().cloned());
    let context_json = serde_json::to_string(&context)
        .map_err(|error| AppError::internal(format!("serialize current context: {error}")))?;
    state
        .storage
        .set_current_context(
            time::OffsetDateTime::now_utc().unix_timestamp(),
            &context_json,
        )
        .await?;

    let data = services::now::get_now_with_state(&state).await?;
    Ok(response::success(data.into()))
}

pub async fn reschedule_now_tasks_to_today(
    State(state): State<AppState>,
    Json(payload): Json<RescheduleNowTasksToTodayRequest>,
) -> Result<Json<ApiResponse<NowData>>, AppError> {
    let commitment_ids = payload
        .commitment_ids
        .into_iter()
        .map(|id| id.trim().to_string())
        .filter(|id| !id.is_empty())
        .collect::<Vec<_>>();

    if commitment_ids.is_empty() {
        return Err(AppError::bad_request("commitment_ids must not be empty"));
    }

    for commitment_id in commitment_ids {
        let commitment = state
            .storage
            .get_commitment_by_id(&commitment_id)
            .await?
            .ok_or_else(|| AppError::not_found("commitment not found"))?;
        assign_commitment_to_current_day(&state, &commitment).await?;
    }

    let data = services::now::get_now_with_state(&state).await?;
    Ok(response::success(data.into()))
}

pub async fn reschedule_now_calendar_event(
    State(state): State<AppState>,
    Json(payload): Json<NowCalendarEventRescheduleRequestData>,
) -> Result<Json<ApiResponse<NowData>>, AppError> {
    let data = services::now::reschedule_calendar_event(
        &state,
        &payload.event_id,
        payload.calendar_id.as_deref(),
        payload.start_ts,
        payload.end_ts,
    )
    .await?;
    Ok(response::success(data.into()))
}

fn remove_commitment_from_all_lanes(context: &mut vel_core::CurrentContextV1, commitment_id: &str) {
    context
        .task_lanes
        .active_commitment_ids
        .retain(|id| id != commitment_id);
    context
        .task_lanes
        .next_up_commitment_ids
        .retain(|id| id != commitment_id);
    context
        .task_lanes
        .if_time_allows_commitment_ids
        .retain(|id| id != commitment_id);
    context
        .task_lanes
        .completed_commitment_ids
        .retain(|id| id != commitment_id);
}

async fn assign_commitment_to_current_day(
    state: &AppState,
    commitment: &vel_core::Commitment,
) -> Result<(), AppError> {
    let timezone = crate::services::timezone::resolve_timezone(&state.storage).await?;
    let current_day =
        crate::services::timezone::current_day_window(&timezone, OffsetDateTime::now_utc())?;
    let session_date = current_day.session_date.clone();
    let due_at = OffsetDateTime::from_unix_timestamp(current_day.end_ts - 1)
        .map_err(|error| AppError::internal(format!("compute current-day due_at: {error}")))?;
    let mut metadata = commitment.metadata_json.clone();
    metadata["assigned_via_now_lane"] = serde_json::json!("next_up");
    metadata["scheduled_for"] = serde_json::json!(session_date.as_str());
    metadata["has_due_time"] = serde_json::json!(false);
    state
        .storage
        .update_commitment(
            commitment.id.as_ref(),
            None,
            None,
            Some(Some(due_at)),
            None,
            None,
            Some(&metadata),
        )
        .await?;
    if commitment.source_type == "todoist" {
        if let Err(error) = services::writeback::todoist_update_task(
            &state.storage,
            &state.config,
            "vel-local",
            commitment.id.as_ref(),
            crate::services::integrations_todoist::TodoistTaskMutation {
                content: None,
                project_id: None,
                scheduled_for: Some(session_date),
                priority: None,
                waiting_on: None,
                review_state: None,
                tags: None,
            },
        )
        .await
        {
            warn!(
                error = %error,
                commitment_id = %commitment.id,
                source_type = %commitment.source_type,
                "failed to write back rescheduled Todoist commitment"
            );
        }
    }
    Ok(())
}

impl From<services::now::NowOutput> for NowData {
    fn from(value: services::now::NowOutput) -> Self {
        Self {
            computed_at: value.computed_at,
            timezone: value.timezone,
            header: value.header.map(Into::into),
            mesh_summary: value.mesh_summary.map(Into::into),
            status_row: value.status_row.map(Into::into),
            context_line: value.context_line.map(Into::into),
            nudge_bars: value.nudge_bars.into_iter().map(Into::into).collect(),
            task_lane: value.task_lane.map(Into::into),
            next_up_items: value.next_up_items.into_iter().map(Into::into).collect(),
            progress: value.progress.into(),
            docked_input: value.docked_input.map(Into::into),
            overview: value.overview.into(),
            summary: value.summary.into(),
            schedule: value.schedule.into(),
            tasks: value.tasks.into(),
            attention: value.attention.into(),
            sources: value.sources.into(),
            freshness: value.freshness.into(),
            trust_readiness: Some(value.trust_readiness.into()),
            planning_profile_summary: value
                .planning_profile_summary
                .map(planning_profile_summary_data),
            commitment_scheduling_summary: value
                .commitment_scheduling_summary
                .map(commitment_scheduling_summary_data),
            check_in: value.check_in.map(CheckInCardData::from),
            day_plan: value.day_plan.map(DayPlanProposalData::from),
            reflow: value.reflow.map(ReflowCardData::from),
            reflow_status: value
                .reflow_status
                .map(CurrentContextReflowStatusData::from),
            action_items: value
                .action_items
                .into_iter()
                .map(ActionItemData::from)
                .collect(),
            review_snapshot: value.review_snapshot.into(),
            pending_writebacks: value
                .pending_writebacks
                .into_iter()
                .map(vel_api_types::WritebackOperationData::from)
                .collect(),
            conflicts: value
                .conflicts
                .into_iter()
                .map(vel_api_types::ConflictCaseData::from)
                .collect(),
            people: value
                .people
                .into_iter()
                .map(vel_api_types::PersonRecordData::from)
                .collect(),
            reasons: value.reasons,
            debug: value.debug.into(),
        }
    }
}

fn is_calendar_backed_task(task: &services::now::NowTaskOutput) -> bool {
    matches!(
        task.source_type.trim().to_ascii_lowercase().as_str(),
        "calendar" | "calendar_event" | "google_calendar" | "gcal" | "apple_calendar"
            | "icloud_calendar"
    )
}

impl From<services::now::NowNextUpItemOutput> for NowNextUpItemData {
    fn from(value: services::now::NowNextUpItemOutput) -> Self {
        Self {
            kind: match value.kind.as_str() {
                "event" => NowTaskKindData::Event,
                "task" => NowTaskKindData::Task,
                _ => NowTaskKindData::Commitment,
            },
            id: value.id,
            title: value.title,
            meta: value.meta,
            detail: value.detail,
            task: value.task.map(Into::into),
        }
    }
}

impl From<services::now::NowMeshSummaryOutput> for NowMeshSummaryData {
    fn from(value: services::now::NowMeshSummaryOutput) -> Self {
        Self {
            authority_node_id: value.authority_node_id,
            authority_label: value.authority_label,
            sync_state: match value.sync_state.as_str() {
                "synced" => NowMeshSyncStateData::Synced,
                "offline" => NowMeshSyncStateData::Offline,
                "local_only" => NowMeshSyncStateData::LocalOnly,
                _ => NowMeshSyncStateData::Stale,
            },
            linked_node_count: value.linked_node_count,
            queued_write_count: value.queued_write_count,
            last_sync_at: value.last_sync_at,
            urgent: value.urgent,
            repair_route: value.repair_route.map(Into::into),
        }
    }
}

impl From<services::now::NowRepairRouteOutput> for NowRepairRouteData {
    fn from(value: services::now::NowRepairRouteOutput) -> Self {
        Self {
            target: match value.target.as_str() {
                "settings_sync" => NowRepairRouteTargetData::SettingsSync,
                "settings_linking" => NowRepairRouteTargetData::SettingsLinking,
                _ => NowRepairRouteTargetData::SettingsRecovery,
            },
            summary: value.summary,
        }
    }
}

impl From<services::now::NowHeaderOutput> for NowHeaderData {
    fn from(value: services::now::NowHeaderOutput) -> Self {
        Self {
            title: value.title,
            buckets: value.buckets.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<services::now::NowHeaderBucketOutput> for NowHeaderBucketData {
    fn from(value: services::now::NowHeaderBucketOutput) -> Self {
        Self {
            kind: match value.kind.as_str() {
                "threads_by_type" => NowHeaderBucketKindData::ThreadsByType,
                "needs_input" => NowHeaderBucketKindData::NeedsInput,
                "new_nudges" => NowHeaderBucketKindData::NewNudges,
                "search_filter" => NowHeaderBucketKindData::SearchFilter,
                "snoozed" => NowHeaderBucketKindData::Snoozed,
                "review_apply" => NowHeaderBucketKindData::ReviewApply,
                "reflow" => NowHeaderBucketKindData::Reflow,
                _ => NowHeaderBucketKindData::FollowUp,
            },
            count: value.count,
            count_display: match value.count_display.as_str() {
                "always_show" => NowCountDisplayModeData::AlwaysShow,
                "hidden_until_active" => NowCountDisplayModeData::HiddenUntilActive,
                _ => NowCountDisplayModeData::ShowNonzero,
            },
            urgent: value.urgent,
            route_target: NowThreadFilterTargetData {
                bucket: match value.kind.as_str() {
                    "threads_by_type" => NowHeaderBucketKindData::ThreadsByType,
                    "needs_input" => NowHeaderBucketKindData::NeedsInput,
                    "new_nudges" => NowHeaderBucketKindData::NewNudges,
                    "search_filter" => NowHeaderBucketKindData::SearchFilter,
                    "snoozed" => NowHeaderBucketKindData::Snoozed,
                    "review_apply" => NowHeaderBucketKindData::ReviewApply,
                    "reflow" => NowHeaderBucketKindData::Reflow,
                    _ => NowHeaderBucketKindData::FollowUp,
                },
                thread_id: value.route_thread_id,
            },
        }
    }
}

impl From<services::now::NowStatusRowOutput> for NowStatusRowData {
    fn from(value: services::now::NowStatusRowOutput) -> Self {
        Self {
            date_label: value.date_label,
            time_label: value.time_label,
            context_label: value.context_label,
            elapsed_label: value.elapsed_label,
        }
    }
}

impl From<services::now::NowContextLineOutput> for NowContextLineData {
    fn from(value: services::now::NowContextLineOutput) -> Self {
        Self {
            text: value.text,
            thread_id: value.thread_id,
            fallback_used: value.fallback_used,
        }
    }
}

impl From<services::now::NowNudgeBarOutput> for NowNudgeBarData {
    fn from(value: services::now::NowNudgeBarOutput) -> Self {
        Self {
            id: value.id,
            kind: match value.kind.as_str() {
                "needs_input" => NowNudgeBarKindData::NeedsInput,
                "review_request" => NowNudgeBarKindData::ReviewRequest,
                "reflow_proposal" => NowNudgeBarKindData::ReflowProposal,
                "thread_continuation" => NowNudgeBarKindData::ThreadContinuation,
                "trust_warning" => NowNudgeBarKindData::TrustWarning,
                "freshness_warning" => NowNudgeBarKindData::FreshnessWarning,
                _ => NowNudgeBarKindData::Nudge,
            },
            title: value.title,
            summary: value.summary,
            urgent: value.urgent,
            primary_thread_id: value.primary_thread_id,
            actions: value.actions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<services::now::NowNudgeActionOutput> for NowNudgeActionData {
    fn from(value: services::now::NowNudgeActionOutput) -> Self {
        Self {
            kind: value.kind,
            label: value.label,
        }
    }
}

impl From<services::now::NowTaskLaneOutput> for NowTaskLaneData {
    fn from(value: services::now::NowTaskLaneOutput) -> Self {
        Self {
            active: value.active.map(Into::into),
            pending: value.pending.into_iter().map(Into::into).collect(),
            active_items: value.active_items.into_iter().map(Into::into).collect(),
            next_up: value.next_up.into_iter().map(Into::into).collect(),
            inbox: value.inbox.into_iter().map(Into::into).collect(),
            if_time_allows: value.if_time_allows.into_iter().map(Into::into).collect(),
            completed: value.completed.into_iter().map(Into::into).collect(),
            recent_completed: value.recent_completed.into_iter().map(Into::into).collect(),
            overflow_count: value.overflow_count,
        }
    }
}

impl From<services::now::NowTaskLaneItemOutput> for NowTaskLaneItemData {
    fn from(value: services::now::NowTaskLaneItemOutput) -> Self {
        Self {
            id: value.id,
            task_kind: match value.task_kind.as_str() {
                "task" => NowTaskKindData::Task,
                "event" => NowTaskKindData::Event,
                _ => NowTaskKindData::Commitment,
            },
            text: value.text,
            title: value.title,
            description: value.description,
            tags: value.tags,
            state: value.state,
            lane: value.lane,
            sort_order: value.sort_order,
            project: value.project,
            primary_thread_id: value.primary_thread_id,
            due_at: value.due_at,
            deadline: value.deadline,
            due_label: value.due_label,
            is_overdue: value.is_overdue,
            deadline_label: value.deadline_label,
            deadline_passed: value.deadline_passed,
        }
    }
}

impl From<services::now::NowDockedInputOutput> for NowDockedInputData {
    fn from(value: services::now::NowDockedInputOutput) -> Self {
        Self {
            supported_intents: value
                .supported_intents
                .into_iter()
                .map(|intent| match intent.as_str() {
                    "task" => NowDockedInputIntentData::Task,
                    "url" => NowDockedInputIntentData::Url,
                    "question" => NowDockedInputIntentData::Question,
                    "note" => NowDockedInputIntentData::Note,
                    "command" => NowDockedInputIntentData::Command,
                    "continuation" => NowDockedInputIntentData::Continuation,
                    "reflection" => NowDockedInputIntentData::Reflection,
                    _ => NowDockedInputIntentData::Scheduling,
                })
                .collect(),
            day_thread_id: value.day_thread_id,
            raw_capture_thread_id: value.raw_capture_thread_id,
        }
    }
}

impl From<services::now::NowOverviewOutput> for NowOverviewData {
    fn from(value: services::now::NowOverviewOutput) -> Self {
        Self {
            dominant_action: value.dominant_action.map(Into::into),
            today_timeline: value.today_timeline.into_iter().map(Into::into).collect(),
            visible_nudge: value.visible_nudge.map(Into::into),
            why_state: value.why_state.into_iter().map(Into::into).collect(),
            suggestions: value.suggestions.into_iter().map(Into::into).collect(),
            decision_options: value.decision_options,
        }
    }
}

impl From<services::now::NowOverviewActionOutput> for NowOverviewActionData {
    fn from(value: services::now::NowOverviewActionOutput) -> Self {
        Self {
            kind: value.kind,
            title: value.title,
            summary: value.summary,
            reference_id: value.reference_id,
        }
    }
}

impl From<services::now::NowOverviewTimelineEntryOutput> for NowOverviewTimelineEntryData {
    fn from(value: services::now::NowOverviewTimelineEntryOutput) -> Self {
        Self {
            kind: value.kind,
            title: value.title,
            timestamp: value.timestamp,
            detail: value.detail,
        }
    }
}

impl From<services::now::NowOverviewNudgeOutput> for NowOverviewNudgeData {
    fn from(value: services::now::NowOverviewNudgeOutput) -> Self {
        Self {
            kind: value.kind,
            title: value.title,
            summary: value.summary,
        }
    }
}

impl From<services::now::NowOverviewWhyStateOutput> for NowOverviewWhyStateData {
    fn from(value: services::now::NowOverviewWhyStateOutput) -> Self {
        Self {
            label: value.label,
            detail: value.detail,
        }
    }
}

impl From<services::now::NowOverviewSuggestionOutput> for NowOverviewSuggestionData {
    fn from(value: services::now::NowOverviewSuggestionOutput) -> Self {
        Self {
            id: value.id,
            kind: value.kind,
            title: value.title,
            summary: value.summary,
        }
    }
}

fn planning_profile_summary_data(
    value: services::planning_profile::PlanningProfileProposalSummary,
) -> PlanningProfileProposalSummaryData {
    PlanningProfileProposalSummaryData {
        pending_count: value.pending_count,
        latest_pending: value.latest_pending.map(planning_profile_summary_item_data),
        latest_applied: value.latest_applied.map(planning_profile_summary_item_data),
        latest_failed: value.latest_failed.map(planning_profile_summary_item_data),
    }
}

fn planning_profile_summary_item_data(
    value: services::planning_profile::PlanningProfileProposalSummaryItem,
) -> PlanningProfileProposalSummaryItemData {
    PlanningProfileProposalSummaryItemData {
        thread_id: value.thread_id,
        state: value.state.into(),
        title: value.title,
        summary: value.summary,
        outcome_summary: value.outcome_summary,
        updated_at: value.updated_at,
    }
}

fn commitment_scheduling_summary_data(
    value: services::commitment_scheduling::CommitmentSchedulingProposalSummary,
) -> CommitmentSchedulingProposalSummaryData {
    CommitmentSchedulingProposalSummaryData {
        pending_count: value.pending_count,
        latest_pending: value
            .latest_pending
            .map(commitment_scheduling_summary_item_data),
        latest_applied: value
            .latest_applied
            .map(commitment_scheduling_summary_item_data),
        latest_failed: value
            .latest_failed
            .map(commitment_scheduling_summary_item_data),
    }
}

fn commitment_scheduling_summary_item_data(
    value: services::commitment_scheduling::CommitmentSchedulingProposalSummaryItem,
) -> CommitmentSchedulingProposalSummaryItemData {
    CommitmentSchedulingProposalSummaryItemData {
        thread_id: value.thread_id,
        state: value.state.into(),
        title: value.title,
        summary: value.summary,
        outcome_summary: value.outcome_summary,
        updated_at: value.updated_at,
    }
}

impl From<services::now::NowSummaryOutput> for NowSummaryData {
    fn from(value: services::now::NowSummaryOutput) -> Self {
        Self {
            mode: value.mode.into(),
            phase: value.phase.into(),
            meds: value.meds.into(),
            risk: value.risk.into(),
        }
    }
}

impl From<services::now::NowLabelOutput> for NowLabelData {
    fn from(value: services::now::NowLabelOutput) -> Self {
        Self {
            key: value.key,
            label: value.label,
        }
    }
}

impl From<services::now::NowRiskSummaryOutput> for NowRiskSummaryData {
    fn from(value: services::now::NowRiskSummaryOutput) -> Self {
        Self {
            level: value.level,
            score: value.score,
            label: value.label,
        }
    }
}

impl From<services::now::NowScheduleOutput> for NowScheduleData {
    fn from(value: services::now::NowScheduleOutput) -> Self {
        Self {
            empty_message: value.empty_message,
            next_event: value.next_event.map(Into::into),
            upcoming_events: value.upcoming_events.into_iter().map(Into::into).collect(),
            following_day_events: value
                .following_day_events
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<services::now::NowEventOutput> for NowEventData {
    fn from(value: services::now::NowEventOutput) -> Self {
        Self {
            event_id: value.event_id,
            calendar_id: value.calendar_id,
            calendar_name: value.calendar_name,
            title: value.title,
            start_ts: value.start_ts,
            end_ts: value.end_ts,
            event_url: value.event_url,
            attachment_url: value.attachment_url,
            location: value.location,
            notes: value.notes,
            attendees: value.attendees,
            video_url: value.video_url,
            video_provider: value.video_provider,
            prep_minutes: value.prep_minutes,
            travel_minutes: value.travel_minutes,
            leave_by_ts: value.leave_by_ts,
            rescheduled: value.rescheduled,
        }
    }
}

impl From<services::now::NowProgressOutput> for NowProgressData {
    fn from(value: services::now::NowProgressOutput) -> Self {
        Self {
            base_count: value.base_count,
            completed_count: value.completed_count,
            backlog_count: value.backlog_count,
            completed_ratio: value.completed_ratio,
            backlog_ratio: value.backlog_ratio,
        }
    }
}

impl From<services::now::NowTasksOutput> for NowTasksData {
    fn from(value: services::now::NowTasksOutput) -> Self {
        Self {
            todoist: value
                .todoist
                .into_iter()
                .filter(|task| !is_calendar_backed_task(task))
                .map(Into::into)
                .collect(),
            other_open: value
                .other_open
                .into_iter()
                .filter(|task| !is_calendar_backed_task(task))
                .map(Into::into)
                .collect(),
            next_commitment: value
                .next_commitment
                .filter(|task| !is_calendar_backed_task(task))
                .map(Into::into),
        }
    }
}

impl From<services::now::NowTaskOutput> for NowTaskData {
    fn from(value: services::now::NowTaskOutput) -> Self {
        Self {
            id: value.id,
            text: value.text,
            title: value.title,
            description: value.description,
            tags: value.tags,
            source_type: value.source_type,
            due_at: value.due_at,
            deadline: value.deadline,
            project: value.project,
            commitment_kind: value.commitment_kind,
        }
    }
}

impl From<services::now::NowAttentionOutput> for NowAttentionData {
    fn from(value: services::now::NowAttentionOutput) -> Self {
        Self {
            state: value.state.into(),
            drift: value.drift.into(),
            severity: value.severity.into(),
            confidence: value.confidence,
            reasons: value.reasons,
        }
    }
}

impl From<services::now::NowSourcesOutput> for NowSourcesData {
    fn from(value: services::now::NowSourcesOutput) -> Self {
        Self {
            git_activity: value.git_activity.map(Into::into),
            health: value.health.map(Into::into),
            mood: value.mood.map(Into::into),
            pain: value.pain.map(Into::into),
            note_document: value.note_document.map(Into::into),
            assistant_message: value.assistant_message.map(Into::into),
        }
    }
}

impl From<services::now::NowSourceActivityOutput> for NowSourceActivityData {
    fn from(value: services::now::NowSourceActivityOutput) -> Self {
        Self {
            label: value.label,
            timestamp: value.timestamp,
            summary: value.summary,
        }
    }
}

impl From<services::now::NowFreshnessOutput> for NowFreshnessData {
    fn from(value: services::now::NowFreshnessOutput) -> Self {
        Self {
            overall_status: value.overall_status,
            sources: value.sources.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<services::now::TrustReadinessOutput> for TrustReadinessData {
    fn from(value: services::now::TrustReadinessOutput) -> Self {
        Self {
            level: value.level,
            headline: value.headline,
            summary: value.summary,
            backup: value.backup.into(),
            freshness: value.freshness.into(),
            review: value.review.into(),
            guidance: value.guidance,
            follow_through: value.follow_through.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<services::now::TrustReadinessFacetOutput> for TrustReadinessFacetData {
    fn from(value: services::now::TrustReadinessFacetOutput) -> Self {
        Self {
            level: value.level,
            label: value.label,
            detail: value.detail,
        }
    }
}

impl From<services::now::TrustReadinessReviewOutput> for TrustReadinessReviewData {
    fn from(value: services::now::TrustReadinessReviewOutput) -> Self {
        Self {
            open_action_count: value.open_action_count,
            pending_execution_reviews: value.pending_execution_reviews,
            pending_writeback_count: value.pending_writeback_count,
            conflict_count: value.conflict_count,
        }
    }
}

impl From<services::now::NowFreshnessEntryOutput> for NowFreshnessEntryData {
    fn from(value: services::now::NowFreshnessEntryOutput) -> Self {
        Self {
            key: value.key,
            label: value.label,
            status: value.status,
            last_sync_at: value.last_sync_at,
            age_seconds: value.age_seconds,
            guidance: value.guidance,
        }
    }
}

impl From<services::now::NowDebugOutput> for NowDebugData {
    fn from(value: services::now::NowDebugOutput) -> Self {
        Self {
            raw_context: value.raw_context,
            signals_used: value.signals_used,
            commitments_used: value.commitments_used,
            risk_used: value.risk_used,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::OffsetDateTime;

    use crate::services;

    #[test]
    fn now_service_output_maps_to_existing_now_dto_shape() {
        let due_at = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let service_output = services::now::NowOutput {
            computed_at: 1_700_000_100,
            timezone: "America/Denver".to_string(),
            header: Some(services::now::NowHeaderOutput {
                title: "Now".to_string(),
                buckets: vec![services::now::NowHeaderBucketOutput {
                    kind: "needs_input".to_string(),
                    count: 1,
                    count_display: "show_nonzero".to_string(),
                    urgent: true,
                    route_thread_id: Some("thr_check_in_1".to_string()),
                }],
            }),
            mesh_summary: Some(services::now::NowMeshSummaryOutput {
                authority_node_id: "vel-desktop".to_string(),
                authority_label: "Vel Desktop".to_string(),
                sync_state: "stale".to_string(),
                linked_node_count: 2,
                queued_write_count: 1,
                last_sync_at: Some(1_700_000_080),
                urgent: true,
                repair_route: Some(services::now::NowRepairRouteOutput {
                    target: "settings_recovery".to_string(),
                    summary: "Sync or queued-write posture needs review before trusting all cross-client state."
                        .to_string(),
                }),
            }),
            status_row: Some(services::now::NowStatusRowOutput {
                date_label: "2026-03-21".to_string(),
                time_label: "09:15".to_string(),
                context_label: "Ship patch".to_string(),
                elapsed_label: "No active task".to_string(),
            }),
            context_line: Some(services::now::NowContextLineOutput {
                text: "Standup check-in is blocking the next part of the day.".to_string(),
                thread_id: None,
                fallback_used: true,
            }),
            nudge_bars: vec![services::now::NowNudgeBarOutput {
                id: "act_check_in_1".to_string(),
                kind: "needs_input".to_string(),
                title: "Standup check-in".to_string(),
                summary: "Vel needs one short answer before the standup can continue."
                    .to_string(),
                urgent: true,
                primary_thread_id: Some("thr_check_in_1".to_string()),
                actions: vec![services::now::NowNudgeActionOutput {
                    kind: "submit".to_string(),
                    label: "Continue standup".to_string(),
                }],
            }],
            task_lane: Some(services::now::NowTaskLaneOutput {
                active: Some(services::now::NowTaskLaneItemOutput {
                    id: "com_1".to_string(),
                    task_kind: "commitment".to_string(),
                    text: "Ship patch".to_string(),
                    title: "Ship patch".to_string(),
                    description: None,
                    tags: Vec::new(),
                    state: "active".to_string(),
                    lane: Some("active".to_string()),
                    sort_order: Some(0),
                    project: Some("Vel".to_string()),
                    primary_thread_id: None,
                    due_at: Some(due_at),
                    deadline: None,
                    due_label: Some("Today".to_string()),
                    is_overdue: false,
                    deadline_label: None,
                    deadline_passed: false,
                }),
                pending: Vec::new(),
                active_items: vec![services::now::NowTaskLaneItemOutput {
                    id: "com_1".to_string(),
                    task_kind: "commitment".to_string(),
                    text: "Ship patch".to_string(),
                    title: "Ship patch".to_string(),
                    description: None,
                    tags: Vec::new(),
                    state: "active".to_string(),
                    lane: Some("active".to_string()),
                    sort_order: Some(0),
                    project: Some("Vel".to_string()),
                    primary_thread_id: None,
                    due_at: Some(due_at),
                    deadline: None,
                    due_label: Some("Today".to_string()),
                    is_overdue: false,
                    deadline_label: None,
                    deadline_passed: false,
                }],
                next_up: Vec::new(),
                inbox: Vec::new(),
                if_time_allows: Vec::new(),
                completed: Vec::new(),
                recent_completed: Vec::new(),
                overflow_count: 0,
            }),
            next_up_items: Vec::new(),
            progress: services::now::NowProgressOutput {
                base_count: 1,
                completed_count: 0,
                backlog_count: 0,
                completed_ratio: 0.0,
                backlog_ratio: 0.0,
            },
            docked_input: Some(services::now::NowDockedInputOutput {
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
            }),
            overview: services::now::NowOverviewOutput {
                dominant_action: Some(services::now::NowOverviewActionOutput {
                    kind: "check_in".to_string(),
                    title: "Standup check-in".to_string(),
                    summary: "Name the one to three commitments that matter most today."
                        .to_string(),
                    reference_id: Some("act_check_in_1".to_string()),
                }),
                today_timeline: vec![services::now::NowOverviewTimelineEntryOutput {
                    kind: "calendar_event".to_string(),
                    title: "Standup".to_string(),
                    timestamp: 1_700_000_400,
                    detail: Some("Desk".to_string()),
                }],
                visible_nudge: Some(services::now::NowOverviewNudgeOutput {
                    kind: "freshness".to_string(),
                    title: "Review operator queue".to_string(),
                    summary: "One supervised review is still pending.".to_string(),
                }),
                why_state: vec![services::now::NowOverviewWhyStateOutput {
                    label: "Phase".to_string(),
                    detail: "Engaged".to_string(),
                }],
                suggestions: Vec::new(),
                decision_options: vec![
                    "accept".to_string(),
                    "choose".to_string(),
                    "thread".to_string(),
                    "close".to_string(),
                ],
            },
            summary: services::now::NowSummaryOutput {
                mode: services::now::NowLabelOutput {
                    key: "day_mode".to_string(),
                    label: "Day".to_string(),
                },
                phase: services::now::NowLabelOutput {
                    key: "engaged".to_string(),
                    label: "Engaged".to_string(),
                },
                meds: services::now::NowLabelOutput {
                    key: "done".to_string(),
                    label: "Done".to_string(),
                },
                risk: services::now::NowRiskSummaryOutput {
                    level: "low".to_string(),
                    score: Some(0.2),
                    label: "low · 20%".to_string(),
                },
            },
            schedule: services::now::NowScheduleOutput {
                empty_message: None,
                next_event: Some(services::now::NowEventOutput {
                    event_id: Some("evt_standup".to_string()),
                    calendar_id: Some("cal_primary".to_string()),
                    calendar_name: Some("Primary".to_string()),
                    title: "Standup".to_string(),
                    start_ts: 1_700_000_400,
                    end_ts: Some(1_700_000_700),
                    all_day: false,
                    event_url: Some("https://calendar.google.com/calendar/event?eid=evt_standup"
                        .to_string()),
                    attachment_url: Some("https://docs.google.com/document/d/standup-notes".to_string()),
                    location: Some("Desk".to_string()),
                    notes: Some("Review blockers before the call.".to_string()),
                    attendees: vec!["alex@example.com".to_string(), "sam@example.com".to_string()],
                    video_url: Some("https://meet.google.com/abc-defg-hij".to_string()),
                    video_provider: Some("google_meet".to_string()),
                    status: Some("confirmed".to_string()),
                    transparency: Some("opaque".to_string()),
                    response_status: Some("accepted".to_string()),
                    prep_minutes: Some(10),
                    travel_minutes: Some(5),
                    leave_by_ts: Some(1_700_000_100),
                    rescheduled: false,
                }),
                following_day_events: vec![],
                upcoming_events: vec![],
            },
            tasks: services::now::NowTasksOutput {
                todoist: vec![services::now::NowTaskOutput {
                    id: "com_1".to_string(),
                    text: "Ship patch".to_string(),
                    title: "Ship patch".to_string(),
                    description: None,
                    tags: Vec::new(),
                    source_type: "todoist".to_string(),
                    due_at: Some(due_at),
                    deadline: None,
                    project: Some("Vel".to_string()),
                    is_inbox_project: false,
                    commitment_kind: Some("todo".to_string()),
                }],
                other_open: vec![],
                next_commitment: None,
            },
            attention: services::now::NowAttentionOutput {
                state: services::now::NowLabelOutput {
                    key: "on_task".to_string(),
                    label: "On task".to_string(),
                },
                drift: services::now::NowLabelOutput {
                    key: "none".to_string(),
                    label: "None".to_string(),
                },
                severity: services::now::NowLabelOutput {
                    key: "none".to_string(),
                    label: "None".to_string(),
                },
                confidence: Some(0.92),
                reasons: vec!["Mode: Day".to_string()],
            },
            sources: services::now::NowSourcesOutput {
                git_activity: Some(services::now::NowSourceActivityOutput {
                    label: "Git activity".to_string(),
                    timestamp: 1_700_000_050,
                    summary: json!({"label":"Recent commit"}),
                }),
                health: None,
                mood: None,
                pain: None,
                note_document: None,
                assistant_message: None,
            },
            freshness: services::now::NowFreshnessOutput {
                overall_status: "fresh".to_string(),
                sources: vec![services::now::NowFreshnessEntryOutput {
                    key: "context".to_string(),
                    label: "Context".to_string(),
                    status: "fresh".to_string(),
                    last_sync_at: Some(1_700_000_090),
                    age_seconds: Some(10),
                    guidance: None,
                }],
            },
            trust_readiness: services::now::TrustReadinessOutput {
                level: "warn".to_string(),
                headline: "Review is pending".to_string(),
                summary: "1 conflict(s) and 1 supervised review(s) still need operator attention."
                    .to_string(),
                backup: services::now::TrustReadinessFacetOutput {
                    level: "ok".to_string(),
                    label: "Backup".to_string(),
                    detail: "Backup trust is healthy.".to_string(),
                },
                freshness: services::now::TrustReadinessFacetOutput {
                    level: "ok".to_string(),
                    label: "Freshness".to_string(),
                    detail: "Current context and integrations look fresh enough to trust."
                        .to_string(),
                },
                review: services::now::TrustReadinessReviewOutput {
                    open_action_count: 1,
                    pending_execution_reviews: 1,
                    pending_writeback_count: 1,
                    conflict_count: 1,
                },
                guidance: vec![
                    "Backup trust is healthy.".to_string(),
                    "Review the remaining conflicts or supervised execution handoffs before risky actions."
                        .to_string(),
                ],
                follow_through: vec![vel_core::ActionItem {
                    id: vel_core::ActionItemId::from("act_recovery_backup".to_string()),
                    surface: vel_core::ActionSurface::Inbox,
                    kind: vel_core::ActionKind::Recovery,
                    permission_mode: vel_core::ActionPermissionMode::UserConfirm,
                    scope_affinity: vel_core::ActionScopeAffinity::Global,
                    title: "Backup is stale".to_string(),
                    summary: "Backup trust is degraded. Create or verify a fresh backup before risky maintenance.".to_string(),
                    project_id: None,
                    project_label: None,
                    project_family: None,
                    state: vel_core::ActionState::Active,
                    rank: 88,
                    surfaced_at: due_at,
                    snoozed_until: None,
                evidence: vec![vel_core::ActionEvidenceRef {
                    source_kind: "backup_trust".to_string(),
                    source_id: "warn".to_string(),
                    label: "Backup trust".to_string(),
                    detail: Some("Backup trust is degraded. Create or verify a fresh backup before risky maintenance.".to_string()),
                }],
                thread_route: None,
                }],
            },
            planning_profile_summary: Some(
                services::planning_profile::PlanningProfileProposalSummary {
                    pending_count: 1,
                    latest_pending: Some(
                        services::planning_profile::PlanningProfileProposalSummaryItem {
                            thread_id: "thr_planning_profile_edit_1".to_string(),
                            state: vel_core::AssistantProposalState::Staged,
                            title: "Add shutdown block".to_string(),
                            summary: "Add a protected shutdown block.".to_string(),
                            outcome_summary: None,
                            updated_at: 1_700_000_095,
                        },
                    ),
                    latest_applied: None,
                    latest_failed: None,
                },
            ),
            commitment_scheduling_summary: Some(
                services::commitment_scheduling::CommitmentSchedulingProposalSummary {
                    pending_count: 1,
                    latest_pending: Some(
                        services::commitment_scheduling::CommitmentSchedulingProposalSummaryItem {
                            thread_id: "thr_day_plan_apply_1".to_string(),
                            state: vel_core::AssistantProposalState::Staged,
                            title: "Apply focus block shift".to_string(),
                            summary: "Move the focus block after the calendar anchor.".to_string(),
                            outcome_summary: None,
                            updated_at: 1_700_000_096,
                        },
                    ),
                    latest_applied: None,
                    latest_failed: None,
                },
            ),
            check_in: Some(vel_core::CheckInCard {
                id: vel_core::ActionItemId::from("act_check_in_1".to_string()),
                source_kind: vel_core::CheckInSourceKind::DailyLoop,
                phase: vel_core::DailyLoopPhase::Standup,
                session_id: "dls_1".to_string(),
                title: "Standup check-in".to_string(),
                summary: "Vel needs one short answer before the standup can continue.".to_string(),
                prompt_id: "standup_prompt_1".to_string(),
                prompt_text: "Name the one to three commitments that matter most today."
                    .to_string(),
                suggested_action_label: Some("Continue standup".to_string()),
                suggested_response: None,
                allow_skip: true,
                blocking: true,
                submit_target: vel_core::CheckInSubmitTarget {
                    kind: vel_core::CheckInSubmitTargetKind::DailyLoopTurn,
                    reference_id: "dls_1".to_string(),
                },
                escalation: Some(vel_core::CheckInEscalation {
                    target: vel_core::CheckInEscalationTarget::Threads,
                    label: "Continue in Threads".to_string(),
                    thread_id: None,
                }),
                transitions: vec![
                    vel_core::CheckInTransition {
                        kind: vel_core::CheckInTransitionKind::Submit,
                        label: "Continue standup".to_string(),
                        target: vel_core::CheckInTransitionTargetKind::DailyLoopTurn,
                        reference_id: Some("dls_1".to_string()),
                        requires_response: true,
                        requires_note: false,
                    },
                    vel_core::CheckInTransition {
                        kind: vel_core::CheckInTransitionKind::Bypass,
                        label: "Skip for now".to_string(),
                        target: vel_core::CheckInTransitionTargetKind::DailyLoopTurn,
                        reference_id: Some("dls_1".to_string()),
                        requires_response: false,
                        requires_note: true,
                    },
                    vel_core::CheckInTransition {
                        kind: vel_core::CheckInTransitionKind::Escalate,
                        label: "Continue in Threads".to_string(),
                        target: vel_core::CheckInTransitionTargetKind::Threads,
                        reference_id: Some("dls_1".to_string()),
                        requires_response: false,
                        requires_note: false,
                    },
                ],
            }),
            day_plan: Some(vel_core::DayPlanProposal {
                headline: "Today has a bounded plan".to_string(),
                summary: "Vel shaped a bounded same-day plan from current commitments, calendar anchors, and routine blocks.".to_string(),
                scheduled_count: 2,
                deferred_count: 1,
                did_not_fit_count: 0,
                needs_judgment_count: 1,
                changes: vec![
                    vel_core::DayPlanChange {
                        kind: vel_core::DayPlanChangeKind::Scheduled,
                        commitment_id: Some("cmt_weekly_review".to_string()),
                        title: "Write weekly review".to_string(),
                        detail: "Write weekly review fits in the next bounded slot for today.".to_string(),
                        project_label: Some("Ops".to_string()),
                        scheduled_start_ts: Some(1_700_001_000),
                        rule_facets: vec![vel_core::ScheduleRuleFacet {
                            kind: vel_core::ScheduleRuleFacetKind::TimeWindow,
                            label: "time:prenoon".to_string(),
                            detail: Some("Task prefers the prenoon window.".to_string()),
                        }],
                    },
                    vel_core::DayPlanChange {
                        kind: vel_core::DayPlanChangeKind::Deferred,
                        commitment_id: Some("cmt_backlog_cleanup".to_string()),
                        title: "Backlog cleanup".to_string(),
                        detail: "Backlog cleanup is marked for local defer and was left out of today's bounded plan.".to_string(),
                        project_label: None,
                        scheduled_start_ts: None,
                        rule_facets: vec![vel_core::ScheduleRuleFacet {
                            kind: vel_core::ScheduleRuleFacetKind::LocalDefer,
                            label: "defer".to_string(),
                            detail: Some("Task is marked for local defer logic.".to_string()),
                        }],
                    },
                ],
                routine_blocks: vec![vel_core::RoutineBlock {
                    id: "routine_morning".to_string(),
                    label: "Morning routine".to_string(),
                    source: vel_core::RoutineBlockSourceKind::Inferred,
                    start_ts: 1_700_000_000,
                    end_ts: 1_700_000_600,
                    protected: true,
                }],
            }),
            reflow: Some(vel_core::ReflowCard {
                id: vel_core::ActionItemId::from("act_reflow_1".to_string()),
                title: "Day changed".to_string(),
                summary:
                    "A scheduled event appears to have slipped past without the plan being updated."
                        .to_string(),
                trigger: vel_core::ReflowTriggerKind::MissedEvent,
                severity: vel_core::ReflowSeverity::Critical,
                accept_mode: vel_core::ReflowAcceptMode::ConfirmRequired,
                suggested_action_label: "Accept".to_string(),
                preview_lines: vec![
                    "Next scheduled event started 20 minutes ago.".to_string(),
                    "Leave-by threshold passed 10 minutes ago.".to_string(),
                ],
                edit_target: vel_core::ReflowEditTarget {
                    target: vel_core::CheckInEscalationTarget::Threads,
                    label: "Edit".to_string(),
                },
                proposal: Some(vel_core::ReflowProposal {
                    headline: "Remaining day needs repair".to_string(),
                    summary:
                        "Vel can now carry a typed remaining-day recovery proposal over the reflow seam before full schedule recomputation lands."
                            .to_string(),
                    moved_count: 0,
                    unscheduled_count: 0,
                    needs_judgment_count: 1,
                    changes: vec![vel_core::ReflowChange {
                        kind: vel_core::ReflowChangeKind::NeedsJudgment,
                        commitment_id: None,
                        title: "Scheduled time already passed".to_string(),
                        detail: "Next scheduled event started 20 minutes ago.".to_string(),
                        project_label: None,
                        scheduled_start_ts: Some(1_700_000_000),
                    }],
                    rule_facets: vec![vel_core::ScheduleRuleFacet {
                        kind: vel_core::ScheduleRuleFacetKind::FixedStart,
                        label: "Fixed start".to_string(),
                        detail: Some(
                            "A due datetime or schedule anchor should stay explicit in the recomputed day."
                                .to_string(),
                        ),
                    }],
                }),
                transitions: vec![
                    vel_core::ReflowTransition {
                        kind: vel_core::ReflowTransitionKind::Accept,
                        label: "Accept".to_string(),
                        target: vel_core::ReflowTransitionTargetKind::ApplySuggestion,
                        confirm_required: true,
                    },
                    vel_core::ReflowTransition {
                        kind: vel_core::ReflowTransitionKind::Edit,
                        label: "Edit".to_string(),
                        target: vel_core::ReflowTransitionTargetKind::Threads,
                        confirm_required: false,
                    },
                ],
            }),
            reflow_status: Some(vel_core::CurrentContextReflowStatus {
                source_context_computed_at: 1_700_000_100,
                recorded_at: 1_700_000_300,
                kind: vel_core::CurrentContextReflowStatusKind::Editing,
                trigger: vel_core::ReflowTriggerKind::MissedEvent,
                severity: vel_core::ReflowSeverity::Critical,
                headline: "Reflow moved to Threads".to_string(),
                detail: "Vel opened a thread-backed reflow follow-up so the day plan can be shaped before anything else changes.".to_string(),
                preview_lines: vec!["Next scheduled event started 20 minutes ago.".to_string()],
                thread_id: Some("thr_reflow_1".to_string()),
            }),
            action_items: vec![vel_core::ActionItem {
                id: vel_core::ActionItemId::from("act_1".to_string()),
                surface: vel_core::ActionSurface::Now,
                kind: vel_core::ActionKind::NextStep,
                permission_mode: vel_core::ActionPermissionMode::UserConfirm,
                scope_affinity: vel_core::ActionScopeAffinity::Global,
                title: "Ship patch".to_string(),
                summary: "Due soon".to_string(),
                project_id: None,
                project_label: None,
                project_family: None,
                state: vel_core::ActionState::Active,
                rank: 70,
                surfaced_at: due_at,
                snoozed_until: None,
                evidence: vec![vel_core::ActionEvidenceRef {
                    source_kind: "commitment".to_string(),
                    source_id: "com_1".to_string(),
                    label: "Ship patch".to_string(),
                    detail: None,
                }],
                thread_route: Some(vel_core::ActionThreadRoute {
                    target: vel_core::ActionThreadRouteTarget::FilteredThreads,
                    label: "Open related threads".to_string(),
                    thread_id: None,
                    thread_type: Some("action_resolution".to_string()),
                    project_id: Some("proj_vel".to_string().into()),
                }),
            }],
            review_snapshot: vel_core::ReviewSnapshot {
                open_action_count: 1,
                triage_count: 0,
                projects_needing_review: 0,
                pending_execution_reviews: 1,
            },
            pending_writebacks: vec![],
            conflicts: vec![],
            people: vec![],
            reasons: vec!["Mode: Day".to_string()],
            debug: services::now::NowDebugOutput {
                raw_context: json!({"mode":"day_mode"}),
                signals_used: vec!["sig_1".to_string()],
                commitments_used: vec!["com_1".to_string()],
                risk_used: vec!["risk_1".to_string()],
            },
        };

        let dto: vel_api_types::NowData = service_output.into();
        let json = serde_json::to_value(dto).unwrap();

        assert_eq!(json["timezone"], "America/Denver");
        assert_eq!(json["header"]["title"], "Now");
        assert_eq!(json["header"]["buckets"][0]["kind"], "needs_input");
        assert_eq!(json["mesh_summary"]["sync_state"], "stale");
        assert_eq!(
            json["mesh_summary"]["repair_route"]["target"],
            "settings_recovery"
        );
        assert_eq!(json["status_row"]["context_label"], "Ship patch");
        assert_eq!(json["context_line"]["fallback_used"], true);
        assert_eq!(json["nudge_bars"][0]["kind"], "needs_input");
        assert_eq!(json["task_lane"]["active"]["task_kind"], "commitment");
        assert_eq!(json["docked_input"]["supported_intents"][4], "command");
        assert_eq!(json["overview"]["dominant_action"]["kind"], "check_in");
        assert_eq!(json["overview"]["decision_options"][2], "thread");
        assert_eq!(json["summary"]["risk"]["label"], "low · 20%");
        assert_eq!(
            json["tasks"]["todoist"][0]["due_at"],
            "2023-11-14T22:13:20Z"
        );
        assert_eq!(
            json["sources"]["git_activity"]["summary"]["label"],
            "Recent commit"
        );
        assert_eq!(json["freshness"]["sources"][0]["key"], "context");
        assert_eq!(json["trust_readiness"]["level"], "warn");
        assert_eq!(
            json["trust_readiness"]["follow_through"][0]["kind"],
            "recovery"
        );
        assert_eq!(
            json["trust_readiness"]["review"]["pending_execution_reviews"],
            1
        );
        assert_eq!(json["planning_profile_summary"]["pending_count"], 1);
        assert_eq!(
            json["commitment_scheduling_summary"]["latest_pending"]["title"],
            "Apply focus block shift"
        );
        assert_eq!(json["check_in"]["phase"], "standup");
        assert_eq!(json["check_in"]["submit_target"]["kind"], "daily_loop_turn");
        assert_eq!(json["check_in"]["escalation"]["target"], "threads");
        assert_eq!(json["check_in"]["transitions"][0]["kind"], "submit");
        assert_eq!(json["check_in"]["transitions"][1]["kind"], "bypass");
        assert_eq!(json["check_in"]["transitions"][2]["target"], "threads");
        assert_eq!(json["day_plan"]["scheduled_count"], 2);
        assert_eq!(json["day_plan"]["deferred_count"], 1);
        assert_eq!(json["day_plan"]["routine_blocks"][0]["source"], "inferred");
        assert_eq!(json["reflow"]["trigger"], "missed_event");
        assert_eq!(json["reflow"]["severity"], "critical");
        assert_eq!(json["reflow"]["edit_target"]["target"], "threads");
        assert_eq!(json["reflow_status"]["kind"], "editing");
        assert_eq!(json["reflow_status"]["thread_id"], "thr_reflow_1");
        assert_eq!(
            json["reflow"]["transitions"][0]["target"],
            "apply_suggestion"
        );
        assert_eq!(json["reflow"]["transitions"][1]["kind"], "edit");
        assert_eq!(json["action_items"][0]["rank"], 70);
        assert_eq!(
            json["action_items"][0]["thread_route"]["target"],
            "filtered_threads"
        );
        assert_eq!(json["review_snapshot"]["open_action_count"], 1);
    }

    #[test]
    fn now_api_filters_calendar_backed_tasks_from_task_streams() {
        let due_at = OffsetDateTime::from_unix_timestamp(1_700_000_000).unwrap();
        let dto: vel_api_types::NowData = services::now::NowOutput {
            computed_at: 1_700_000_100,
            timezone: "America/Denver".to_string(),
            header: None,
            mesh_summary: None,
            status_row: None,
            context_line: None,
            nudge_bars: Vec::new(),
            task_lane: None,
            next_up_items: Vec::new(),
            progress: services::now::NowProgressOutput {
                base_count: 1,
                completed_count: 0,
                backlog_count: 0,
                completed_ratio: 0.0,
                backlog_ratio: 0.0,
            },
            docked_input: None,
            overview: services::now::NowOverviewOutput {
                dominant_action: None,
                today_timeline: Vec::new(),
                visible_nudge: None,
                why_state: Vec::new(),
                suggestions: Vec::new(),
                decision_options: Vec::new(),
            },
            summary: services::now::NowSummaryOutput {
                mode: services::now::NowLabelOutput {
                    key: "day_mode".to_string(),
                    label: "Day".to_string(),
                },
                phase: services::now::NowLabelOutput {
                    key: "engaged".to_string(),
                    label: "Engaged".to_string(),
                },
                meds: services::now::NowLabelOutput {
                    key: "done".to_string(),
                    label: "Done".to_string(),
                },
                risk: services::now::NowRiskSummaryOutput {
                    level: "low".to_string(),
                    score: Some(0.2),
                    label: "low · 20%".to_string(),
                },
            },
            schedule: services::now::NowScheduleOutput {
                empty_message: None,
                next_event: None,
                upcoming_events: Vec::new(),
                following_day_events: Vec::new(),
            },
            tasks: services::now::NowTasksOutput {
                todoist: vec![services::now::NowTaskOutput {
                    id: "evt_task_2".to_string(),
                    text: "Calendar item in todoist lane".to_string(),
                    title: "Calendar item in todoist lane".to_string(),
                    description: None,
                    tags: Vec::new(),
                    source_type: "google_calendar".to_string(),
                    due_at: Some(due_at),
                    deadline: None,
                    project: None,
                    is_inbox_project: false,
                    commitment_kind: Some("todo".to_string()),
                }],
                other_open: vec![services::now::NowTaskOutput {
                    id: "evt_task_3".to_string(),
                    text: "Calendar item in other_open".to_string(),
                    title: "Calendar item in other_open".to_string(),
                    description: None,
                    tags: Vec::new(),
                    source_type: "calendar_event".to_string(),
                    due_at: Some(due_at),
                    deadline: None,
                    project: None,
                    is_inbox_project: false,
                    commitment_kind: Some("todo".to_string()),
                }],
                next_commitment: Some(services::now::NowTaskOutput {
                    id: "evt_task_4".to_string(),
                    text: "Calendar item in next commitment".to_string(),
                    title: "Calendar item in next commitment".to_string(),
                    description: None,
                    tags: Vec::new(),
                    source_type: "google_calendar".to_string(),
                    due_at: Some(due_at),
                    deadline: None,
                    project: None,
                    is_inbox_project: false,
                    commitment_kind: Some("todo".to_string()),
                }),
            },
            attention: services::now::NowAttentionOutput {
                state: services::now::NowLabelOutput {
                    key: "on_task".to_string(),
                    label: "On task".to_string(),
                },
                drift: services::now::NowLabelOutput {
                    key: "none".to_string(),
                    label: "None".to_string(),
                },
                severity: services::now::NowLabelOutput {
                    key: "none".to_string(),
                    label: "None".to_string(),
                },
                confidence: None,
                reasons: Vec::new(),
            },
            sources: services::now::NowSourcesOutput {
                git_activity: None,
                health: None,
                mood: None,
                pain: None,
                note_document: None,
                assistant_message: None,
            },
            freshness: services::now::NowFreshnessOutput {
                overall_status: "ok".to_string(),
                sources: Vec::new(),
            },
            trust_readiness: services::now::TrustReadinessOutput {
                level: "ok".to_string(),
                headline: "Ready".to_string(),
                summary: "Ready".to_string(),
                backup: services::now::TrustReadinessFacetOutput {
                    level: "ok".to_string(),
                    label: "Backup".to_string(),
                    detail: "Fresh backup".to_string(),
                },
                freshness: services::now::TrustReadinessFacetOutput {
                    level: "ok".to_string(),
                    label: "Freshness".to_string(),
                    detail: "Fresh".to_string(),
                },
                review: services::now::TrustReadinessReviewOutput {
                    open_action_count: 0,
                    pending_execution_reviews: 0,
                    pending_writeback_count: 0,
                    conflict_count: 0,
                },
                guidance: Vec::new(),
                follow_through: Vec::new(),
            },
            planning_profile_summary: None,
            commitment_scheduling_summary: None,
            check_in: None,
            day_plan: None,
            reflow: None,
            reflow_status: None,
            action_items: Vec::new(),
            review_snapshot: vel_core::ReviewSnapshot {
                open_action_count: 0,
                triage_count: 0,
                projects_needing_review: 0,
                pending_execution_reviews: 0,
            },
            pending_writebacks: Vec::new(),
            conflicts: Vec::new(),
            people: Vec::new(),
            reasons: Vec::new(),
            debug: services::now::NowDebugOutput {
                raw_context: json!({}),
                signals_used: Vec::new(),
                commitments_used: Vec::new(),
                risk_used: Vec::new(),
            },
        }
        .into();
        let json = serde_json::to_value(dto).unwrap();

        assert_eq!(json["tasks"]["todoist"], json!([]));
        assert_eq!(json["tasks"]["other_open"], json!([]));
        assert!(json["tasks"]["next_commitment"].is_null());
        assert_eq!(json["next_up_items"], json!([]));
    }
}
