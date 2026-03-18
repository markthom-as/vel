use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use tokio::sync::broadcast;
use vel_config::AppConfig;
use vel_core::{ArtifactId, CaptureId, PrivacyClass, RefRelationType, RunEventType, RunStatus};
use vel_storage::{CaptureInsert, SignalInsert, Storage};
use veld::{policy_config::PolicyConfig, services::context_runs, state::AppState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioKind {
    Today,
    Morning,
    EndOfDay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureFixture {
    pub content_text: String,
    pub capture_type: String,
    pub occurred_at: i64,
    pub source_device: Option<String>,
    pub privacy_class: PrivacyClass,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalFixture {
    pub signal_type: String,
    pub source: String,
    pub source_ref: Option<String>,
    pub timestamp: i64,
    pub payload_json: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DayScenarioFixture {
    #[serde(with = "time::serde::rfc3339")]
    pub now: OffsetDateTime,
    pub captures: Vec<CaptureFixture>,
    pub signals: Vec<SignalFixture>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryEvent {
    pub event_type: RunEventType,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayReport {
    pub context_json: Value,
    pub artifact_json: Value,
    pub run_status: RunStatus,
    pub boundary_events: Vec<BoundaryEvent>,
    pub ref_count: usize,
}

pub async fn replay_day_scenario(
    kind: ScenarioKind,
    fixture: &DayScenarioFixture,
) -> Result<ReplayReport, Box<dyn std::error::Error + Send + Sync>> {
    let storage = Storage::connect(":memory:").await?;
    storage.migrate().await?;

    if let Some(timezone) = fixture.timezone.as_deref() {
        storage
            .set_setting("timezone", &serde_json::json!(timezone))
            .await?;
    }

    let capture_ids = seed_captures(&storage, fixture).await?;
    seed_signals(&storage, fixture).await?;

    let artifact_root = unique_artifact_root();
    let state = test_state(storage.clone(), artifact_root.clone());

    let output_json = match kind {
        ScenarioKind::Today => serde_json::to_value(
            context_runs::generate_today_at(&state, fixture.now)
                .await
                .map_err(boxed_error)?
                .data,
        )?,
        ScenarioKind::Morning => serde_json::to_value(
            context_runs::generate_morning_at(&state, fixture.now)
                .await
                .map_err(boxed_error)?
                .data,
        )?,
        ScenarioKind::EndOfDay => serde_json::to_value(
            context_runs::generate_end_of_day_at(&state, fixture.now)
                .await
                .map_err(boxed_error)?
                .data,
        )?,
    };

    let run = storage
        .list_runs(None, None, 1)
        .await?
        .into_iter()
        .next()
        .expect("simulation run should be persisted");
    let events = storage.list_run_events(run.id.as_ref()).await?;
    let refs = storage.list_refs_from("run", run.id.as_ref()).await?;
    let artifact_id = refs
        .iter()
        .find(|reference| reference.to_type == "artifact")
        .map(|reference| reference.to_id.clone())
        .expect("simulation run should link an artifact");
    let artifact = storage
        .get_artifact_by_id(&ArtifactId::from(artifact_id.clone()))
        .await?
        .expect("linked artifact should exist");
    let artifact_refs = storage.list_refs_from("artifact", &artifact_id).await?;
    let artifact_json = serde_json::from_slice::<Value>(&fs::read(
        PathBuf::from(&state.config.artifact_root).join(&artifact.storage_uri),
    )?)?;

    for capture_id in capture_ids {
        assert!(
            artifact_refs.iter().any(|reference| {
                reference.from_type == "artifact"
                    && reference.to_type == "capture"
                    && reference.to_id == capture_id.as_ref()
                    && reference.relation_type == RefRelationType::DerivedFrom
            }),
            "simulation should create capture provenance refs"
        );
    }

    let _ = fs::remove_dir_all(&artifact_root);

    Ok(ReplayReport {
        context_json: normalize_report_json(output_json),
        artifact_json: normalize_report_json(artifact_json),
        run_status: run.status,
        boundary_events: events
            .into_iter()
            .map(|event| BoundaryEvent {
                payload: normalize_event_payload(event.event_type, event.payload_json),
                event_type: event.event_type,
            })
            .collect(),
        ref_count: refs.len() + artifact_refs.len(),
    })
}

fn test_state(storage: Storage, artifact_root: PathBuf) -> AppState {
    let config = AppConfig {
        artifact_root: artifact_root.to_string_lossy().to_string(),
        ..AppConfig::default()
    };
    let (broadcast_tx, _) = broadcast::channel(8);
    AppState::new(
        storage,
        config,
        PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    )
}

async fn seed_captures(
    storage: &Storage,
    fixture: &DayScenarioFixture,
) -> Result<Vec<CaptureId>, Box<dyn std::error::Error + Send + Sync>> {
    let mut ids = Vec::with_capacity(fixture.captures.len());
    for capture in &fixture.captures {
        let capture_id = storage
            .insert_capture_at(
                CaptureInsert {
                    content_text: capture.content_text.clone(),
                    capture_type: capture.capture_type.clone(),
                    source_device: capture.source_device.clone(),
                    privacy_class: capture.privacy_class,
                },
                capture.occurred_at,
            )
            .await?;
        ids.push(capture_id);
    }
    Ok(ids)
}

async fn seed_signals(
    storage: &Storage,
    fixture: &DayScenarioFixture,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for signal in &fixture.signals {
        storage
            .insert_signal(SignalInsert {
                signal_type: signal.signal_type.clone(),
                source: signal.source.clone(),
                source_ref: signal.source_ref.clone(),
                timestamp: signal.timestamp,
                payload_json: Some(signal.payload_json.clone()),
            })
            .await?;
    }
    Ok(())
}

fn normalize_event_payload(event_type: RunEventType, payload: Value) -> Value {
    match event_type {
        RunEventType::ArtifactWritten => serde_json::json!({ "artifact_written": true }),
        RunEventType::RunStarted | RunEventType::RunSucceeded | RunEventType::RefsCreated => {
            serde_json::json!({})
        }
        _ => payload,
    }
}

fn normalize_report_json(value: Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.into_iter().map(normalize_report_json).collect()),
        Value::Object(entries) => Value::Object(
            entries
                .into_iter()
                .map(|(key, value)| {
                    let normalized = if key == "capture_id" {
                        Value::String("<capture_id>".to_string())
                    } else {
                        normalize_report_json(value)
                    };
                    (key, normalized)
                })
                .collect(),
        ),
        other => other,
    }
}

fn unique_artifact_root() -> PathBuf {
    std::env::temp_dir().join(format!("vel_sim_{}", uuid::Uuid::new_v4().simple()))
}

fn boxed_error(error: veld::errors::AppError) -> Box<dyn std::error::Error + Send + Sync> {
    Box::new(std::io::Error::other(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    fn fixture() -> DayScenarioFixture {
        let now = datetime!(2026-03-18 15:30:00 UTC);
        DayScenarioFixture {
            now,
            timezone: Some("UTC".to_string()),
            captures: vec![
                CaptureFixture {
                    content_text: "follow up with Sam on the review agenda".to_string(),
                    capture_type: "note".to_string(),
                    occurred_at: now.unix_timestamp() - 3600,
                    source_device: Some("phone".to_string()),
                    privacy_class: PrivacyClass::Private,
                },
                CaptureFixture {
                    content_text: "remember the forecast budget before the 4pm check-in"
                        .to_string(),
                    capture_type: "note".to_string(),
                    occurred_at: now.unix_timestamp() - 1200,
                    source_device: Some("laptop".to_string()),
                    privacy_class: PrivacyClass::Private,
                },
            ],
            signals: vec![
                SignalFixture {
                    signal_type: "external_task".to_string(),
                    source: "todoist".to_string(),
                    source_ref: Some("task-1".to_string()),
                    timestamp: now.unix_timestamp() - 1800,
                    payload_json: serde_json::json!({
                        "text": "follow up with Sam on the review agenda",
                    }),
                },
                SignalFixture {
                    signal_type: "calendar_event".to_string(),
                    source: "calendar".to_string(),
                    source_ref: Some("event-1".to_string()),
                    timestamp: now.unix_timestamp() - 900,
                    payload_json: serde_json::json!({
                        "title": "forecast budget check-in",
                    }),
                },
            ],
        }
    }

    #[tokio::test]
    async fn replay_day_scenario_is_deterministic() {
        let first = replay_day_scenario(ScenarioKind::Today, &fixture())
            .await
            .expect("first replay should succeed");
        let second = replay_day_scenario(ScenarioKind::Today, &fixture())
            .await
            .expect("second replay should succeed");

        assert_eq!(first, second);
        assert_eq!(first.run_status, RunStatus::Succeeded);
        assert_eq!(first.context_json, first.artifact_json);
        assert_eq!(
            first.boundary_events,
            vec![
                BoundaryEvent {
                    event_type: RunEventType::RunCreated,
                    payload: serde_json::json!({ "kind": "context_generation" }),
                },
                BoundaryEvent {
                    event_type: RunEventType::RunStarted,
                    payload: serde_json::json!({}),
                },
                BoundaryEvent {
                    event_type: RunEventType::ContextGenerated,
                    payload: serde_json::json!({ "context_kind": "today" }),
                },
                BoundaryEvent {
                    event_type: RunEventType::ArtifactWritten,
                    payload: serde_json::json!({ "artifact_written": true }),
                },
                BoundaryEvent {
                    event_type: RunEventType::RefsCreated,
                    payload: serde_json::json!({}),
                },
                BoundaryEvent {
                    event_type: RunEventType::RunSucceeded,
                    payload: serde_json::json!({}),
                },
            ]
        );
        assert!(
            first.ref_count >= 3,
            "replay should capture the artifact link plus provenance refs"
        );
        assert!(first
            .artifact_json
            .get("date")
            .and_then(Value::as_str)
            .is_some_and(|date| date == "2026-03-18"));
    }
}
