//! **Evaluation orchestration (recompute-and-persist).**
//!
//! This is the single entry point for "compute current truth now": risk, inference, nudges, suggestions.
//! Read-only surfaces (explain, context/current, context/timeline, GET risk) must **not** call this module
//! or any of the recompute services (risk::run, inference::run, nudge_engine::evaluate, suggestions::evaluate_after_nudges).

use crate::{
    broadcast::WsEnvelope, errors::AppError, policy_config::PolicyConfig, state::AppState,
};
use vel_storage::Storage;

pub const CONTEXT_UPDATED_WS_EVENT_TYPE: &str = "context:updated";

/// Result of a full evaluation run.
pub struct EvaluateResult {
    pub inferred_states: u32,
    pub nudges_created_or_updated: u32,
}

/// Service-local payload for context update broadcasts.
#[derive(serde::Serialize)]
pub struct ContextUpdatedBroadcastPayload {
    pub computed_at: i64,
    pub context: serde_json::Value,
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

pub async fn get_context_updated_payload(
    state: &AppState,
) -> Result<Option<ContextUpdatedBroadcastPayload>, AppError> {
    let Some((computed_at, context)) = state.storage.get_current_context().await? else {
        return Ok(None);
    };
    Ok(Some(ContextUpdatedBroadcastPayload {
        computed_at,
        context: context.into_json(),
    }))
}

pub async fn broadcast_context_updated(state: &AppState) -> Result<(), AppError> {
    let Some(payload_data) = get_context_updated_payload(state).await? else {
        return Ok(());
    };
    let payload = serde_json::to_value(payload_data).map_err(|error| {
        AppError::internal(format!("serialize current context for websocket: {error}"))
    })?;
    let _ = state
        .broadcast_tx
        .send(WsEnvelope::new(CONTEXT_UPDATED_WS_EVENT_TYPE, payload));
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use vel_core::ContextMigrator;

    #[test]
    fn current_context_v1_parses_typed_v1_shape() {
        let raw = json!({
            "mode": "day_mode",
            "morning_state": "engaged",
            "meds_status": "pending",
            "custom_field": { "ok": true }
        });

        let typed = ContextMigrator::from_json_value(raw.clone()).expect("typed context should parse");
        assert_eq!(typed.mode, "day_mode");
        assert_eq!(typed.morning_state, "engaged");
        assert_eq!(typed.meds_status, "pending");
        assert!(typed.extra.contains_key("custom_field"));
    }

    #[test]
    fn context_migrator_uses_defaults_for_missing_fields() {
        let typed = ContextMigrator::from_json_value(json!({})).expect("empty context should parse");
        assert_eq!(typed.mode, "");
        assert_eq!(typed.meds_status, "");
    }
}
