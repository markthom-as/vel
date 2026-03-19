use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use vel_api_types::{
    ApiResponse, DailyLoopPhaseData, DailyLoopSessionData, DailyLoopStartRequestData,
    DailyLoopTurnRequestData,
};

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

pub async fn submit_turn(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(mut request): Json<DailyLoopTurnRequestData>,
) -> Result<Json<ApiResponse<DailyLoopSessionData>>, AppError> {
    request.session_id = session_id;
    let session = services::daily_loop::submit_turn(&state.storage, request.into()).await?;
    Ok(response::success(session.into()))
}
