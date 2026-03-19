use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::{AppleBehaviorMetric, AppleBehaviorSummary, AppleBehaviorSummaryScope};
use vel_storage::{SignalRecord, Storage};

use crate::{
    errors::AppError,
    services::timezone::{resolve_timezone, same_local_day, start_of_local_day_timestamp},
};

pub const SUPPORTED_METRIC_KEYS: [&str; 3] = ["step_count", "stand_hours", "exercise_minutes"];

pub async fn get_summary(
    storage: &Storage,
    _config: &AppConfig,
) -> Result<Option<AppleBehaviorSummary>, AppError> {
    let timezone = resolve_timezone(storage).await?;
    let now = OffsetDateTime::now_utc();
    let now_ts = now.unix_timestamp();
    let start_of_day = start_of_local_day_timestamp(&timezone, now)?;
    let signals = storage
        .list_signals(Some("health_metric"), Some(start_of_day), 128)
        .await?;

    let mut selected = Vec::new();
    for metric_key in SUPPORTED_METRIC_KEYS {
        let signal = signals
            .iter()
            .filter(|signal| signal.timestamp >= start_of_day)
            .filter(|signal| {
                is_supported_metric(metric_type(signal)) && metric_type(signal) == metric_key
            })
            .filter_map(|signal| {
                OffsetDateTime::from_unix_timestamp(signal.timestamp)
                    .ok()
                    .filter(|timestamp| same_local_day(&timezone, *timestamp, now))
                    .map(|_| signal)
            })
            .max_by_key(|signal| signal.timestamp);
        if let Some(signal) = signal {
            selected.push(metric_from_signal(metric_key, signal)?);
        }
    }

    if selected.is_empty() {
        return Ok(None);
    }

    let freshest_timestamp = selected
        .iter()
        .map(|metric| metric.recorded_at)
        .max()
        .unwrap_or(now_ts);
    let freshness_seconds = Some((now_ts - freshest_timestamp).max(0));
    let reasons = summary_reasons(&selected, freshness_seconds);

    Ok(Some(AppleBehaviorSummary {
        generated_at: now_ts,
        timezone: timezone.name,
        scope: AppleBehaviorSummaryScope::Daily,
        headline: headline_for_metrics(&selected),
        metrics: selected,
        reasons,
        freshness_seconds,
    }))
}

pub fn is_supported_metric(metric_type: &str) -> bool {
    SUPPORTED_METRIC_KEYS.contains(&metric_type.trim())
}

pub fn summary_to_source_activity(summary: &AppleBehaviorSummary) -> JsonValue {
    json!({
        "timestamp": summary.generated_at,
        "headline": summary.headline,
        "scope": "daily",
        "freshness_seconds": summary.freshness_seconds,
        "metrics": summary.metrics.iter().map(|metric| json!({
            "metric_key": metric.metric_key,
            "display_label": metric.display_label,
            "value": metric.value,
            "unit": metric.unit,
            "recorded_at": metric.recorded_at,
            "reasons": metric.reasons,
        })).collect::<Vec<_>>(),
        "reasons": summary.reasons,
    })
}

fn metric_from_signal(
    metric_key: &str,
    signal: &SignalRecord,
) -> Result<AppleBehaviorMetric, AppError> {
    let payload = &signal.payload_json;
    let value = numeric_value(payload.get("value"))
        .ok_or_else(|| AppError::internal(format!("health metric {metric_key} is not numeric")))?;
    let unit = payload
        .get("unit")
        .and_then(JsonValue::as_str)
        .unwrap_or(default_unit(metric_key))
        .to_string();
    let source_app = payload
        .get("source_app")
        .and_then(JsonValue::as_str)
        .unwrap_or("Health");
    let device = payload
        .get("device")
        .and_then(JsonValue::as_str)
        .unwrap_or("Apple device");

    Ok(AppleBehaviorMetric {
        metric_key: metric_key.to_string(),
        display_label: display_label(metric_key).to_string(),
        value,
        unit,
        recorded_at: signal.timestamp,
        reasons: vec![
            format!(
                "{} came from {} on {} at {}.",
                display_label(metric_key),
                source_app,
                device,
                signal.timestamp
            ),
            format!(
                "Persisted health_metric signal {} backs this {} rollup.",
                signal.signal_id,
                display_label(metric_key).to_lowercase()
            ),
        ],
    })
}

fn metric_type(signal: &SignalRecord) -> &str {
    signal
        .payload_json
        .get("metric_type")
        .and_then(JsonValue::as_str)
        .unwrap_or_default()
}

fn numeric_value(value: Option<&JsonValue>) -> Option<f64> {
    match value {
        Some(JsonValue::Number(number)) => number.as_f64(),
        Some(JsonValue::String(text)) => text.parse().ok(),
        _ => None,
    }
}

fn display_label(metric_key: &str) -> &'static str {
    match metric_key {
        "step_count" => "Steps",
        "stand_hours" => "Stand hours",
        "exercise_minutes" => "Exercise minutes",
        _ => "Health metric",
    }
}

fn default_unit(metric_key: &str) -> &'static str {
    match metric_key {
        "step_count" => "count",
        "stand_hours" => "hours",
        "exercise_minutes" => "minutes",
        _ => "count",
    }
}

fn headline_for_metrics(metrics: &[AppleBehaviorMetric]) -> String {
    let labels = metrics
        .iter()
        .map(|metric| metric.display_label.to_lowercase())
        .collect::<Vec<_>>();
    format!(
        "Today's Apple behavior summary covers {}.",
        labels.join(", ")
    )
}

fn summary_reasons(metrics: &[AppleBehaviorMetric], freshness_seconds: Option<i64>) -> Vec<String> {
    let mut reasons = Vec::new();
    if let Some(freshness_seconds) = freshness_seconds {
        reasons.push(format!(
            "Freshness is based on the newest bounded health signal from {} seconds ago.",
            freshness_seconds
        ));
    }
    for metric in metrics {
        reasons.push(format!(
            "{} is included because a persisted {} signal was recorded at {}.",
            metric.display_label, metric.metric_key, metric.recorded_at
        ));
        if let Some(source_reason) = metric.reasons.first() {
            reasons.push(source_reason.clone());
        }
    }
    reasons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_metric_set_is_bounded() {
        assert!(is_supported_metric("step_count"));
        assert!(is_supported_metric("stand_hours"));
        assert!(is_supported_metric("exercise_minutes"));
        assert!(!is_supported_metric("heart_rate"));
    }
}
