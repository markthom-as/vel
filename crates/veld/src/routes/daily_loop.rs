use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use uuid::Uuid;
use vel_api_types::{
    ApiResponse, DailyLoopCheckInEventData, DailyLoopCheckInEventsQueryData,
    DailyLoopCheckInSkipRequestData, DailyLoopCheckInSkipResponseData,
    DailyLoopCheckInSubmitRequestData, DailyLoopCheckInSubmitResponseData,
    DailyLoopOverdueActionData, DailyLoopOverdueApplyRequestData,
    DailyLoopOverdueApplyResponseData, DailyLoopOverdueConfirmRequestData,
    DailyLoopOverdueConfirmResponseData, DailyLoopOverdueGuessConfidenceData,
    DailyLoopOverdueMenuItemData, DailyLoopOverdueMenuRequestData,
    DailyLoopOverdueMenuResponseData, DailyLoopOverdueStateSnapshotData,
    DailyLoopOverdueUndoRequestData, DailyLoopOverdueUndoResponseData,
    DailyLoopOverdueVelGuessData, DailyLoopPhaseData, DailyLoopSessionData,
    DailyLoopStartRequestData, DailyLoopTurnRequestData,
};
use vel_core::{CommitmentStatus, DailyLoopPhase, RunEventType, RunId, RunKind, RunStatus};

