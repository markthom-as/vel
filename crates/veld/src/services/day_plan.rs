use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{
    CanonicalScheduleRules, Commitment, CommitmentStatus, CurrentContextV1, DayPlanChange,
    DayPlanChangeKind, DayPlanProposal, RoutineBlock, ScheduleRuleFacet, ScheduleRuleFacetKind,
    ScheduleTimeWindow,
};
use vel_storage::{SignalRecord, Storage};

use crate::errors::AppError;

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

pub async fn derive_current_day_plan(
    storage: &Storage,
    context: &CurrentContextV1,
    now_ts: i64,
) -> Result<Option<DayPlanProposal>, AppError> {
    let timezone = crate::services::timezone::resolve_timezone(storage).await?;
    let current_day = crate::services::timezone::current_day_window(
        &timezone,
        OffsetDateTime::from_unix_timestamp(now_ts).unwrap_or(OffsetDateTime::UNIX_EPOCH),
    )?;
    let commitments = storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, 128)
        .await?;
    let events = storage
        .list_signals_in_window(
            Some("calendar_event"),
            current_day.start_ts,
            current_day.end_ts,
            64,
        )
        .await?
        .into_iter()
        .filter(|signal| signal_overlaps_current_day(signal, &current_day))
        .collect::<Vec<_>>();

    let planning_inputs =
        crate::services::planning_profile::load_day_planning_inputs(storage, context, now_ts)
            .await?;
    let routine_blocks = planning_inputs.routine_blocks;
    let constraints = planning_inputs.planning_constraints;
    let tasks = collect_schedule_tasks(context, commitments, &current_day, &constraints);
    if tasks.is_empty() && routine_blocks.is_empty() {
        return Ok(None);
    }

    let mut working_windows =
        available_day_windows(&events, &routine_blocks, &constraints, now_ts, &current_day);
    let mut changes = Vec::new();
    let overflow_requires_judgment =
        crate::services::planning_profile::require_judgment_for_overflow(&constraints);
    let max_items = crate::services::planning_profile::max_scheduled_items(&constraints);
    let overflow_start = max_items.unwrap_or(tasks.len()).min(tasks.len());
    let mut tasks = tasks;
    let overflow_tasks = tasks.split_off(overflow_start);

    for task in tasks {
        if task.scheduler_rules.local_defer {
            changes.push(DayPlanChange {
                kind: DayPlanChangeKind::Deferred,
                commitment_id: Some(task.commitment_id.clone()),
                title: task.title.clone(),
                detail: format!(
                    "{} is marked for local defer and was left out of today's bounded plan.",
                    task.title
                ),
                project_label: task.project_label.clone(),
                scheduled_start_ts: None,
                rule_facets: rule_facets_for_task(&task),
            });
            continue;
        }

        if let Some(fixed_start_ts) = task.fixed_start_ts {
            if window_can_fit_fixed(
                working_windows.as_slice(),
                fixed_start_ts,
                task.duration_minutes,
            ) {
                reserve_fixed_window(&mut working_windows, fixed_start_ts, task.duration_minutes);
                changes.push(DayPlanChange {
                    kind: DayPlanChangeKind::Scheduled,
                    commitment_id: Some(task.commitment_id.clone()),
                    title: task.title.clone(),
                    detail: format!(
                        "{} is anchored to a fixed time in today's plan.",
                        task.title
                    ),
                    project_label: task.project_label.clone(),
                    scheduled_start_ts: Some(fixed_start_ts),
                    rule_facets: rule_facets_for_task(&task),
                });
            } else {
                changes.push(DayPlanChange {
                    kind: DayPlanChangeKind::NeedsJudgment,
                    commitment_id: Some(task.commitment_id.clone()),
                    title: task.title.clone(),
                    detail: format!(
                        "{} is anchored to a fixed time that conflicts with today's remaining windows.",
                        task.title
                    ),
                    project_label: task.project_label.clone(),
                    scheduled_start_ts: Some(fixed_start_ts),
                    rule_facets: rule_facets_for_task(&task),
                });
            }
            continue;
        }

        if let Some(slot_start_ts) =
            reserve_window_for_task(&mut working_windows, &task, now_ts, &current_day)
        {
            changes.push(DayPlanChange {
                kind: DayPlanChangeKind::Scheduled,
                commitment_id: Some(task.commitment_id.clone()),
                title: task.title.clone(),
                detail: format!("{} fits in the next bounded slot for today.", task.title),
                project_label: task.project_label.clone(),
                scheduled_start_ts: Some(slot_start_ts),
                rule_facets: rule_facets_for_task(&task),
            });
        } else {
            changes.push(DayPlanChange {
                kind: if overflow_requires_judgment {
                    DayPlanChangeKind::NeedsJudgment
                } else {
                    DayPlanChangeKind::DidNotFit
                },
                commitment_id: Some(task.commitment_id.clone()),
                title: task.title.clone(),
                detail: format!(
                    "{} did not fit in today's remaining bounded windows.",
                    task.title
                ),
                project_label: task.project_label.clone(),
                scheduled_start_ts: None,
                rule_facets: rule_facets_for_task(&task),
            });
        }
    }

    for task in overflow_tasks {
        changes.push(DayPlanChange {
            kind: if overflow_requires_judgment {
                DayPlanChangeKind::NeedsJudgment
            } else {
                DayPlanChangeKind::Deferred
            },
            commitment_id: Some(task.commitment_id.clone()),
            title: task.title.clone(),
            detail: format!(
                "{} was left out of today's bounded plan because the operator cap on scheduled items was reached.",
                task.title
            ),
            project_label: task.project_label.clone(),
            scheduled_start_ts: None,
            rule_facets: rule_facets_for_task(&task),
        });
    }

    let scheduled_count = changes
        .iter()
        .filter(|change| change.kind == DayPlanChangeKind::Scheduled)
        .count() as u32;
    let deferred_count = changes
        .iter()
        .filter(|change| change.kind == DayPlanChangeKind::Deferred)
        .count() as u32;
    let did_not_fit_count = changes
        .iter()
        .filter(|change| change.kind == DayPlanChangeKind::DidNotFit)
        .count() as u32;
    let needs_judgment_count = changes
        .iter()
        .filter(|change| change.kind == DayPlanChangeKind::NeedsJudgment)
        .count() as u32;

    let summary = match (
        scheduled_count,
        deferred_count,
        did_not_fit_count,
        needs_judgment_count,
    ) {
        (_, _, _, count) if count > 0 => {
            "Vel shaped today's plan and found at least one anchored item that still needs judgment."
                .to_string()
        }
        (_, _, count, _) if count > 0 => {
            "Vel shaped today's plan and found work that still does not fit.".to_string()
        }
        (_, count, _, _) if count > 0 => {
            "Vel shaped today's plan and deferred lower-priority work explicitly.".to_string()
        }
        _ => "Vel shaped a bounded same-day plan from current commitments, calendar anchors, and routine blocks."
            .to_string(),
    };

    Ok(Some(DayPlanProposal {
        headline: "Today has a bounded plan".to_string(),
        summary,
        scheduled_count,
        deferred_count,
        did_not_fit_count,
        needs_judgment_count,
        changes,
        routine_blocks,
    }))
}

