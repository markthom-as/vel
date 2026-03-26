use serde_json::{json, Value as JsonValue};
use time::OffsetDateTime;
use vel_config::AppConfig;
use vel_core::{AppleBehaviorMetric, AppleBehaviorSummary, AppleBehaviorSummaryScope};
use vel_storage::{SignalRecord, Storage};

use crate::{
    errors::AppError,
    services::timezone::{
        resolve_timezone, same_local_day, start_of_local_day_timestamp, ResolvedTimeZone,
    },
};

pub const SUPPORTED_METRIC_KEYS: [&str; 3] = ["step_count", "stand_hours", "exercise_minutes"];
pub const SUPPORTED_WATCH_SIGNAL_KEYS: [&str; 7] = [
    "drifting",
    "on_track",
    "need_focus",
    "wake",
    "waking_up",
    "heart_rate",
    "motion",
];

#[derive(Debug, Clone)]
pub struct RecentWatchSignalSummary {
    pub signal_type: String,
    pub timestamp: i64,
    pub label: String,
    pub note: Option<String>,
}

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

    let watch_signals =
        select_recent_watch_signals(storage, start_of_day, &timezone, now).await?;

    if selected.is_empty() && watch_signals.is_empty() {
        return Ok(None);
    }

    let freshest_timestamp = selected
        .iter()
        .map(|metric| metric.recorded_at)
        .chain(watch_signals.iter().map(|signal| signal.timestamp))
        .max()
        .unwrap_or(now_ts);
    let freshness_seconds = Some((now_ts - freshest_timestamp).max(0));
    let reasons = summary_reasons(&selected, &watch_signals, freshness_seconds);

    Ok(Some(AppleBehaviorSummary {
        generated_at: now_ts,
        timezone: timezone.name,
        scope: AppleBehaviorSummaryScope::Daily,
        headline: headline_for_summary(&selected, &watch_signals),
        metrics: selected,
        reasons,
        freshness_seconds,
    }))
}

pub async fn recent_watch_signal_summaries(
    storage: &Storage,
) -> Result<Vec<RecentWatchSignalSummary>, AppError> {
    let timezone = resolve_timezone(storage).await?;
    let now = OffsetDateTime::now_utc();
    let start_of_day = start_of_local_day_timestamp(&timezone, now)?;
    Ok(select_recent_watch_signals(storage, start_of_day, &timezone, now)
        .await?
        .into_iter()
        .map(|signal| RecentWatchSignalSummary {
            signal_type: signal.signal_type.clone(),
            timestamp: signal.timestamp,
            label: watch_signal_display_label(signal.signal_type.as_str()).to_string(),
            note: signal
                .payload_json
                .get("note")
                .and_then(JsonValue::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned),
        })
        .collect())
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

async fn select_recent_watch_signals(
    storage: &Storage,
    start_of_day: i64,
    timezone: &ResolvedTimeZone,
    now: OffsetDateTime,
) -> Result<Vec<SignalRecord>, AppError> {
    let mut selected = Vec::new();
    for signal_key in SUPPORTED_WATCH_SIGNAL_KEYS {
        let signal_type = format!("watch_signal:{signal_key}");
        let signal = storage
            .list_signals(Some(&signal_type), Some(start_of_day), 16)
            .await?
            .into_iter()
            .filter(|signal| signal.timestamp >= start_of_day)
            .filter_map(|signal| {
                OffsetDateTime::from_unix_timestamp(signal.timestamp)
                    .ok()
                    .filter(|timestamp| same_local_day(timezone, *timestamp, now))
                    .map(|_| signal)
            })
            .max_by_key(|signal| signal.timestamp);
        if let Some(signal) = signal {
            selected.push(signal);
        }
    }
    selected.sort_by_key(|signal| std::cmp::Reverse(signal.timestamp));
    Ok(selected)
}

fn headline_for_summary(metrics: &[AppleBehaviorMetric], watch_signals: &[SignalRecord]) -> String {
    if !metrics.is_empty() && watch_signals.is_empty() {
        return headline_for_metrics(metrics);
    }

    if metrics.is_empty() {
        let labels = watch_signals
            .iter()
            .map(|signal| watch_signal_display_label(signal.signal_type.as_str()).to_lowercase())
            .collect::<Vec<_>>();
        return format!(
            "Today's Apple behavior summary reflects recent watch signals: {}.",
            labels.join(", ")
        );
    }

    let metric_labels = metrics
        .iter()
        .map(|metric| metric.display_label.to_lowercase())
        .collect::<Vec<_>>();
    format!(
        "Today's Apple behavior summary covers {} and recent watch signals.",
        metric_labels.join(", ")
    )
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

fn summary_reasons(
    metrics: &[AppleBehaviorMetric],
    watch_signals: &[SignalRecord],
    freshness_seconds: Option<i64>,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if let Some(freshness_seconds) = freshness_seconds {
        reasons.push(format!(
            "Freshness is based on the newest bounded Apple health or watch signal from {} seconds ago.",
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
    for signal in watch_signals {
        let label = watch_signal_display_label(signal.signal_type.as_str());
        reasons.push(format!(
            "{} was recorded at {} from persisted signal {}.",
            label, signal.timestamp, signal.signal_id
        ));
        if let Some(note) = signal.payload_json.get("note").and_then(JsonValue::as_str) {
            let trimmed = note.trim();
            if !trimmed.is_empty() {
                reasons.push(format!("{label} note: {trimmed}."));
            }
        }
        reasons.push(format!(
            "Signal {} keeps the Apple summary grounded in watch-originated operator input.",
            signal.signal_type
        ));
    }
    reasons
}

fn watch_signal_display_label(signal_type: &str) -> &'static str {
    match signal_type.strip_prefix("watch_signal:").unwrap_or(signal_type) {
        "drifting" => "Recent drifting signal",
        "on_track" => "Recent on-track signal",
        "need_focus" => "Recent need-focus signal",
        "wake" | "waking_up" => "Recent wake signal",
        "heart_rate" => "Recent heart-rate signal",
        "motion" => "Recent motion signal",
        _ => "Recent watch signal",
    }
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
