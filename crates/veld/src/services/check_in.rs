use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use vel_core::{
    ActionItemId, CheckInCard, CheckInEscalation, CheckInEscalationTarget, CheckInSourceKind,
    CheckInSubmitTarget, CheckInSubmitTargetKind, CheckInTransition, CheckInTransitionKind,
    CheckInTransitionTargetKind, DailyLoopCheckInResolution, DailyLoopCheckInResolutionKind,
    DailyLoopPhase, DailyLoopSession, DailyLoopTurnAction, DailyLoopTurnRequest,
};
use vel_storage::{DailyCheckInEventInsert, DailyCheckInEventRecord, Storage};

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

pub async fn list_session_check_in_events(
    storage: &Storage,
    session_id: &str,
    check_in_type: Option<&str>,
    session_phase: Option<&str>,
    include_skipped: bool,
    limit: u32,
) -> Result<Vec<DailyCheckInEventRecord>, AppError> {
    storage
        .list_daily_check_in_events_for_session(
            session_id,
            check_in_type,
            session_phase,
            include_skipped,
            limit,
        )
        .await
        .map_err(|error| AppError::internal(format!("list daily check-in events: {error}")))
}

#[derive(Debug)]
pub struct DailyCheckInSubmitInput {
    pub check_in_type: String,
    pub session_phase: String,
    pub source: String,
    pub prompt_id: String,
    pub answered_at: Option<i64>,
    pub text: Option<String>,
    pub scale: Option<i64>,
    pub keywords: Vec<String>,
    pub confidence: Option<f64>,
    pub skipped: bool,
    pub skip_reason_code: Option<String>,
    pub skip_reason_text: Option<String>,
    pub replace_if_conflict: bool,
    pub run_id: Option<String>,
}

#[derive(Debug)]
pub struct DailyCheckInSubmitResult {
    pub check_in_event_id: String,
    pub supersedes_event_id: Option<String>,
}

#[derive(Debug)]
pub struct DailyCheckInSkipInput {
    pub source: Option<String>,
    pub answered_at: Option<i64>,
    pub reason_code: Option<String>,
    pub reason_text: Option<String>,
}

#[derive(Debug)]
pub struct DailyCheckInSkipResult {
    pub check_in_event_id: String,
    pub session_id: String,
    pub supersedes_event_id: Option<String>,
}

