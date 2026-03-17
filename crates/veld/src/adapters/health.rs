//! Health adapter: ingest local health/activity snapshots and emit replay-safe health_metric signals.

use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};

pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let path = match &config.health_snapshot_path {
        Some(path) => path,
        None => return Ok(0),
    };

    let content = tokio::fs::read_to_string(path).await.map_err(|e| {
        crate::errors::AppError::internal(format!("read health snapshot {}: {}", path, e))
    })?;
    let snapshot: HealthSnapshot = serde_json::from_str(&content).map_err(|e| {
        crate::errors::AppError::internal(format!("parse health snapshot: {}", e))
    })?;

    let default_source = snapshot
        .source
        .clone()
        .unwrap_or_else(|| "health".to_string());
    let mut count = 0u32;
    for sample in snapshot.samples {
        let metric_type = sample.metric_type.trim();
        if metric_type.is_empty() {
            continue;
        }

        let payload = serde_json::json!({
            "metric_type": metric_type,
            "value": sample.value,
            "unit": sample.unit,
            "source_app": sample.source_app,
            "device": sample.device,
            "metadata": sample
                .metadata
                .clone()
                .unwrap_or_else(|| serde_json::json!({})),
        });

        let signal_id = storage
            .insert_signal(SignalInsert {
                signal_type: "health_metric".to_string(),
                source: sample
                    .source
                    .clone()
                    .unwrap_or_else(|| default_source.clone()),
                source_ref: Some(sample_source_ref(&sample)),
                timestamp: sample.timestamp,
                payload_json: Some(payload),
            })
            .await
            .map_err(crate::errors::AppError::from)?;
        if signal_id.starts_with("sig_") {
            count += 1;
        }
    }

    Ok(count)
}

fn sample_source_ref(sample: &HealthSample) -> String {
    if let Some(source_ref) = sample.source_ref.as_deref().filter(|value| !value.is_empty()) {
        return source_ref.to_string();
    }

    let source_app = sample.source_app.as_deref().unwrap_or("unknown");
    let unit = sample.unit.as_deref().unwrap_or("-");
    format!(
        "health:{}:{}:{}:{}",
        sample.metric_type.trim(),
        sample.timestamp,
        sample.value,
        source_app.replace(':', "_").replace('/', "_")
    ) + &format!(":{unit}")
}

#[derive(Debug, serde::Deserialize)]
struct HealthSnapshot {
    source: Option<String>,
    #[serde(default)]
    samples: Vec<HealthSample>,
}

#[derive(Debug, serde::Deserialize)]
struct HealthSample {
    metric_type: String,
    timestamp: i64,
    value: serde_json::Value,
    unit: Option<String>,
    source: Option<String>,
    source_app: Option<String>,
    device: Option<String>,
    source_ref: Option<String>,
    metadata: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_ref_prefers_explicit_identity() {
        let sample = HealthSample {
            metric_type: "step_count".to_string(),
            timestamp: 123,
            value: serde_json::json!(4400),
            unit: Some("count".to_string()),
            source: None,
            source_app: Some("Health".to_string()),
            device: None,
            source_ref: Some("hk:sample-1".to_string()),
            metadata: None,
        };

        assert_eq!(sample_source_ref(&sample), "hk:sample-1");
    }
}
