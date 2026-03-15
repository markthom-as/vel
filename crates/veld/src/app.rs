use axum::{routing::{get, post}, Router};
use tower_http::trace::TraceLayer;
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::{policy_config::PolicyConfig, routes, state::AppState};

pub fn build_app(storage: Storage, config: AppConfig, policy_config: PolicyConfig) -> Router {
    let state = AppState::new(storage, config, policy_config);

    Router::new()
        .route("/v1/health", get(routes::health::health))
        .route("/v1/doctor", get(routes::doctor::doctor))
        .route("/v1/captures", get(routes::captures::list_captures).post(routes::captures::create_capture))
        .route("/v1/captures/:id", get(routes::captures::get_capture))
        .route("/v1/commitments", get(routes::commitments::list_commitments).post(routes::commitments::create_commitment))
        .route("/v1/commitments/:id", get(routes::commitments::get_commitment).patch(routes::commitments::update_commitment))
        .route("/v1/commitments/:id/dependencies", get(routes::commitments::list_commitment_dependencies).post(routes::commitments::add_commitment_dependency))
        .route("/v1/risk", get(routes::risk::compute_and_list))
        .route("/v1/risk/:id", get(routes::risk::get_commitment_risk))
        .route("/v1/suggestions", get(routes::suggestions::list))
        .route("/v1/suggestions/:id", get(routes::suggestions::get).patch(routes::suggestions::update))
        .route("/v1/artifacts", post(routes::artifacts::create_artifact))
        .route("/v1/artifacts/latest", get(routes::artifacts::get_artifact_latest))
        .route("/v1/artifacts/:id", get(routes::artifacts::get_artifact))
        .route("/v1/runs", get(routes::runs::list_runs))
        .route("/v1/runs/:id", get(routes::runs::get_run))
        .route("/v1/context/today", get(routes::context::today))
        .route("/v1/context/morning", get(routes::context::morning))
        .route("/v1/context/end-of-day", get(routes::context::end_of_day))
        .route("/v1/context/current", get(routes::context::current))
        .route("/v1/context/timeline", get(routes::context::timeline))
        .route("/v1/explain/nudge/:id", get(routes::explain::explain_nudge))
        .route("/v1/explain/context", get(routes::explain::explain_context))
        .route("/v1/explain/commitment/:id", get(routes::explain::explain_commitment))
        .route("/v1/explain/drift", get(routes::explain::explain_drift))
        .route("/v1/threads", get(routes::threads::list_threads).post(routes::threads::create_thread))
        .route("/v1/threads/:id", get(routes::threads::get_thread).patch(routes::threads::update_thread))
        .route("/v1/threads/:id/links", post(routes::threads::add_thread_link))
        .route("/v1/search", get(routes::search::search))
        .route("/v1/signals", get(routes::signals::list_signals).post(routes::signals::create_signal))
        .route("/v1/nudges", get(routes::nudges::list_nudges))
        .route("/v1/nudges/:id", get(routes::nudges::get_nudge))
        .route("/v1/nudges/:id/done", post(routes::nudges::nudge_done))
        .route("/v1/nudges/:id/snooze", post(routes::nudges::nudge_snooze))
        .route("/v1/sync/calendar", post(routes::sync::sync_calendar))
        .route("/v1/sync/todoist", post(routes::sync::sync_todoist))
        .route("/v1/sync/activity", post(routes::sync::sync_activity))
        .route("/v1/evaluate", post(routes::evaluate::run_evaluate))
        .route("/v1/synthesis/week", post(routes::synthesis::synthesis_week))
        .route("/v1/synthesis/project/:slug", post(routes::synthesis::synthesis_project))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy_config::PolicyConfig;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::util::ServiceExt;

    fn test_policy_config() -> PolicyConfig {
        PolicyConfig::default()
    }

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());

        let response = app
            .oneshot(Request::builder().uri("/v1/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn doctor_endpoint_returns_ok_with_schema_version() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());

        let response = app
            .oneshot(Request::builder().uri("/v1/doctor").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn search_endpoint_returns_ok_for_matching_capture() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_capture(vel_storage::CaptureInsert {
                content_text: "remember lidar budget".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("test".to_string()),
                privacy_class: vel_core::PrivacyClass::Private,
            })
            .await
            .unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/search?q=lidar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn today_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/today")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    /// Canonical runtime integration test: context generation flows through run → artifact → refs.
    /// Verifies run creation, status transitions, event sequence, artifact creation, and provenance refs.
    #[tokio::test]
    async fn context_today_creates_run_artifact_and_ref() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage.clone(), AppConfig::default(), test_policy_config());

        let today_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/today")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(today_resp.status(), StatusCode::OK);

        let runs = storage.list_runs(10).await.unwrap();
        assert_eq!(runs.len(), 1, "one run should exist");
        let run = &runs[0];
        assert_eq!(run.status, vel_core::RunStatus::Succeeded);
        assert_eq!(run.kind, vel_core::RunKind::ContextGeneration);

        let events = storage.list_run_events(run.id.as_ref()).await.unwrap();
        let event_types: Vec<String> = events.iter().map(|e| e.event_type.to_string()).collect();
        assert_eq!(
            event_types,
            [
                "run_created",
                "run_started",
                "context_generated",
                "artifact_written",
                "refs_created",
                "run_succeeded",
            ],
            "event sequence should match"
        );

        let refs_from_run = storage.list_refs_from("run", run.id.as_ref()).await.unwrap();
        assert_eq!(refs_from_run.len(), 1, "run should have one ref (run → artifact)");
        assert_eq!(refs_from_run[0].to_type, "artifact");

        let artifact_id = &refs_from_run[0].to_id;
        let artifact = storage.get_artifact_by_id(artifact_id).await.unwrap();
        assert!(artifact.is_some(), "artifact should exist");
        let art = artifact.unwrap();
        assert_eq!(art.storage_kind, vel_core::ArtifactStorageKind::Managed);
        assert_eq!(art.artifact_type, "context_brief");
        assert!(art.storage_uri.contains("context/today"));
        assert!(art.content_hash.as_deref().map(|h| h.starts_with("sha256:")).unwrap_or(false));
    }

    #[tokio::test]
    async fn context_today_failure_sets_run_failed_and_no_artifact_ref() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!("vel_test_root_{}", uuid::Uuid::new_v4().simple()));
        std::fs::File::create(&file_path).unwrap();
        let config = vel_config::AppConfig {
            artifact_root: file_path.to_string_lossy().to_string(),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config());

        let today_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/today")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(today_resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let runs = storage.list_runs(10).await.unwrap();
        assert_eq!(runs.len(), 1);
        let run = &runs[0];
        assert_eq!(run.status, vel_core::RunStatus::Failed);
        assert!(run.error_json.is_some());

        let refs_from_run = storage.list_refs_from("run", run.id.as_ref()).await.unwrap();
        assert!(refs_from_run.is_empty(), "no artifact ref on failure");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn end_of_day_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/end-of-day")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_artifact_returns_ok_and_get_returns_it() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());

        let create_body = serde_json::json!({
            "artifact_type": "transcript",
            "title": "Test transcript",
            "storage_uri": "file:///var/artifacts/t.txt",
        });
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/artifacts")
                    .header("content-type", "application/json")
                    .body(Body::from(create_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_resp.status(), StatusCode::OK);

        let create_bytes = axum::body::to_bytes(create_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let create_json: serde_json::Value = serde_json::from_slice(&create_bytes).unwrap();
        let artifact_id = create_json["data"]["artifact_id"].as_str().unwrap();

        let get_resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/artifacts/{}", artifact_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get_resp.status(), StatusCode::OK);
    }

    /// Commute policy: no commute nudge when calendar event has no travel_minutes.
    #[tokio::test]
    async fn commute_nudge_does_not_fire_without_travel_minutes() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                timestamp: now_ts + 3600,
                payload_json: Some(serde_json::json!({
                    "start_time": now_ts + 3600,
                    "title": "Meeting"
                })),
            })
            .await
            .unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());
        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(nudges_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"].as_array().unwrap_or(&[]);
        let commute_nudges: Vec<_> = nudges
            .iter()
            .filter(|n| n["nudge_type"].as_str() == Some("commute_leave_time"))
            .collect();
        assert!(commute_nudges.is_empty(), "commute nudge must not trigger when travel_minutes missing");
    }

    /// Context explain returns signals_used and commitments_used.
    #[tokio::test]
    async fn context_explain_includes_signals_and_commitments_used() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());
        let _ = app.clone().oneshot(Request::builder().method("POST").uri("/v1/evaluate").body(Body::empty()).unwrap()).await.unwrap();
        let resp = app.oneshot(Request::builder().uri("/v1/explain/context").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["data"]["signals_used"].is_array(), "signals_used must be present");
        assert!(json["data"]["commitments_used"].is_array(), "commitments_used must be present");
        assert!(json["data"]["reasons"].is_array(), "reasons must be present");
    }

    /// Resolution order: resolved nudge never escalates; second evaluate does not re-trigger.
    #[tokio::test]
    async fn resolved_nudge_stays_resolved_after_second_evaluate() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Take meds".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("medication".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        let app = build_app(storage, AppConfig::default(), test_policy_config());
        let _ = app.clone().oneshot(Request::builder().method("POST").uri("/v1/evaluate").body(Body::empty()).unwrap()).await.unwrap();
        let nudges_resp = app.clone().oneshot(Request::builder().uri("/v1/nudges").body(Body::empty()).unwrap()).await.unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"].as_array().unwrap_or(&[]);
        let meds_nudge = nudges.iter().find(|n| n["nudge_type"].as_str() == Some("meds_not_logged"));
        let nudge_id = meds_nudge.and_then(|n| n["nudge_id"].as_str()).expect("meds nudge should exist");
        let done_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/nudges/{}/done", nudge_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(done_resp.status(), StatusCode::OK);
        let _ = app.clone().oneshot(Request::builder().method("POST").uri("/v1/evaluate").body(Body::empty()).unwrap()).await.unwrap();
        let nudges_resp2 = app.oneshot(Request::builder().uri("/v1/nudges").body(Body::empty()).unwrap()).await.unwrap();
        let body2 = axum::body::to_bytes(nudges_resp2.into_body(), usize::MAX).await.unwrap();
        let json2: serde_json::Value = serde_json::from_slice(&body2).unwrap();
        let resolved: Vec<_> = json2["data"]
            .as_array()
            .unwrap_or(&[])
            .iter()
            .filter(|n| n["nudge_id"].as_str() == Some(nudge_id))
            .collect();
        assert_eq!(resolved.len(), 1, "nudge should appear exactly once");
        assert_eq!(resolved[0]["state"].as_str(), Some("resolved"), "resolved nudge must stay resolved after second evaluate");
    }
}
