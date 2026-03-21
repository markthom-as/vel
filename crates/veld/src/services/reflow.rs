use serde_json::{json, Value as JsonValue};
use time::{Duration, OffsetDateTime, Time};
use uuid::Uuid;
use vel_core::{
    ActionItemId, CanonicalScheduleRules, CheckInEscalationTarget, Commitment, CommitmentStatus,
    CurrentContextReflowStatus, CurrentContextReflowStatusKind, CurrentContextV1, ReflowAcceptMode,
    ReflowCard, ReflowChange, ReflowChangeKind, ReflowEditTarget, ReflowProposal, ReflowSeverity,
    ReflowTransition, ReflowTransitionKind, ReflowTransitionTargetKind, ReflowTriggerKind,
    ScheduleRuleFacet, ScheduleRuleFacetKind, ScheduleTimeWindow,
};
use vel_storage::{SignalRecord, Storage};

use crate::errors::AppError;

struct ReflowCandidate {
    trigger: ReflowTriggerKind,
    severity: ReflowSeverity,
    summary: String,
    preview_lines: Vec<String>,
}

#[derive(Clone)]
struct ScheduleTask {
    commitment_id: String,
    title: String,
    project_label: Option<String>,
    duration_minutes: i64,
    fixed_start_ts: Option<i64>,
    due_ts: Option<i64>,
    scheduler_rules: CanonicalScheduleRules,
}

#[derive(Clone, Copy)]
struct ScheduleWindow {
    start_ts: i64,
    end_ts: i64,
}

pub fn derive_reflow(context: &CurrentContextV1, now_ts: i64) -> Option<ReflowCard> {
    if current_status_for_snapshot(context).is_some() {
        return None;
    }

    build_card_from_candidate(context, derive_candidate(context, now_ts)?)
}

pub fn current_status_for_snapshot(
    context: &CurrentContextV1,
) -> Option<&CurrentContextReflowStatus> {
    context
        .reflow_status
        .as_ref()
        .filter(|status| status.source_context_computed_at == context.computed_at)
}

pub async fn derive_current_reflow(
    storage: &Storage,
    context: &CurrentContextV1,
    now_ts: i64,
) -> Result<Option<ReflowCard>, AppError> {
    if current_status_for_snapshot(context).is_some() {
        return Ok(None);
    }

    let Some(candidate) = derive_candidate(context, now_ts) else {
        return Ok(None);
    };

    build_card_from_candidate_with_storage(storage, context, candidate, now_ts).await
}

pub async fn apply_current_reflow(
    storage: &Storage,
    now_ts: i64,
    confirmed: bool,
) -> Result<CurrentContextReflowStatus, AppError> {
    let (_, mut context) = storage
        .get_current_context()
        .await?
        .ok_or_else(|| AppError::not_found("current context not found"))?;

    if current_status_for_snapshot(&context).is_some() {
        return Err(AppError::bad_request("current reflow already handled"));
    }

    let candidate = derive_candidate(&context, now_ts)
        .ok_or_else(|| AppError::not_found("reflow not available"))?;
    let card = build_card_from_candidate_with_storage(storage, &context, candidate, now_ts)
        .await?
        .ok_or_else(|| AppError::not_found("reflow not available"))?;
    if card.accept_mode == ReflowAcceptMode::ConfirmRequired && !confirmed {
        return Err(AppError::bad_request("reflow confirmation required"));
    }
    if requires_thread_continuation(&card) {
        return edit_current_reflow(storage, now_ts).await;
    }

    let staged_proposal = match card.proposal.as_ref() {
        Some(proposal) => crate::services::commitment_scheduling::staged_commitment_scheduling_proposal_from_reflow(
            storage,
            proposal,
        )
        .await?,
        None => None,
    };

    let thread_id = if let Some(proposal) = staged_proposal {
        let thread_id = format!("thr_{}", Uuid::new_v4().simple());
        let mut metadata = json!({
            "source": "reflow",
            "resolution_state": "applied",
            "context_computed_at": context.computed_at,
            "trigger": card.trigger.to_string(),
            "severity": card.severity.to_string(),
            "summary": card.summary,
            "preview_lines": card.preview_lines,
        });
        if let Some(object) = metadata.as_object_mut() {
            object.extend(
                crate::services::commitment_scheduling::applyable_proposal_metadata(&proposal)
                    .as_object()
                    .cloned()
                    .unwrap_or_default(),
            );
        }
        storage
            .insert_thread(
                &thread_id,
                "reflow_edit",
                "Reflow edit",
                "open",
                &metadata.to_string(),
            )
            .await?;
        storage
            .insert_thread_link(&thread_id, "current_context", "singleton", "reflow")
            .await?;
        crate::services::commitment_scheduling::apply_staged_commitment_scheduling_proposal(
            storage, &thread_id,
        )
        .await?;
        Some(thread_id)
    } else {
        None
    };

    let status = CurrentContextReflowStatus {
        source_context_computed_at: context.computed_at,
        recorded_at: now_ts,
        kind: CurrentContextReflowStatusKind::Applied,
        trigger: card.trigger,
        severity: card.severity,
        headline: "Reflow accepted".to_string(),
        detail:
            "Vel marked this schedule drift for backend reflow follow-through and suppressed the current card until context changes."
                .to_string(),
        preview_lines: card.preview_lines.clone(),
        thread_id,
    };

    persist_status(storage, &mut context, status.clone(), now_ts).await?;
    Ok(status)
}

