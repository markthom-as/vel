use time::OffsetDateTime;
use uuid::Uuid;
use vel_core::{
    AppleClientSurface, AppleRequestedOperation, AppleResponseEvidence, AppleResponseMode,
    AppleScheduleEvent, AppleScheduleSnapshot, AppleVoiceIntent,
    AppleVoiceTurnQueuedMutationSummary, AppleVoiceTurnRequest, AppleVoiceTurnResponse,
    PrivacyClass,
};
use vel_storage::{CaptureInsert, SignalInsert};

use crate::{
    errors::AppError,
    services::{apple_behavior, client_sync, now},
    state::AppState,
};

const APPLE_CAPTURE_TYPE: &str = "voice_note";

pub async fn apple_voice_turn(
    state: &AppState,
    request: AppleVoiceTurnRequest,
) -> Result<AppleVoiceTurnResponse, AppError> {
    let transcript = request.transcript.trim();
    if transcript.is_empty() {
        return Err(AppError::bad_request("transcript must not be empty"));
    }

    let capture_id = persist_transcript_capture(state, &request, transcript).await?;
    let intent = request
        .intents
        .first()
        .copied()
        .unwrap_or(AppleVoiceIntent::Capture);

    match intent {
        AppleVoiceIntent::CurrentSchedule => {
            let now = now::get_now(&state.storage, &state.config).await?;
            Ok(schedule_response(request.operation, capture_id, now))
        }
        AppleVoiceIntent::ExplainWhy | AppleVoiceIntent::MorningBriefing => {
            let now = now::get_now(&state.storage, &state.config).await?;
            Ok(explain_response(request.operation, capture_id, now))
        }
        AppleVoiceIntent::NextCommitment => {
            let now = now::get_now(&state.storage, &state.config).await?;
            Ok(next_commitment_response(request.operation, capture_id, now))
        }
        AppleVoiceIntent::ActiveNudges => {
            active_nudges_response(&state.storage, request.operation, capture_id).await
        }
        AppleVoiceIntent::SnoozeNudge => {
            apply_nudge_snooze(state, request.operation, capture_id, transcript).await
        }
        AppleVoiceIntent::CompleteCommitment => {
            apply_commitment_done(state, request.operation, capture_id).await
        }
        AppleVoiceIntent::BehaviorSummary => {
            behavior_summary_response(state, request.operation, capture_id).await
        }
        AppleVoiceIntent::Capture => {
            Ok(capture_only_response(request.operation, capture_id))
        }
    }
}

async fn persist_transcript_capture(
    state: &AppState,
    request: &AppleVoiceTurnRequest,
    transcript: &str,
) -> Result<vel_core::CaptureId, AppError> {
    let source_device = request
        .provenance
        .as_ref()
        .and_then(|provenance| provenance.source_device.clone())
        .or_else(|| Some(surface_source_device(request.surface).to_string()));
    let capture_id = state
        .storage
        .insert_capture(CaptureInsert {
            content_text: transcript.to_string(),
            capture_type: APPLE_CAPTURE_TYPE.to_string(),
            source_device,
            privacy_class: PrivacyClass::Private,
        })
        .await?;

    let payload_json = serde_json::json!({
        "capture_id": capture_id.to_string(),
        "transcript": transcript,
        "surface": surface_source_device(request.surface),
        "operation": format!("{:?}", request.operation).to_lowercase(),
        "intents": request
            .intents
            .iter()
            .map(intent_token)
            .collect::<Vec<_>>(),
        "provenance": request.provenance.as_ref().map(|provenance| serde_json::json!({
            "source_device": provenance.source_device,
            "locale": provenance.locale,
            "transcript_origin": provenance.transcript_origin,
            "recorded_at": provenance.recorded_at.map(|value| value.unix_timestamp()),
            "offline_captured_at": provenance.offline_captured_at.map(|value| value.unix_timestamp()),
            "queued_at": provenance.queued_at.map(|value| value.unix_timestamp()),
        })),
    });
    let _ = state
        .storage
        .insert_signal(SignalInsert {
            signal_type: "capture_created".to_string(),
            source: "vel".to_string(),
            source_ref: Some(capture_id.to_string()),
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            payload_json: Some(payload_json),
        })
        .await;

    Ok(capture_id)
}

