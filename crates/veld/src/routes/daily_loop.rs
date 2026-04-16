use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
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
use vel_core::DailyLoopPhase;

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
    let menu = services::daily_loop_overdue::menu(
        &state.storage,
        &session_id,
        services::daily_loop_overdue::OverdueMenuInput {
            today: request.today,
            include_vel_guess: request.include_vel_guess,
            limit: request.limit,
        },
    )
    .await?;
    Ok(response::success(map_overdue_menu(menu)))
}

pub async fn overdue_confirm(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<DailyLoopOverdueConfirmRequestData>,
) -> Result<Json<ApiResponse<DailyLoopOverdueConfirmResponseData>>, AppError> {
    let confirm = services::daily_loop_overdue::confirm(
        &state.storage,
        &session_id,
        services::daily_loop_overdue::OverdueConfirmInput {
            commitment_id: request.commitment_id,
            action: map_overdue_action_from_data(request.action),
            payload: request.payload.map(|payload| {
                services::daily_loop_overdue::OverdueReschedulePayload {
                    due_at: payload.due_at,
                    source: payload.source,
                }
            }),
            operator_reason: request.operator_reason,
        },
    )
    .await?;
    Ok(response::success(DailyLoopOverdueConfirmResponseData {
        proposal_id: confirm.proposal_id,
        confirmation_token: confirm.confirmation_token,
        requires_confirmation: confirm.requires_confirmation,
        write_scope: confirm.write_scope,
        idempotency_hint: confirm.idempotency_hint,
    }))
}

pub async fn overdue_apply(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<DailyLoopOverdueApplyRequestData>,
) -> Result<Json<ApiResponse<DailyLoopOverdueApplyResponseData>>, AppError> {
    let applied = services::daily_loop_overdue::apply(
        &state.storage,
        &session_id,
        services::daily_loop_overdue::OverdueApplyInput {
            proposal_id: request.proposal_id,
            idempotency_key: request.idempotency_key,
            confirmation_token: request.confirmation_token,
        },
    )
    .await?;
    Ok(response::success(map_overdue_apply(applied)))
}

pub async fn overdue_undo(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(request): Json<DailyLoopOverdueUndoRequestData>,
) -> Result<Json<ApiResponse<DailyLoopOverdueUndoResponseData>>, AppError> {
    let undone = services::daily_loop_overdue::undo(
        &state.storage,
        &session_id,
        services::daily_loop_overdue::OverdueUndoInput {
            action_event_id: request.action_event_id,
            idempotency_key: request.idempotency_key,
        },
    )
    .await?;
    Ok(response::success(map_overdue_undo(undone)))
}

fn map_overdue_menu(
    menu: services::daily_loop_overdue::OverdueMenu,
) -> DailyLoopOverdueMenuResponseData {
    DailyLoopOverdueMenuResponseData {
        session_id: menu.session_id,
        items: menu
            .items
            .into_iter()
            .map(|item| DailyLoopOverdueMenuItemData {
                commitment_id: item.commitment_id,
                title: item.title,
                due_at: item.due_at,
                actions: item
                    .actions
                    .into_iter()
                    .map(map_overdue_action_to_data)
                    .collect(),
                vel_due_guess: item
                    .vel_due_guess
                    .map(|guess| DailyLoopOverdueVelGuessData {
                        suggested_due_at: guess.suggested_due_at,
                        confidence: match guess.confidence {
                            services::daily_loop_overdue::OverdueGuessConfidence::Low => {
                                DailyLoopOverdueGuessConfidenceData::Low
                            }
                            services::daily_loop_overdue::OverdueGuessConfidence::Medium => {
                                DailyLoopOverdueGuessConfidenceData::Medium
                            }
                            services::daily_loop_overdue::OverdueGuessConfidence::High => {
                                DailyLoopOverdueGuessConfidenceData::High
                            }
                        },
                        reason: guess.reason,
                    }),
            })
            .collect(),
    }
}

fn map_overdue_apply(
    applied: services::daily_loop_overdue::OverdueApplyOutput,
) -> DailyLoopOverdueApplyResponseData {
    DailyLoopOverdueApplyResponseData {
        applied: applied.applied,
        action_event_id: applied.action_event_id,
        run_id: applied.run_id,
        before: map_overdue_snapshot(applied.before),
        after: map_overdue_snapshot(applied.after),
        undo_supported: applied.undo_supported,
    }
}

fn map_overdue_undo(
    undone: services::daily_loop_overdue::OverdueUndoOutput,
) -> DailyLoopOverdueUndoResponseData {
    DailyLoopOverdueUndoResponseData {
        undone: undone.undone,
        run_id: undone.run_id,
        before: map_overdue_snapshot(undone.before),
        after: map_overdue_snapshot(undone.after),
    }
}

fn map_overdue_snapshot(
    snapshot: services::daily_loop_overdue::OverdueStateSnapshot,
) -> DailyLoopOverdueStateSnapshotData {
    DailyLoopOverdueStateSnapshotData {
        due_at: snapshot.due_at,
        status: snapshot.status,
    }
}

fn map_overdue_action_from_data(
    action: DailyLoopOverdueActionData,
) -> services::daily_loop_overdue::OverdueAction {
    match action {
        DailyLoopOverdueActionData::Close => services::daily_loop_overdue::OverdueAction::Close,
        DailyLoopOverdueActionData::Reschedule => {
            services::daily_loop_overdue::OverdueAction::Reschedule
        }
        DailyLoopOverdueActionData::BackToInbox => {
            services::daily_loop_overdue::OverdueAction::BackToInbox
        }
        DailyLoopOverdueActionData::Tombstone => {
            services::daily_loop_overdue::OverdueAction::Tombstone
        }
    }
}

fn map_overdue_action_to_data(
    action: services::daily_loop_overdue::OverdueAction,
) -> DailyLoopOverdueActionData {
    match action {
        services::daily_loop_overdue::OverdueAction::Close => DailyLoopOverdueActionData::Close,
        services::daily_loop_overdue::OverdueAction::Reschedule => {
            DailyLoopOverdueActionData::Reschedule
        }
        services::daily_loop_overdue::OverdueAction::BackToInbox => {
            DailyLoopOverdueActionData::BackToInbox
        }
        services::daily_loop_overdue::OverdueAction::Tombstone => {
            DailyLoopOverdueActionData::Tombstone
        }
    }
}