pub async fn edit_current_reflow(
    storage: &Storage,
    now_ts: i64,
) -> Result<CurrentContextReflowStatus, AppError> {
    let (_, mut context) = storage
        .get_current_context()
        .await?
        .ok_or_else(|| AppError::not_found("current context not found"))?;

    if current_status_for_snapshot(&context).is_some() {
        return Err(AppError::bad_request("current reflow already handled"));
    }

    let candidate = derive_candidate(&context, now_ts)
        .ok_or_else(|| AppError::not_found("reflow not available"))?;
    let card = build_card_from_candidate_with_storage(storage, &context, candidate, now_ts)
        .await?
        .ok_or_else(|| AppError::not_found("reflow not available"))?;
    let thread_id = format!("thr_{}", Uuid::new_v4().simple());
    let mut metadata = json!({
        "source": "reflow",
        "resolution_state": "editing",
        "context_computed_at": context.computed_at,
        "trigger": card.trigger.to_string(),
        "severity": card.severity.to_string(),
        "summary": card.summary,
        "preview_lines": card.preview_lines,
    });
    if let Some(proposal) = match card.proposal.as_ref() {
        Some(proposal) => crate::services::commitment_scheduling::staged_commitment_scheduling_proposal_from_reflow(
            storage,
            proposal,
        )
        .await?,
        None => None,
    } {
        if let Some(object) = metadata.as_object_mut() {
            object.extend(
                crate::services::commitment_scheduling::applyable_proposal_metadata(&proposal)
                    .as_object()
                    .cloned()
                    .unwrap_or_default(),
            );
        }
    }
    storage
        .insert_thread(
            &thread_id,
            "reflow_edit",
            "Reflow edit",
            "open",
            &metadata.to_string(),
        )
        .await?;
    storage
        .insert_thread_link(&thread_id, "current_context", "singleton", "reflow")
        .await?;

    let status = CurrentContextReflowStatus {
        source_context_computed_at: context.computed_at,
        recorded_at: now_ts,
        kind: CurrentContextReflowStatusKind::Editing,
        trigger: card.trigger,
        severity: card.severity,
        headline: "Reflow moved to Threads".to_string(),
        detail:
            "Vel opened a thread-backed reflow follow-up so the day plan can be shaped before anything else changes."
                .to_string(),
        preview_lines: card.preview_lines.clone(),
        thread_id: Some(thread_id),
    };

    persist_status(storage, &mut context, status.clone(), now_ts).await?;
    Ok(status)
}

pub fn transitions_for_card(card: &ReflowCard) -> Vec<ReflowTransition> {
    vec![
        ReflowTransition {
            kind: ReflowTransitionKind::Accept,
            label: card.suggested_action_label.clone(),
            target: ReflowTransitionTargetKind::ApplySuggestion,
            confirm_required: card.accept_mode == ReflowAcceptMode::ConfirmRequired,
        },
        ReflowTransition {
            kind: ReflowTransitionKind::Edit,
            label: card.edit_target.label.clone(),
            target: ReflowTransitionTargetKind::Threads,
            confirm_required: false,
        },
    ]
}

fn requires_thread_continuation(card: &ReflowCard) -> bool {
    card.accept_mode == ReflowAcceptMode::ConfirmRequired
        || card
            .proposal
            .as_ref()
            .map(|proposal| proposal.needs_judgment_count > 0)
            .unwrap_or(false)
}

fn derive_candidate(context: &CurrentContextV1, now_ts: i64) -> Option<ReflowCandidate> {
    if now_ts - context.computed_at > 30 * 60 {
        Some(ReflowCandidate {
            trigger: ReflowTriggerKind::StaleSchedule,
            severity: ReflowSeverity::High,
            summary:
                "Vel's day plan is stale enough that the current schedule may no longer be trustworthy."
                    .to_string(),
            preview_lines: preview_for_stale_schedule(context, now_ts),
        })
    } else if let Some(next_event_start_ts) = context.next_event_start_ts {
        if next_event_start_ts < now_ts - 15 * 60 {
            Some(ReflowCandidate {
                trigger: ReflowTriggerKind::MissedEvent,
                severity: ReflowSeverity::Critical,
                summary:
                    "A scheduled event appears to have slipped past without the plan being updated."
                        .to_string(),
                preview_lines: preview_for_missed_event(context, next_event_start_ts, now_ts),
            })
        } else {
            None
        }
    } else {
        drift_candidate(context)
    }
}

fn build_card_from_candidate(
    context: &CurrentContextV1,
    candidate: ReflowCandidate,
) -> Option<ReflowCard> {
    let ReflowCandidate {
        trigger,
        severity,
        summary,
        preview_lines,
    } = candidate;
    let accept_mode = match severity {
        ReflowSeverity::Medium => ReflowAcceptMode::DirectAccept,
        ReflowSeverity::High | ReflowSeverity::Critical => ReflowAcceptMode::ConfirmRequired,
    };

    let mut card = ReflowCard {
        id: ActionItemId::from(format!("act_reflow_{}_{}", trigger, context.computed_at)),
        title: "Day changed".to_string(),
        summary,
        trigger,
        severity,
        accept_mode,
        suggested_action_label: "Accept".to_string(),
        preview_lines,
        edit_target: ReflowEditTarget {
            target: CheckInEscalationTarget::Threads,
            label: "Edit".to_string(),
        },
        proposal: None,
        transitions: Vec::new(),
    };
    card.proposal = Some(build_proposal(context, &card));
    card.transitions = transitions_for_card(&card);
    Some(card)
}

async fn build_card_from_candidate_with_storage(
    storage: &Storage,
    context: &CurrentContextV1,
    candidate: ReflowCandidate,
    now_ts: i64,
) -> Result<Option<ReflowCard>, AppError> {
    let mut card = match build_card_from_candidate(context, candidate) {
        Some(card) => card,
        None => return Ok(None),
    };
    card.proposal = Some(build_remaining_day_proposal(storage, context, &card, now_ts).await?);
    card.transitions = transitions_for_card(&card);
    Ok(Some(card))
}

