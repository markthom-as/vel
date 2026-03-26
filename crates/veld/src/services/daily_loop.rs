use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::{
    DailyCommitmentDraft, DailyDeferredTask, DailyFocusBlockProposal, DailyLoopCheckInResolution,
    DailyLoopPhase, DailyLoopPrompt, DailyLoopPromptKind, DailyLoopSession, DailyLoopSessionId,
    DailyLoopSessionOutcome, DailyLoopSessionState, DailyLoopStartRequest, DailyLoopStatus,
    DailyLoopTurnAction, DailyLoopTurnRequest, DailyLoopTurnState, DailyStandupBucket,
    DailyStandupOutcome, MorningIntentSignal, MorningOverviewState, DAILY_LOOP_MAX_COMMITMENTS,
    DAILY_LOOP_MAX_QUESTIONS,
};
use vel_storage::{CommitmentInsert, DailySessionRecord, Storage};

use crate::{errors::AppError, services::daily_loop_inputs};

pub async fn start_session(
    storage: &Storage,
    config: &AppConfig,
    request: DailyLoopStartRequest,
) -> Result<DailyLoopSession, AppError> {
    match request.phase {
        DailyLoopPhase::MorningOverview => start_morning_overview(storage, config, request).await,
        DailyLoopPhase::Standup => start_standup(storage, request).await,
    }
}

pub async fn get_active_session(
    storage: &Storage,
    session_date: &str,
    phase: DailyLoopPhase,
) -> Result<Option<DailyLoopSession>, AppError> {
    Ok(storage
        .get_active_daily_session_for_date(session_date, phase)
        .await?
        .map(|record| record.session))
}

pub fn assistant_requested_phase(text: &str) -> Option<DailyLoopPhase> {
    let normalized = text.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return None;
    }
    if let Some(command) = vel_command_lang::shell::explicit_command_name(&normalized) {
        match command.as_str() {
            "standup" => return Some(DailyLoopPhase::Standup),
            "morning" => return Some(DailyLoopPhase::MorningOverview),
            _ => {}
        }
    }
    if normalized.contains("standup") {
        return Some(DailyLoopPhase::Standup);
    }

    const MORNING_MARKERS: &[&str] = &[
        "good morning",
        "start my day",
        "start the day",
        "start my morning",
        "morning overview",
        "morning briefing",
    ];
    if MORNING_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
    {
        Some(DailyLoopPhase::MorningOverview)
    } else {
        None
    }
}

pub fn assistant_prefers_resume(text: &str) -> bool {
    let normalized = text.to_ascii_lowercase();
    normalized.contains("resume")
        || normalized.contains("continue")
        || matches!(
            vel_command_lang::shell::explicit_command_name(&normalized).as_deref(),
            Some("checkin" | "check-in" | "check_in")
        )
}

async fn assistant_requested_check_in_phase(
    storage: &Storage,
) -> Result<Option<DailyLoopPhase>, AppError> {
    let timezone = crate::services::timezone::resolve_timezone(storage).await?;
    let session_date =
        crate::services::timezone::current_day_date_string(&timezone, OffsetDateTime::now_utc())?;

    for phase in [DailyLoopPhase::Standup, DailyLoopPhase::MorningOverview] {
        if get_active_session(storage, &session_date, phase)
            .await?
            .is_some()
        {
            return Ok(Some(phase));
        }
    }

    Ok(None)
}

pub async fn start_or_resume_assistant_session(
    storage: &Storage,
    config: &AppConfig,
    transcript: &str,
    surface: vel_core::DailyLoopSurface,
) -> Result<Option<DailyLoopSession>, AppError> {
    let phase = if matches!(
        vel_command_lang::shell::explicit_command_name(&transcript.to_ascii_lowercase()).as_deref(),
        Some("checkin" | "check-in" | "check_in")
    ) {
        assistant_requested_check_in_phase(storage).await?
    } else {
        assistant_requested_phase(transcript)
    };

    let Some(phase) = phase else {
        return Ok(None);
    };
    let timezone = crate::services::timezone::resolve_timezone(storage).await?;
    let session_date =
        crate::services::timezone::current_day_date_string(&timezone, OffsetDateTime::now_utc())?;
    if assistant_prefers_resume(transcript) {
        if let Some(active) = get_active_session(storage, &session_date, phase).await? {
            return Ok(Some(active));
        }
    }

    start_session(
        storage,
        config,
        DailyLoopStartRequest {
            phase,
            session_date,
            start: vel_core::DailyLoopStartMetadata {
                source: vel_core::DailyLoopStartSource::Manual,
                surface,
            },
        },
    )
    .await
    .map(Some)
}