fn schedule_response(
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
    now: now::NowOutput,
) -> AppleVoiceTurnResponse {
    let schedule = schedule_snapshot(&now);
    let summary = schedule
        .next_event
        .as_ref()
        .map(|event| format!("Next up: {}.", event.title))
        .or_else(|| schedule.focus_summary.clone())
        .unwrap_or_else(|| "No upcoming events are on the backend schedule.".to_string());
    let mut evidence = Vec::new();
    if let Some(event) = schedule.next_event.as_ref() {
        evidence.push(AppleResponseEvidence {
            kind: "calendar_event".to_string(),
            label: event.title.clone(),
            detail: describe_schedule_event(event),
            source_id: Some(format!("calendar:{}:{}", event.title, event.start_ts)),
        });
    } else if let Some(empty) = now.schedule.empty_message.as_ref() {
        evidence.push(AppleResponseEvidence {
            kind: "now_snapshot".to_string(),
            label: "No upcoming schedule items".to_string(),
            detail: empty.clone(),
            source_id: None,
        });
    }

    AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::SpokenSummary,
        summary,
        capture_id: Some(capture_id),
        reasons: non_empty_reasons(
            schedule.reasons.clone(),
            "Schedule answers come from backend /v1/now state.".to_string(),
        ),
        evidence,
        queued_mutation: None,
        schedule: Some(schedule),
        behavior_summary: None,
    }
}

fn explain_response(
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
    now: now::NowOutput,
) -> AppleVoiceTurnResponse {
    let mut reasons = now.reasons.clone();
    if reasons.is_empty() {
        reasons = now.attention.reasons.clone();
    }
    if reasons.is_empty() {
        reasons.push("No current-context reasons are available yet.".to_string());
    }

    let mut evidence = Vec::new();
    if let Some(commitment) = now.tasks.next_commitment.as_ref() {
        evidence.push(AppleResponseEvidence {
            kind: "commitment".to_string(),
            label: commitment.text.clone(),
            detail: "Current backend next-commitment candidate.".to_string(),
            source_id: Some(commitment.id.clone()),
        });
    }
    if let Some(event) = now.schedule.next_event.as_ref() {
        evidence.push(AppleResponseEvidence {
            kind: "calendar_event".to_string(),
            label: event.title.clone(),
            detail: describe_schedule_event_from_now(event),
            source_id: Some(format!("calendar:{}:{}", event.title, event.start_ts)),
        });
    }
    if evidence.is_empty() {
        evidence.push(AppleResponseEvidence {
            kind: "current_context".to_string(),
            label: "Current backend context".to_string(),
            detail: "Reasons were derived from the persisted current context snapshot.".to_string(),
            source_id: None,
        });
    }

    AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::SpokenSummary,
        summary: reasons.join(" "),
        capture_id: Some(capture_id),
        reasons,
        evidence,
        queued_mutation: None,
        schedule: Some(schedule_snapshot(&now)),
        behavior_summary: None,
    }
}

fn next_commitment_response(
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
    now: now::NowOutput,
) -> AppleVoiceTurnResponse {
    let mut evidence = Vec::new();
    let (summary, reasons) = if let Some(commitment) = now.tasks.next_commitment.as_ref() {
        evidence.push(AppleResponseEvidence {
            kind: "commitment".to_string(),
            label: commitment.text.clone(),
            detail: "Selected from the backend Now task snapshot.".to_string(),
            source_id: Some(commitment.id.clone()),
        });
        (
            format!("Next commitment: {}.", commitment.text),
            vec!["The backend Now snapshot selected this next commitment.".to_string()],
        )
    } else {
        (
            "There is no next commitment in backend state right now.".to_string(),
            vec!["No next commitment is set in the persisted current context.".to_string()],
        )
    };

    AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::SpokenSummary,
        summary,
        capture_id: Some(capture_id),
        reasons,
        evidence,
        queued_mutation: None,
        schedule: Some(schedule_snapshot(&now)),
        behavior_summary: None,
    }
}

