//! Health domain reducer: maps health_metric signals into CurrentContextV1 health summary fields.

use vel_core::CurrentContextV1;
use vel_storage::SignalRecord;

use crate::services::inference::SignalReducer;

pub struct HealthReducer;

impl SignalReducer for HealthReducer {
    fn name(&self) -> &'static str {
        "health"
    }

    fn reduce(&self, ctx: CurrentContextV1, signals: &[SignalRecord]) -> CurrentContextV1 {
        let latest_health = signals
            .iter()
            .filter(|s| s.signal_type == "health_metric")
            .max_by_key(|s| s.timestamp);

        let Some(signal) = latest_health else {
            return ctx;
        };

        let Some(metric_type) = signal
            .payload_json
            .get("metric_type")
            .and_then(serde_json::Value::as_str)
        else {
            return ctx;
        };

        let health_summary = serde_json::json!({
            "timestamp": signal.timestamp,
            "metric_type": metric_type,
            "value": signal.payload_json.get("value").cloned().unwrap_or(serde_json::Value::Null),
            "unit": signal.payload_json.get("unit").and_then(|v| v.as_str()),
            "source_app": signal.payload_json.get("source_app").and_then(|v| v.as_str()),
            "device": signal.payload_json.get("device").and_then(|v| v.as_str()),
        });

        CurrentContextV1 {
            health_summary: Some(health_summary),
            ..ctx
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
    fn health_reducer_returns_ctx_unchanged_when_no_health_signals() {
        let reducer = HealthReducer;
        let ctx = CurrentContextV1::default();
        let signals: Vec<SignalRecord> =
            vec![make_signal("sig_cal", "calendar_event", 1_700_000_000)];

        let result = reducer.reduce(ctx.clone(), &signals);

        assert!(result.health_summary.is_none());
    }

    #[test]
    fn health_reducer_populates_health_summary_from_health_metric_signal() {
        let reducer = HealthReducer;
        let ctx = CurrentContextV1::default();
        let mut signal = make_signal("sig_health", "health_metric", 1_700_000_000);
        signal.payload_json = json!({
            "metric_type": "heart_rate",
            "value": 72,
            "unit": "bpm",
            "source_app": "apple_health",
            "device": "apple_watch",
        });
        let signals = vec![signal];

        let result = reducer.reduce(ctx, &signals);

        let summary = result
            .health_summary
            .expect("health_summary should be populated");
        assert_eq!(summary["metric_type"], "heart_rate");
        assert_eq!(summary["unit"], "bpm");
    }

    #[test]
    fn health_reducer_uses_latest_signal_when_multiple_health_signals_present() {
        let reducer = HealthReducer;
        let ctx = CurrentContextV1::default();
        let mut old_signal = make_signal("sig_old", "health_metric", 1_700_000_000);
        old_signal.payload_json = json!({ "metric_type": "steps", "value": 1000 });
        let mut new_signal = make_signal("sig_new", "health_metric", 1_700_003_600);
        new_signal.payload_json = json!({ "metric_type": "heart_rate", "value": 72 });
        let signals = vec![old_signal, new_signal];

        let result = reducer.reduce(ctx, &signals);

        let summary = result
            .health_summary
            .expect("health_summary should be populated");
        assert_eq!(summary["metric_type"], "heart_rate");
    }
}