fn build_proposal(context: &CurrentContextV1, card: &ReflowCard) -> ReflowProposal {
    let (change_kind, title, scheduled_start_ts) = match card.trigger {
        ReflowTriggerKind::SlippedPlannedBlock => (
            ReflowChangeKind::Moved,
            "Planned block likely needs to move".to_string(),
            None,
        ),
        ReflowTriggerKind::TaskNoLongerFits => (
            ReflowChangeKind::Unscheduled,
            "At least one task may no longer fit today".to_string(),
            context.next_commitment_due_at,
        ),
        ReflowTriggerKind::MissedEvent => (
            ReflowChangeKind::NeedsJudgment,
            "Scheduled time already passed".to_string(),
            context.next_event_start_ts,
        ),
        ReflowTriggerKind::MajorSyncChange => (
            ReflowChangeKind::NeedsJudgment,
            "Recent sync changed today's plan".to_string(),
            context.next_commitment_due_at,
        ),
        ReflowTriggerKind::StaleSchedule => (
            ReflowChangeKind::NeedsJudgment,
            "Current day plan is stale".to_string(),
            context.next_commitment_due_at,
        ),
    };
    let detail = card
        .preview_lines
        .first()
        .cloned()
        .unwrap_or_else(|| card.summary.clone());
    let change = ReflowChange {
        kind: change_kind,
        commitment_id: None,
        title,
        detail,
        project_label: None,
        scheduled_start_ts,
    };
    let mut rule_facets = Vec::new();
    if context.next_event_start_ts.is_some() {
        rule_facets.push(ScheduleRuleFacet {
            kind: ScheduleRuleFacetKind::FixedStart,
            label: "Fixed start".to_string(),
            detail: Some(
                "A due datetime or schedule anchor should stay explicit in the recomputed day."
                    .to_string(),
            ),
        });
    }
    if matches!(
        context.drift_type.as_deref(),
        Some("morning_drift" | "prep_drift")
    ) {
        rule_facets.push(ScheduleRuleFacet {
            kind: ScheduleRuleFacetKind::TimeWindow,
            label: "Morning window".to_string(),
            detail: Some(
                "Current drift originated from a bounded daily window that should remain explainable."
                    .to_string(),
            ),
        });
    }
    ReflowProposal {
        headline: "Remaining day needs repair".to_string(),
        summary:
            "Vel can now carry a typed remaining-day recovery proposal over the reflow seam before full schedule recomputation lands."
                .to_string(),
        moved_count: u32::from(matches!(change.kind, ReflowChangeKind::Moved)),
        unscheduled_count: u32::from(matches!(change.kind, ReflowChangeKind::Unscheduled)),
        needs_judgment_count: u32::from(matches!(change.kind, ReflowChangeKind::NeedsJudgment)),
        changes: vec![change],
        rule_facets,
    }
}

async fn build_remaining_day_proposal(
    storage: &Storage,
    context: &CurrentContextV1,
    card: &ReflowCard,
    now_ts: i64,
) -> Result<ReflowProposal, AppError> {
    let commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 128)
        .await?;
    let events = storage
        .list_signals(Some("calendar_event"), Some(day_start_ts(now_ts)), 64)
        .await?;
    let planning_inputs =
        crate::services::planning_profile::load_day_planning_inputs(storage, context, now_ts)
            .await?;
    let windows = remaining_day_windows(
        &events,
        &planning_inputs.routine_blocks,
        &planning_inputs.planning_constraints,
        now_ts,
    );
    let tasks = collect_schedule_tasks(
        context,
        commitments,
        now_ts,
        &planning_inputs.planning_constraints,
    );
    let mut changes = Vec::new();
    let mut facets = Vec::new();
    let overflow_requires_judgment =
        crate::services::planning_profile::require_judgment_for_overflow(
            &planning_inputs.planning_constraints,
        );

    if card.trigger == ReflowTriggerKind::MissedEvent {
        let detail = card
            .preview_lines
            .first()
            .cloned()
            .unwrap_or_else(|| "A scheduled event already slipped.".to_string());
        changes.push(ReflowChange {
            kind: ReflowChangeKind::NeedsJudgment,
            commitment_id: None,
            title: "Missed scheduled event".to_string(),
            detail,
            project_label: None,
            scheduled_start_ts: context.next_event_start_ts,
        });
        facets.push(ScheduleRuleFacet {
            kind: ScheduleRuleFacetKind::FixedStart,
            label: "Fixed start".to_string(),
            detail: Some(
                "Missed calendar anchors still require operator judgment before the plan moves."
                    .to_string(),
            ),
        });
    }

    let mut working_windows = windows;
    for task in tasks {
        facets.extend(rule_facets_for_task(&task));

        if let Some(fixed_start_ts) = task.fixed_start_ts {
            if fixed_start_ts <= now_ts
                || !window_can_fit_fixed(
                    working_windows.as_slice(),
                    fixed_start_ts,
                    task.duration_minutes,
                )
            {
                changes.push(ReflowChange {
                    kind: ReflowChangeKind::NeedsJudgment,
                    commitment_id: Some(task.commitment_id.clone()),
                    title: task.title.clone(),
                    detail: format!(
                        "{} is anchored to a fixed time that no longer fits the remaining schedule.",
                        task.title
                    ),
                    project_label: task.project_label.clone(),
                    scheduled_start_ts: Some(fixed_start_ts),
                });
            }
            continue;
        }

        let Some(slot_start_ts) = reserve_window_for_task(&mut working_windows, &task, now_ts)
        else {
            changes.push(ReflowChange {
                kind: if overflow_requires_judgment {
                    ReflowChangeKind::NeedsJudgment
                } else {
                    ReflowChangeKind::Unscheduled
                },
                commitment_id: Some(task.commitment_id.clone()),
                title: task.title.clone(),
                detail: format!(
                    "{} no longer fits in the remaining day without operator intervention.",
                    task.title
                ),
                project_label: task.project_label.clone(),
                scheduled_start_ts: None,
            });
            continue;
        };

        changes.push(ReflowChange {
            kind: ReflowChangeKind::Moved,
            commitment_id: Some(task.commitment_id.clone()),
            title: task.title.clone(),
            detail: format!(
                "{} can move to the next available slot in the remaining day.",
                task.title
            ),
            project_label: task.project_label.clone(),
            scheduled_start_ts: Some(slot_start_ts),
        });
    }

    if changes.is_empty() {
        return Ok(build_proposal(context, card));
    }

    dedupe_rule_facets(&mut facets);
    let moved_count = changes
        .iter()
        .filter(|change| change.kind == ReflowChangeKind::Moved)
        .count() as u32;
    let unscheduled_count = changes
        .iter()
        .filter(|change| change.kind == ReflowChangeKind::Unscheduled)
        .count() as u32;
    let needs_judgment_count = changes
        .iter()
        .filter(|change| change.kind == ReflowChangeKind::NeedsJudgment)
        .count() as u32;

    let summary = match (moved_count, unscheduled_count, needs_judgment_count) {
        (_, _, count) if count > 0 => {
            "Vel recomputed the remaining day and found at least one item that still needs operator judgment.".to_string()
        }
        (_, count, _) if count > 0 => {
            "Vel recomputed the remaining day and found work that no longer fits today.".to_string()
        }
        _ => "Vel recomputed the remaining day and found explicit moved follow-through.".to_string(),
    };

    Ok(ReflowProposal {
        headline: "Remaining day recomputed".to_string(),
        summary,
        moved_count,
        unscheduled_count,
        needs_judgment_count,
        changes,
        rule_facets: facets,
    })
}

