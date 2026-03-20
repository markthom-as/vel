use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::{
    normalize_risk_level, ActionItem, ActionKind, CheckInCard, Commitment, CommitmentStatus,
    ConflictCaseRecord, CurrentContextReflowStatus, CurrentContextV1, DayPlanProposal, ReflowCard,
    ReflowSeverity, ReviewSnapshot, WritebackOperationRecord,
};
use vel_storage::{SignalRecord, Storage};

use crate::{errors::AppError, services::integrations};

#[derive(Debug, Clone)]
pub struct NowOutput {
    pub computed_at: i64,
    pub timezone: String,
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
    pub title: String,
    pub start_ts: i64,
    pub end_ts: Option<i64>,
    pub all_day: bool,
    pub location: Option<String>,
    pub status: Option<String>,
    pub transparency: Option<String>,
    pub response_status: Option<String>,
    pub prep_minutes: Option<i64>,
    pub travel_minutes: Option<i64>,
    pub leave_by_ts: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct NowTaskOutput {
    pub id: String,
    pub text: String,
    pub source_type: String,
    pub due_at: Option<OffsetDateTime>,
    pub project: Option<String>,
    pub commitment_kind: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NowScheduleOutput {
    pub empty_message: Option<String>,
    pub next_event: Option<NowEventOutput>,
    pub upcoming_events: Vec<NowEventOutput>,
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
    let now = OffsetDateTime::now_utc();
    let now_ts = now.unix_timestamp();
    let timezone = crate::services::timezone::resolve_timezone(storage).await?;
    let current_day = crate::services::timezone::current_day_window(&timezone, now)?;
    let check_in = crate::services::check_in::get_current_check_in(storage, &timezone).await?;
    let apple_behavior_summary =
        crate::services::apple_behavior::get_summary(storage, config).await?;
    let action_queue = crate::services::operator_queue::build_action_items(storage, config).await?;
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
            check_in,
            &action_queue,
            planning_profile_summary,
            commitment_scheduling_summary,
        ));
    };

    let commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 64)
        .await?;
    let sorted_commitments = sort_commitments(commitments, &timezone, now);
    let task_buckets = split_now_tasks(&context, sorted_commitments, &timezone, now, &current_day);

    let signal_ids = context.signals_used.clone();
    let calendar_selection = integrations::google_calendar_selection_filter(storage).await?;
    let mut events = storage
        .list_signals_by_ids(&signal_ids)
        .await?
        .into_iter()
        .filter(|signal| calendar_selection.includes_signal(signal))
        .filter_map(calendar_event_from_signal)
        .filter(|event| event_overlaps_current_day(event, &current_day))
        .collect::<Vec<_>>();
    if events.is_empty() {
        events = storage
            .list_signals(Some("calendar_event"), Some(current_day.start_ts), 32)
            .await?
            .into_iter()
            .filter(|signal| calendar_selection.includes_signal(signal))
            .filter_map(calendar_event_from_signal)
            .filter(|event| event_overlaps_current_day(event, &current_day))
            .collect();
    }
    events.sort_by_key(|event| event.start_ts);
    let next_event = events.iter().find(|event| event.start_ts > now_ts).cloned();
    let upcoming_events: Vec<NowEventOutput> = events
        .into_iter()
        .filter(|event| event.end_ts.unwrap_or(event.start_ts) >= now_ts)
        .take(5)
        .collect();

    let integrations = integrations::get_integrations_with_config(storage, config).await?;
    let integrations = IntegrationSnapshotOutput {
        google_calendar: CalendarStatusOutput {
            connected: integrations.google_calendar.connected,
            all_calendars_selected: integrations.google_calendar.all_calendars_selected,
            any_selected: integrations
                .google_calendar
                .calendars
                .iter()
                .any(|calendar| calendar.selected),
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

    Ok(NowOutput {
        computed_at,
        timezone: timezone.name,
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
    check_in: Option<CheckInCard>,
    action_queue: &crate::services::operator_queue::ActionQueueSnapshot,
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
        dominant_action.and_then(|current| current.reference_id.as_ref())
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
        .take(5)
        .map(|commitment| now_task(&commitment))
        .collect::<Vec<_>>();
    let pullable = pullable_tasks
        .into_iter()
        .take(6)
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
    if context
        .next_commitment_id
        .as_deref()
        .is_some_and(|id| id == commitment.id.as_ref())
    {
        return true;
    }
    if context
        .commitments_used
        .iter()
        .any(|id| id == commitment.id.as_ref())
    {
        return true;
    }

    commitment_priority_key(commitment, timezone, now, current_day).0 <= 3
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
    NowTaskOutput {
        id: commitment.id.as_ref().to_string(),
        text: commitment.text.clone(),
        source_type: commitment.source_type.clone(),
        due_at: commitment.due_at,
        project: commitment.project.clone(),
        commitment_kind: commitment.commitment_kind.clone(),
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

fn event_is_relevant(event: &NowEventOutput) -> bool {
    let transparency = event.transparency.as_deref().unwrap_or_default();
    let response_status = event.response_status.as_deref().unwrap_or_default();
    let status = event.status.as_deref().unwrap_or_default();
    !event.all_day
        && !transparency.eq_ignore_ascii_case("transparent")
        && !transparency.eq_ignore_ascii_case("free")
        && !response_status.eq_ignore_ascii_case("declined")
        && !status.eq_ignore_ascii_case("cancelled")
}

fn calendar_event_from_signal(signal: SignalRecord) -> Option<NowEventOutput> {
    if signal.signal_type != "calendar_event" {
        return None;
    }
    let payload = signal.payload_json;
    let start_ts = payload
        .get("start")?
        .as_i64()
        .or_else(|| payload.get("start_ts")?.as_i64())?;
    let title = payload
        .get("title")
        .and_then(|value| value.as_str())
        .unwrap_or("Untitled event")
        .to_string();
    let travel_minutes = payload
        .get("travel_minutes")
        .and_then(|value| value.as_i64());
    Some(NowEventOutput {
        title,
        start_ts,
        end_ts: payload.get("end").and_then(|value| value.as_i64()),
        all_day: payload
            .get("all_day")
            .and_then(|value| value.as_bool())
            .unwrap_or(false),
        location: payload
            .get("location")
            .and_then(|value| value.as_str())
            .map(str::to_string),
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
    })
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
        CommitmentId, CommitmentStatus, ContextMigrator,
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
