use std::path::PathBuf;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use time::{Duration, OffsetDateTime};
use tower::util::ServiceExt;
use vel_api_types::{ApiResponse, AppleBehaviorSummaryData};
use vel_config::AppConfig;
use vel_storage::{SignalInsert, Storage};
use veld::{app::build_app, policy_config::PolicyConfig};

const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";

fn test_policy_config() -> PolicyConfig {
    PolicyConfig::default()
}

async fn test_storage() -> Storage {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
}

fn write_health_snapshot(snapshot: serde_json::Value) -> PathBuf {
    let file_path = std::env::temp_dir().join(format!(
        "vel_health_behavior_{}.json",
        uuid::Uuid::new_v4().simple()
    ));
    std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();
    file_path
}

fn sync_health_request() -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/sync/health")
        .header(OPERATOR_AUTH_HEADER, "operator-secret")
        .body(Body::empty())
        .unwrap()
}

fn behavior_summary_request() -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri("/v1/apple/behavior-summary")
        .header(OPERATOR_AUTH_HEADER, "operator-secret")
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn apple_behavior_summary_rolls_up_supported_metrics_for_today() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    let file_path = write_health_snapshot(serde_json::json!({
        "source": "healthkit",
        "samples": [
            {
                "metric_type": "step_count",
                "timestamp": now.unix_timestamp(),
                "value": 6842,
                "unit": "count",
                "source_app": "Health",
                "device": "Apple Watch"
            },
            {
                "metric_type": "stand_hours",
                "timestamp": (now - Duration::minutes(20)).unix_timestamp(),
                "value": 9,
                "unit": "hours",
                "source_app": "Health",
                "device": "Apple Watch"
            },
            {
                "metric_type": "exercise_minutes",
                "timestamp": (now - Duration::minutes(40)).unix_timestamp(),
                "value": 27,
                "unit": "minutes",
                "source_app": "Workout",
                "device": "Apple Watch"
            }
        ]
    }));
    let config = AppConfig {
        health_snapshot_path: Some(file_path.to_string_lossy().to_string()),
        ..Default::default()
    };
    let app = build_app(storage, config, test_policy_config(), None, None);

    let sync_response = app.clone().oneshot(sync_health_request()).await.unwrap();
    assert_eq!(sync_response.status(), StatusCode::OK);

    let response = app.oneshot(behavior_summary_request()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<AppleBehaviorSummaryData> = serde_json::from_slice(&body).unwrap();
    let summary = payload.data.expect("behavior summary");

    assert_eq!(summary.metrics.len(), 3);
    assert_eq!(summary.metrics[0].metric_key, "step_count");
    assert_eq!(summary.metrics[0].value, 6842.0);
    assert_eq!(summary.metrics[1].metric_key, "stand_hours");
    assert_eq!(summary.metrics[1].value, 9.0);
    assert_eq!(summary.metrics[2].metric_key, "exercise_minutes");
    assert_eq!(summary.metrics[2].value, 27.0);

    let _ = std::fs::remove_file(file_path);
}

#[tokio::test]
async fn apple_behavior_summary_ignores_out_of_scope_health_metrics() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    let file_path = write_health_snapshot(serde_json::json!({
        "source": "healthkit",
        "samples": [
            {
                "metric_type": "step_count",
                "timestamp": now.unix_timestamp(),
                "value": 5100,
                "unit": "count",
                "source_app": "Health"
            },
            {
                "metric_type": "heart_rate",
                "timestamp": now.unix_timestamp(),
                "value": 88,
                "unit": "bpm",
                "source_app": "Health"
            },
            {
                "metric_type": "sleep_duration",
                "timestamp": now.unix_timestamp(),
                "value": 450,
                "unit": "minutes",
                "source_app": "Health"
            }
        ]
    }));
    let config = AppConfig {
        health_snapshot_path: Some(file_path.to_string_lossy().to_string()),
        ..Default::default()
    };
    let app = build_app(storage, config, test_policy_config(), None, None);

    let sync_response = app.clone().oneshot(sync_health_request()).await.unwrap();
    assert_eq!(sync_response.status(), StatusCode::OK);

    let response = app.oneshot(behavior_summary_request()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<AppleBehaviorSummaryData> = serde_json::from_slice(&body).unwrap();
    let summary = payload.data.expect("behavior summary");

    assert_eq!(summary.metrics.len(), 1);
    assert_eq!(summary.metrics[0].metric_key, "step_count");
    assert!(summary
        .metrics
        .iter()
        .all(|metric| metric.metric_key != "heart_rate" && metric.metric_key != "sleep_duration"));

    let _ = std::fs::remove_file(file_path);
}