pub fn assistant_entry_summary(session: &DailyLoopSession) -> String {
    let prompt = session
        .current_prompt
        .as_ref()
        .map(|prompt| prompt.text.as_str())
        .unwrap_or("Continue in the daily loop.");
    match session.phase {
        DailyLoopPhase::MorningOverview => format!("Morning overview ready. {prompt}"),
        DailyLoopPhase::Standup => format!("Standup ready. {prompt}"),
    }
}

pub fn commitment_action_labels(session: &DailyLoopSession) -> Vec<&'static str> {
    let mut actions = vec!["accept"];

    if session
        .current_prompt
        .as_ref()
        .map(|prompt| prompt.allow_skip)
        .unwrap_or(false)
    {
        actions.push("defer");
    }

    actions.push("choose");
    actions.push("close");
    actions
}

pub fn session_continuity_summary(session: &DailyLoopSession) -> String {
    match (&session.phase, &session.current_prompt, &session.state) {
        (
            DailyLoopPhase::MorningOverview,
            Some(prompt),
            DailyLoopSessionState::MorningOverview(state),
        ) => format!(
            "Morning overview is waiting on question {} of {} with {} captured signal(s).",
            prompt.ordinal,
            DAILY_LOOP_MAX_QUESTIONS,
            state.signals.len()
        ),
        (DailyLoopPhase::MorningOverview, None, DailyLoopSessionState::MorningOverview(state)) => {
            format!(
                "Morning overview captured {} signal(s) for today's bounded orientation.",
                state.signals.len()
            )
        }
        (DailyLoopPhase::Standup, Some(prompt), DailyLoopSessionState::Standup(state)) => format!(
            "Standup is waiting on question {} with {} commitment draft(s) and {} deferred item(s).",
            prompt.ordinal,
            state.commitments.len(),
            state.deferred_tasks.len()
        ),
        (DailyLoopPhase::Standup, None, DailyLoopSessionState::Standup(state)) => format!(
            "Standup currently holds {} commitment draft(s) and {} deferred item(s).",
            state.commitments.len(),
            state.deferred_tasks.len()
        ),
        (phase, _, _) => format!(
            "{} session continuity is available.",
            match phase {
                DailyLoopPhase::MorningOverview => "Morning overview",
                DailyLoopPhase::Standup => "Standup",
            }
        ),
    }
}

pub async fn submit_turn(
    storage: &Storage,
    request: DailyLoopTurnRequest,
) -> Result<DailyLoopSession, AppError> {
    let Some(record) = storage
        .get_daily_session(request.session_id.as_ref())
        .await?
    else {
        return Err(AppError::not_found("daily loop session not found"));
    };

    let (request, resolution) =
        crate::services::check_in::prepare_turn_request(&record.session, request)?;
    crate::services::check_in::persist_resolution_follow_through(
        storage,
        &record.session,
        resolution.as_ref(),
    )
    .await?;

    if matches!(request.action, DailyLoopTurnAction::Resume)
        || matches!(
            record.session.status,
            DailyLoopStatus::Completed | DailyLoopStatus::Cancelled
        )
    {
        return Ok(record.session);
    }

    match record.session.phase {
        DailyLoopPhase::MorningOverview => {
            advance_morning_turn(storage, record, request, resolution).await
        }
        DailyLoopPhase::Standup => advance_standup_turn(storage, record, request, resolution).await,
    }
}

