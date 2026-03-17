//! POST /v1/evaluate — single orchestrated recompute-and-persist. Read-only routes must not call this.

use axum::extract::State;
use axum::Json;
use serde_json::json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, CurrentContextData, EvaluateResultData, WsEventType};

use crate::broadcast::WsEnvelope;
use crate::services::evaluate;
use crate::{errors::AppError, state::AppState};

pub async fn run_evaluate(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<EvaluateResultData>>, AppError> {
    let result = evaluate::run(&state.storage, &state.policy_config).await?;
    broadcast_context_updated(&state).await?;
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
    let Some((computed_at, context_json)) = state.storage.get_current_context().await? else {
        return Ok(());
    };
    let context = serde_json::from_str(&context_json).unwrap_or_else(|_| json!({}));
    let payload = serde_json::to_value(CurrentContextData {
        computed_at,
        context,
    })
    .map_err(|error| AppError::internal(format!("serialize current context for websocket: {error}")))?;
    let _ = state
        .broadcast_tx
        .send(WsEnvelope::new(WsEventType::ContextUpdated, payload));
    Ok(())
}
