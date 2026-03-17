//! POST /v1/evaluate — single orchestrated recompute-and-persist. Read-only routes must not call this.

use crate::services::evaluate;
use crate::{broadcast::WsEnvelope, errors::AppError, state::AppState};
use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, CurrentContextData, EvaluateResultData, WsEventType};

pub async fn run_evaluate(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<EvaluateResultData>>, AppError> {
    let result = evaluate::run_and_broadcast(&state).await?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        EvaluateResultData {
            inferred_states: result.inferred_states,
            nudges_created_or_updated: result.nudges_created_or_updated,
        },
        request_id,
    )))
}

pub async fn broadcast_context_updated(state: &AppState) -> Result<(), AppError> {
    let Some(payload_data) = evaluate::get_context_updated_payload(state).await? else {
        return Ok(());
    };
    let payload = serde_json::to_value(CurrentContextData {
        computed_at: payload_data.computed_at,
        context: payload_data.context,
    })
    .map_err(|error| {
        AppError::internal(format!(
            "serialize context updated payload for websocket broadcast: {error}"
        ))
    })?;
    let _ = state
        .broadcast_tx
        .send(WsEnvelope::new(WsEventType::ContextUpdated, payload));
    Ok(())
}