pub async fn submit_check_in(
    storage: &Storage,
    session_id: &str,
    input: DailyCheckInSubmitInput,
) -> Result<DailyCheckInSubmitResult, AppError> {
    let check_in_type = check_in_type_label(&input.check_in_type)?;
    let session_phase = session_phase_label_input(&input.session_phase)?;
    let source = input.source.trim().to_lowercase();
    if source != "user" && source != "inferred" {
        return Err(AppError::bad_request("source must be `user` or `inferred`"));
    }

    let prompt_id = input.prompt_id.trim().to_string();
    if prompt_id.is_empty() {
        return Err(AppError::bad_request("prompt_id is required"));
    }

    if input.skipped {
        if input
            .text
            .as_ref()
            .and_then(|value| (!value.trim().is_empty()).then_some(()))
            .is_some()
            && input.scale.is_none()
        {
            return Err(AppError::bad_request(
                "skipped check-ins cannot include a free-form response",
            ));
        }
    } else if input.scale.is_none() {
        let has_text = input
            .text
            .as_ref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        if !has_text {
            return Err(AppError::bad_request(
                "non-skipped check-ins require text or scale",
            ));
        }
    }

    if input.scale.is_some() && (input.scale.unwrap_or(0) < -10 || input.scale.unwrap_or(0) > 10) {
        return Err(AppError::bad_request("scale must be between -10 and 10"));
    }

    if !input
        .confidence
        .map(|value| (0.0..=1.0).contains(&value))
        .unwrap_or(true)
    {
        return Err(AppError::bad_request(
            "confidence must be between 0.0 and 1.0",
        ));
    }

    let response_text = input
        .text
        .as_ref()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty());
    let text = response_text.map(str::to_string);
    let keywords = if input.keywords.is_empty() {
        extract_keywords(response_text)
    } else {
        input
            .keywords
            .into_iter()
            .filter_map(normalize_keyword)
            .collect::<Vec<_>>()
    };

    let supersedes_event_id = if input.replace_if_conflict {
        let candidates = storage
            .list_daily_check_in_events_for_session(
                session_id,
                Some(check_in_type.as_str()),
                Some(session_phase.as_str()),
                true,
                16,
            )
            .await
            .map_err(|error| {
                AppError::internal(format!("find replacement candidate for check-in: {error}"))
            })?;
        candidates
            .into_iter()
            .find(|record| record.prompt_id == prompt_id)
            .map(|record| record.event_id.clone())
    } else {
        None
    };

    let check_in_event_id = storage
        .insert_daily_check_in_event(DailyCheckInEventInsert {
            session_id: session_id.to_string(),
            prompt_id: prompt_id.clone(),
            check_in_type,
            session_phase,
            source: source.clone(),
            answered_at: input.answered_at,
            text,
            scale: input.scale,
            scale_min: -10,
            scale_max: 10,
            keywords_json: JsonValue::Array(keywords),
            confidence: input.confidence,
            schema_version: 1,
            skipped: input.skipped,
            skip_reason_code: input.skip_reason_code.map(|value| value.trim().to_string()),
            skip_reason_text: input.skip_reason_text.map(|value| value.trim().to_string()),
            replaced_by_event_id: supersedes_event_id.clone(),
            run_id: input.run_id,
            meta_json: serde_json::json!({
                "source": source,
                "prompt_id": prompt_id,
                "scale_min": -10,
                "scale_max": 10,
            }),
        })
        .await
        .map_err(|error| AppError::internal(format!("insert daily check-in event: {error}")))?;

    Ok(DailyCheckInSubmitResult {
        check_in_event_id,
        supersedes_event_id,
    })
}

pub async fn skip_check_in(
    storage: &Storage,
    check_in_event_id: &str,
    input: DailyCheckInSkipInput,
) -> Result<DailyCheckInSkipResult, AppError> {
    let target_event_id = check_in_event_id.to_string();
    let target_event = storage
        .get_daily_check_in_event(&target_event_id)
        .await?
        .ok_or_else(|| AppError::not_found("check-in event not found"))?;

    let source = input.source.unwrap_or_else(|| "user".to_string());
    let source = source.trim().to_lowercase();
    if source != "user" && source != "inferred" {
        return Err(AppError::bad_request("source must be `user` or `inferred`"));
    }

    let reason_code = input
        .reason_code
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let reason_text = input
        .reason_text
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    if reason_code.is_none() && reason_text.is_none() {
        return Err(AppError::bad_request(
            "skip requires a reason_code or reason_text",
        ));
    }

    let new_check_in_event_id = storage
        .insert_daily_check_in_event(DailyCheckInEventInsert {
            session_id: target_event.session_id.clone(),
            prompt_id: target_event.prompt_id.clone(),
            check_in_type: target_event.check_in_type,
            session_phase: target_event.session_phase,
            source: source.clone(),
            answered_at: input.answered_at,
            text: None,
            scale: None,
            scale_min: -10,
            scale_max: 10,
            keywords_json: JsonValue::Array(vec![]),
            confidence: None,
            schema_version: 1,
            skipped: true,
            skip_reason_code: reason_code,
            skip_reason_text: reason_text,
            replaced_by_event_id: Some(target_event_id.clone()),
            run_id: None,
            meta_json: serde_json::json!({
                "source": source.clone(),
                "supersedes_event_id": target_event_id.clone(),
                "skipped_by": "api",
            }),
        })
        .await
        .map_err(|error| {
            AppError::internal(format!("insert daily check-in skip event: {error}"))
        })?;

    Ok(DailyCheckInSkipResult {
        check_in_event_id: new_check_in_event_id,
        session_id: target_event.session_id,
        supersedes_event_id: Some(target_event_id),
    })
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
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let response_text = resolution
            .response_text
            .as_deref()
            .or(resolution.note_text.as_deref());
        let skipped = matches!(resolution.kind, DailyLoopCheckInResolutionKind::Bypassed);
        let skip_reason_text = if skipped {
            resolution.note_text.clone()
        } else {
            None
        };

        storage
            .insert_daily_check_in_event(DailyCheckInEventInsert {
                session_id: session.id.to_string(),
                prompt_id: resolution.prompt_id.clone(),
                check_in_type: check_in_type_for_prompt(&session.phase, &resolution.response_text),
                session_phase: session_phase_label(&session.phase),
                source: "user".to_string(),
                answered_at: Some(now),
                text: response_text.map(ToString::to_string),
                scale: None,
                scale_min: -10,
                scale_max: 10,
                keywords_json: JsonValue::Array(extract_keywords(response_text)),
                confidence: Some(1.0),
                schema_version: 1,
                skipped,
                skip_reason_code: if skipped {
                    Some("user_skip".to_string())
                } else {
                    None
                },
                skip_reason_text,
                replaced_by_event_id: None,
                run_id: None,
                meta_json: serde_json::json!({
                    "phase": session_phase_label(&session.phase),
                    "prompt_id": resolution.prompt_id,
                    "ordinal": resolution.ordinal,
                    "check_in_kind": check_in_kind_label(&resolution.kind),
                }),
            })
            .await?;

        update_follow_through_thread_status(storage, session, resolution).await?;
    }
    Ok(())
}