async fn start_morning_overview(
    storage: &Storage,
    config: &AppConfig,
    request: DailyLoopStartRequest,
) -> Result<DailyLoopSession, AppError> {
    if let Some(active) = storage
        .get_active_daily_session_for_date(&request.session_date, DailyLoopPhase::MorningOverview)
        .await?
    {
        return Ok(active.session);
    }

    let snapshot =
        daily_loop_inputs::load_daily_loop_inputs(storage, config, &request.session_date).await?;
    let session = DailyLoopSession {
        id: DailyLoopSessionId::new(),
        session_date: request.session_date,
        phase: DailyLoopPhase::MorningOverview,
        status: DailyLoopStatus::WaitingForInput,
        start: request.start,
        turn_state: DailyLoopTurnState::WaitingForInput,
        current_prompt: Some(morning_prompt_for_ordinal(1)),
        state: DailyLoopSessionState::MorningOverview(MorningOverviewState {
            snapshot: snapshot.summary,
            friction_callouts: snapshot.friction_callouts,
            signals: Vec::new(),
            check_in_history: Vec::new(),
        }),
        outcome: None,
    };

    Ok(storage
        .create_daily_session(&session, OffsetDateTime::now_utc())
        .await?
        .session)
}

async fn start_standup(
    storage: &Storage,
    request: DailyLoopStartRequest,
) -> Result<DailyLoopSession, AppError> {
    if let Some(active) = storage
        .get_active_daily_session_for_date(&request.session_date, DailyLoopPhase::Standup)
        .await?
    {
        return Ok(active.session);
    }

    let carried_signals = storage
        .get_latest_daily_session_for_date(&request.session_date, DailyLoopPhase::MorningOverview)
        .await?
        .and_then(morning_signals_from_record)
        .unwrap_or_default();
    let candidates = list_candidate_titles(storage).await?;

    let prompt = standup_prompt_for_ordinal(1, candidates.is_empty());
    let session = DailyLoopSession {
        id: DailyLoopSessionId::new(),
        session_date: request.session_date,
        phase: DailyLoopPhase::Standup,
        status: DailyLoopStatus::WaitingForInput,
        start: request.start,
        turn_state: DailyLoopTurnState::WaitingForInput,
        current_prompt: Some(prompt),
        state: DailyLoopSessionState::Standup(build_standup_state(carried_signals, candidates)),
        outcome: None,
    };

    Ok(storage
        .create_daily_session(&session, OffsetDateTime::now_utc())
        .await?
        .session)
}

async fn advance_morning_turn(
    storage: &Storage,
    record: DailySessionRecord,
    request: DailyLoopTurnRequest,
    resolution: Option<DailyLoopCheckInResolution>,
) -> Result<DailyLoopSession, AppError> {
    let DailyLoopSessionState::MorningOverview(mut state) = record.session.state.clone() else {
        return Err(AppError::internal("expected morning overview state"));
    };
    append_resolution(&mut state.check_in_history, resolution);

    let current_ordinal = record
        .session
        .current_prompt
        .as_ref()
        .map(|prompt| prompt.ordinal)
        .unwrap_or(DAILY_LOOP_MAX_QUESTIONS);

    if matches!(request.action, DailyLoopTurnAction::Submit) {
        if let Some(text) = request.response_text.as_deref().map(str::trim) {
            if !text.is_empty() {
                state.signals.push(signal_from_response(text));
            }
        }
    }

    let now = OffsetDateTime::now_utc();
    if current_ordinal >= DAILY_LOOP_MAX_QUESTIONS {
        let outcome = DailyLoopSessionOutcome::MorningOverview {
            signals: state.signals.clone(),
            check_in_history: state.check_in_history.clone(),
        };
        return Ok(storage
            .complete_daily_session(
                request.session_id.as_ref(),
                &DailyLoopSessionState::MorningOverview(state),
                &outcome,
                now,
            )
            .await?
            .ok_or_else(|| AppError::not_found("daily loop session not found after completion"))?
            .session);
    }

    let next_prompt = morning_prompt_for_ordinal(current_ordinal + 1);
    Ok(storage
        .update_daily_session_state(
            request.session_id.as_ref(),
            DailyLoopStatus::WaitingForInput,
            DailyLoopTurnState::WaitingForInput,
            Some(&next_prompt),
            &DailyLoopSessionState::MorningOverview(state),
            None,
            now,
        )
        .await?
        .ok_or_else(|| AppError::not_found("daily loop session not found after update"))?
        .session)
}

