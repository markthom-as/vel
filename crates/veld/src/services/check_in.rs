use time::OffsetDateTime;
use vel_core::{
    ActionItemId, CheckInCard, CheckInEscalation, CheckInEscalationTarget, CheckInSourceKind,
    CheckInSubmitTarget, CheckInSubmitTargetKind, CheckInTransition, CheckInTransitionKind,
    CheckInTransitionTargetKind, DailyLoopCheckInResolution, DailyLoopCheckInResolutionKind,
    DailyLoopPhase, DailyLoopSession, DailyLoopTurnAction, DailyLoopTurnRequest,
};
use vel_storage::Storage;

use crate::{
    errors::AppError,
    services::{daily_loop, timezone::ResolvedTimeZone},
};

pub async fn get_current_check_in(
    storage: &Storage,
    timezone: &ResolvedTimeZone,
) -> Result<Option<CheckInCard>, AppError> {
    let session_date =
        crate::services::timezone::current_day_date_string(timezone, OffsetDateTime::now_utc())?;

    for phase in [DailyLoopPhase::Standup, DailyLoopPhase::MorningOverview] {
        if let Some(session) = daily_loop::get_active_session(storage, &session_date, phase).await?
        {
            if let Some(card) = card_from_daily_loop_session(storage, session).await? {
                return Ok(Some(card));
            }
        }
    }

    Ok(None)
}

fn escalation_thread_id(session_id: &str, prompt_id: &str) -> String {
    format!("thr_check_in_{}_{}", session_id, prompt_id)
}

async fn ensure_follow_through_thread(
    storage: &Storage,
    session_id: &str,
    phase: DailyLoopPhase,
    prompt: &vel_core::DailyLoopPrompt,
) -> Result<String, AppError> {
    let thread_id = escalation_thread_id(session_id, &prompt.prompt_id);
    if storage.get_thread_by_id(&thread_id).await?.is_none() {
        let metadata = serde_json::json!({
            "source": "check_in",
            "resolution_state": "pending",
            "phase": match phase {
                DailyLoopPhase::MorningOverview => "morning_overview",
                DailyLoopPhase::Standup => "standup",
            },
            "session_id": session_id,
            "prompt_id": prompt.prompt_id,
            "prompt_text": prompt.text,
            "ordinal": prompt.ordinal,
        })
        .to_string();
        storage
            .insert_thread(
                &thread_id,
                "daily_loop_check_in",
                "Check-in follow-through",
                "open",
                &metadata,
            )
            .await?;
        let _ = storage
            .insert_thread_link(
                &thread_id,
                "daily_loop_session",
                session_id,
                "follow_through",
            )
            .await?;
    }
    Ok(thread_id)
}

async fn update_follow_through_thread_status(
    storage: &Storage,
    session: &DailyLoopSession,
    resolution: &DailyLoopCheckInResolution,
) -> Result<(), AppError> {
    let thread_id = escalation_thread_id(session.id.as_ref(), &resolution.prompt_id);
    if storage.get_thread_by_id(&thread_id).await?.is_none() {
        return Ok(());
    }
    let status = match resolution.kind {
        DailyLoopCheckInResolutionKind::Submitted => "resolved",
        DailyLoopCheckInResolutionKind::Bypassed => "deferred",
    };
    storage.update_thread_status(&thread_id, status).await?;
    Ok(())
}

pub fn prepare_turn_request(
    session: &DailyLoopSession,
    request: DailyLoopTurnRequest,
) -> Result<(DailyLoopTurnRequest, Option<DailyLoopCheckInResolution>), AppError> {
    let Some(prompt) = session.current_prompt.as_ref() else {
        return Ok((request, None));
    };

    match request.action {
        DailyLoopTurnAction::Submit => {
            let response_text = request
                .response_text
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string)
                .ok_or_else(|| {
                    AppError::bad_request("check-in submit requires a non-empty response")
                })?;
            Ok((
                DailyLoopTurnRequest {
                    response_text: Some(response_text.clone()),
                    ..request
                },
                Some(DailyLoopCheckInResolution {
                    prompt_id: prompt.prompt_id.clone(),
                    ordinal: prompt.ordinal,
                    kind: DailyLoopCheckInResolutionKind::Submitted,
                    response_text: Some(response_text),
                    note_text: None,
                }),
            ))
        }
        DailyLoopTurnAction::Skip => {
            if !prompt.allow_skip {
                return Err(AppError::bad_request("current check-in cannot be bypassed"));
            }
            let note_text = request
                .response_text
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string)
                .ok_or_else(|| {
                    AppError::bad_request("check-in bypass requires a short operator note")
                })?;
            Ok((
                DailyLoopTurnRequest {
                    response_text: None,
                    ..request
                },
                Some(DailyLoopCheckInResolution {
                    prompt_id: prompt.prompt_id.clone(),
                    ordinal: prompt.ordinal,
                    kind: DailyLoopCheckInResolutionKind::Bypassed,
                    response_text: None,
                    note_text: Some(note_text),
                }),
            ))
        }
        DailyLoopTurnAction::Resume => Ok((request, None)),
    }
}

