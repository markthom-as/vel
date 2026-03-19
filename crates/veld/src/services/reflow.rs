use vel_core::{
    ActionItemId, CheckInEscalationTarget, CurrentContextV1, ReflowAcceptMode, ReflowCard,
    ReflowEditTarget, ReflowSeverity, ReflowTransition, ReflowTransitionKind,
    ReflowTransitionTargetKind, ReflowTriggerKind,
};

pub fn derive_reflow(context: &CurrentContextV1, now_ts: i64) -> Option<ReflowCard> {
    let candidate = if now_ts - context.computed_at > 30 * 60 {
        Some((
            ReflowTriggerKind::StaleSchedule,
            ReflowSeverity::High,
            "Vel's day plan is stale enough that the current schedule may no longer be trustworthy."
                .to_string(),
            preview_for_stale_schedule(context, now_ts),
        ))
    } else if let Some(next_event_start_ts) = context.next_event_start_ts {
        if next_event_start_ts < now_ts - 15 * 60 {
            Some((
                ReflowTriggerKind::MissedEvent,
                ReflowSeverity::Critical,
                "A scheduled event appears to have slipped past without the plan being updated."
                    .to_string(),
                preview_for_missed_event(context, next_event_start_ts, now_ts),
            ))
        } else {
            None
        }
    } else {
        drift_candidate(context)
    }?;

    let (trigger, severity, summary, preview_lines) = candidate;
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

fn drift_candidate(
    context: &CurrentContextV1,
) -> Option<(ReflowTriggerKind, ReflowSeverity, String, Vec<String>)> {
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
    Some((trigger, severity, summary, preview_lines))
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
}
