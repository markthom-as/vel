use axum::{extract::State, Json};
use vel_api_types::{
    ApiResponse, AppleBehaviorSummaryData, AppleVoiceTurnRequestData, AppleVoiceTurnResponseData,
};

use crate::{errors::AppError, routes::response, services, state::AppState};

/// GET /v1/apple/behavior-summary
///
/// Returns the backend-owned bounded Apple behavior summary derived from persisted health_metric
/// signals. Apple clients render this summary rather than synthesizing local heuristics.
pub async fn apple_behavior_summary(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AppleBehaviorSummaryData>>, AppError> {
    let data = services::apple_behavior::get_summary(&state.storage, &state.config)
        .await?
        .ok_or_else(|| AppError::not_found("apple behavior summary is not available"))?;
    Ok(response::success(data.into()))
}

/// POST /v1/apple/voice/turn
///
/// Apple voice turns are backend-owned. Swift clients submit transcript + intent hints and render
/// the typed response; MorningBriefing intents start or resume the shared `/v1/daily-loop/*`
/// authority after transcript capture, while other schedule/query answers remain grounded in
/// backend `/v1/now` truth.
pub async fn apple_voice_turn(
    State(state): State<AppState>,
    Json(payload): Json<AppleVoiceTurnRequestData>,
) -> Result<Json<ApiResponse<AppleVoiceTurnResponseData>>, AppError> {
    let data = services::apple_voice::apple_voice_turn(&state, payload.into()).await?;
    Ok(response::success(data.into()))
}
