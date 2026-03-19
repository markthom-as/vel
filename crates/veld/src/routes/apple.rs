use axum::{extract::State, Json};
use vel_api_types::{ApiResponse, AppleVoiceTurnRequestData, AppleVoiceTurnResponseData};

use crate::{errors::AppError, routes::response, services, state::AppState};

/// POST /v1/apple/voice/turn
///
/// Apple voice turns are backend-owned. Swift clients submit transcript + intent hints and render
/// the typed response; schedule/query answers remain grounded in backend `/v1/now` truth.
pub async fn apple_voice_turn(
    State(state): State<AppState>,
    Json(payload): Json<AppleVoiceTurnRequestData>,
) -> Result<Json<ApiResponse<AppleVoiceTurnResponseData>>, AppError> {
    let data = services::apple_voice::apple_voice_turn(&state, payload.into()).await?;
    Ok(response::success(data.into()))
}