pub async fn persist_resolution_follow_through(
    storage: &Storage,
    session: &DailyLoopSession,
    resolution: Option<&DailyLoopCheckInResolution>,
) -> Result<(), AppError> {
    if let Some(resolution) = resolution {
        update_follow_through_thread_status(storage, session, resolution).await?;
    }
    Ok(())
}

pub fn commitment_action_labels_for_card(card: &CheckInCard) -> Vec<&'static str> {
    let mut actions = vec!["accept"];
    if card.allow_skip {
        actions.push("defer");
    }
    if card.escalation.is_some() {
        actions.push("choose");
    }
    actions.push("close");
    actions
}

pub fn transitions_for_card(card: &CheckInCard) -> Vec<CheckInTransition> {
    let mut transitions = vec![CheckInTransition {
        kind: CheckInTransitionKind::Submit,
        label: card
            .suggested_action_label
            .clone()
            .unwrap_or_else(|| "Continue".to_string()),
        target: CheckInTransitionTargetKind::DailyLoopTurn,
        reference_id: Some(card.submit_target.reference_id.clone()),
        requires_response: true,
        requires_note: false,
    }];

    if card.allow_skip {
        transitions.push(CheckInTransition {
            kind: CheckInTransitionKind::Bypass,
            label: "Skip for now".to_string(),
            target: CheckInTransitionTargetKind::DailyLoopTurn,
            reference_id: Some(card.submit_target.reference_id.clone()),
            requires_response: false,
            requires_note: true,
        });
    }

    if let Some(escalation) = &card.escalation {
        transitions.push(CheckInTransition {
            kind: CheckInTransitionKind::Escalate,
            label: escalation.label.clone(),
            target: CheckInTransitionTargetKind::Threads,
            reference_id: escalation.thread_id.clone(),
            requires_response: false,
            requires_note: false,
        });
    }

    transitions
}

async fn card_from_daily_loop_session(
    storage: &Storage,
    session: DailyLoopSession,
) -> Result<Option<CheckInCard>, AppError> {
    let continuity_summary = daily_loop::session_continuity_summary(&session);
    let DailyLoopSession {
        id,
        phase,
        current_prompt,
        ..
    } = session;
    let Some(prompt) = current_prompt else {
        return Ok(None);
    };
    let thread_id = ensure_follow_through_thread(storage, id.as_ref(), phase, &prompt).await?;
    let (title, suggested_action_label, suggested_response) = match phase {
        DailyLoopPhase::MorningOverview => (
            "Morning check-in".to_string(),
            Some("Continue morning overview".to_string()),
            None,
        ),
        DailyLoopPhase::Standup => (
            "Standup check-in".to_string(),
            Some("Continue standup".to_string()),
            None,
        ),
    };

    let mut card = CheckInCard {
        id: ActionItemId::from(format!("act_check_in_{}_{}", id, prompt.prompt_id)),
        source_kind: CheckInSourceKind::DailyLoop,
        phase,
        session_id: id.to_string(),
        title,
        summary: continuity_summary,
        prompt_id: prompt.prompt_id,
        prompt_text: prompt.text,
        suggested_action_label,
        suggested_response,
        allow_skip: prompt.allow_skip,
        blocking: true,
        submit_target: CheckInSubmitTarget {
            kind: CheckInSubmitTargetKind::DailyLoopTurn,
            reference_id: id.to_string(),
        },
        escalation: Some(CheckInEscalation {
            target: CheckInEscalationTarget::Threads,
            label: "Continue in Threads".to_string(),
            thread_id: Some(thread_id.clone()),
        }),
        transitions: Vec::new(),
    };
    card.transitions = transitions_for_card(&card);
    Ok(Some(card))
}

#[cfg(test)]
mod tests {
    use super::*;
    use vel_core::{
        DailyLoopPhase, DailyLoopPrompt, DailyLoopPromptKind, DailyLoopSession,
        DailyLoopSessionState, DailyLoopStartMetadata, DailyLoopStartSource, DailyLoopStatus,
        DailyLoopSurface, DailyLoopTurnAction, DailyLoopTurnRequest, DailyLoopTurnState,
        DailyStandupOutcome,
    };