async fn advance_standup_turn(
    storage: &Storage,
    record: DailySessionRecord,
    request: DailyLoopTurnRequest,
    resolution: Option<DailyLoopCheckInResolution>,
) -> Result<DailyLoopSession, AppError> {
    let DailyLoopSessionState::Standup(mut state) = record.session.state.clone() else {
        return Err(AppError::internal("expected standup state"));
    };
    append_resolution(&mut state.check_in_history, resolution);
    let current_ordinal = record
        .session
        .current_prompt
        .as_ref()
        .map(|prompt| prompt.ordinal)
        .unwrap_or(DAILY_LOOP_MAX_QUESTIONS);

    if matches!(request.action, DailyLoopTurnAction::Submit) {
        if let Some(text) = request.response_text.as_deref() {
            apply_standup_response(&mut state, text);
        }
    }

    let now = OffsetDateTime::now_utc();
    if current_ordinal >= DAILY_LOOP_MAX_QUESTIONS || state.commitments.len() >= 3 {
        return finalize_standup(storage, request.session_id.as_ref(), state, now).await;
    }

    if current_ordinal == 1 && state.commitments.is_empty() {
        let reprompt = standup_prompt_for_ordinal(2, true);
        return Ok(storage
            .update_daily_session_state(
                request.session_id.as_ref(),
                DailyLoopStatus::WaitingForInput,
                DailyLoopTurnState::WaitingForInput,
                Some(&reprompt),
                &DailyLoopSessionState::Standup(state),
                None,
                now,
            )
            .await?
            .ok_or_else(|| AppError::not_found("daily loop session not found after update"))?
            .session);
    }

    if current_ordinal == 2 && state.commitments.is_empty() {
        return finalize_standup(storage, request.session_id.as_ref(), state, now).await;
    }

    let next_prompt = standup_prompt_for_ordinal(current_ordinal + 1, false);
    Ok(storage
        .update_daily_session_state(
            request.session_id.as_ref(),
            DailyLoopStatus::WaitingForInput,
            DailyLoopTurnState::WaitingForInput,
            Some(&next_prompt),
            &DailyLoopSessionState::Standup(state),
            None,
            now,
        )
        .await?
        .ok_or_else(|| AppError::not_found("daily loop session not found after update"))?
        .session)
}

async fn finalize_standup(
    storage: &Storage,
    session_id: &str,
    mut state: DailyStandupOutcome,
    now: OffsetDateTime,
) -> Result<DailyLoopSession, AppError> {
    state.commitments.truncate(DAILY_LOOP_MAX_COMMITMENTS);
    for draft in &state.commitments {
        let _ = storage
            .insert_commitment(CommitmentInsert {
                text: draft.title.clone(),
                source_type: "daily_loop".to_string(),
                source_id: session_id.to_string(),
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("daily_loop".to_string()),
                metadata_json: Some(serde_json::json!({
                    "phase": "standup",
                    "bucket": format!("{:?}", draft.bucket).to_lowercase(),
                })),
            })
            .await?;
    }
    let outcome = DailyLoopSessionOutcome::Standup(state.clone());
    Ok(storage
        .complete_daily_session(
            session_id,
            &DailyLoopSessionState::Standup(state),
            &outcome,
            now,
        )
        .await?
        .ok_or_else(|| AppError::not_found("daily loop session not found after completion"))?
        .session)
}

fn morning_prompt_for_ordinal(ordinal: u8) -> DailyLoopPrompt {
    let text = match ordinal {
        1 => "What most needs to happen before noon?",
        2 => "What could derail today if ignored?",
        _ => "What deserves protected focus time today?",
    };
    DailyLoopPrompt {
        prompt_id: format!("morning_prompt_{ordinal}"),
        kind: DailyLoopPromptKind::IntentQuestion,
        text: text.to_string(),
        ordinal,
        allow_skip: true,
    }
}