fn collect_schedule_tasks(
    context: &CurrentContextV1,
    commitments: Vec<Commitment>,
    now_ts: i64,
    constraints: &[vel_core::PlanningConstraint],
) -> Vec<ScheduleTask> {
    let end_of_day = day_end_ts(now_ts);
    let mut tasks = commitments
        .into_iter()
        .filter_map(|commitment| {
            schedule_task_from_commitment(context, commitment, now_ts, end_of_day, constraints)
        })
        .collect::<Vec<_>>();
    tasks.sort_by_key(|task| {
        (
            task.fixed_start_ts.unwrap_or(i64::MAX),
            task.due_ts.unwrap_or(i64::MAX),
            task.duration_minutes,
        )
    });
    tasks.truncate(6);
    tasks
}

fn schedule_task_from_commitment(
    context: &CurrentContextV1,
    commitment: Commitment,
    _now_ts: i64,
    end_of_day: i64,
    constraints: &[vel_core::PlanningConstraint],
) -> Option<ScheduleTask> {
    let due_ts = commitment.due_at.map(|value| value.unix_timestamp());
    let scheduler_rules = commitment.scheduler_rules();
    let relevant = context
        .next_commitment_id
        .as_ref()
        .map(|id| id == commitment.id.as_ref())
        .unwrap_or(false)
        || due_ts.map(|value| value <= end_of_day).unwrap_or(false)
        || scheduler_rules.local_urgency
        || context
            .commitments_used
            .iter()
            .any(|id| id == commitment.id.as_ref());
    if !relevant {
        return None;
    }

    let mut scheduler_rules = scheduler_rules;
    if scheduler_rules.time_window.is_none() {
        scheduler_rules.time_window =
            crate::services::planning_profile::default_time_window(constraints);
    }
    let duration_minutes = scheduler_rules.duration_minutes.unwrap_or(30);
    Some(ScheduleTask {
        commitment_id: commitment.id.to_string(),
        title: commitment.text,
        project_label: commitment.project,
        duration_minutes,
        fixed_start_ts: fixed_start_ts(due_ts, &scheduler_rules),
        due_ts,
        scheduler_rules,
    })
}

fn fixed_start_ts(due_ts: Option<i64>, rules: &CanonicalScheduleRules) -> Option<i64> {
    if rules.fixed_start {
        return due_ts;
    }
    None
}

fn remaining_day_windows(
    events: &[SignalRecord],
    routine_blocks: &[vel_core::RoutineBlock],
    constraints: &[vel_core::PlanningConstraint],
    now_ts: i64,
) -> Vec<ScheduleWindow> {
    let mut windows = vec![ScheduleWindow {
        start_ts: now_ts,
        end_ts: day_end_ts(now_ts),
    }];
    for event in events
        .iter()
        .filter_map(|signal| calendar_event_block(signal, constraints))
    {
        windows = subtract_block(windows, event);
    }
    for routine in routine_blocks
        .iter()
        .filter(|block| block.protected && block.end_ts > now_ts)
    {
        windows = subtract_block(
            windows,
            ScheduleWindow {
                start_ts: routine.start_ts.max(now_ts),
                end_ts: routine.end_ts,
            },
        );
    }
    windows
        .into_iter()
        .filter(|window| window.end_ts - window.start_ts >= 15 * 60)
        .collect()
}

fn calendar_event_block(
    signal: &SignalRecord,
    constraints: &[vel_core::PlanningConstraint],
) -> Option<ScheduleWindow> {
    let payload = &signal.payload_json;
    let start_ts = json_i64(payload, "start").or_else(|| json_i64(payload, "start_ts"))?;
    let end_ts = json_i64(payload, "end")
        .or_else(|| json_i64(payload, "end_ts"))
        .unwrap_or(start_ts + 30 * 60);
    let prep_minutes = json_i64(payload, "prep_minutes").unwrap_or(0);
    let travel_minutes = json_i64(payload, "travel_minutes").unwrap_or(0);
    let buffer_before =
        crate::services::planning_profile::reserve_buffer_before_calendar_minutes(constraints);
    let buffer_after =
        crate::services::planning_profile::reserve_buffer_after_calendar_minutes(constraints);
    Some(ScheduleWindow {
        start_ts: start_ts - ((prep_minutes + travel_minutes + buffer_before) * 60),
        end_ts: end_ts + (buffer_after * 60),
    })
}