    #[tokio::test]
    async fn builds_check_in_card_from_daily_loop_prompt() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let session = DailyLoopSession {
            id: "dls_test".to_string().into(),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhase::Standup,
            status: DailyLoopStatus::WaitingForInput,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Web,
            },
            turn_state: DailyLoopTurnState::WaitingForInput,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "standup_prompt_1".to_string(),
                kind: DailyLoopPromptKind::CommitmentReduction,
                text: "Name the one to three commitments that matter most today.".to_string(),
                ordinal: 1,
                allow_skip: true,
            }),
            state: DailyLoopSessionState::Standup(DailyStandupOutcome {
                commitments: vec![],
                deferred_tasks: vec![],
                confirmed_calendar: vec![],
                focus_blocks: vec![],
                check_in_history: vec![],
            }),
            outcome: None,
        };

        let card = card_from_daily_loop_session(&storage, session)
            .await
            .expect("card should build")
            .expect("card should exist");

        assert_eq!(card.phase, DailyLoopPhase::Standup);
        assert_eq!(card.submit_target.reference_id, "dls_test");
        let escalation = card.escalation.as_ref().expect("escalation");
        assert_eq!(escalation.label, "Continue in Threads");
        assert_eq!(
            escalation.thread_id.as_deref(),
            Some("thr_check_in_dls_test_standup_prompt_1")
        );
        assert_eq!(card.id.as_ref(), "act_check_in_dls_test_standup_prompt_1");
        assert_eq!(card.transitions.len(), 3);
        assert_eq!(card.transitions[0].kind, CheckInTransitionKind::Submit);
        assert_eq!(card.transitions[1].kind, CheckInTransitionKind::Bypass);
        assert_eq!(card.transitions[2].kind, CheckInTransitionKind::Escalate);
        assert_eq!(
            card.transitions[2].reference_id.as_deref(),
            Some("thr_check_in_dls_test_standup_prompt_1")
        );
        assert_eq!(
            commitment_action_labels_for_card(&card),
            vec!["accept", "defer", "choose", "close"]
        );
        assert!(card.summary.contains("Standup is waiting on question 1"));
    }

    #[test]
    fn submit_check_in_requires_non_empty_response() {
        let session = DailyLoopSession {
            id: "dls_test".to_string().into(),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhase::Standup,
            status: DailyLoopStatus::WaitingForInput,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Web,
            },
            turn_state: DailyLoopTurnState::WaitingForInput,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "standup_prompt_1".to_string(),
                kind: DailyLoopPromptKind::CommitmentReduction,
                text: "Name the one to three commitments that matter most today.".to_string(),
                ordinal: 1,
                allow_skip: true,
            }),
            state: DailyLoopSessionState::Standup(DailyStandupOutcome {
                commitments: vec![],
                deferred_tasks: vec![],
                confirmed_calendar: vec![],
                focus_blocks: vec![],
                check_in_history: vec![],
            }),
            outcome: None,
        };

        let result = prepare_turn_request(
            &session,
            DailyLoopTurnRequest {
                session_id: session.id.clone(),
                action: DailyLoopTurnAction::Submit,
                response_text: Some("   ".to_string()),
            },
        );

        assert!(result.is_err());
    }

    #[test]
    fn bypass_check_in_requires_note_and_emits_resolution() {
        let session = DailyLoopSession {
            id: "dls_test".to_string().into(),
            session_date: "2026-03-19".to_string(),
            phase: DailyLoopPhase::Standup,
            status: DailyLoopStatus::WaitingForInput,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Web,
            },
            turn_state: DailyLoopTurnState::WaitingForInput,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "standup_prompt_1".to_string(),
                kind: DailyLoopPromptKind::CommitmentReduction,
                text: "Name the one to three commitments that matter most today.".to_string(),
                ordinal: 1,
                allow_skip: true,
            }),
            state: DailyLoopSessionState::Standup(DailyStandupOutcome {
                commitments: vec![],
                deferred_tasks: vec![],
                confirmed_calendar: vec![],
                focus_blocks: vec![],
                check_in_history: vec![],
            }),
            outcome: None,
        };

        let (request, resolution) = prepare_turn_request(
            &session,
            DailyLoopTurnRequest {
                session_id: session.id.clone(),
                action: DailyLoopTurnAction::Skip,
                response_text: Some("Need to sort scope first".to_string()),
            },
        )
        .expect("bypass should validate");

        assert!(request.response_text.is_none());
        let resolution = resolution.expect("resolution should exist");
        assert_eq!(resolution.kind, DailyLoopCheckInResolutionKind::Bypassed);
        assert_eq!(
            resolution.note_text.as_deref(),
            Some("Need to sort scope first")
        );
    }
}