fn check_in_type_for_prompt(phase: &DailyLoopPhase, response_text: &Option<String>) -> String {
    if let Some(text) = response_text {
        if text.to_lowercase().contains("mood") {
            return "mood".to_string();
        }
    }

    match phase {
        DailyLoopPhase::MorningOverview => "other".to_string(),
        DailyLoopPhase::Standup => "other".to_string(),
    }
}

fn session_phase_label(phase: &DailyLoopPhase) -> String {
    match phase {
        DailyLoopPhase::MorningOverview => "morning".to_string(),
        DailyLoopPhase::Standup => "standup".to_string(),
    }
}

pub(crate) fn session_phase_label_input(value: &str) -> Result<String, AppError> {
    match value.trim() {
        "morning" => Ok("morning".to_string()),
        "standup" => Ok("standup".to_string()),
        _ => Err(AppError::bad_request(
            "session_phase must be `morning` or `standup`",
        )),
    }
}

fn check_in_type_label(value: &str) -> Result<String, AppError> {
    match value.trim() {
        "mood" => Ok("mood".to_string()),
        "body" => Ok("body".to_string()),
        "sleep" => Ok("sleep".to_string()),
        "dream" => Ok("dream".to_string()),
        "pain" => Ok("pain".to_string()),
        "other" => Ok("other".to_string()),
        _ => Err(AppError::bad_request(
            "check_in_type must be one of mood, body, sleep, dream, pain, other",
        )),
    }
}

fn normalize_keyword(value: String) -> Option<JsonValue> {
    let value = value.trim().to_lowercase();
    if value.is_empty() {
        None
    } else {
        Some(JsonValue::String(value))
    }
}

fn check_in_kind_label(kind: &DailyLoopCheckInResolutionKind) -> &'static str {
    match kind {
        DailyLoopCheckInResolutionKind::Submitted => "submitted",
        DailyLoopCheckInResolutionKind::Bypassed => "bypassed",
    }
}

fn extract_keywords(text: Option<&str>) -> Vec<JsonValue> {
    text.map(|value| {
        value
            .split_whitespace()
            .map(|token| token.trim_matches(|ch: char| !ch.is_alphanumeric()))
            .filter(|token| token.len() >= 3)
            .map(|token| JsonValue::String(token.to_ascii_lowercase()))
            .collect()
    })
    .unwrap_or_default()
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