async fn active_nudges_response(
    storage: &vel_storage::Storage,
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
) -> Result<AppleVoiceTurnResponse, AppError> {
    let nudges = storage.list_nudges(Some("active"), 5).await?;
    let mut evidence = nudges
        .iter()
        .map(|nudge| AppleResponseEvidence {
            kind: "nudge".to_string(),
            label: nudge.message.clone(),
            detail: format!("{} nudge at {} level", nudge.nudge_type, nudge.level),
            source_id: Some(nudge.nudge_id.clone()),
        })
        .collect::<Vec<_>>();
    if evidence.is_empty() {
        evidence.push(AppleResponseEvidence {
            kind: "nudge".to_string(),
            label: "No active nudges".to_string(),
            detail: "The backend nudge queue is currently clear.".to_string(),
            source_id: None,
        });
    }

    Ok(AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::Card,
        summary: if let Some(nudge) = nudges.first() {
            format!("Top nudge: {}.", nudge.message)
        } else {
            "There are no active nudges right now.".to_string()
        },
        capture_id: Some(capture_id),
        reasons: vec!["Active nudges were read from backend persisted nudge state.".to_string()],
        evidence,
        queued_mutation: None,
        schedule: None,
        behavior_summary: None,
    })
}

async fn behavior_summary_response(
    state: &AppState,
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
) -> Result<AppleVoiceTurnResponse, AppError> {
    let summary = apple_behavior::get_summary(&state.storage, &state.config)
        .await?
        .ok_or_else(|| AppError::not_found("apple behavior summary is not available"))?;
    let evidence = summary
        .metrics
        .iter()
        .map(|metric| AppleResponseEvidence {
            kind: "health_metric".to_string(),
            label: metric.display_label.clone(),
            detail: metric.reasons.join(" "),
            source_id: None,
        })
        .collect::<Vec<_>>();

    Ok(AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::SpokenSummary,
        summary: summary.headline.clone(),
        capture_id: Some(capture_id),
        reasons: summary.reasons.clone(),
        evidence,
        queued_mutation: None,
        schedule: None,
        behavior_summary: Some(summary),
    })
}

async fn apply_nudge_snooze(
    state: &AppState,
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
    transcript: &str,
) -> Result<AppleVoiceTurnResponse, AppError> {
    let active_nudges = state.storage.list_nudges(Some("active"), 5).await?;
    if active_nudges.len() != 1 {
        return Ok(ambiguous_action_response(
            operation,
            capture_id,
            "Stored the transcript, but I could not safely choose which nudge to snooze."
                .to_string(),
        ));
    }

    let target = &active_nudges[0];
    let minutes = parse_snooze_minutes(transcript).unwrap_or(10);
    let result = client_sync::apply_client_actions(
        state,
        vec![client_sync::ClientAction {
            action_id: Some(format!("apple_voice_{}", Uuid::new_v4().simple())),
            action_type: client_sync::ClientActionKind::NudgeSnooze,
            target_id: Some(target.nudge_id.clone()),
            text: None,
            minutes: Some(minutes),
            payload: None,
        }],
    )
    .await?;
    let applied = result.results.first().ok_or_else(|| {
        AppError::internal("apple voice nudge snooze did not return an action result")
    })?;
    let queued = applied.status != "applied";

    Ok(AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::Confirmation,
        summary: if queued {
            format!("Queued a {} minute snooze for {}.", minutes, target.message)
        } else {
            format!(
                "Applied a {} minute snooze for {}.",
                minutes, target.message
            )
        },
        capture_id: Some(capture_id),
        reasons: vec![
            "Low-risk Apple voice mutations reuse the existing sync action path.".to_string(),
        ],
        evidence: vec![AppleResponseEvidence {
            kind: "nudge".to_string(),
            label: target.message.clone(),
            detail: format!("Resolved against persisted nudge {}", target.nudge_id),
            source_id: Some(target.nudge_id.clone()),
        }],
        queued_mutation: Some(AppleVoiceTurnQueuedMutationSummary {
            mutation_kind: "nudge_snooze".to_string(),
            queued,
            summary: applied.message.clone(),
            action_reference_id: Some(target.nudge_id.clone()),
        }),
        schedule: None,
        behavior_summary: None,
    })
}

