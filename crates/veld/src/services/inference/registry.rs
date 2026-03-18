//! ReducerRegistry: ordered pipeline of SignalReducers applied deterministically.

use vel_core::CurrentContextV1;
use vel_storage::SignalRecord;

use crate::services::inference::reducers::{
    calendar::CalendarReducer, git::GitActivityReducer, health::HealthReducer,
    messages::MessagesReducer,
};
use crate::services::inference::SignalReducer;

/// Registry of signal reducers applied in an explicit, replay-deterministic order.
pub struct ReducerRegistry {
    reducers: Vec<Box<dyn SignalReducer>>,
}

impl ReducerRegistry {
    /// Create registry with default reducer order.
    ///
    /// Reducer order is intentional and replay-deterministic:
    /// [CalendarReducer, MessagesReducer, HealthReducer, GitActivityReducer]
    pub fn new() -> Self {
        Self {
            reducers: vec![
                Box::new(CalendarReducer),
                Box::new(MessagesReducer),
                Box::new(HealthReducer),
                Box::new(GitActivityReducer),
            ],
        }
    }

    /// Apply all reducers in registration order, threading context state through each one.
    pub fn apply_all(&self, ctx: CurrentContextV1, signals: &[SignalRecord]) -> CurrentContextV1 {
        let mut ctx = ctx;
        for reducer in &self.reducers {
            tracing::trace!(reducer = reducer.name(), "applying signal reducer");
            ctx = reducer.reduce(ctx, signals);
        }
        ctx
    }

    /// Return reducer names in registration order (for diagnostics and tests).
    pub fn reducer_names(&self) -> Vec<&'static str> {
        self.reducers.iter().map(|r| r.name()).collect()
    }
}

impl Default for ReducerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use vel_storage::SignalRecord;

    fn make_signal(signal_id: &str, signal_type: &str, timestamp: i64) -> SignalRecord {
        SignalRecord {
            signal_id: signal_id.to_string(),
            signal_type: signal_type.to_string(),
            source: "test".to_string(),
            source_ref: None,
            timestamp,
            payload_json: json!({}),
            created_at: timestamp,
        }
    }

    #[test]
    fn registry_has_four_reducers_in_documented_order() {
        let registry = ReducerRegistry::new();
        let names = registry.reducer_names();
        assert_eq!(names, ["calendar", "messages", "health", "git_activity"]);
    }

    #[test]
    fn registry_apply_all_is_replay_deterministic() {
        let registry = ReducerRegistry::new();
        let now_ts = 1_700_000_000;
        let ctx = CurrentContextV1 {
            computed_at: now_ts,
            ..CurrentContextV1::default()
        };
        let mut git_signal = make_signal("sig_git", "git_activity", now_ts - 60);
        git_signal.payload_json = json!({ "repo_name": "vel", "branch": "main" });
        let mut health_signal = make_signal("sig_health", "health_metric", now_ts - 120);
        health_signal.payload_json = json!({ "metric_type": "heart_rate", "value": 72 });
        let signals = vec![git_signal, health_signal];

        let result1 = registry.apply_all(ctx.clone(), &signals);
        let result2 = registry.apply_all(ctx.clone(), &signals);

        // Structural equality: same input always produces same output
        assert_eq!(
            serde_json::to_value(&result1).unwrap(),
            serde_json::to_value(&result2).unwrap()
        );
    }

    #[test]
    fn registry_apply_all_passes_context_through_unchanged_when_no_signals() {
        let registry = ReducerRegistry::new();
        let ctx = CurrentContextV1 {
            computed_at: 1_700_000_000,
            morning_state: "engaged".to_string(),
            mode: "morning_mode".to_string(),
            ..CurrentContextV1::default()
        };

        let result = registry.apply_all(ctx.clone(), &[]);

        assert_eq!(result.morning_state, ctx.morning_state);
        assert_eq!(result.mode, ctx.mode);
    }
}
