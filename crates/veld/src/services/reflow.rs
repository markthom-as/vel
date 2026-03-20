use serde_json::json;
use uuid::Uuid;
use vel_core::{
    ActionItemId, CheckInEscalationTarget, CurrentContextReflowStatus,
    CurrentContextReflowStatusKind, CurrentContextV1, ReflowAcceptMode, ReflowCard,
    ReflowEditTarget, ReflowSeverity, ReflowTransition, ReflowTransitionKind,
    ReflowTransitionTargetKind, ReflowTriggerKind,
};
use vel_storage::Storage;

use crate::errors::AppError;

struct ReflowCandidate {
    trigger: ReflowTriggerKind,
    severity: ReflowSeverity,
    summary: String,
    preview_lines: Vec<String>,
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
    let card = build_card_from_candidate(&context, candidate)
        .ok_or_else(|| AppError::not_found("reflow not available"))?;
    if card.accept_mode == ReflowAcceptMode::ConfirmRequired && !confirmed {
        return Err(AppError::bad_request("reflow confirmation required"));
    }

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
        thread_id: None,
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
    let card = build_card_from_candidate(&context, candidate)
        .ok_or_else(|| AppError::not_found("reflow not available"))?;
    let thread_id = format!("thr_{}", Uuid::new_v4().simple());
    let metadata = json!({
        "source": "reflow",
        "resolution_state": "editing",
        "context_computed_at": context.computed_at,
        "trigger": card.trigger.to_string(),
        "severity": card.severity.to_string(),
        "summary": card.summary,
        "preview_lines": card.preview_lines,
    })
    .to_string();
    storage
        .insert_thread(&thread_id, "reflow_edit", "Reflow edit", "open", &metadata)
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
        transitions: Vec::new(),
    };
    card.transitions = transitions_for_card(&card);
    Some(card)
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
    use vel_storage::Storage;

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
}
