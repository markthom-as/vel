use time::OffsetDateTime;
use vel_core::{
    ActionItemId, CheckInCard, CheckInEscalation, CheckInEscalationTarget, CheckInSourceKind,
    CheckInSubmitTarget, CheckInSubmitTargetKind, DailyLoopPhase, DailyLoopSession,
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
        crate::services::timezone::local_date_string(timezone, OffsetDateTime::now_utc());

    for phase in [DailyLoopPhase::Standup, DailyLoopPhase::MorningOverview] {
        if let Some(session) = daily_loop::get_active_session(storage, &session_date, phase).await?
        {
            if let Some(card) = card_from_daily_loop_session(session) {
                return Ok(Some(card));
            }
        }
    }

    Ok(None)
}

fn card_from_daily_loop_session(session: DailyLoopSession) -> Option<CheckInCard> {
    let DailyLoopSession {
        id,
        phase,
        current_prompt,
        ..
    } = session;
    let prompt = current_prompt?;
    let (title, summary, suggested_action_label, suggested_response) = match phase {
        DailyLoopPhase::MorningOverview => (
            "Morning check-in".to_string(),
            "Vel needs one short answer before the morning overview can continue.".to_string(),
            Some("Continue morning overview".to_string()),
            None,
        ),
        DailyLoopPhase::Standup => (
            "Standup check-in".to_string(),
            "Vel needs one short answer before the standup can continue.".to_string(),
            Some("Continue standup".to_string()),
            None,
        ),
    };

    Some(CheckInCard {
        id: ActionItemId::from(format!("act_check_in_{}_{}", id, prompt.prompt_id)),
        source_kind: CheckInSourceKind::DailyLoop,
        phase,
        session_id: id.to_string(),
        title,
        summary,
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
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use vel_core::{
        DailyLoopPhase, DailyLoopPrompt, DailyLoopPromptKind, DailyLoopSession,
        DailyLoopSessionState, DailyLoopStartMetadata, DailyLoopStartSource, DailyLoopStatus,
        DailyLoopSurface, DailyLoopTurnState, DailyStandupOutcome,
    };

    #[test]
    fn builds_check_in_card_from_daily_loop_prompt() {
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
            }),
            outcome: None,
        };

        let card = card_from_daily_loop_session(session).expect("card should build");

        assert_eq!(card.phase, DailyLoopPhase::Standup);
        assert_eq!(card.submit_target.reference_id, "dls_test");
        assert_eq!(card.escalation.unwrap().label, "Continue in Threads");
        assert_eq!(card.id.as_ref(), "act_check_in_dls_test_standup_prompt_1");
    }
}