async fn apply_commitment_done(
    state: &AppState,
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
) -> Result<AppleVoiceTurnResponse, AppError> {
    let now = now::get_now(&state.storage, &state.config).await?;
    let Some(commitment) = now.tasks.next_commitment else {
        return Ok(ambiguous_action_response(
            operation,
            capture_id,
            "Stored the transcript, but there is no backend next commitment to complete."
                .to_string(),
        ));
    };
    let result = client_sync::apply_client_actions(
        state,
        vec![client_sync::ClientAction {
            action_id: Some(format!("apple_voice_{}", Uuid::new_v4().simple())),
            action_type: client_sync::ClientActionKind::CommitmentDone,
            target_id: Some(commitment.id.clone()),
            text: None,
            minutes: None,
            payload: None,
        }],
    )
    .await?;
    let applied = result.results.first().ok_or_else(|| {
        AppError::internal("apple voice commitment completion did not return an action result")
    })?;
    let queued = applied.status != "applied";

    Ok(AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::Confirmation,
        summary: if queued {
            format!("Queued completion for {}.", commitment.text)
        } else {
            format!("Applied completion for {}.", commitment.text)
        },
        capture_id: Some(capture_id),
        reasons: vec![
            "Low-risk Apple voice mutations reuse the existing sync action path.".to_string(),
        ],
        evidence: vec![AppleResponseEvidence {
            kind: "commitment".to_string(),
            label: commitment.text.clone(),
            detail: "Resolved against the backend next commitment.".to_string(),
            source_id: Some(commitment.id.clone()),
        }],
        queued_mutation: Some(AppleVoiceTurnQueuedMutationSummary {
            mutation_kind: "commitment_done".to_string(),
            queued,
            summary: applied.message.clone(),
            action_reference_id: Some(commitment.id),
        }),
        schedule: None,
        behavior_summary: None,
    })
}

fn capture_only_response(
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
) -> AppleVoiceTurnResponse {
    AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::Confirmation,
        summary: "Transcript stored as backend capture provenance.".to_string(),
        capture_id: Some(capture_id),
        reasons: vec!["No backend query or safe action was requested.".to_string()],
        evidence: vec![AppleResponseEvidence {
            kind: "capture".to_string(),
            label: "Transcript capture persisted".to_string(),
            detail: "Apple voice input was stored before any reply path.".to_string(),
            source_id: None,
        }],
        queued_mutation: None,
        schedule: None,
        behavior_summary: None,
    }
}

fn ambiguous_action_response(
    operation: AppleRequestedOperation,
    capture_id: vel_core::CaptureId,
    summary: String,
) -> AppleVoiceTurnResponse {
    AppleVoiceTurnResponse {
        operation,
        mode: AppleResponseMode::ClarificationRequired,
        summary,
        capture_id: Some(capture_id),
        reasons: vec![
            "Unsupported or ambiguous Apple voice actions fail closed after transcript capture."
                .to_string(),
        ],
        evidence: vec![AppleResponseEvidence {
            kind: "capture".to_string(),
            label: "Transcript capture persisted".to_string(),
            detail: "No mutation was applied without a safe backend target.".to_string(),
            source_id: None,
        }],
        queued_mutation: None,
        schedule: None,
        behavior_summary: None,
    }
}