fn subtract_block(windows: Vec<ScheduleWindow>, block: ScheduleWindow) -> Vec<ScheduleWindow> {
    let mut next = Vec::new();
    for window in windows {
        if block.end_ts <= window.start_ts || block.start_ts >= window.end_ts {
            next.push(window);
            continue;
        }
        if block.start_ts > window.start_ts {
            next.push(ScheduleWindow {
                start_ts: window.start_ts,
                end_ts: block.start_ts,
            });
        }
        if block.end_ts < window.end_ts {
            next.push(ScheduleWindow {
                start_ts: block.end_ts,
                end_ts: window.end_ts,
            });
        }
    }
    next
}

fn reserve_window_for_task(
    windows: &mut Vec<ScheduleWindow>,
    task: &ScheduleTask,
    now_ts: i64,
) -> Option<i64> {
    let duration_seconds = task.duration_minutes * 60;
    let window_bounds = preferred_window_bounds(task.scheduler_rules.time_window, now_ts);
    for index in 0..windows.len() {
        let window = windows[index];
        let start_ts = window.start_ts.max(window_bounds.0);
        let end_cap = window.end_ts.min(window_bounds.1);
        if end_cap - start_ts < duration_seconds {
            continue;
        }

        windows[index].start_ts = start_ts + duration_seconds;
        return Some(start_ts);
    }
    None
}

fn preferred_window_bounds(time_window: Option<ScheduleTimeWindow>, now_ts: i64) -> (i64, i64) {
    let now = unix_to_time(now_ts);
    let date = now.date();
    match time_window {
        Some(ScheduleTimeWindow::Prenoon) => (
            date.with_time(Time::from_hms(6, 0, 0).unwrap())
                .assume_utc()
                .unix_timestamp(),
            date.with_time(Time::from_hms(12, 0, 0).unwrap())
                .assume_utc()
                .unix_timestamp(),
        ),
        Some(ScheduleTimeWindow::Afternoon) => (
            date.with_time(Time::from_hms(12, 0, 0).unwrap())
                .assume_utc()
                .unix_timestamp(),
            date.with_time(Time::from_hms(17, 0, 0).unwrap())
                .assume_utc()
                .unix_timestamp(),
        ),
        Some(ScheduleTimeWindow::Evening) => (
            date.with_time(Time::from_hms(17, 0, 0).unwrap())
                .assume_utc()
                .unix_timestamp(),
            date.with_time(Time::from_hms(21, 0, 0).unwrap())
                .assume_utc()
                .unix_timestamp(),
        ),
        Some(ScheduleTimeWindow::Night) => (
            date.with_time(Time::from_hms(21, 0, 0).unwrap())
                .assume_utc()
                .unix_timestamp(),
            day_end_ts(now_ts),
        ),
        Some(ScheduleTimeWindow::Day) => (
            date.with_time(Time::from_hms(6, 0, 0).unwrap())
                .assume_utc()
                .unix_timestamp(),
            day_end_ts(now_ts),
        ),
        _ => (now_ts, day_end_ts(now_ts)),
    }
}

fn window_can_fit_fixed(
    windows: &[ScheduleWindow],
    fixed_start_ts: i64,
    duration_minutes: i64,
) -> bool {
    let duration_seconds = duration_minutes * 60;
    windows.iter().any(|window| {
        fixed_start_ts >= window.start_ts && fixed_start_ts + duration_seconds <= window.end_ts
    })
}

fn rule_facets_for_task(task: &ScheduleTask) -> Vec<ScheduleRuleFacet> {
    let mut facets = task.scheduler_rules.to_rule_facets();
    if task.fixed_start_ts.is_some()
        && !facets
            .iter()
            .any(|facet| facet.kind == ScheduleRuleFacetKind::FixedStart)
    {
        facets.push(ScheduleRuleFacet {
            kind: ScheduleRuleFacetKind::FixedStart,
            label: "fixed_start".to_string(),
            detail: Some("Task has a fixed scheduled start.".to_string()),
        });
    }
    facets
}

fn dedupe_rule_facets(facets: &mut Vec<ScheduleRuleFacet>) {
    let mut seen = std::collections::HashSet::new();
    facets.retain(|facet| seen.insert((facet_kind_key(facet.kind), facet.label.clone())));
}

fn facet_kind_key(kind: ScheduleRuleFacetKind) -> &'static str {
    match kind {
        ScheduleRuleFacetKind::BlockTarget => "block_target",
        ScheduleRuleFacetKind::Duration => "duration",
        ScheduleRuleFacetKind::CalendarFree => "calendar_free",
        ScheduleRuleFacetKind::FixedStart => "fixed_start",
        ScheduleRuleFacetKind::TimeWindow => "time_window",
        ScheduleRuleFacetKind::LocalUrgency => "local_urgency",
        ScheduleRuleFacetKind::LocalDefer => "local_defer",
    }
}

fn json_i64(value: &JsonValue, key: &str) -> Option<i64> {
    value.get(key).and_then(|entry| {
        entry
            .as_i64()
            .or_else(|| entry.as_str().and_then(|text| text.parse::<i64>().ok()))
    })
}

fn unix_to_time(unix_ts: i64) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(unix_ts).unwrap_or(OffsetDateTime::UNIX_EPOCH)
}

fn day_start_ts(now_ts: i64) -> i64 {
    let now = unix_to_time(now_ts);
    now.date()
        .with_time(Time::MIDNIGHT)
        .assume_utc()
        .unix_timestamp()
}

fn day_end_ts(now_ts: i64) -> i64 {
    let now = unix_to_time(now_ts);
    (now.date().with_time(Time::MIDNIGHT).assume_utc() + Duration::days(1)).unix_timestamp()
}

async fn persist_status(
    storage: &Storage,
    context: &mut CurrentContextV1,
    status: CurrentContextReflowStatus,
    now_ts: i64,
) -> Result<(), AppError> {
    context.reflow_status = Some(status);
    let context_json =
        serde_json::to_string(context).map_err(|error| AppError::internal(error.to_string()))?;
    storage
        .set_current_context(context.computed_at, &context_json)
        .await?;
    storage
        .insert_context_timeline(now_ts, &context_json, None)
        .await?;
    Ok(())
}