use crate::{errors::AppError, routes::response, services, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ActiveSessionQuery {
    pub session_date: String,
    pub phase: DailyLoopPhaseData,
}

pub async fn start_session(
    State(state): State<AppState>,
    Json(request): Json<DailyLoopStartRequestData>,
) -> Result<Json<ApiResponse<DailyLoopSessionData>>, AppError> {
    let session =
        services::daily_loop::start_session(&state.storage, &state.config, request.into()).await?;
    Ok(response::success(session.into()))
}

pub async fn active_session(
    State(state): State<AppState>,
    Query(query): Query<ActiveSessionQuery>,
) -> Result<Json<ApiResponse<Option<DailyLoopSessionData>>>, AppError> {
    let session = services::daily_loop::get_active_session(
        &state.storage,
        &query.session_date,
        query.phase.into(),
    )
    .await?;
    Ok(response::success(session.map(Into::into)))
}

pub async fn list_session_check_in_events(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Query(query): Query<DailyLoopCheckInEventsQueryData>,
) -> Result<Json<ApiResponse<Vec<DailyLoopCheckInEventData>>>, AppError> {
    state
        .storage
        .get_daily_session(session_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("daily loop session not found"))?;

    let records = services::check_in::list_session_check_in_events(
        &state.storage,
        &session_id,
        query.check_in_type.as_deref(),
        query.session_phase.as_deref(),
        query.include_skipped,
        query.limit.unwrap_or(50),
    )
    .await?;

    let events = records
        .into_iter()
        .map(|record| DailyLoopCheckInEventData {
            event_id: record.event_id,
            session_id: record.session_id,
            prompt_id: record.prompt_id,
            check_in_type: record.check_in_type,
            session_phase: record.session_phase,
            source: record.source,
            answered_at: record.answered_at,
            text: record.text,
            scale: record.scale,
            scale_min: record.scale_min,
            scale_max: record.scale_max,
            keywords_json: record.keywords_json,
            confidence: record.confidence,
            schema_version: record.schema_version,
            skipped: record.skipped,
            skip_reason_code: record.skip_reason_code,
            skip_reason_text: record.skip_reason_text,
            replaced_by_event_id: record.replaced_by_event_id,
            meta_json: record.meta_json,
            created_at: record.created_at,
            updated_at: record.updated_at,
            run_id: record.run_id,
        })
        .collect();

    Ok(response::success(events))
}

pub async fn submit_check_in(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<DailyLoopCheckInSubmitRequestData>,
) -> Result<Json<ApiResponse<DailyLoopCheckInSubmitResponseData>>, AppError> {
    let session = state
        .storage
        .get_daily_session(session_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("daily loop session not found"))?;
    let session_phase = match request.session_phase.trim() {
        "morning" => "morning".to_string(),
        "standup" => "standup".to_string(),
        _ => {
            return Err(AppError::bad_request(
                "session_phase must be `morning` or `standup`",
            ))
        }
    };
    let expected_session_phase = match session.session.phase {
        DailyLoopPhase::MorningOverview => "morning".to_string(),
        DailyLoopPhase::Standup => "standup".to_string(),
    };
    if session_phase != expected_session_phase {
        return Err(AppError::bad_request(
            "session_phase does not match active session phase",
        ));
    }

    let request = services::check_in::DailyCheckInSubmitInput {
        check_in_type: request.check_in_type,
        session_phase,
        source: request.source,
        prompt_id: request.prompt_id,
        answered_at: request.answered_at,
        text: request.text,
        scale: request.scale,
        keywords: request.keywords,
        confidence: request.confidence,
        skipped: request.skipped,
        skip_reason_code: request.skip_reason_code,
        skip_reason_text: request.skip_reason_text,
        replace_if_conflict: request.replace_if_conflict,
        run_id: None,
    };

    let result = services::check_in::submit_check_in(&state.storage, &session_id, request).await?;

    Ok(response::success(DailyLoopCheckInSubmitResponseData {
        check_in_event_id: result.check_in_event_id,
        session_id,
        status: "recorded".to_string(),
        supersedes_event_id: result.supersedes_event_id,
    }))
}

pub async fn skip_check_in(
    State(state): State<AppState>,
    Path(check_in_event_id): Path<String>,
    Json(request): Json<DailyLoopCheckInSkipRequestData>,
) -> Result<Json<ApiResponse<DailyLoopCheckInSkipResponseData>>, AppError> {
    let request = services::check_in::DailyCheckInSkipInput {
        source: request.source,
        answered_at: request.answered_at,
        reason_code: request.reason_code,
        reason_text: request.reason_text,
    };
    let result =
        services::check_in::skip_check_in(&state.storage, &check_in_event_id, request).await?;

    Ok(response::success(DailyLoopCheckInSkipResponseData {
        check_in_event_id: result.check_in_event_id,
        session_id: result.session_id,
        status: "skipped".to_string(),
        supersedes_event_id: result.supersedes_event_id,
    }))
}

pub async fn submit_turn(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(mut request): Json<DailyLoopTurnRequestData>,
) -> Result<Json<ApiResponse<DailyLoopSessionData>>, AppError> {
    request.session_id = session_id;
    let session = services::daily_loop::submit_turn(&state.storage, request.into()).await?;
    Ok(response::success(session.into()))
}

pub async fn overdue_menu(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<DailyLoopOverdueMenuRequestData>,
) -> Result<Json<ApiResponse<DailyLoopOverdueMenuResponseData>>, AppError> {
    ensure_standup_session(&state, &session_id).await?;
    let now = OffsetDateTime::now_utc();
    let overdue = state
        .storage
        .list_commitments(Some(CommitmentStatus::Open), None, None, request.limit)
        .await?
        .into_iter()
        .filter(|commitment| commitment.due_at.is_some_and(|due| due < now))
        .collect::<Vec<_>>();
    let mut overdue = overdue;
    overdue.sort_by_key(|commitment| {
        commitment
            .due_at
            .map(|due| due.unix_timestamp())
            .unwrap_or(0)
    });

    let items = overdue
        .into_iter()
        .map(|commitment| {
            let guess = if request.include_vel_guess {
                Some(build_vel_guess(commitment.due_at, now))
            } else {
                None
            };
            DailyLoopOverdueMenuItemData {
                commitment_id: commitment.id.to_string(),
                title: commitment.text,
                due_at: commitment.due_at.and_then(|due| due.format(&Rfc3339).ok()),
                actions: vec![
                    DailyLoopOverdueActionData::Close,
                    DailyLoopOverdueActionData::Reschedule,
                    DailyLoopOverdueActionData::BackToInbox,
                    DailyLoopOverdueActionData::Tombstone,
                ],
                vel_due_guess: guess,
            }
        })
        .collect::<Vec<_>>();

    Ok(response::success(DailyLoopOverdueMenuResponseData {
        session_id,
        items,
    }))
}

pub async fn overdue_confirm(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<DailyLoopOverdueConfirmRequestData>,
) -> Result<Json<ApiResponse<DailyLoopOverdueConfirmResponseData>>, AppError> {
    ensure_standup_session(&state, &session_id).await?;
    let commitment = state
        .storage
        .get_commitment_by_id(request.commitment_id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    if !matches!(commitment.status, CommitmentStatus::Open) {
        return Err(AppError::bad_request(
            "only open commitments can be changed through overdue workflow",
        ));
    }
    if matches!(request.action, DailyLoopOverdueActionData::Reschedule) && request.payload.is_none()
    {
        return Err(AppError::bad_request(
            "reschedule action requires payload with due_at",
        ));
    }
    if let Some(payload) = request.payload.as_ref() {
        let _ = OffsetDateTime::parse(&payload.due_at, &Rfc3339)
            .map_err(|_| AppError::bad_request("payload.due_at must be RFC3339"))?;
    }

    let proposal_id = format!("ovdp_{}", Uuid::new_v4().simple());
    let confirmation_token = format!("confirm:{proposal_id}");
    let write_scope = write_scope_for_action(request.action, request.commitment_id.trim());
    let idempotency_hint = format!(
        "ovd:{}:{}:{}",
        session_id,
        request.commitment_id.trim(),
        action_label(request.action)
    );
    let metadata = OverdueProposalMetadata {
        session_id: session_id.clone(),
        commitment_id: request.commitment_id.trim().to_string(),
        action: request.action,
        payload: request.payload.clone(),
        operator_reason: request.operator_reason.clone(),
        before: snapshot_from_commitment(&commitment),
        confirmation_token: confirmation_token.clone(),
        idempotency_hint: idempotency_hint.clone(),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
    };
    let metadata_json = serde_json::to_string(&metadata)
        .map_err(|error| AppError::internal(format!("serialize proposal metadata: {error}")))?;
    state
        .storage
        .insert_thread(
            &proposal_id,
            "daily_loop_overdue_proposal",
            &format!("Overdue {}", action_label(request.action)),
            "confirmed",
            &metadata_json,
        )
        .await?;

    Ok(response::success(DailyLoopOverdueConfirmResponseData {
        proposal_id,
        confirmation_token,
        requires_confirmation: true,
        write_scope,
        idempotency_hint,
    }))
}

pub async fn overdue_apply(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<DailyLoopOverdueApplyRequestData>,
) -> Result<Json<ApiResponse<DailyLoopOverdueApplyResponseData>>, AppError> {
    ensure_standup_session(&state, &session_id).await?;
    let idempotency_thread_id = format!(
        "thr_ovd_apply_{}",
        normalize_idempotency_key(&request.idempotency_key)
    );
    if let Some(cached) = load_idempotent_apply_response(&state, &idempotency_thread_id).await? {
        return Ok(response::success(cached));
    }

    let proposal = load_overdue_proposal(&state, request.proposal_id.trim()).await?;
    if proposal.session_id != session_id {
        return Err(AppError::bad_request(
            "proposal does not belong to this session",
        ));
    }
    if proposal.confirmation_token != request.confirmation_token {
        return Err(AppError::conflict(
            "invalid confirmation token for proposal",
        ));
    }

    let commitment = state
        .storage
        .get_commitment_by_id(proposal.commitment_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let before = snapshot_from_commitment(&commitment);

    let (new_status, due_update) = apply_target_for_action(&proposal)?;
    state
        .storage
        .update_commitment(
            proposal.commitment_id.as_str(),
            None,
            Some(new_status),
            due_update,
            None,
            None,
            None,
        )
        .await?;
    let updated = state
        .storage
        .get_commitment_by_id(proposal.commitment_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found after apply"))?;
    let after = snapshot_from_commitment(&updated);

    let run_id = record_overdue_mutation_run(
        &state,
        "overdue_apply",
        &json!({
            "proposal_id": request.proposal_id,
            "action": action_label(proposal.action),
            "idempotency_key": request.idempotency_key,
            "before": before,
            "after": after
        }),
    )
    .await?;
    let action_event_id = format!("ovdevt_{}", Uuid::new_v4().simple());
    let action_metadata = OverdueActionEventMetadata {
        session_id: session_id.clone(),
        commitment_id: proposal.commitment_id.clone(),
        action: proposal.action,
        before: before.clone(),
        after: after.clone(),
        run_id: run_id.clone(),
        applied_at: OffsetDateTime::now_utc().unix_timestamp(),
        undone: false,
    };
    state
        .storage
        .insert_thread(
            &action_event_id,
            "daily_loop_overdue_action",
            &format!("Overdue {}", action_label(proposal.action)),
            "applied",
            &serde_json::to_string(&action_metadata).map_err(|error| {
                AppError::internal(format!("serialize action metadata: {error}"))
            })?,
        )
        .await?;
    state
        .storage
        .update_thread_status(request.proposal_id.trim(), "applied")
        .await?;

    let response_data = DailyLoopOverdueApplyResponseData {
        applied: true,
        action_event_id,
        run_id,
        before,
        after,
        undo_supported: true,
    };
    store_idempotent_apply_response(
        &state,
        &idempotency_thread_id,
        &request.idempotency_key,
        &response_data,
    )
    .await?;

    Ok(response::success(response_data))
}

pub async fn overdue_undo(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<DailyLoopOverdueUndoRequestData>,
) -> Result<Json<ApiResponse<DailyLoopOverdueUndoResponseData>>, AppError> {
    ensure_standup_session(&state, &session_id).await?;
    let idempotency_thread_id = format!(
        "thr_ovd_undo_{}",
        normalize_idempotency_key(&request.idempotency_key)
    );
    if let Some(cached) = load_idempotent_undo_response(&state, &idempotency_thread_id).await? {
        return Ok(response::success(cached));
    }
    let action_event = load_overdue_action_event(&state, request.action_event_id.trim()).await?;
    if action_event.session_id != session_id {
        return Err(AppError::bad_request(
            "action event does not belong to this session",
        ));
    }
    if action_event.undone {
        return Err(AppError::bad_request("action is already undone"));
    }

    let commitment = state
        .storage
        .get_commitment_by_id(action_event.commitment_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found"))?;
    let before = snapshot_from_commitment(&commitment);

    let restored_status = CommitmentStatus::from_str(action_event.before.status.as_str())
        .map_err(|_| AppError::bad_request("stored undo status is invalid"))?;
    let restored_due = parse_optional_due_at(action_event.before.due_at.as_deref())?;
    state
        .storage
        .update_commitment(
            action_event.commitment_id.as_str(),
            None,
            Some(restored_status),
            Some(restored_due),
            None,
            None,
            None,
        )
        .await?;
    let updated = state
        .storage
        .get_commitment_by_id(action_event.commitment_id.as_str())
        .await?
        .ok_or_else(|| AppError::not_found("commitment not found after undo"))?;
    let after = snapshot_from_commitment(&updated);

    let run_id = record_overdue_mutation_run(
        &state,
        "overdue_undo",
        &json!({
            "action_event_id": request.action_event_id,
            "idempotency_key": request.idempotency_key,
            "before": before,
            "after": after
        }),
    )
    .await?;

    let response_data = DailyLoopOverdueUndoResponseData {
        undone: true,
        run_id,
        before,
        after,
    };
    let updated_action = OverdueActionEventMetadata {
        undone: true,
        ..action_event
    };
    state
        .storage
        .update_thread_status(request.action_event_id.trim(), "undone")
        .await?;
    state
        .storage
        .update_thread_metadata(
            request.action_event_id.trim(),
            &serde_json::to_string(&updated_action).map_err(|error| {
                AppError::internal(format!("serialize updated action metadata: {error}"))
            })?,
        )
        .await?;
    store_idempotent_undo_response(
        &state,
        &idempotency_thread_id,
        &request.idempotency_key,
        &response_data,
    )
    .await?;

    Ok(response::success(response_data))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OverdueProposalMetadata {
    session_id: String,
    commitment_id: String,
    action: DailyLoopOverdueActionData,
    payload: Option<vel_api_types::DailyLoopOverdueReschedulePayloadData>,
    operator_reason: Option<String>,
    before: DailyLoopOverdueStateSnapshotData,
    confirmation_token: String,
    idempotency_hint: String,
    created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OverdueActionEventMetadata {
    session_id: String,
    commitment_id: String,
    action: DailyLoopOverdueActionData,
    before: DailyLoopOverdueStateSnapshotData,
    after: DailyLoopOverdueStateSnapshotData,
    run_id: String,
    applied_at: i64,
    undone: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredApplyIdempotency {
    idempotency_key: String,
    response: DailyLoopOverdueApplyResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredUndoIdempotency {
    idempotency_key: String,
    response: DailyLoopOverdueUndoResponseData,
}

async fn ensure_standup_session(state: &AppState, session_id: &str) -> Result<(), AppError> {
    let record = state
        .storage
        .get_daily_session(session_id)
        .await?
        .ok_or_else(|| AppError::not_found("daily loop session not found"))?;
    if !matches!(record.session.phase, DailyLoopPhase::Standup) {
        return Err(AppError::bad_request(
            "overdue workflow is only supported for standup sessions",
        ));
    }
    Ok(())
}

fn build_vel_guess(
    due_at: Option<OffsetDateTime>,
    now: OffsetDateTime,
) -> DailyLoopOverdueVelGuessData {
    let base = due_at.unwrap_or(now);
    let suggested_due_at = base + Duration::days(1);
    DailyLoopOverdueVelGuessData {
        suggested_due_at: suggested_due_at
            .format(&Rfc3339)
            .unwrap_or_else(|_| suggested_due_at.to_string()),
        confidence: DailyLoopOverdueGuessConfidenceData::Medium,
        reason: "next free block + similar task duration".to_string(),
    }
}

fn snapshot_from_commitment(
    commitment: &vel_core::Commitment,
) -> DailyLoopOverdueStateSnapshotData {
    DailyLoopOverdueStateSnapshotData {
        due_at: commitment.due_at.and_then(|due| due.format(&Rfc3339).ok()),
        status: commitment.status.to_string(),
    }
}

fn write_scope_for_action(action: DailyLoopOverdueActionData, commitment_id: &str) -> Vec<String> {
    match action {
        DailyLoopOverdueActionData::Close | DailyLoopOverdueActionData::Tombstone => {
            vec![format!("commitment:{commitment_id}:status")]
        }
        DailyLoopOverdueActionData::Reschedule | DailyLoopOverdueActionData::BackToInbox => {
            vec![format!("commitment:{commitment_id}:due_at")]
        }
    }
}

fn action_label(action: DailyLoopOverdueActionData) -> &'static str {
    match action {
        DailyLoopOverdueActionData::Close => "close",
        DailyLoopOverdueActionData::Reschedule => "reschedule",
        DailyLoopOverdueActionData::BackToInbox => "back_to_inbox",
        DailyLoopOverdueActionData::Tombstone => "tombstone",
    }
}

fn normalize_idempotency_key(key: &str) -> String {
    let mut out = String::with_capacity(key.len().min(80));
    for ch in key.chars() {
        if out.len() >= 80 {
            break;
        }
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        "default".to_string()
    } else {
        out
    }
}

fn parse_optional_due_at(value: Option<&str>) -> Result<Option<OffsetDateTime>, AppError> {
    let Some(value) = value else {
        return Ok(None);
    };
    OffsetDateTime::parse(value, &Rfc3339)
        .map(Some)
        .map_err(|_| AppError::bad_request("stored due_at is not valid RFC3339"))
}

async fn load_overdue_proposal(
    state: &AppState,
    proposal_id: &str,
) -> Result<OverdueProposalMetadata, AppError> {
    let record = state
        .storage
        .get_thread_by_id(proposal_id)
        .await?
        .ok_or_else(|| AppError::not_found("proposal not found"))?;
    if record.1 != "daily_loop_overdue_proposal" {
        return Err(AppError::bad_request(
            "proposal id is not an overdue proposal",
        ));
    }
    serde_json::from_str::<OverdueProposalMetadata>(&record.4)
        .map_err(|error| AppError::internal(format!("invalid proposal metadata: {error}")))
}

async fn load_overdue_action_event(
    state: &AppState,
    action_event_id: &str,
) -> Result<OverdueActionEventMetadata, AppError> {
    let record = state
        .storage
        .get_thread_by_id(action_event_id)
        .await?
        .ok_or_else(|| AppError::not_found("action event not found"))?;
    if record.1 != "daily_loop_overdue_action" {
        return Err(AppError::bad_request(
            "action event id is not an overdue action event",
        ));
    }
    serde_json::from_str::<OverdueActionEventMetadata>(&record.4)
        .map_err(|error| AppError::internal(format!("invalid action-event metadata: {error}")))
}

fn apply_target_for_action(
    proposal: &OverdueProposalMetadata,
) -> Result<(CommitmentStatus, Option<Option<OffsetDateTime>>), AppError> {
    match proposal.action {
        DailyLoopOverdueActionData::Close => Ok((CommitmentStatus::Done, None)),
        DailyLoopOverdueActionData::Tombstone => Ok((CommitmentStatus::Cancelled, None)),
        DailyLoopOverdueActionData::BackToInbox => Ok((CommitmentStatus::Open, Some(None))),
        DailyLoopOverdueActionData::Reschedule => {
            let payload = proposal
                .payload
                .as_ref()
                .ok_or_else(|| AppError::bad_request("reschedule proposal is missing payload"))?;
            let due = OffsetDateTime::parse(&payload.due_at, &Rfc3339)
                .map_err(|_| AppError::bad_request("reschedule due_at must be RFC3339"))?;
            Ok((CommitmentStatus::Open, Some(Some(due))))
        }
    }
}

async fn record_overdue_mutation_run(
    state: &AppState,
    operation: &str,
    payload: &serde_json::Value,
) -> Result<String, AppError> {
    let run_id = RunId::new();
    state
        .storage
        .create_run(&run_id, RunKind::Agent, &json!({"operation": operation}))
        .await?;
    let now = OffsetDateTime::now_utc().unix_timestamp();
    state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Running,
            Some(now),
            None,
            None,
            None,
        )
        .await?;
    state
        .storage
        .append_run_event_auto(
            run_id.as_ref(),
            RunEventType::RunStarted,
            &json!({ "operation": operation }),
        )
        .await?;
    state
        .storage
        .append_run_event_auto(run_id.as_ref(), RunEventType::MutationProposed, payload)
        .await?;
    state
        .storage
        .append_run_event_auto(run_id.as_ref(), RunEventType::MutationCommitted, payload)
        .await?;
    state
        .storage
        .update_run_status(
            run_id.as_ref(),
            RunStatus::Succeeded,
            None,
            Some(OffsetDateTime::now_utc().unix_timestamp()),
            Some(payload),
            None,
        )
        .await?;
    Ok(run_id.to_string())
}

async fn load_idempotent_apply_response(
    state: &AppState,
    idempotency_thread_id: &str,
) -> Result<Option<DailyLoopOverdueApplyResponseData>, AppError> {
    let Some(record) = state
        .storage
        .get_thread_by_id(idempotency_thread_id)
        .await?
    else {
        return Ok(None);
    };
    let stored = serde_json::from_str::<StoredApplyIdempotency>(&record.4).map_err(|error| {
        AppError::internal(format!("invalid apply idempotency metadata: {error}"))
    })?;
    Ok(Some(stored.response))
}

async fn store_idempotent_apply_response(
    state: &AppState,
    idempotency_thread_id: &str,
    idempotency_key: &str,
    response: &DailyLoopOverdueApplyResponseData,
) -> Result<(), AppError> {
    let stored = StoredApplyIdempotency {
        idempotency_key: idempotency_key.to_string(),
        response: response.clone(),
    };
    state
        .storage
        .insert_thread(
            idempotency_thread_id,
            "daily_loop_overdue_apply_idempotency",
            "Overdue apply idempotency",
            "applied",
            &serde_json::to_string(&stored).map_err(|error| {
                AppError::internal(format!("serialize apply idempotency metadata: {error}"))
            })?,
        )
        .await?;
    Ok(())
}

async fn load_idempotent_undo_response(
    state: &AppState,
    idempotency_thread_id: &str,
) -> Result<Option<DailyLoopOverdueUndoResponseData>, AppError> {
    let Some(record) = state
        .storage
        .get_thread_by_id(idempotency_thread_id)
        .await?
    else {
        return Ok(None);
    };
    let stored = serde_json::from_str::<StoredUndoIdempotency>(&record.4).map_err(|error| {
        AppError::internal(format!("invalid undo idempotency metadata: {error}"))
    })?;
    Ok(Some(stored.response))
}

async fn store_idempotent_undo_response(
    state: &AppState,
    idempotency_thread_id: &str,
    idempotency_key: &str,
    response: &DailyLoopOverdueUndoResponseData,
) -> Result<(), AppError> {
    let stored = StoredUndoIdempotency {
        idempotency_key: idempotency_key.to_string(),
        response: response.clone(),
    };
    state
        .storage
        .insert_thread(
            idempotency_thread_id,
            "daily_loop_overdue_undo_idempotency",
            "Overdue undo idempotency",
            "undone",
            &serde_json::to_string(&stored).map_err(|error| {
                AppError::internal(format!("serialize undo idempotency metadata: {error}"))
            })?,
        )
        .await?;
    Ok(())
}
