//! **Evaluation orchestration (recompute-and-persist).**
//!
//! This is the single entry point for "compute current truth now": risk, inference, nudges, suggestions.
//! Read-only surfaces (explain, context/current, context/timeline, GET risk) must **not** call this module
//! or any of the recompute services (risk::run, inference::run, nudge_engine::evaluate, suggestions::evaluate_after_nudges).

use crate::{
    broadcast::WsEnvelope, errors::AppError, policy_config::PolicyConfig, state::AppState,
};
use serde_json::json;
use vel_api_types::{CurrentContextData, WsEventType};
use vel_storage::Storage;

/// Result of a full evaluation run.
pub struct EvaluateResult {
    pub inferred_states: u32,
    pub nudges_created_or_updated: u32,
}

/// Run full evaluation: risk → inference → nudges → suggestions. Recompute-and-persist only.
/// Call this from POST /v1/evaluate or explicit refresh; never from GET explain/context/risk.
pub async fn run(
    storage: &Storage,
    policy_config: &PolicyConfig,
) -> Result<EvaluateResult, AppError> {
    let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();

    let _ = crate::services::risk::run(storage, now_ts).await?;
    let states = crate::services::inference::run(storage).await?;
    let nudges = crate::services::nudge_engine::evaluate(storage, policy_config, states).await?;
    if let Err(e) =
        crate::services::suggestions::evaluate_after_nudges(storage, policy_config).await
    {
        tracing::warn!(error = %e, "suggestions evaluate_after_nudges");
    }

    Ok(EvaluateResult {
        inferred_states: states as u32,
        nudges_created_or_updated: nudges,
    })
}

pub async fn run_and_broadcast(state: &AppState) -> Result<EvaluateResult, AppError> {
    let result = run(&state.storage, &state.policy_config).await?;
    broadcast_context_updated(state).await?;
    Ok(result)
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
    .map_err(|error| {
        AppError::internal(format!("serialize current context for websocket: {error}"))
    })?;
    let _ = state
        .broadcast_tx
        .send(WsEnvelope::new(WsEventType::ContextUpdated, payload));
    Ok(())
}
