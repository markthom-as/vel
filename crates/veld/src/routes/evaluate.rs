use axum::extract::State;
use axum::Json;
use uuid::Uuid;
use vel_api_types::{ApiResponse, EvaluateResultData};

use crate::{errors::AppError, state::AppState};
use crate::services::{inference, nudge_engine, risk, suggestions};

pub async fn run_evaluate(State(state): State<AppState>) -> Result<Json<ApiResponse<EvaluateResultData>>, AppError> {
    let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
    let _ = risk::run(&state.storage, now_ts).await?;
    let states = inference::run(&state.storage).await?;
    let nudges = nudge_engine::evaluate(&state.storage, &state.policy_config, states).await?;
    if let Err(e) = suggestions::evaluate_after_nudges(&state.storage).await {
        tracing::warn!(error = %e, "suggestions evaluate_after_nudges");
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        EvaluateResultData {
            inferred_states: states as u32,
            nudges_created_or_updated: nudges,
        },
        request_id,
    )))
}