fn standup_prompt_for_ordinal(ordinal: u8, reprompt: bool) -> DailyLoopPrompt {
    let text = match ordinal {
        1 => "Name the one to three commitments that matter most today.",
        2 if reprompt => "I still need at least one real commitment. What will you finish today?",
        2 => "What should be explicitly deferred so the top commitments stay realistic?",
        _ => "What calendar or focus block needs protection so these commitments actually happen?",
    };
    DailyLoopPrompt {
        prompt_id: format!("standup_prompt_{ordinal}"),
        kind: if ordinal == 1 || reprompt {
            DailyLoopPromptKind::CommitmentReduction
        } else {
            DailyLoopPromptKind::ConstraintCheck
        },
        text: text.to_string(),
        ordinal,
        allow_skip: true,
    }
}

fn signal_from_response(text: &str) -> MorningIntentSignal {
    let lower = text.to_ascii_lowercase();
    if lower.contains("focus") || lower.contains("block") {
        MorningIntentSignal::FocusIntent {
            text: text.to_string(),
        }
    } else if lower.contains("meeting") || lower.contains("calendar") {
        MorningIntentSignal::MeetingDoubt {
            text: text.to_string(),
        }
    } else {
        MorningIntentSignal::MustDoHint {
            text: text.to_string(),
        }
    }
}

fn build_standup_state(
    carried_signals: Vec<MorningIntentSignal>,
    candidates: Vec<String>,
) -> DailyStandupOutcome {
    let mut commitments = Vec::new();
    for (idx, signal) in carried_signals.into_iter().enumerate() {
        if idx >= DAILY_LOOP_MAX_COMMITMENTS {
            break;
        }
        commitments.push(DailyCommitmentDraft {
            title: morning_signal_text(&signal),
            bucket: match idx {
                0 => DailyStandupBucket::Must,
                1 => DailyStandupBucket::Should,
                _ => DailyStandupBucket::Stretch,
            },
            source_ref: Some("morning_signal".to_string()),
        });
    }
    for title in candidates {
        if commitments.len() >= DAILY_LOOP_MAX_COMMITMENTS {
            break;
        }
        if commitments.iter().any(|draft| draft.title == title) {
            continue;
        }
        commitments.push(DailyCommitmentDraft {
            title,
            bucket: if commitments.is_empty() {
                DailyStandupBucket::Must
            } else {
                DailyStandupBucket::Should
            },
            source_ref: Some("open_commitment".to_string()),
        });
    }

    DailyStandupOutcome {
        commitments,
        deferred_tasks: Vec::new(),
        confirmed_calendar: Vec::new(),
        focus_blocks: Vec::new(),
        check_in_history: Vec::new(),
    }
}

fn append_resolution(
    history: &mut Vec<DailyLoopCheckInResolution>,
    resolution: Option<DailyLoopCheckInResolution>,
) {
    if let Some(resolution) = resolution {
        history.push(resolution);
    }
}

async fn list_candidate_titles(storage: &Storage) -> Result<Vec<String>, AppError> {
    Ok(storage
        .list_commitments(Some(vel_core::CommitmentStatus::Open), None, None, 16)
        .await?
        .into_iter()
        .map(|commitment| commitment.text)
        .collect())
}

fn morning_signals_from_record(record: DailySessionRecord) -> Option<Vec<MorningIntentSignal>> {
    match record.session.outcome {
        Some(DailyLoopSessionOutcome::MorningOverview { signals, .. }) => Some(signals),
        _ => match record.session.state {
            DailyLoopSessionState::MorningOverview(state) => Some(state.signals),
            _ => None,
        },
    }
}

fn morning_signal_text(signal: &MorningIntentSignal) -> String {
    match signal {
        MorningIntentSignal::MustDoHint { text }
        | MorningIntentSignal::FocusIntent { text }
        | MorningIntentSignal::MeetingDoubt { text } => text.clone(),
    }
}