fn collect_schedule_tasks(
    context: &CurrentContextV1,
    commitments: Vec<Commitment>,
    current_day: &crate::services::timezone::CurrentDayWindow,
    constraints: &[vel_core::PlanningConstraint],
) -> Vec<ScheduleTask> {
    let mut tasks = commitments
        .into_iter()
        .filter_map(|commitment| {
            schedule_task_from_commitment(context, commitment, current_day, constraints)
        })
        .collect::<Vec<_>>();
    tasks.sort_by_key(|task| {
        (
            task.fixed_start_ts.unwrap_or(i64::MAX),
            !task.scheduler_rules.local_urgency,
            task.due_ts.unwrap_or(i64::MAX),
            task.duration_minutes,
        )
    });
    tasks.truncate(8);
    tasks
}

fn schedule_task_from_commitment(
    context: &CurrentContextV1,
    commitment: Commitment,
    current_day: &crate::services::timezone::CurrentDayWindow,
    constraints: &[vel_core::PlanningConstraint],
) -> Option<ScheduleTask> {
    let due_ts = commitment.due_at.map(|value| value.unix_timestamp());
    let relevant = context
        .next_commitment_id
        .as_ref()
        .map(|id| id == commitment.id.as_ref())
        .unwrap_or(false)
        || due_ts
            .map(|value| value >= current_day.start_ts && value < current_day.end_ts)
            .unwrap_or(false)
        || context
            .commitments_used
            .iter()
            .any(|id| id == commitment.id.as_ref())
        || commitment.scheduler_rules().local_urgency
        || commitment.scheduler_rules().local_defer;
    if !relevant {
        return None;
    }

    let mut scheduler_rules = commitment.scheduler_rules();
    if scheduler_rules.time_window.is_none() {
        scheduler_rules.time_window =
            crate::services::planning_profile::default_time_window(constraints);
    }
    Some(ScheduleTask {
        commitment_id: commitment.id.to_string(),
        title: commitment.text,
        project_label: commitment.project,
        duration_minutes: scheduler_rules.duration_minutes.unwrap_or(30),
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

fn available_day_windows(
    events: &[SignalRecord],
    routine_blocks: &[RoutineBlock],
    constraints: &[vel_core::PlanningConstraint],
    now_ts: i64,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> Vec<ScheduleWindow> {
    let mut windows = vec![ScheduleWindow {
        start_ts: now_ts,
        end_ts: current_day.end_ts,
    }];
    for event in events
        .iter()
        .filter_map(|signal| calendar_event_block(signal, constraints))
    {
        windows = subtract_block(windows, event);
    }
    for routine in routine_blocks.iter().filter(|block| block.protected) {
        windows = subtract_block(
            windows,
            ScheduleWindow {
                start_ts: routine.start_ts,
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
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> Option<i64> {
    let duration_seconds = task.duration_minutes * 60;
    let window_bounds =
        preferred_window_bounds(task.scheduler_rules.time_window, now_ts, current_day);
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

fn reserve_fixed_window(
    windows: &mut Vec<ScheduleWindow>,
    fixed_start_ts: i64,
    duration_minutes: i64,
) {
    let block = ScheduleWindow {
        start_ts: fixed_start_ts,
        end_ts: fixed_start_ts + (duration_minutes * 60),
    };
    *windows = subtract_block(std::mem::take(windows), block);
}

fn preferred_window_bounds(
    time_window: Option<ScheduleTimeWindow>,
    now_ts: i64,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> (i64, i64) {
    let prenoon_start = current_day.start_ts + (2 * 60 * 60);
    let afternoon_start = current_day.start_ts + (8 * 60 * 60);
    let evening_start = current_day.start_ts + (13 * 60 * 60);
    let night_start = current_day.start_ts + (17 * 60 * 60);
    match time_window {
        Some(ScheduleTimeWindow::Prenoon) => (prenoon_start, afternoon_start),
        Some(ScheduleTimeWindow::Afternoon) => (afternoon_start, evening_start),
        Some(ScheduleTimeWindow::Evening) => (evening_start, night_start),
        Some(ScheduleTimeWindow::Night) => (night_start, current_day.end_ts),
        Some(ScheduleTimeWindow::Day) => (prenoon_start, current_day.end_ts),
        _ => (now_ts, current_day.end_ts),
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

fn json_i64(value: &JsonValue, key: &str) -> Option<i64> {
    value.get(key).and_then(|entry| {
        entry
            .as_i64()
            .or_else(|| entry.as_str().and_then(|text| text.parse::<i64>().ok()))
    })
}

fn signal_overlaps_current_day(
    signal: &SignalRecord,
    current_day: &crate::services::timezone::CurrentDayWindow,
) -> bool {
    let payload = &signal.payload_json;
    let Some(start_ts) = json_i64(payload, "start").or_else(|| json_i64(payload, "start_ts"))
    else {
        return false;
    };
    let end_ts = json_i64(payload, "end")
        .or_else(|| json_i64(payload, "end_ts"))
        .unwrap_or(start_ts);
    start_ts < current_day.end_ts && end_ts >= current_day.start_ts
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use time::{macros::datetime, OffsetDateTime};
    use vel_core::{
        CurrentContextV1, DurableRoutineBlock, PlanningConstraint, PlanningConstraintKind,
        RoutineBlockSourceKind, RoutinePlanningProfile, ScheduleRuleFacetKind, ScheduleTimeWindow,
    };
    use vel_storage::{CommitmentInsert, Storage};

    use super::derive_current_day_plan;

    async fn test_storage() -> Storage {
        let storage = Storage::connect("sqlite::memory:")
            .await
            .expect("storage opens");
        storage.migrate().await.expect("storage migrates");
        storage
            .set_setting("timezone", &serde_json::json!("America/Denver"))
            .await
            .expect("timezone should save");
        storage
    }

    async fn seed_commitment(
        storage: &Storage,
        text: &str,
        due_at_ts: Option<i64>,
        project: Option<&str>,
        metadata: serde_json::Value,
    ) {
        storage
            .insert_commitment(CommitmentInsert {
                text: text.to_string(),
                source_type: "manual".to_string(),
                source_id: "manual_test".to_string(),
                status: vel_core::CommitmentStatus::Open,
                due_at: due_at_ts.and_then(|ts| OffsetDateTime::from_unix_timestamp(ts).ok()),
                project: project.map(str::to_string),
                commitment_kind: None,
                metadata_json: Some(metadata),
            })
            .await
            .expect("commitment should insert");
    }

    async fn seed_calendar_event(storage: &Storage, start_ts: i64, end_ts: i64, title: &str) {
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: Some(format!("google_calendar:test:{}", start_ts)),
                timestamp: start_ts,
                payload_json: Some(json!({
                    "title": title,
                    "start": start_ts,
                    "end": end_ts,
                })),
            })
            .await
            .expect("calendar event should insert");
    }

    async fn seed_profile(storage: &Storage, profile: RoutinePlanningProfile) {
        crate::services::planning_profile::save_routine_planning_profile(storage, &profile)
            .await
            .expect("profile should save");
    }

    #[tokio::test]
    async fn derives_day_plan_with_scheduled_and_routine_block() {
        let storage = test_storage().await;
        let now_ts = datetime!(2026-03-16 14:00:00 UTC).unix_timestamp();
        seed_commitment(
            &storage,
            "Deep work @30m",
            None,
            Some("Project Atlas"),
            json!({ "labels": ["urgent", "block:focus", "time:prenoon"] }),
        )
        .await;
        seed_calendar_event(&storage, now_ts + (60 * 60), now_ts + (90 * 60), "Standup").await;

        let plan = derive_current_day_plan(
            &storage,
            &CurrentContextV1 {
                computed_at: now_ts,
                mode: "morning_mode".to_string(),
                morning_state: "engaged".to_string(),
                prep_window_active: true,
                commitments_used: vec![],
                ..CurrentContextV1::default()
            },
            now_ts,
        )
        .await
        .expect("day plan derivation")
        .expect("day plan");

        assert_eq!(plan.scheduled_count, 1);
        assert_eq!(plan.routine_blocks.len(), 1);
        assert!(plan
            .changes
            .iter()
            .any(|change| change.kind == vel_core::DayPlanChangeKind::Scheduled));
    }

    #[tokio::test]
    async fn derives_day_plan_with_deferred_and_did_not_fit_outcomes() {
        let storage = test_storage().await;
        let now_ts = datetime!(2026-03-17 09:30:00 UTC).unix_timestamp();
        seed_commitment(
            &storage,
            "Backlog cleanup @30m",
            None,
            None,
            json!({ "labels": ["@defer"] }),
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

        let plan = derive_current_day_plan(&storage, &CurrentContextV1::default(), now_ts)
            .await
            .expect("day plan derivation")
            .expect("day plan");

        assert_eq!(plan.deferred_count, 1);
        assert_eq!(plan.did_not_fit_count, 1);
    }

    #[tokio::test]
    async fn derives_day_plan_with_fixed_start_judgment_when_anchor_conflicts() {
        let storage = test_storage().await;
        let now_ts = 1_700_000_000 + (8 * 60 * 60);
        let fixed_start = now_ts + (30 * 60);
        seed_commitment(
            &storage,
            "Call accountant",
            Some(fixed_start),
            None,
            json!({ "labels": ["fixed_start"] }),
        )
        .await;
        seed_calendar_event(
            &storage,
            fixed_start - (10 * 60),
            fixed_start + (50 * 60),
            "Overlapping event",
        )
        .await;

        let plan = derive_current_day_plan(&storage, &CurrentContextV1::default(), now_ts)
            .await
            .expect("day plan derivation")
            .expect("day plan");

        assert_eq!(plan.needs_judgment_count, 1);
        assert!(plan
            .changes
            .iter()
            .any(|change| change.kind == vel_core::DayPlanChangeKind::NeedsJudgment));
    }

    #[tokio::test]
    async fn derives_day_plan_from_operator_declared_routine_blocks_before_inference() {
        let storage = test_storage().await;
        let now_ts = 1_710_788_400;
        seed_profile(
            &storage,
            RoutinePlanningProfile {
                routine_blocks: vec![DurableRoutineBlock {
                    id: "routine_saved".to_string(),
                    label: "Saved focus block".to_string(),
                    source: RoutineBlockSourceKind::OperatorDeclared,
                    local_timezone: "America/Denver".to_string(),
                    start_local_time: "09:00".to_string(),
                    end_local_time: "10:30".to_string(),
                    days_of_week: vec![1],
                    protected: true,
                    active: true,
                }],
                planning_constraints: vec![],
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

        let plan = derive_current_day_plan(
            &storage,
            &CurrentContextV1 {
                computed_at: now_ts,
                mode: "morning_mode".to_string(),
                morning_state: "engaged".to_string(),
                prep_window_active: true,
                ..CurrentContextV1::default()
            },
            now_ts,
        )
        .await
        .expect("day plan derivation")
        .expect("day plan");

        assert_eq!(plan.routine_blocks.len(), 1);
        assert_eq!(plan.routine_blocks[0].label, "Saved focus block");
        assert_eq!(
            plan.routine_blocks[0].source,
            RoutineBlockSourceKind::OperatorDeclared
        );
    }

    #[tokio::test]
    async fn derives_day_plan_with_durable_constraints_for_default_window_and_cap() {
        let storage = test_storage().await;
        let now_ts = 1_710_788_400;
        seed_profile(
            &storage,
            RoutinePlanningProfile {
                routine_blocks: vec![],
                planning_constraints: vec![
                    PlanningConstraint {
                        id: "default_window".to_string(),
                        label: "Prefer prenoon".to_string(),
                        kind: PlanningConstraintKind::DefaultTimeWindow,
                        detail: None,
                        time_window: Some(ScheduleTimeWindow::Prenoon),
                        minutes: None,
                        max_items: None,
                        active: true,
                    },
                    PlanningConstraint {
                        id: "cap".to_string(),
                        label: "Cap to one".to_string(),
                        kind: PlanningConstraintKind::MaxScheduledItems,
                        detail: None,
                        time_window: None,
                        minutes: None,
                        max_items: Some(1),
                        active: true,
                    },
                ],
            },
        )
        .await;
        seed_commitment(
            &storage,
            "Write review @30m",
            Some(now_ts + (60 * 60)),
            Some("Ops"),
            json!({ "labels": ["urgent"] }),
        )
        .await;
        seed_commitment(
            &storage,
            "Inbox cleanup @30m",
            Some(now_ts + (2 * 60 * 60)),
            None,
            json!({ "labels": ["urgent"] }),
        )
        .await;

        let plan = derive_current_day_plan(&storage, &CurrentContextV1::default(), now_ts)
            .await
            .expect("day plan derivation")
            .expect("day plan");

        assert_eq!(plan.scheduled_count, 1);
        assert_eq!(plan.deferred_count, 1);
        assert!(plan
            .changes
            .iter()
            .any(|change| change.kind == vel_core::DayPlanChangeKind::Deferred));
        assert!(plan
            .changes
            .iter()
            .flat_map(|change| change.rule_facets.iter())
            .any(|facet| facet.kind == ScheduleRuleFacetKind::TimeWindow));
    }
}