fn schedule_snapshot(now: &now::NowOutput) -> AppleScheduleSnapshot {
    AppleScheduleSnapshot {
        generated_at: now.computed_at,
        timezone: now.timezone.clone(),
        focus_summary: now
            .tasks
            .next_commitment
            .as_ref()
            .map(|task| format!("Next commitment: {}", task.text)),
        next_event: now
            .schedule
            .next_event
            .as_ref()
            .map(|event| AppleScheduleEvent {
                title: event.title.clone(),
                start_ts: event.start_ts,
                end_ts: event.end_ts,
                location: event.location.clone(),
                leave_by_ts: event.leave_by_ts,
            }),
        upcoming_events: now
            .schedule
            .upcoming_events
            .iter()
            .map(|event| AppleScheduleEvent {
                title: event.title.clone(),
                start_ts: event.start_ts,
                end_ts: event.end_ts,
                location: event.location.clone(),
                leave_by_ts: event.leave_by_ts,
            })
            .collect(),
        reasons: non_empty_reasons(
            now.reasons.clone(),
            "Apple schedule answers are derived from backend Now output.".to_string(),
        ),
    }
}

fn non_empty_reasons(mut reasons: Vec<String>, fallback: String) -> Vec<String> {
    if reasons.is_empty() {
        reasons.push(fallback);
    }
    reasons
}

fn describe_schedule_event(event: &AppleScheduleEvent) -> String {
    match &event.location {
        Some(location) => format!("Starts at {} in {}", event.start_ts, location),
        None => format!("Starts at {}", event.start_ts),
    }
}

fn describe_schedule_event_from_now(event: &now::NowEventOutput) -> String {
    match &event.location {
        Some(location) => format!("Starts at {} in {}", event.start_ts, location),
        None => format!("Starts at {}", event.start_ts),
    }
}

fn parse_snooze_minutes(transcript: &str) -> Option<u32> {
    for token in transcript
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-')
        .filter(|token| !token.is_empty())
    {
        if let Ok(minutes) = token.parse::<u32>() {
            return Some(minutes);
        }
        let normalized = token.to_ascii_lowercase();
        let minutes = match normalized.as_str() {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            "ten" => Some(10),
            "fifteen" => Some(15),
            "twenty" => Some(20),
            "thirty" => Some(30),
            "forty" => Some(40),
            "forty-five" | "fortyfive" => Some(45),
            "fifty" => Some(50),
            "sixty" => Some(60),
            _ => None,
        };
        if minutes.is_some() {
            return minutes;
        }
    }
    None
}

fn surface_source_device(surface: AppleClientSurface) -> &'static str {
    match surface {
        AppleClientSurface::IosVoice => "apple_ios_voice",
        AppleClientSurface::IosCapture => "apple_ios_capture",
        AppleClientSurface::WatchBriefing => "apple_watch_briefing",
        AppleClientSurface::WatchQuickAction => "apple_watch_quick_action",
        AppleClientSurface::MacContext => "apple_mac_context",
    }
}

fn intent_token(intent: &AppleVoiceIntent) -> &'static str {
    match intent {
        AppleVoiceIntent::Capture => "capture",
        AppleVoiceIntent::MorningBriefing => "morning_briefing",
        AppleVoiceIntent::CurrentSchedule => "current_schedule",
        AppleVoiceIntent::NextCommitment => "next_commitment",
        AppleVoiceIntent::ActiveNudges => "active_nudges",
        AppleVoiceIntent::ExplainWhy => "explain_why",
        AppleVoiceIntent::BehaviorSummary => "behavior_summary",
        AppleVoiceIntent::CompleteCommitment => "complete_commitment",
        AppleVoiceIntent::SnoozeNudge => "snooze_nudge",
    }
}