fn apply_standup_response(state: &mut DailyStandupOutcome, text: &str) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return;
    }

    if state.commitments.len() < DAILY_LOOP_MAX_COMMITMENTS {
        for title in split_items(trimmed) {
            if state.commitments.len() >= DAILY_LOOP_MAX_COMMITMENTS {
                state.deferred_tasks.push(DailyDeferredTask {
                    title,
                    source_ref: None,
                    reason: "Outside the top three".to_string(),
                });
            } else if !state.commitments.iter().any(|draft| draft.title == title) {
                let bucket = match state.commitments.len() {
                    0 => DailyStandupBucket::Must,
                    1 => DailyStandupBucket::Should,
                    _ => DailyStandupBucket::Stretch,
                };
                state.commitments.push(DailyCommitmentDraft {
                    title,
                    bucket,
                    source_ref: None,
                });
            }
        }
    } else {
        if state.confirmed_calendar.is_empty() {
            state.confirmed_calendar.push(trimmed.to_string());
        }
        if state.focus_blocks.is_empty() {
            let now = OffsetDateTime::now_utc();
            state.focus_blocks.push(DailyFocusBlockProposal {
                label: "Protected focus".to_string(),
                start_at: now + time::Duration::hours(1),
                end_at: now + time::Duration::hours(2),
                reason: trimmed.to_string(),
            });
        }
    }
}

fn split_items(text: &str) -> Vec<String> {
    text.split([',', '\n'])
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{commitment_action_labels, session_continuity_summary};
    use vel_core::{
        DailyCommitmentDraft, DailyDeferredTask, DailyLoopPhase, DailyLoopPrompt,
        DailyLoopPromptKind, DailyLoopSession, DailyLoopSessionState, DailyLoopStartMetadata,
        DailyLoopStartSource, DailyLoopStatus, DailyLoopSurface, DailyLoopTurnState,
        DailyStandupBucket, DailyStandupOutcome, MorningIntentSignal, MorningOverviewState,
    };

    #[test]
    fn standup_session_helpers_expose_bounded_actions_and_continuity() {
        let session = DailyLoopSession {
            id: "dls_test".to_string().into(),
            session_date: "2026-03-20".to_string(),
            phase: DailyLoopPhase::Standup,
            status: DailyLoopStatus::WaitingForInput,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Web,
            },
            turn_state: DailyLoopTurnState::WaitingForInput,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "prompt_1".to_string(),
                kind: DailyLoopPromptKind::CommitmentReduction,
                text: "Reduce this to three commitments.".to_string(),
                ordinal: 2,
                allow_skip: true,
            }),
            state: DailyLoopSessionState::Standup(DailyStandupOutcome {
                commitments: vec![DailyCommitmentDraft {
                    title: "Ship contract slice".to_string(),
                    bucket: DailyStandupBucket::Must,
                    source_ref: None,
                }],
                deferred_tasks: vec![DailyDeferredTask {
                    title: "Triage inbox".to_string(),
                    source_ref: None,
                    reason: "Not part of the top three".to_string(),
                }],
                confirmed_calendar: vec![],
                focus_blocks: vec![],
                check_in_history: vec![],
            }),
            outcome: None,
        };

        assert_eq!(
            commitment_action_labels(&session),
            vec!["accept", "defer", "choose", "close"]
        );
        assert_eq!(
            session_continuity_summary(&session),
            "Standup is waiting on question 2 with 1 commitment draft(s) and 1 deferred item(s)."
        );
    }

    #[test]
    fn morning_continuity_summary_mentions_question_budget() {
        let session = DailyLoopSession {
            id: "dls_morning".to_string().into(),
            session_date: "2026-03-20".to_string(),
            phase: DailyLoopPhase::MorningOverview,
            status: DailyLoopStatus::WaitingForInput,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Cli,
            },
            turn_state: DailyLoopTurnState::WaitingForInput,
            current_prompt: Some(DailyLoopPrompt {
                prompt_id: "prompt_morning_1".to_string(),
                kind: DailyLoopPromptKind::IntentQuestion,
                text: "What matters before noon?".to_string(),
                ordinal: 1,
                allow_skip: true,
            }),
            state: DailyLoopSessionState::MorningOverview(MorningOverviewState {
                snapshot: "Two meetings before lunch.".to_string(),
                friction_callouts: vec![],
                signals: vec![MorningIntentSignal::FocusIntent {
                    text: "Protect a block".to_string(),
                }],
                check_in_history: vec![],
            }),
            outcome: None,
        };

        assert_eq!(
            session_continuity_summary(&session),
            "Morning overview is waiting on question 1 of 3 with 1 captured signal(s)."
        );
    }
}