fn drift_candidate(context: &CurrentContextV1) -> Option<ReflowCandidate> {
    let drift_type = context.drift_type.as_deref()?;
    let severity = match context.drift_severity.as_deref() {
        Some("high") => ReflowSeverity::High,
        Some("medium") => ReflowSeverity::Medium,
        Some("danger") => ReflowSeverity::Critical,
        _ => ReflowSeverity::Medium,
    };
    let trigger = match drift_type {
        "prep_drift" | "morning_drift" => ReflowTriggerKind::SlippedPlannedBlock,
        _ => ReflowTriggerKind::StaleSchedule,
    };
    let summary = match trigger {
        ReflowTriggerKind::SlippedPlannedBlock => {
            "The current plan has slipped enough that Vel should recalculate the remaining day."
                .to_string()
        }
        _ => "The current schedule looks stale and may need recalculation.".to_string(),
    };
    let mut preview_lines = context
        .attention_reasons
        .iter()
        .take(3)
        .cloned()
        .collect::<Vec<_>>();
    if preview_lines.is_empty() {
        preview_lines.push(format!("Detected drift: {}", drift_type.replace('_', " ")));
    }
    Some(ReflowCandidate {
        trigger,
        severity,
        summary,
        preview_lines,
    })
}

fn preview_for_stale_schedule(context: &CurrentContextV1, now_ts: i64) -> Vec<String> {
    let mut preview = vec![format!(
        "Current context is {} minutes old.",
        (now_ts - context.computed_at) / 60
    )];
    preview.extend(context.attention_reasons.iter().take(2).cloned());
    preview
}