#[tokio::test]
async fn apple_behavior_summary_reports_freshness_and_explainable_reasons() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    let step_timestamp = (now - Duration::minutes(5)).unix_timestamp();
    let stand_timestamp = (now - Duration::minutes(17)).unix_timestamp();
    let exercise_timestamp = (now - Duration::minutes(42)).unix_timestamp();
    let file_path = write_health_snapshot(serde_json::json!({
        "source": "healthkit",
        "samples": [
            {
                "metric_type": "step_count",
                "timestamp": step_timestamp,
                "value": 9200,
                "unit": "count",
                "source_app": "Health",
                "device": "Apple Watch"
            },
            {
                "metric_type": "stand_hours",
                "timestamp": stand_timestamp,
                "value": 11,
                "unit": "hours",
                "source_app": "Health",
                "device": "Apple Watch"
            },
            {
                "metric_type": "exercise_minutes",
                "timestamp": exercise_timestamp,
                "value": 36,
                "unit": "minutes",
                "source_app": "Workout",
                "device": "Apple Watch"
            }
        ]
    }));
    let config = AppConfig {
        health_snapshot_path: Some(file_path.to_string_lossy().to_string()),
        ..Default::default()
    };
    let app = build_app(storage, config, test_policy_config(), None, None);

    let sync_response = app.clone().oneshot(sync_health_request()).await.unwrap();
    assert_eq!(sync_response.status(), StatusCode::OK);

    let response = app.oneshot(behavior_summary_request()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<AppleBehaviorSummaryData> = serde_json::from_slice(&body).unwrap();
    let summary = payload.data.expect("behavior summary");

    assert!(summary.freshness_seconds.is_some());
    assert!(summary.freshness_seconds.unwrap() >= 0);
    assert!(summary.reasons.iter().any(|reason| reason.contains("step")
        || reason.contains("stand")
        || reason.contains("exercise")));
    assert!(summary
        .reasons
        .iter()
        .any(|reason| reason.contains("Apple Watch")
            || reason.contains("Health")
            || reason.contains("Workout")));
    assert!(
        summary
            .metrics
            .iter()
            .all(|metric| !metric.reasons.is_empty()),
        "each bounded metric should explain the persisted signal behind it"
    );
    assert_eq!(summary.metrics[0].recorded_at, step_timestamp);
    assert_eq!(summary.metrics[1].recorded_at, stand_timestamp);
    assert_eq!(summary.metrics[2].recorded_at, exercise_timestamp);

    let _ = std::fs::remove_file(file_path);
}

#[tokio::test]
async fn apple_behavior_summary_returns_watch_signal_only_summary() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    storage
        .insert_signal(SignalInsert {
            signal_type: "watch_signal:drifting".to_string(),
            source: "vel".to_string(),
            source_ref: Some("watch:test-drifting".to_string()),
            timestamp: now.unix_timestamp(),
            payload_json: Some(serde_json::json!({
                "signal_type": "drifting",
                "note": "Losing focus during setup"
            })),
        })
        .await
        .unwrap();
    storage
        .insert_signal(SignalInsert {
            signal_type: "watch_signal:need_focus".to_string(),
            source: "vel".to_string(),
            source_ref: Some("watch:test-focus".to_string()),
            timestamp: (now - Duration::minutes(3)).unix_timestamp(),
            payload_json: Some(serde_json::json!({
                "signal_type": "need_focus",
                "note": "Need a quieter block"
            })),
        })
        .await
        .unwrap();
    storage
        .insert_signal(SignalInsert {
            signal_type: "watch_signal:drifting".to_string(),
            source: "vel".to_string(),
            source_ref: Some("watch:old-drifting".to_string()),
            timestamp: (now - Duration::days(1)).unix_timestamp(),
            payload_json: Some(serde_json::json!({
                "signal_type": "drifting",
                "note": "stale"
            })),
        })
        .await
        .unwrap();

    let app = build_app(
        storage,
        AppConfig::default(),
        test_policy_config(),
        None,
        None,
    );
    let response = app.oneshot(behavior_summary_request()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<AppleBehaviorSummaryData> = serde_json::from_slice(&body).unwrap();
    let summary = payload.data.expect("behavior summary");

    assert!(summary.metrics.is_empty());
    assert!(summary.headline.contains("watch signals"));
    assert!(summary
        .reasons
        .iter()
        .any(|reason| reason.contains("drifting")));
    assert!(summary
        .reasons
        .iter()
        .any(|reason| reason.contains("Need a quieter block")));
    assert!(!summary
        .reasons
        .iter()
        .any(|reason| reason.contains("stale")));
}

#[tokio::test]
async fn apple_behavior_summary_includes_watch_signal_context_alongside_metrics() {
    let storage = test_storage().await;
    let now = OffsetDateTime::now_utc();
    let file_path = write_health_snapshot(serde_json::json!({
        "source": "healthkit",
        "samples": [
            {
                "metric_type": "step_count",
                "timestamp": now.unix_timestamp(),
                "value": 7100,
                "unit": "count",
                "source_app": "Health",
                "device": "Apple Watch"
            }
        ]
    }));
    let config = AppConfig {
        health_snapshot_path: Some(file_path.to_string_lossy().to_string()),
        ..Default::default()
    };
    let app = build_app(storage.clone(), config, test_policy_config(), None, None);

    let sync_response = app.clone().oneshot(sync_health_request()).await.unwrap();
    assert_eq!(sync_response.status(), StatusCode::OK);

    storage
        .insert_signal(SignalInsert {
            signal_type: "watch_signal:waking_up".to_string(),
            source: "vel".to_string(),
            source_ref: Some("watch:test-wake".to_string()),
            timestamp: (now - Duration::minutes(2)).unix_timestamp(),
            payload_json: Some(serde_json::json!({
                "signal_type": "waking_up",
                "note": "Alarm dismissed"
            })),
        })
        .await
        .unwrap();

    let response = app.oneshot(behavior_summary_request()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let payload: ApiResponse<AppleBehaviorSummaryData> = serde_json::from_slice(&body).unwrap();
    let summary = payload.data.expect("behavior summary");

    assert_eq!(summary.metrics.len(), 1);
    assert!(summary.headline.contains("recent watch signals"));
    assert!(summary.reasons.iter().any(|reason| reason.contains("wake")));
    assert!(summary
        .reasons
        .iter()
        .any(|reason| reason.contains("Alarm dismissed")));

    let _ = std::fs::remove_file(file_path);
}