fn preview_for_missed_event(
    context: &CurrentContextV1,
    next_event_start_ts: i64,
    now_ts: i64,
) -> Vec<String> {
    let mut preview = vec![format!(
        "Next scheduled event started {} minutes ago.",
        (now_ts - next_event_start_ts) / 60
    )];
    if let Some(leave_by_ts) = context.leave_by_ts {
        preview.push(format!(
            "Leave-by threshold passed {} minutes ago.",
            (now_ts - leave_by_ts) / 60
        ));
    }
    preview.extend(context.attention_reasons.iter().take(2).cloned());
    preview
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use time::OffsetDateTime;
    use vel_core::{
        DurableRoutineBlock, PlanningConstraint, PlanningConstraintKind, RoutineBlockSourceKind,
        RoutinePlanningProfile,
    };
    use vel_storage::Storage;
    use vel_storage::{CommitmentInsert, SignalInsert};

    async fn test_storage() -> Storage {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
    }

    async fn seed_context(storage: &Storage, context: CurrentContextV1) {
        storage
            .set_current_context(
                context.computed_at,
                &serde_json::to_string(&context).expect("context json"),
            )
            .await
            .unwrap();
    }

    async fn seed_commitment(
        storage: &Storage,
        text: &str,
        due_ts: Option<i64>,
        project: Option<&str>,
        metadata_json: JsonValue,
    ) {
        storage
            .insert_commitment(CommitmentInsert {
                text: text.to_string(),
                source_type: "todoist".to_string(),
                source_id: format!("todoist_{text}"),
                status: CommitmentStatus::Open,
                due_at: due_ts.and_then(|value| OffsetDateTime::from_unix_timestamp(value).ok()),
                project: project.map(str::to_string),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(metadata_json),
            })
            .await
            .unwrap();
    }

    async fn seed_calendar_event(storage: &Storage, start_ts: i64, end_ts: i64, title: &str) {
        storage
            .insert_signal(SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: Some(format!("evt_{title}")),
                timestamp: start_ts,
                payload_json: Some(json!({
                    "title": title,
                    "start": start_ts,
                    "end": end_ts,
                })),
            })
            .await
            .unwrap();
    }

    async fn seed_profile(storage: &Storage, profile: RoutinePlanningProfile) {
        crate::services::planning_profile::save_routine_planning_profile(storage, &profile)
            .await
            .expect("profile should save");
    }

    #[test]
    fn derives_critical_reflow_for_missed_event() {
        let context = CurrentContextV1 {
            computed_at: 1_700_000_000,
            next_event_start_ts: Some(1_700_000_000),
            leave_by_ts: Some(1_699_999_700),
            attention_reasons: vec!["Prep window active".to_string()],
            ..CurrentContextV1::default()
        };

        let card = derive_reflow(&context, 1_700_001_200).expect("reflow should exist");

        assert_eq!(card.trigger, ReflowTriggerKind::MissedEvent);
        assert_eq!(card.severity, ReflowSeverity::Critical);
        assert_eq!(card.accept_mode, ReflowAcceptMode::ConfirmRequired);
        assert_eq!(card.transitions.len(), 2);
        assert_eq!(card.transitions[0].kind, ReflowTransitionKind::Accept);
        assert!(card.transitions[0].confirm_required);
        assert_eq!(card.transitions[1].kind, ReflowTransitionKind::Edit);
        let proposal = card.proposal.expect("proposal should exist");
        assert_eq!(proposal.needs_judgment_count, 1);
        assert_eq!(proposal.changes[0].kind, ReflowChangeKind::NeedsJudgment);
        assert_eq!(
            proposal.rule_facets[0].kind,
            ScheduleRuleFacetKind::FixedStart
        );
    }

    #[test]
    fn derives_medium_reflow_for_drift() {
        let context = CurrentContextV1 {
            computed_at: 1_700_000_000,
            drift_type: Some("morning_drift".to_string()),
            drift_severity: Some("medium".to_string()),
            attention_reasons: vec!["Morning not started".to_string()],
            ..CurrentContextV1::default()
        };

        let card = derive_reflow(&context, 1_700_000_100).expect("reflow should exist");

        assert_eq!(card.trigger, ReflowTriggerKind::SlippedPlannedBlock);
        assert_eq!(card.severity, ReflowSeverity::Medium);
        assert_eq!(card.edit_target.target, CheckInEscalationTarget::Threads);
        assert!(!card.transitions[0].confirm_required);
        let proposal = card.proposal.expect("proposal should exist");
        assert_eq!(proposal.moved_count, 1);
        assert_eq!(proposal.changes[0].kind, ReflowChangeKind::Moved);
    }

    #[test]
    fn suppresses_reflow_card_when_current_snapshot_already_handled() {
        let context = CurrentContextV1 {
            computed_at: 1_700_000_000,
            drift_type: Some("morning_drift".to_string()),
            drift_severity: Some("medium".to_string()),
            reflow_status: Some(CurrentContextReflowStatus {
                source_context_computed_at: 1_700_000_000,
                recorded_at: 1_700_000_300,
                kind: CurrentContextReflowStatusKind::Applied,
                trigger: ReflowTriggerKind::SlippedPlannedBlock,
                severity: ReflowSeverity::Medium,
                headline: "Reflow accepted".to_string(),
                detail: "Vel marked this schedule drift for reflow review.".to_string(),
                preview_lines: vec![],
                thread_id: None,
            }),
            ..CurrentContextV1::default()
        };

        assert!(derive_reflow(&context, 1_700_000_100).is_none());
    }

    #[tokio::test]
    async fn apply_current_reflow_requires_confirmation_for_confirm_required_cards() {
        let storage = test_storage().await;
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: 1_700_000_000,
                next_event_start_ts: Some(1_700_000_000),
                leave_by_ts: Some(1_699_999_700),
                attention_reasons: vec!["Prep window active".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;

        let error = apply_current_reflow(&storage, 1_700_001_200, false)
            .await
            .expect_err("confirmation should be required");

        assert!(error.to_string().contains("confirmation required"));
    }

    #[tokio::test]
    async fn edit_current_reflow_creates_thread_backed_status() {
        let storage = test_storage().await;
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: 1_700_000_000,
                drift_type: Some("morning_drift".to_string()),
                drift_severity: Some("medium".to_string()),
                attention_reasons: vec!["Morning not started".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;

        let status = edit_current_reflow(&storage, 1_700_000_300)
            .await
            .expect("edit should create status");

        assert_eq!(status.kind, CurrentContextReflowStatusKind::Editing);
        let thread_id = status.thread_id.as_ref().expect("thread id");
        let thread = storage
            .get_thread_by_id(thread_id)
            .await
            .unwrap()
            .expect("thread exists");
        assert_eq!(thread.1, "reflow_edit");
        let links = storage.list_thread_links(thread_id).await.unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].1, "current_context");
    }

    #[tokio::test]
    async fn apply_current_reflow_persists_status_and_suppresses_card() {
        let storage = test_storage().await;
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: 1_700_000_000,
                drift_type: Some("morning_drift".to_string()),
                drift_severity: Some("medium".to_string()),
                attention_reasons: vec!["Morning not started".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;

        let status = apply_current_reflow(&storage, 1_700_000_300, true)
            .await
            .expect("apply should persist status");

        assert_eq!(status.kind, CurrentContextReflowStatusKind::Applied);
        let (_, context) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("current context");
        assert!(current_status_for_snapshot(&context).is_some());
        assert!(derive_reflow(&context, 1_700_000_400).is_none());
    }

    #[tokio::test]
    async fn apply_current_reflow_updates_commitment_schedule_when_proposal_is_actionable() {
        let storage = test_storage().await;
        let now_ts = day_start_ts(1_700_000_000) + (8 * 60 * 60);
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: now_ts - 600,
                drift_type: Some("morning_drift".to_string()),
                drift_severity: Some("medium".to_string()),
                attention_reasons: vec!["Morning slipped".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;
        seed_commitment(
            &storage,
            "Deep work @30m",
            None,
            Some("Project Atlas"),
            json!({ "labels": ["urgent", "block:focus"] }),
        )
        .await;
        seed_calendar_event(&storage, now_ts, now_ts + (30 * 60), "Standup").await;

        let status = apply_current_reflow(&storage, now_ts + 60, true)
            .await
            .expect("apply should update commitment schedule");

        assert_eq!(status.kind, CurrentContextReflowStatusKind::Applied);
        let thread_id = status.thread_id.as_ref().expect("thread id");
        let thread = storage
            .get_thread_by_id(thread_id)
            .await
            .unwrap()
            .expect("thread exists");
        let metadata: serde_json::Value =
            serde_json::from_str(&thread.4).expect("thread metadata should parse");
        assert_eq!(metadata["proposal_state"], "applied");
        assert_eq!(metadata["applied_via"], "commitment_scheduling_apply");

        let commitments = storage
            .list_commitments(Some(CommitmentStatus::Open), None, None, 8)
            .await
            .expect("list commitments");
        assert_eq!(commitments.len(), 1);
        assert!(commitments[0].due_at.is_some());
    }

    #[tokio::test]
    async fn apply_current_reflow_escalates_confirm_required_cases_to_threads() {
        let storage = test_storage().await;
        let now_ts = day_start_ts(1_700_000_000) + (8 * 60 * 60);
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: now_ts - 60,
                next_event_start_ts: Some(now_ts - (20 * 60)),
                leave_by_ts: Some(now_ts - (10 * 60)),
                attention_reasons: vec!["Standup slipped".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;
        seed_commitment(
            &storage,
            "Deep work @30m",
            None,
            Some("Project Atlas"),
            json!({ "labels": ["urgent", "block:focus"] }),
        )
        .await;
        seed_calendar_event(&storage, now_ts, now_ts + (30 * 60), "Standup").await;

        let status = apply_current_reflow(&storage, now_ts, true)
            .await
            .expect("confirm-required reflow should escalate");

        assert_eq!(status.kind, CurrentContextReflowStatusKind::Editing);
        let thread_id = status.thread_id.as_ref().expect("thread id");
        let thread = storage
            .get_thread_by_id(thread_id)
            .await
            .unwrap()
            .expect("thread exists");
        let metadata: serde_json::Value =
            serde_json::from_str(&thread.4).expect("thread metadata should parse");
        assert_eq!(thread.1, "reflow_edit");
        assert_eq!(metadata["proposal_state"], "staged");
        let commitments = storage
            .list_commitments(Some(CommitmentStatus::Open), None, None, 8)
            .await
            .expect("list commitments");
        assert_eq!(commitments.len(), 1);
        assert!(commitments[0].due_at.is_none());
    }

    #[tokio::test]
    async fn derive_current_reflow_recomputes_moved_tasks_from_remaining_windows() {
        let storage = test_storage().await;
        let now_ts = day_start_ts(1_700_000_000) + (8 * 60 * 60);
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: now_ts - 600,
                drift_type: Some("morning_drift".to_string()),
                drift_severity: Some("medium".to_string()),
                attention_reasons: vec!["Morning slipped".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;
        seed_commitment(
            &storage,
            "Deep work @30m",
            None,
            Some("Project Atlas"),
            json!({ "labels": ["urgent", "block:focus"] }),
        )
        .await;
        seed_calendar_event(&storage, now_ts, now_ts + (30 * 60), "Standup").await;

        let (_, context) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("current context");
        let card = derive_current_reflow(&storage, &context, now_ts + 60)
            .await
            .expect("reflow derivation")
            .expect("reflow card");
        let proposal = card.proposal.expect("proposal");

        assert_eq!(proposal.moved_count, 1);
        assert_eq!(proposal.unscheduled_count, 0);
        assert!(proposal
            .changes
            .iter()
            .any(|change| change.kind == ReflowChangeKind::Moved));
        assert!(proposal
            .rule_facets
            .iter()
            .any(|facet| facet.kind == ScheduleRuleFacetKind::BlockTarget));
    }

    #[tokio::test]
    async fn derive_current_reflow_marks_unscheduled_tasks_that_do_not_fit() {
        let storage = test_storage().await;
        let now_ts = day_end_ts(1_700_000_000) - (30 * 60);
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: now_ts - 3600,
                drift_type: Some("stale_schedule".to_string()),
                drift_severity: Some("high".to_string()),
                attention_reasons: vec!["Plan is stale".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;
        seed_commitment(
            &storage,
            "Write proposal @2h",
            None,
            Some("Project Atlas"),
            json!({ "labels": ["urgent", "cal:free"] }),
        )
        .await;

        let (_, context) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("current context");
        let card = derive_current_reflow(&storage, &context, now_ts)
            .await
            .expect("reflow derivation")
            .expect("reflow card");
        let proposal = card.proposal.expect("proposal");

        assert_eq!(proposal.unscheduled_count, 1);
        assert!(proposal
            .changes
            .iter()
            .any(|change| change.kind == ReflowChangeKind::Unscheduled));
    }

    #[tokio::test]
    async fn derive_current_reflow_respects_operator_declared_routine_blocks() {
        let storage = test_storage().await;
        let now_ts = 1_710_788_400;
        seed_profile(
            &storage,
            RoutinePlanningProfile {
                routine_blocks: vec![DurableRoutineBlock {
                    id: "routine_saved".to_string(),
                    label: "Saved focus block".to_string(),
                    source: RoutineBlockSourceKind::OperatorDeclared,
                    local_timezone: "UTC".to_string(),
                    start_local_time: "19:00".to_string(),
                    end_local_time: "20:00".to_string(),
                    days_of_week: vec![1],
                    protected: true,
                    active: true,
                }],
                planning_constraints: vec![],
            },
        )
        .await;
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: now_ts - 3600,
                drift_type: Some("stale_schedule".to_string()),
                drift_severity: Some("high".to_string()),
                attention_reasons: vec!["Plan is stale".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;
        seed_commitment(
            &storage,
            "Deep work @30m",
            None,
            Some("Project Atlas"),
            json!({ "labels": ["urgent"] }),
        )
        .await;

        let (_, context) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("current context");
        let card = derive_current_reflow(&storage, &context, now_ts)
            .await
            .expect("reflow derivation")
            .expect("reflow card");
        let proposal = card.proposal.expect("proposal");
        let moved = proposal
            .changes
            .iter()
            .find(|change| change.kind == ReflowChangeKind::Moved)
            .expect("moved change");

        assert!(moved.scheduled_start_ts.expect("scheduled start") >= now_ts + (60 * 60));
    }

    #[tokio::test]
    async fn derive_current_reflow_uses_overflow_judgment_constraint() {
        let storage = test_storage().await;
        let now_ts = day_end_ts(1_700_000_000) - (30 * 60);
        seed_profile(
            &storage,
            RoutinePlanningProfile {
                routine_blocks: vec![],
                planning_constraints: vec![PlanningConstraint {
                    id: "overflow".to_string(),
                    label: "Require judgment".to_string(),
                    kind: PlanningConstraintKind::RequireJudgmentForOverflow,
                    detail: None,
                    time_window: None,
                    minutes: None,
                    max_items: None,
                    active: true,
                }],
            },
        )
        .await;
        seed_context(
            &storage,
            CurrentContextV1 {
                computed_at: now_ts - 3600,
                drift_type: Some("stale_schedule".to_string()),
                drift_severity: Some("high".to_string()),
                attention_reasons: vec!["Plan is stale".to_string()],
                ..CurrentContextV1::default()
            },
        )
        .await;
        seed_commitment(
            &storage,
            "Write proposal @2h",
            None,
            Some("Project Atlas"),
            json!({ "labels": ["urgent", "cal:free"] }),
        )
        .await;

        let (_, context) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("current context");
        let card = derive_current_reflow(&storage, &context, now_ts)
            .await
            .expect("reflow derivation")
            .expect("reflow card");
        let proposal = card.proposal.expect("proposal");

        assert_eq!(proposal.needs_judgment_count, 1);
        assert!(proposal
            .changes
            .iter()
            .any(|change| change.kind == ReflowChangeKind::NeedsJudgment));
    }
}
