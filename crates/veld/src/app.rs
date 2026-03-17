use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::{policy_config::PolicyConfig, routes, state::AppState};

/// Builds the app from storage/config; used by tests. Production uses build_app_with_state.
#[allow(dead_code)]
pub fn build_app(
    storage: Storage,
    config: AppConfig,
    policy_config: PolicyConfig,
    llm_router: Option<std::sync::Arc<vel_llm::Router>>,
    chat_profile_id: Option<String>,
) -> Router {
    let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
    let state = AppState::new(
        storage,
        config,
        policy_config,
        broadcast_tx,
        llm_router,
        chat_profile_id,
    );

    build_app_with_state(state)
}

pub fn build_app_with_state(state: AppState) -> Router {
    Router::new()
        .route("/v1/health", get(routes::health::health))
        .route("/v1/doctor", get(routes::doctor::doctor))
        .route(
            "/v1/captures",
            get(routes::captures::list_captures).post(routes::captures::create_capture),
        )
        .route("/v1/captures/:id", get(routes::captures::get_capture))
        .route(
            "/v1/commitments",
            get(routes::commitments::list_commitments).post(routes::commitments::create_commitment),
        )
        .route(
            "/v1/commitments/:id",
            get(routes::commitments::get_commitment).patch(routes::commitments::update_commitment),
        )
        .route(
            "/v1/commitments/:id/dependencies",
            get(routes::commitments::list_commitment_dependencies)
                .post(routes::commitments::add_commitment_dependency),
        )
        .route("/v1/risk", get(routes::risk::list_risk))
        .route("/v1/risk/:id", get(routes::risk::get_commitment_risk))
        .route("/v1/suggestions", get(routes::suggestions::list))
        .route(
            "/v1/suggestions/:id",
            get(routes::suggestions::get).patch(routes::suggestions::update),
        )
        .route(
            "/v1/artifacts",
            get(routes::artifacts::list_artifacts).post(routes::artifacts::create_artifact),
        )
        .route(
            "/v1/artifacts/latest",
            get(routes::artifacts::get_artifact_latest),
        )
        .route("/v1/artifacts/:id", get(routes::artifacts::get_artifact))
        .route("/v1/runs", get(routes::runs::list_runs))
        .route(
            "/v1/runs/:id",
            get(routes::runs::get_run).patch(routes::runs::update_run),
        )
        .route("/v1/context/today", get(routes::context::today))
        .route("/v1/context/morning", get(routes::context::morning))
        .route("/v1/context/end-of-day", get(routes::context::end_of_day))
        .route("/v1/context/current", get(routes::context::current))
        .route("/v1/context/timeline", get(routes::context::timeline))
        .route("/v1/explain/nudge/:id", get(routes::explain::explain_nudge))
        .route("/v1/explain/context", get(routes::explain::explain_context))
        .route(
            "/v1/explain/commitment/:id",
            get(routes::explain::explain_commitment),
        )
        .route("/v1/explain/drift", get(routes::explain::explain_drift))
        .route(
            "/v1/threads",
            get(routes::threads::list_threads).post(routes::threads::create_thread),
        )
        .route(
            "/v1/threads/:id",
            get(routes::threads::get_thread).patch(routes::threads::update_thread),
        )
        .route(
            "/v1/threads/:id/links",
            post(routes::threads::add_thread_link),
        )
        .route("/v1/search", get(routes::search::search))
        .route(
            "/v1/signals",
            get(routes::signals::list_signals).post(routes::signals::create_signal),
        )
        .route("/v1/nudges", get(routes::nudges::list_nudges))
        .route("/v1/nudges/:id", get(routes::nudges::get_nudge))
        .route("/v1/nudges/:id/done", post(routes::nudges::nudge_done))
        .route("/v1/nudges/:id/snooze", post(routes::nudges::nudge_snooze))
        .route("/v1/sync/calendar", post(routes::sync::sync_calendar))
        .route("/v1/sync/todoist", post(routes::sync::sync_todoist))
        .route("/v1/sync/activity", post(routes::sync::sync_activity))
        .route("/v1/sync/git", post(routes::sync::sync_git))
        .route("/v1/sync/messaging", post(routes::sync::sync_messaging))
        .route("/v1/sync/notes", post(routes::sync::sync_notes))
        .route("/v1/sync/transcripts", post(routes::sync::sync_transcripts))
        .route("/v1/evaluate", post(routes::evaluate::run_evaluate))
        .route("/api/components", get(routes::components::list_components))
        .route(
            "/api/components/:id/logs",
            get(routes::components::list_component_logs),
        )
        .route(
            "/api/components/:id/restart",
            post(routes::components::restart_component),
        )
        .route(
            "/api/integrations",
            get(routes::integrations::get_integrations),
        )
        .route(
            "/api/integrations/:id/logs",
            get(routes::integrations::list_integration_logs),
        )
        .route(
            "/api/integrations/google-calendar",
            axum::routing::patch(routes::integrations::patch_google_calendar),
        )
        .route(
            "/api/integrations/google-calendar/disconnect",
            post(routes::integrations::disconnect_google_calendar),
        )
        .route(
            "/api/integrations/google-calendar/auth/start",
            post(routes::integrations::start_google_calendar_auth),
        )
        .route(
            "/api/integrations/google-calendar/oauth/callback",
            get(routes::integrations::google_calendar_oauth_callback),
        )
        .route(
            "/api/integrations/todoist",
            axum::routing::patch(routes::integrations::patch_todoist),
        )
        .route(
            "/api/integrations/todoist/disconnect",
            post(routes::integrations::disconnect_todoist),
        )
        .route(
            "/v1/synthesis/week",
            post(routes::synthesis::synthesis_week),
        )
        .route(
            "/v1/synthesis/project/:slug",
            post(routes::synthesis::synthesis_project),
        )
        .route("/ws", get(routes::ws::ws_handler))
        .merge(routes::chat::chat_routes())
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy_config::PolicyConfig;
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use tower::util::ServiceExt;

    fn test_policy_config() -> PolicyConfig {
        PolicyConfig::default()
    }

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn doctor_endpoint_returns_ok_with_schema_version() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/doctor")
                    .body(Body::empty())
                    .unwrap(),
            )
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
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

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
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

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
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

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

        let runs = storage.list_runs(10, None, None).await.unwrap();
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

        let refs_from_run = storage
            .list_refs_from("run", run.id.as_ref())
            .await
            .unwrap();
        assert_eq!(
            refs_from_run.len(),
            1,
            "run should have one ref (run → artifact)"
        );
        assert_eq!(refs_from_run[0].to_type, "artifact");

        let artifact_id = &refs_from_run[0].to_id;
        let artifact = storage.get_artifact_by_id(artifact_id).await.unwrap();
        assert!(artifact.is_some(), "artifact should exist");
        let art = artifact.unwrap();
        assert_eq!(art.storage_kind, vel_core::ArtifactStorageKind::Managed);
        assert_eq!(art.artifact_type, "context_brief");
        assert!(art.storage_uri.contains("context/today"));
        assert!(art
            .content_hash
            .as_deref()
            .map(|h| h.starts_with("sha256:"))
            .unwrap_or(false));
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
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

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

        let runs = storage.list_runs(10, None, None).await.unwrap();
        assert_eq!(runs.len(), 1);
        let run = &runs[0];
        assert_eq!(run.status, vel_core::RunStatus::Failed);
        assert!(run.error_json.is_some());

        let refs_from_run = storage
            .list_refs_from("run", run.id.as_ref())
            .await
            .unwrap();
        assert!(refs_from_run.is_empty(), "no artifact ref on failure");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn update_run_retry_scheduled_persists_metadata_and_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::ContextGeneration,
                &serde_json::json!({ "context_kind": "today" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "retry_scheduled",
                            "retry_after_seconds": 30,
                            "reason": "transient_failure"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"];
        assert_eq!(data["status"], "retry_scheduled");
        assert_eq!(data["retry_reason"], "transient_failure");
        assert!(data.get("retry_scheduled_at").is_some());

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::RetryScheduled);
        assert_eq!(
            run.output_json.as_ref().and_then(|v| v.get("retry_reason")),
            Some(&serde_json::json!("transient_failure"))
        );

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        let retry_event = events
            .iter()
            .find(|event| event.event_type == vel_core::RunEventType::RunRetryScheduled)
            .expect("retry scheduling event should be appended");
        assert_eq!(retry_event.payload_json["reason"], "transient_failure");
        assert!(retry_event.payload_json.get("retry_at").is_some());
    }

    #[tokio::test]
    async fn update_run_emits_runs_updated_websocket_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        let mut rx = broadcast_tx.subscribe();
        let state = crate::state::AppState::new(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        );
        let app = build_app_with_state(state);

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "blocked",
                            "blocked_reason": "waiting_on_dependency"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let envelope = rx
            .recv()
            .await
            .expect("websocket event should be broadcast");
        assert_eq!(envelope.event_type.to_string(), "runs:updated");
        assert_eq!(envelope.payload["id"], run_id.as_ref());
        assert_eq!(envelope.payload["kind"], "synthesis");
        assert_eq!(envelope.payload["status"], "blocked");
    }

    #[tokio::test]
    async fn update_run_blocked_persists_blocked_reason() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "blocked",
                            "blocked_reason": "waiting_on_dependency"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"];
        assert_eq!(data["status"], "blocked");
        assert_eq!(data["blocked_reason"], "waiting_on_dependency");

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::Blocked);
        assert_eq!(
            run.output_json
                .as_ref()
                .and_then(|v| v.get("blocked_reason")),
            Some(&serde_json::json!("waiting_on_dependency"))
        );
    }

    #[tokio::test]
    async fn update_run_rejects_retry_fields_for_non_retry_status() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::ContextGeneration,
                &serde_json::json!({ "context_kind": "today" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "blocked",
                            "retry_after_seconds": 30
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::Queued);
    }

    #[tokio::test]
    async fn update_run_rejects_conflicting_retry_fields() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let body = serde_json::to_string(&vel_api_types::RunUpdateRequest {
            status: "retry_scheduled".to_string(),
            retry_at: Some(time::OffsetDateTime::now_utc()),
            retry_after_seconds: Some(30),
            reason: None,
            allow_unsupported_retry: false,
            blocked_reason: None,
        })
        .unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert_eq!(
            events.len(),
            1,
            "no retry event should be appended on invalid input"
        );
    }

    #[tokio::test]
    async fn update_run_rejects_unsupported_retry_without_override() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Search,
                &serde_json::json!({ "query": "lidar" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "retry_scheduled",
                            "reason": "transient_failure"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["error"]["message"]
            .as_str()
            .expect("error message should be present")
            .contains("allow_unsupported_retry=true"));

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::Queued);
        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert_eq!(
            events.len(),
            1,
            "no retry event should be appended on rejection"
        );
    }

    #[tokio::test]
    async fn update_run_allows_unsupported_retry_with_override() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Search,
                &serde_json::json!({ "query": "lidar" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "retry_scheduled",
                            "reason": "operator_override",
                            "allow_unsupported_retry": true
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"];
        assert_eq!(data["status"], "retry_scheduled");
        assert_eq!(data["retry_reason"], "operator_override");
        assert_eq!(data["automatic_retry_supported"], false);
        assert_eq!(data["unsupported_retry_override"], true);
        assert_eq!(
            data["unsupported_retry_override_reason"],
            "manual operator override"
        );

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::RetryScheduled);
        assert_eq!(
            run.output_json
                .as_ref()
                .and_then(|v| v.get("unsupported_retry_override"))
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_eq!(
            run.output_json
                .as_ref()
                .and_then(|v| v.get("unsupported_retry_override_reason"))
                .and_then(serde_json::Value::as_str),
            Some("manual operator override")
        );
        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        let retry_event = events
            .iter()
            .find(|event| event.event_type == vel_core::RunEventType::RunRetryScheduled)
            .expect("retry event should be appended when override is used");
        assert_eq!(retry_event.payload_json["unsupported_retry_override"], true);
        assert_eq!(
            retry_event.payload_json["unsupported_retry_override_reason"],
            "manual operator override"
        );
    }

    #[tokio::test]
    async fn get_run_includes_automatic_retry_policy() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/runs/{}", run_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"];
        assert_eq!(data["automatic_retry_supported"], true);
        assert_eq!(
            data["automatic_retry_reason"],
            "worker can re-execute the original run input"
        );
    }

    #[tokio::test]
    async fn list_runs_includes_unsupported_automatic_retry_policy() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Search,
                &serde_json::json!({ "query": "lidar" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/runs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let run = json["data"]
            .as_array()
            .and_then(|runs| runs.iter().find(|run| run["id"] == run_id.as_ref()))
            .expect("run should be present in list response");
        assert_eq!(run["automatic_retry_supported"], false);
        assert_eq!(
            run["automatic_retry_reason"],
            "search runs do not have an automatic retry executor"
        );
    }

    #[tokio::test]
    async fn end_of_day_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

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
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

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
                source_ref: None,
                timestamp: now_ts + 3600,
                payload_json: Some(serde_json::json!({
                    "start_time": now_ts + 3600,
                    "title": "Meeting"
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
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let commute_nudges: Vec<_> = nudges
            .iter()
            .filter(|n| n["nudge_type"].as_str() == Some("commute_leave_time"))
            .collect();
        assert!(
            commute_nudges.is_empty(),
            "commute nudge must not trigger when travel_minutes missing"
        );
    }

    /// Canonical day: commute nudge fires when calendar event has travel_minutes and we are in leave-by window.
    #[tokio::test]
    async fn commute_nudge_fires_with_travel_minutes_in_leave_window() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let event_start = now_ts + 30 * 60;
        let travel_minutes: i64 = 40;
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: None,
                timestamp: event_start,
                payload_json: Some(serde_json::json!({
                    "start_time": event_start,
                    "title": "Meeting with Dimitri",
                    "travel_minutes": travel_minutes
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
        let _ = app
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
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let commute_nudges: Vec<_> = nudges
            .iter()
            .filter(|n| n["nudge_type"].as_str() == Some("commute_leave_time"))
            .collect();
        assert!(
            !commute_nudges.is_empty(),
            "commute nudge must fire when travel_minutes set and in leave-by window"
        );
    }

    /// Context explain returns signals_used and commitments_used.
    #[tokio::test]
    async fn context_explain_includes_signals_and_commitments_used() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "git_activity".to_string(),
                source: "git".to_string(),
                source_ref: Some("git:/home/jove/code/vel|main|commit|abc123".to_string()),
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(serde_json::json!({
                    "repo": "/home/jove/code/vel",
                    "branch": "main",
                    "operation": "commit",
                    "message": "hydrate explain",
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
        let _ = app
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
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/context")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(
            json["data"]["signals_used"].is_array(),
            "signals_used must be present"
        );
        assert!(
            json["data"]["commitments_used"].is_array(),
            "commitments_used must be present"
        );
        assert!(
            json["data"]["reasons"].is_array(),
            "reasons must be present"
        );
        let summaries = json["data"]["signal_summaries"]
            .as_array()
            .map(|value| value.as_slice())
            .unwrap_or_default();
        let git_summary = summaries
            .iter()
            .find(|summary| summary["signal_type"].as_str() == Some("git_activity"))
            .expect("git_activity summary must be present");
        assert_eq!(git_summary["summary"]["branch"], "main");
        assert_eq!(git_summary["summary"]["operation"], "commit");
    }

    /// Read boundary: explain endpoints must not create commitment_risk or nudge_events rows (repo-feedback 001).
    #[tokio::test]
    async fn explain_endpoints_do_not_mutate_persisted_state() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "vel_read_boundary_{}.db",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        let path_str = path.to_string_lossy().to_string();

        let storage = Storage::connect(&path_str).await.unwrap();
        storage.migrate().await.unwrap();
        let commitment_id = storage
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
            .unwrap()
            .as_ref()
            .to_string();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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

        let storage2 = Storage::connect(&path_str).await.unwrap();
        let current_context_before = storage2.get_current_context().await.unwrap();
        let inferred_state_before = storage2.count_inferred_state().await.unwrap();
        let context_timeline_before = storage2.count_context_timeline().await.unwrap();
        let risk_before = storage2.count_commitment_risk().await.unwrap();
        let nudge_events_before = storage2.count_nudge_events().await.unwrap();
        let nudge_id = storage2
            .list_nudges(None, 10)
            .await
            .unwrap()
            .into_iter()
            .next()
            .expect("evaluate should create at least one nudge")
            .nudge_id;

        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/context")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/drift")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/explain/commitment/{}", commitment_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/explain/nudge/{}", nudge_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/timeline?limit=10")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let current_context_after = storage2.get_current_context().await.unwrap();
        let inferred_state_after = storage2.count_inferred_state().await.unwrap();
        let context_timeline_after = storage2.count_context_timeline().await.unwrap();
        let risk_after = storage2.count_commitment_risk().await.unwrap();
        let nudge_events_after = storage2.count_nudge_events().await.unwrap();

        assert_eq!(
            current_context_before, current_context_after,
            "read-only explain/context routes must not mutate current_context"
        );
        assert_eq!(
            inferred_state_before, inferred_state_after,
            "read-only explain/context routes must not create inferred_state rows"
        );
        assert_eq!(
            context_timeline_before, context_timeline_after,
            "read-only explain/context routes must not create context_timeline rows"
        );
        assert_eq!(
            risk_before, risk_after,
            "read-only explain/context routes must not create commitment_risk rows"
        );
        assert_eq!(
            nudge_events_before, nudge_events_after,
            "read-only explain/context routes must not create nudge_events rows"
        );

        let _ = std::fs::remove_file(&path);
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
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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
        let nudges_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let meds_nudge = nudges
            .iter()
            .find(|n| n["nudge_type"].as_str() == Some("meds_not_logged"));
        let nudge_id = meds_nudge
            .and_then(|n| n["nudge_id"].as_str())
            .expect("meds nudge should exist");
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
        let _ = app
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
        let nudges_resp2 = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body2 = axum::body::to_bytes(nudges_resp2.into_body(), usize::MAX)
            .await
            .unwrap();
        let json2: serde_json::Value = serde_json::from_slice(&body2).unwrap();
        let resolved: Vec<_> = json2["data"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|n| n["nudge_id"].as_str() == Some(nudge_id))
            .collect();
        assert_eq!(resolved.len(), 1, "nudge should appear exactly once");
        assert_eq!(
            resolved[0]["state"].as_str(),
            Some("resolved"),
            "resolved nudge must stay resolved after second evaluate"
        );
    }

    // --- Canonical day fixture: Meeting with Dimitri at 11:00, prep 30 min, travel 40 min, meds/prep/commute open ---
    async fn canonical_day_fixture(
        storage: &vel_storage::Storage,
        now_ts: i64,
        event_offset_minutes: i64,
    ) -> (i64, i64, String, String, String) {
        let event_start = now_ts + event_offset_minutes * 60;
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: None,
                timestamp: event_start,
                payload_json: Some(serde_json::json!({
                    "start_time": event_start,
                    "title": "Meeting with Dimitri",
                    "location": "Salt Lake City",
                    "prep_minutes": 30,
                    "travel_minutes": 40
                })),
            })
            .await
            .unwrap();
        let meds_id = storage
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
        let prep_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Prepare for Meeting with Dimitri".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: Some("vel".to_string()),
                commitment_kind: Some("prep".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        let commute_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Commute to Meeting with Dimitri".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("commute".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        (
            event_start,
            now_ts,
            meds_id.as_ref().to_string(),
            prep_id.as_ref().to_string(),
            commute_id.as_ref().to_string(),
        )
    }

    /// §6.1 Context assertions: prep/commute window, meds status, next commitment present.
    #[tokio::test]
    async fn canonical_day_context_assertions() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"]["context"];
        assert!(
            data.get("prep_window_active").is_some(),
            "prep_window_active must be present"
        );
        assert!(
            data.get("commute_window_active").is_some(),
            "commute_window_active must be present"
        );
        assert!(
            data.get("meds_status").is_some(),
            "meds_status must be present"
        );
        assert!(data.get("mode").is_some(), "mode must be present");
    }

    /// §6.2 Risk assertions: risk list non-empty, factors present.
    #[tokio::test]
    async fn canonical_day_risk_assertions() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/risk")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let list = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        assert!(
            !list.is_empty(),
            "risk list should be non-empty when commitments exist"
        );
    }

    /// §6.3 Nudge: snooze suppresses repeated firing until snoozed_until.
    #[tokio::test]
    async fn canonical_day_nudge_snooze_suppresses() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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
        let nudges_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let commute = nudges
            .iter()
            .find(|n| n["nudge_type"].as_str() == Some("commute_leave_time"));
        let nudge_id = commute
            .and_then(|n| n["nudge_id"].as_str())
            .expect("commute nudge should exist");
        let _snooze_until = now_ts + 15 * 60;
        let snooze_body = serde_json::json!({ "minutes": 15 }).to_string();
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/nudges/{}/snooze", nudge_id))
                    .header("content-type", "application/json")
                    .body(Body::from(snooze_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let _ = app
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
        let nudges_resp2 = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body2 = axum::body::to_bytes(nudges_resp2.into_body(), usize::MAX)
            .await
            .unwrap();
        let json2: serde_json::Value = serde_json::from_slice(&body2).unwrap();
        let same_nudge: Vec<_> = json2["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default()
            .iter()
            .filter(|n| n["nudge_id"].as_str() == Some(nudge_id))
            .collect();
        assert_eq!(
            same_nudge.len(),
            1,
            "snoozed nudge should still appear once"
        );
        assert_eq!(
            same_nudge[0]["state"].as_str(),
            Some("snoozed"),
            "nudge should be snoozed"
        );
    }

    /// §6.3 Nudge: event start suppresses or resolves stale commute nudge.
    #[tokio::test]
    async fn canonical_day_event_start_suppresses_commute_nudge() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let event_start = now_ts - 60;
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: None,
                timestamp: event_start,
                payload_json: Some(serde_json::json!({
                    "start_time": event_start,
                    "title": "Meeting with Dimitri",
                    "travel_minutes": 40
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
        let _ = app
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
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let active_commute: Vec<_> = json["data"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|n| {
                n["nudge_type"].as_str() == Some("commute_leave_time")
                    && n["state"].as_str() == Some("active")
            })
            .collect();
        assert!(
            active_commute.is_empty(),
            "commute nudge should be resolved or absent after event start passed"
        );
    }

    /// §6.5 Explain: context explain references commitment ids and signal ids.
    #[tokio::test]
    async fn canonical_day_explain_references_commitments_and_signals() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let (_, _, meds_id, prep_id, _) = canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/context")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let commitments_used = json["data"]["commitments_used"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let signals_used = json["data"]["signals_used"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let signal_summaries = json["data"]["signal_summaries"]
            .as_array()
            .map(|value| value.as_slice())
            .unwrap_or_default();
        assert!(
            !signals_used.is_empty(),
            "signals_used must reference calendar signal"
        );
        let calendar_summary = signal_summaries
            .iter()
            .find(|summary| summary["signal_type"].as_str() == Some("calendar_event"))
            .expect("calendar_event summary must be present");
        assert!(
            calendar_summary["summary"]["title"].is_string()
                || calendar_summary["summary"]["travel_minutes"].is_number()
        );
        let commitment_ids: Vec<&str> =
            commitments_used.iter().filter_map(|c| c.as_str()).collect();
        assert!(
            commitment_ids.contains(&meds_id.as_str())
                || commitment_ids.contains(&prep_id.as_str()),
            "commitments_used should include fixture commitments"
        );
    }

    /// §6.6 Synthesis: project synthesis artifact created with open commitments.
    #[tokio::test]
    async fn canonical_day_project_synthesis_artifact() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/synthesis/project/vel")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(
            json["data"].get("artifact_id").is_some() || json["data"].get("run_id").is_some(),
            "project synthesis should return artifact or run"
        );
    }

    /// Variant A (success path): meds done reduces active nudges.
    #[tokio::test]
    async fn canonical_day_variant_a_success_meds_done() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let (_, _, meds_id, _, _) = canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("/v1/commitments/{}", meds_id))
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"status":"done"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let _ = app
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
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let meds_nudges: Vec<_> = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default()
            .iter()
            .filter(|n| {
                n["nudge_type"].as_str() == Some("meds_not_logged")
                    && (n["state"].as_str() == Some("active")
                        || n["state"].as_str() == Some("snoozed"))
            })
            .collect();
        assert!(
            meds_nudges.is_empty(),
            "meds nudge should be gone after commitment done"
        );
    }

    /// Variant B (drift path): in danger window, drift and commute nudge present.
    #[tokio::test]
    async fn canonical_day_variant_b_drift_commute_danger() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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
        let context_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(context_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let ctx: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let current_context = &ctx["data"]["context"];
        let drift_type = current_context.get("drift_type");
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nbody = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let njson: serde_json::Value = serde_json::from_slice(&nbody).unwrap();
        let commute: Vec<_> = njson["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default()
            .iter()
            .filter(|n| n["nudge_type"].as_str() == Some("commute_leave_time"))
            .collect();
        assert!(
            !commute.is_empty(),
            "commute nudge should exist in danger window (variant B)"
        );
        assert!(
            drift_type.is_some() || current_context.get("attention_state").is_some(),
            "drift or attention state should be present"
        );
    }

    /// Variant C (suggestion path): repeated commute danger triggers increase_commute_buffer suggestion.
    #[tokio::test]
    async fn canonical_day_variant_c_suggestion_from_repeated_evidence() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let window_start = now_ts - 7 * 86400;
        for _ in 0..2 {
            let _ = storage
                .insert_nudge(vel_storage::NudgeInsert {
                    nudge_type: "commute_leave_time".to_string(),
                    level: "danger".to_string(),
                    state: "resolved".to_string(),
                    related_commitment_id: None,
                    message: "You may be late.".to_string(),
                    snoozed_until: None,
                    resolved_at: Some(window_start + 86400),
                    signals_snapshot_json: None,
                    inference_snapshot_json: None,
                    metadata_json: None,
                })
                .await
                .unwrap();
        }
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
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
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/suggestions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let suggestions = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let commute_buf: Vec<_> = suggestions
            .iter()
            .filter(|s| s["suggestion_type"].as_str() == Some("increase_commute_buffer"))
            .collect();
        assert!(!commute_buf.is_empty(), "increase_commute_buffer suggestion should appear after repeated commute danger (variant C)");
    }

    // --- Chat API (ticket 034) ---

    #[tokio::test]
    async fn chat_list_conversations_empty() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/conversations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["ok"].as_bool().unwrap());
        assert!(json["data"]
            .as_array()
            .map(|a| a.is_empty())
            .unwrap_or(false));
    }

    #[tokio::test]
    async fn chat_create_conversation_then_list() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let create_body = r#"{"title":"Test conv","kind":"general"}"#;
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(create_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(create_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["ok"].as_bool().unwrap());
        let id = json["data"]["id"].as_str().unwrap();
        assert!(id.starts_with("conv_"));

        let list_resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/conversations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list_body = axum::body::to_bytes(list_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let list_json: serde_json::Value = serde_json::from_slice(&list_body).unwrap();
        let convs = list_json["data"].as_array().unwrap();
        assert_eq!(convs.len(), 1);
        assert_eq!(convs[0]["id"].as_str().unwrap(), id);
    }

    #[tokio::test]
    async fn chat_get_conversation_404() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/conversations/conv_nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn chat_create_message_then_list() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"user","kind":"text","content":{"text":"hello"}}"#;
        let msg_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);
        let msg_resp_body = axum::body::to_bytes(msg_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let msg_json: serde_json::Value = serde_json::from_slice(&msg_resp_body).unwrap();
        let user_msg = &msg_json["data"]["user_message"];
        assert!(user_msg["id"].as_str().unwrap().starts_with("msg_"));
        assert_eq!(user_msg["content"]["text"].as_str().unwrap(), "hello");

        let list_resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list_body = axum::body::to_bytes(list_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let list_json: serde_json::Value = serde_json::from_slice(&list_body).unwrap();
        assert_eq!(list_json["data"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn chat_inbox_empty() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/inbox")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["ok"].as_bool().unwrap());
        assert!(json["data"]
            .as_array()
            .map(|a| a.is_empty())
            .unwrap_or(false));
    }

    #[tokio::test]
    async fn chat_list_conversation_interventions() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let conversation_id = storage
            .create_conversation(vel_storage::ConversationInsert {
                id: "conv_test".to_string(),
                title: Some("T".to_string()),
                kind: "general".to_string(),
                pinned: false,
                archived: false,
            })
            .await
            .unwrap();
        let message_id = storage
            .create_message(vel_storage::MessageInsert {
                id: "msg_test".to_string(),
                conversation_id: conversation_id.as_ref().to_string(),
                role: "assistant".to_string(),
                kind: "reminder_card".to_string(),
                content_json: r#"{"title":"Reminder"}"#.to_string(),
                status: None,
                importance: None,
            })
            .await
            .unwrap();
        storage
            .create_intervention(vel_storage::InterventionInsert {
                id: "intv_test".to_string(),
                message_id: message_id.as_ref().to_string(),
                kind: "reminder".to_string(),
                state: "active".to_string(),
                surfaced_at: 100,
                resolved_at: None,
                snoozed_until: None,
                confidence: None,
                source_json: None,
                provenance_json: None,
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

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/conversations/conv_test/interventions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = json["data"].as_array().unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0]["id"].as_str().unwrap(), "intv_test");
        assert_eq!(data[0]["message_id"].as_str().unwrap(), "msg_test");
    }

    #[tokio::test]
    async fn chat_assistant_card_message_creates_intervention() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"assistant","kind":"reminder_card","content":{"title":"Take meds","reason":"morning routine"}}"#;
        let msg_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);
        let msg_resp_body = axum::body::to_bytes(msg_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let msg_json: serde_json::Value = serde_json::from_slice(&msg_resp_body).unwrap();
        let message_id = msg_json["data"]["user_message"]["id"].as_str().unwrap();

        let interventions = storage
            .get_interventions_by_conversation(conv_id)
            .await
            .unwrap();
        assert_eq!(interventions.len(), 1);
        assert_eq!(interventions[0].message_id.as_ref(), message_id);
        assert_eq!(interventions[0].kind, "reminder");
        assert_eq!(interventions[0].state, "active");

        let events = storage
            .list_events_by_aggregate("intervention", interventions[0].id.as_ref(), 10)
            .await
            .unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_name == "intervention.created"));
    }

    #[tokio::test]
    async fn chat_assistant_card_message_emits_typed_websocket_events() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        let mut rx = broadcast_tx.subscribe();
        let state = crate::state::AppState::new(
            storage,
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        );
        let app = build_app_with_state(state);

        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"assistant","kind":"reminder_card","content":{"title":"Take meds","reason":"morning routine"}}"#;
        let msg_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);

        let intervention_event = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
            .await
            .unwrap()
            .unwrap();
        let message_event = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            intervention_event.event_type,
            vel_api_types::WsEventType::InterventionsNew
        );
        assert_eq!(
            intervention_event.payload["kind"].as_str().unwrap(),
            "reminder"
        );
        assert_eq!(
            intervention_event.payload["state"].as_str().unwrap(),
            "active"
        );

        assert_eq!(
            message_event.event_type,
            vel_api_types::WsEventType::MessagesNew
        );
        assert_eq!(message_event.payload["role"].as_str().unwrap(), "assistant");
        assert_eq!(
            message_event.payload["kind"].as_str().unwrap(),
            "reminder_card"
        );
        assert_eq!(
            message_event.payload["conversation_id"].as_str().unwrap(),
            conv_id
        );

        let message_id = message_event.payload["id"].as_str().unwrap();
        assert_eq!(
            intervention_event.payload["message_id"].as_str().unwrap(),
            message_id
        );
    }

    #[tokio::test]
    async fn chat_message_provenance_hydrates_message_and_intervention_data() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"assistant","kind":"reminder_card","content":{"title":"Take meds","reason":"morning routine","confidence":0.82}}"#;
        let msg_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);
        let msg_resp_body = axum::body::to_bytes(msg_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let msg_json: serde_json::Value = serde_json::from_slice(&msg_resp_body).unwrap();
        let message_id = msg_json["data"]["user_message"]["id"].as_str().unwrap();

        let provenance_resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/messages/{}/provenance", message_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(provenance_resp.status(), StatusCode::OK);
        let provenance_body = axum::body::to_bytes(provenance_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let provenance_json: serde_json::Value = serde_json::from_slice(&provenance_body).unwrap();
        let data = &provenance_json["data"];

        assert_eq!(data["message_id"].as_str().unwrap(), message_id);

        let events = data["events"].as_array().unwrap();
        assert!(!events.is_empty());
        assert!(events.iter().any(|event| {
            event["event_name"].as_str() == Some("message.created")
                || event["event_name"].as_str() == Some("message.updated")
        }));

        let linked_objects = data["linked_objects"].as_array().unwrap();
        assert!(linked_objects.iter().any(|object| {
            object["kind"].as_str() == Some("message")
                && object["id"].as_str() == Some(message_id)
                && object["conversation_id"].as_str() == Some(conv_id)
                && object["message_kind"].as_str() == Some("reminder_card")
        }));
        assert!(linked_objects.iter().any(|object| {
            object["kind"].as_str() == Some("intervention")
                && object["message_id"].as_str() == Some(message_id)
                && object["intervention_kind"].as_str() == Some("reminder")
                && object["state"].as_str() == Some("active")
                && object["source"]["title"].as_str() == Some("Take meds")
                && object["source"]["reason"].as_str() == Some("morning routine")
                && object["provenance"]["message_id"].as_str() == Some(message_id)
                && object["provenance"]["conversation_id"].as_str() == Some(conv_id)
        }));

        let signals = data["signals"].as_array().unwrap();
        assert!(signals.iter().any(|signal| {
            signal["kind"].as_str() == Some("message_content")
                && signal["message_kind"].as_str() == Some("reminder_card")
                && signal["title"].as_str() == Some("Take meds")
                && signal["reason"].as_str() == Some("morning routine")
        }));
        assert!(signals.iter().any(|signal| {
            signal["kind"].as_str() == Some("intervention_source")
                && signal["intervention_kind"].as_str() == Some("reminder")
                && signal["payload"]["title"].as_str() == Some("Take meds")
        }));
        assert!(signals.iter().any(|signal| {
            signal["kind"].as_str() == Some("intervention_provenance")
                && signal["intervention_id"].is_string()
                && signal["payload"]["message_kind"].as_str() == Some("reminder_card")
        }));

        let policy_decisions = data["policy_decisions"].as_array().unwrap();
        assert!(policy_decisions.iter().any(|decision| {
            decision["kind"].as_str() == Some("message_policy")
                && decision["message_kind"].as_str() == Some("reminder_card")
                && decision["reason"].as_str() == Some("morning routine")
                && decision["confidence"].as_f64() == Some(0.82)
        }));
        assert!(policy_decisions.iter().any(|decision| {
            decision["kind"].as_str() == Some("intervention_state")
                && decision["intervention_kind"].as_str() == Some("reminder")
                && decision["state"].as_str() == Some("active")
        }));
    }

    #[tokio::test]
    async fn chat_assistant_text_message_does_not_create_intervention() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"assistant","kind":"text","content":{"text":"plain reply"}}"#;
        let msg_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);

        let interventions = storage
            .get_interventions_by_conversation(conv_id)
            .await
            .unwrap();
        assert!(interventions.is_empty());
    }

    #[tokio::test]
    async fn sync_calendar_ingests_tzid_events() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_calendar_tzid_{}.ics",
            uuid::Uuid::new_v4().simple()
        ));
        let ics = r#"BEGIN:VCALENDAR
VERSION:2.0
BEGIN:VEVENT
UID:evt_tzid_1
DTSTART;TZID=America/Denver:20260116T110000
DTEND;TZID=America/Denver:20260116T120000
SUMMARY:Planning meeting
LOCATION:Studio
END:VEVENT
END:VCALENDAR
"#;
        std::fs::write(&file_path, ics).unwrap();

        let config = vel_config::AppConfig {
            calendar_ics_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/calendar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let signals = storage
            .list_signals(Some("calendar_event"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1, "TZID calendar event should be ingested");

        let expected_start = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2026, time::Month::January, 16).unwrap(),
            time::Time::from_hms(11, 0, 0).unwrap(),
        )
        .assume_offset(time::UtcOffset::from_hms(-7, 0, 0).unwrap())
        .unix_timestamp();
        let expected_end = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2026, time::Month::January, 16).unwrap(),
            time::Time::from_hms(12, 0, 0).unwrap(),
        )
        .assume_offset(time::UtcOffset::from_hms(-7, 0, 0).unwrap())
        .unix_timestamp();

        assert_eq!(signals[0].timestamp, expected_start);
        assert_eq!(signals[0].payload_json["event_id"], "evt_tzid_1");
        assert_eq!(signals[0].payload_json["title"], "Planning meeting");
        assert_eq!(signals[0].payload_json["location"], "Studio");
        assert_eq!(signals[0].payload_json["start"], expected_start);
        assert_eq!(signals[0].payload_json["end"], expected_end);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_calendar_preserves_explicit_prep_and_travel_minutes() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_calendar_fields_{}.ics",
            uuid::Uuid::new_v4().simple()
        ));
        let ics = r#"BEGIN:VCALENDAR
VERSION:2.0
BEGIN:VEVENT
UID:evt_fields_1
DTSTART:20260116T180000Z
DTEND:20260116T190000Z
SUMMARY:Client review
LOCATION:HQ
X-VEL-PREP-MINUTES:30
X-VEL-TRAVEL-MINUTES:40
END:VEVENT
END:VCALENDAR
"#;
        std::fs::write(&file_path, ics).unwrap();

        let config = vel_config::AppConfig {
            calendar_ics_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/calendar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let signals = storage
            .list_signals(Some("calendar_event"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].payload_json["event_id"], "evt_fields_1");
        assert_eq!(signals[0].payload_json["prep_minutes"], 30);
        assert_eq!(signals[0].payload_json["travel_minutes"], 40);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_transcripts_ingests_rows_and_signals() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_transcripts_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "source": "chatgpt",
            "conversation_id": "conv_external",
            "messages": [
                {
                    "timestamp": 1700000000,
                    "role": "user",
                    "content": "What did we decide about Vel?",
                    "metadata": { "project_hint": "vel" }
                },
                {
                    "timestamp": 1700000060,
                    "role": "assistant",
                    "content": "You said to prioritize repeated personal use.",
                    "metadata": {}
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            transcript_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/transcripts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let transcripts = storage
            .list_assistant_transcripts_by_conversation("conv_external")
            .await
            .unwrap();
        assert_eq!(transcripts.len(), 2);
        assert_eq!(transcripts[0].source, "chatgpt");
        assert_eq!(transcripts[0].role, "user");

        let signals = storage
            .list_signals(Some("assistant_message"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 2);
        assert_eq!(signals[0].source, "chatgpt");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_transcripts_replay_is_deduplicated() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_transcripts_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!([
            {
                "id": "tr_fixed_1",
                "source": "chatgpt",
                "conversation_id": "conv_dedupe",
                "timestamp": 1700000100,
                "role": "user",
                "content": "hello",
                "metadata": {}
            }
        ]);
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            transcript_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        for _ in 0..2 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/v1/sync/transcripts")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        let transcripts = storage
            .list_assistant_transcripts_by_conversation("conv_dedupe")
            .await
            .unwrap();
        assert_eq!(transcripts.len(), 1);

        let signals = storage
            .list_signals(Some("assistant_message"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_activity_ingests_snapshot_events() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_activity_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "source": "workstation",
            "events": [
                {
                    "signal_type": "shell_login",
                    "timestamp": 1700001000,
                    "host": "ws-1",
                    "details": { "tty": "pts/1" }
                },
                {
                    "signal_type": "computer_activity",
                    "timestamp": 1700001060,
                    "host": "ws-1",
                    "details": { "app": "zed" }
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            activity_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/activity")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let shell_signals = storage
            .list_signals(Some("shell_login"), None, 10)
            .await
            .unwrap();
        let activity_signals = storage
            .list_signals(Some("computer_activity"), None, 10)
            .await
            .unwrap();
        assert_eq!(shell_signals.len(), 1);
        assert_eq!(activity_signals.len(), 1);
        assert_eq!(shell_signals[0].source, "workstation");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_notes_ingests_markdown_files_as_captures() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir().join(format!("vel_notes_{}", uuid::Uuid::new_v4().simple()));
        let nested_dir = dir.join("daily");
        std::fs::create_dir_all(&nested_dir).unwrap();
        std::fs::write(nested_dir.join("today.md"), "# Today\nShip notes sync\n").unwrap();
        std::fs::write(dir.join("ignore.json"), "{\"skip\":true}").unwrap();

        let config = vel_config::AppConfig {
            notes_path: Some(dir.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/notes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let captures = storage.list_captures_recent(10, false).await.unwrap();
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].capture_type, "note_document");
        assert!(captures[0].content_text.contains("Ship notes sync"));

        let signals = storage
            .list_signals(Some("note_document"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].source, "notes");
        assert_eq!(signals[0].payload_json["path"], "daily/today.md");
        assert_eq!(signals[0].payload_json["title"], "Today");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn sync_notes_replay_is_deduplicated() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir().join(format!("vel_notes_{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("ideas.md"), "# Ideas\nMore context\n").unwrap();

        let config = vel_config::AppConfig {
            notes_path: Some(dir.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        for _ in 0..2 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/v1/sync/notes")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        assert_eq!(storage.capture_count().await.unwrap(), 1);
        let signals = storage
            .list_signals(Some("note_document"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn sync_git_replay_is_deduplicated_by_source_ref() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!("vel_git_{}.json", uuid::Uuid::new_v4().simple()));
        let snapshot = serde_json::json!({
            "source": "git",
            "events": [
                {
                    "timestamp": 1700002000,
                    "repo": "/home/jove/code/vel",
                    "repo_name": "vel",
                    "branch": "main",
                    "operation": "commit",
                    "commit_oid": "abc123",
                    "message": "feat: add git sync"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            git_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        for _ in 0..2 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/v1/sync/git")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        let signals = storage
            .list_signals(Some("git_activity"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(
            signals[0].source_ref.as_deref(),
            Some("git:/home/jove/code/vel|main|commit|abc123|1700002000")
        );

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_messaging_ingests_snapshot_and_triggers_evaluation() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_messages_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "source": "messaging",
            "account_id": "local-default",
            "threads": [
                {
                    "thread_id": "thr_ops",
                    "platform": "sms",
                    "title": "Review reschedule",
                    "participants": [
                        { "id": "me", "name": "Me", "is_me": true },
                        { "id": "+15551234567", "name": "Sam", "is_me": false }
                    ],
                    "latest_timestamp": now,
                    "waiting_state": "me",
                    "scheduling_related": true,
                    "urgent": true,
                    "summary": "Need to answer the review reschedule request.",
                    "snippet": "Can we move the review to 3?"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            messaging_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        for _ in 0..2 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/v1/sync/messaging")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        let signals = storage
            .list_signals(Some("message_thread"), None, 10)
            .await
            .unwrap();
        let expected_source_ref = format!("messaging:sms:local-default:thr_ops:{now}");
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].source, "messaging");
        assert_eq!(
            signals[0].source_ref.as_deref(),
            Some(expected_source_ref.as_str())
        );
        assert_eq!(signals[0].payload_json["platform"], "sms");
        assert_eq!(signals[0].payload_json["thread_id"], "thr_ops");
        assert_eq!(signals[0].payload_json["waiting_state"], "me");
        assert_eq!(signals[0].payload_json["scheduling_related"], true);
        assert_eq!(signals[0].payload_json["urgent"], true);
        assert_eq!(
            signals[0].payload_json["snippet"],
            "Can we move the review to 3?"
        );

        let (_, context_json) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("sync should trigger evaluate and store current context");
        let context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
        assert_eq!(context["message_waiting_on_me_count"], 1);
        assert_eq!(context["message_scheduling_thread_count"], 1);
        assert_eq!(context["message_urgent_thread_count"], 1);

        let nudges = storage.list_nudges(None, 20).await.unwrap();
        let response_debt = nudges
            .iter()
            .find(|n| n.nudge_type == "response_debt")
            .expect("sync-triggered evaluate should create response_debt nudge");
        assert_eq!(response_debt.state, "active");
        assert_eq!(response_debt.level, "warning");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn evaluate_includes_messaging_summary_in_current_context() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "message_thread".to_string(),
                source: "messaging".to_string(),
                source_ref: Some(format!("messaging:gmail:default:thr_1:{}", now)),
                timestamp: now,
                payload_json: Some(serde_json::json!({
                    "thread_id": "thr_1",
                    "platform": "gmail",
                    "title": "Dimitri follow-up",
                    "participants": [
                        { "id": "me@example.com", "name": "Me", "is_me": true },
                        { "id": "dimitri@example.com", "name": "Dimitri", "is_me": false }
                    ],
                    "latest_timestamp": now,
                    "waiting_state": "me",
                    "scheduling_related": true,
                    "urgent": true,
                    "snippet": "Can you send the updated draft?"
                })),
            })
            .await
            .unwrap();

        let evaluate_result = crate::services::evaluate::run(&storage, &test_policy_config()).await;
        assert!(evaluate_result.is_ok());

        let (_, context_json) = storage.get_current_context().await.unwrap().unwrap();
        let context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
        assert_eq!(context["message_waiting_on_me_count"], 1);
        assert_eq!(context["message_scheduling_thread_count"], 1);
        assert_eq!(context["message_urgent_thread_count"], 1);
        assert_eq!(
            context["message_summary"]["top_threads"][0]["title"],
            "Dimitri follow-up"
        );
    }

    #[tokio::test]
    async fn evaluate_creates_response_debt_nudge_from_messaging_context() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "message_thread".to_string(),
                source: "messaging".to_string(),
                source_ref: Some(format!("messaging:sms:default:thr_sched:{}", now)),
                timestamp: now,
                payload_json: Some(serde_json::json!({
                    "thread_id": "thr_sched",
                    "platform": "sms",
                    "title": "Team reschedule",
                    "latest_timestamp": now,
                    "waiting_state": "me",
                    "scheduling_related": true,
                    "urgent": true,
                    "snippet": "Can we move the standup to 3?"
                })),
            })
            .await
            .unwrap();

        let evaluate_result = crate::services::evaluate::run(&storage, &test_policy_config()).await;
        assert!(evaluate_result.is_ok());

        let nudges = storage.list_nudges(None, 20).await.unwrap();
        let nudge = nudges
            .iter()
            .find(|n| n.nudge_type == "response_debt")
            .expect("response_debt nudge should exist");
        assert_eq!(nudge.state, "active");
        assert_eq!(nudge.level, "warning");
        assert_eq!(
            nudge.message,
            "You have messages waiting on you, including scheduling follow-up."
        );
        let inference = nudge
            .inference_snapshot_json
            .as_ref()
            .map(|s| serde_json::from_str::<serde_json::Value>(s).unwrap())
            .unwrap();
        assert_eq!(inference["message_waiting_on_me_count"], 1);
        assert_eq!(inference["message_scheduling_thread_count"], 1);
        assert_eq!(inference["message_urgent_thread_count"], 1);
        let metadata = &nudge.metadata_json;
        assert_eq!(metadata["policy"], "response_debt");
    }

    #[tokio::test]
    async fn response_debt_nudge_resolves_when_waiting_count_clears() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .set_current_context(
                now,
                &serde_json::json!({
                    "message_waiting_on_me_count": 0,
                    "message_scheduling_thread_count": 0,
                    "message_urgent_thread_count": 0
                })
                .to_string(),
            )
            .await
            .unwrap();

        let nudge_id = storage
            .insert_nudge(vel_storage::NudgeInsert {
                nudge_type: "response_debt".to_string(),
                level: "warning".to_string(),
                state: "active".to_string(),
                related_commitment_id: None,
                message: "You have messages waiting on you.".to_string(),
                snoozed_until: None,
                resolved_at: None,
                signals_snapshot_json: None,
                inference_snapshot_json: None,
                metadata_json: Some(serde_json::json!({ "policy": "response_debt" })),
            })
            .await
            .unwrap();

        let updated_result =
            crate::services::nudge_engine::evaluate(&storage, &test_policy_config(), 0).await;
        assert!(updated_result.is_ok());
        let updated = updated_result.unwrap_or_default();
        assert_eq!(updated, 1);

        let nudges = storage.list_nudges(None, 20).await.unwrap();
        let nudge = nudges
            .iter()
            .find(|n| n.nudge_id == nudge_id)
            .expect("response_debt nudge should still exist");
        assert_eq!(nudge.state, "resolved");
        assert!(nudge.resolved_at.is_some());
    }

    #[tokio::test]
    async fn sync_todoist_reopens_and_updates_existing_commitment() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let commitment_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Old title".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todoist_123".to_string()),
                status: vel_core::CommitmentStatus::Done,
                due_at: None,
                project: Some("old".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({ "todoist_id": "123" })),
            })
            .await
            .unwrap();
        storage
            .update_commitment(
                commitment_id.as_ref(),
                None,
                Some(vel_core::CommitmentStatus::Done),
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_todoist_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "items": [
                {
                    "id": "123",
                    "content": "Updated title",
                    "checked": false,
                    "due": { "date": "2026-03-17T09:30:00" },
                    "labels": ["health"],
                    "project_id": "proj-1"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            todoist_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/todoist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let updated = storage
            .get_commitment_by_id(commitment_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.status, vel_core::CommitmentStatus::Open);
        assert_eq!(updated.text, "Updated title");
        assert_eq!(updated.project.as_deref(), Some("proj-1"));
        assert_eq!(updated.commitment_kind.as_deref(), Some("medication"));
        assert!(updated.resolved_at.is_none());

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_todoist_marks_commitment_done_when_task_checked() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let commitment_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Ship feature".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todoist_456".to_string()),
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({ "todoist_id": "456" })),
            })
            .await
            .unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_todoist_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "items": [
                {
                    "id": "456",
                    "content": "Ship feature",
                    "checked": true,
                    "labels": [],
                    "project_id": "proj-2"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            todoist_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/todoist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let updated = storage
            .get_commitment_by_id(commitment_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.status, vel_core::CommitmentStatus::Done);
        assert!(updated.resolved_at.is_some());

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn inference_uses_shell_login_as_workstation_activity() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "shell_login".to_string(),
                source: "workstation".to_string(),
                source_ref: None,
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(
                    serde_json::json!({ "host": "ws-1", "activity": "shell_login" }),
                ),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);

        let ctx_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ctx_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(ctx_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["context"]["inferred_activity"],
            "computer_active"
        );
        assert_eq!(json["data"]["context"]["morning_state"], "engaged");
        assert!(json["data"]["context"]["git_activity_summary"].is_null());
    }

    #[tokio::test]
    async fn inference_uses_git_activity_as_workstation_activity() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "git_activity".to_string(),
                source: "git".to_string(),
                source_ref: Some(
                    "git:/home/jove/code/vel|main|commit|abc123|1700002000".to_string(),
                ),
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(serde_json::json!({
                    "repo": "/home/jove/code/vel",
                    "branch": "main",
                    "operation": "commit",
                    "commit_oid": "abc123"
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);

        let ctx_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ctx_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(ctx_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["data"]["context"]["inferred_activity"], "coding");
        assert_eq!(json["data"]["context"]["morning_state"], "engaged");
        assert_eq!(
            json["data"]["context"]["git_activity_summary"]["repo"],
            "vel"
        );
        assert_eq!(
            json["data"]["context"]["git_activity_summary"]["branch"],
            "main"
        );
        assert_eq!(
            json["data"]["context"]["git_activity_summary"]["operation"],
            "commit"
        );
    }

    #[tokio::test]
    async fn chat_settings_get_and_patch() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let get_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get_resp.status(), StatusCode::OK);

        let patch_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/settings")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"disable_proactive":true}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["data"]["disable_proactive"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn integrations_google_calendar_settings_and_auth_start() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let config = AppConfig {
            base_url: "http://127.0.0.1:4130".to_string(),
            ..Default::default()
        };
        let app = build_app(storage, config, test_policy_config(), None, None);

        let patch_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/google-calendar")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"client_id":"gcal-client","client_secret":"gcal-secret"}"#.to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let patch_body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let patch_json: serde_json::Value = serde_json::from_slice(&patch_body).unwrap();
        assert_eq!(patch_json["data"]["google_calendar"]["configured"], true);
        assert_eq!(patch_json["data"]["google_calendar"]["connected"], false);
        assert_eq!(patch_json["data"]["google_calendar"]["has_client_id"], true);
        assert_eq!(
            patch_json["data"]["google_calendar"]["has_client_secret"],
            true
        );

        let auth_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/integrations/google-calendar/auth/start")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(auth_resp.status(), StatusCode::OK);
        let auth_body = axum::body::to_bytes(auth_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let auth_json: serde_json::Value = serde_json::from_slice(&auth_body).unwrap();
        let auth_url = auth_json["data"]["auth_url"]
            .as_str()
            .expect("auth_url should be returned");
        assert!(auth_url.starts_with("https://accounts.google.com/o/oauth2/v2/auth?"));
        assert!(auth_url.contains("client_id=gcal-client"));
        assert!(auth_url.contains("redirect_uri=http%3A%2F%2F127.0.0.1%3A4130%2Fapi%2Fintegrations%2Fgoogle-calendar%2Foauth%2Fcallback"));
        assert!(
            auth_url.contains("scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fcalendar.readonly")
        );
    }

    #[tokio::test]
    async fn integrations_store_sensitive_tokens_separately_from_public_settings() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/google-calendar")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"client_id":"gcal-client","client_secret":"gcal-secret"}"#.to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        let _ = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/todoist")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"api_token":"todoist-token"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let settings = storage.get_all_settings().await.unwrap();
        let google_public = settings
            .get("integration_google_calendar")
            .expect("google public settings should exist");
        let google_secrets = settings
            .get("integration_google_calendar_secrets")
            .expect("google secrets should exist");
        let todoist_public = settings
            .get("integration_todoist")
            .expect("todoist public settings should exist");
        let todoist_secrets = settings
            .get("integration_todoist_secrets")
            .expect("todoist secrets should exist");

        assert_eq!(google_public["client_id"], "gcal-client");
        assert!(
            google_public.get("client_secret").is_none()
                || google_public["client_secret"].is_null()
        );
        assert!(
            google_public.get("refresh_token").is_none()
                || google_public["refresh_token"].is_null()
        );
        assert_eq!(google_secrets["client_secret"], "gcal-secret");

        assert!(todoist_public.get("api_token").is_none() || todoist_public["api_token"].is_null());
        assert_eq!(todoist_secrets["api_token"], "todoist-token");
    }

    #[tokio::test]
    async fn integrations_todoist_patch_and_disconnect() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let patch_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/todoist")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"api_token":"todoist-token"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let patch_body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let patch_json: serde_json::Value = serde_json::from_slice(&patch_body).unwrap();
        assert_eq!(patch_json["data"]["todoist"]["configured"], true);
        assert_eq!(patch_json["data"]["todoist"]["connected"], true);
        assert_eq!(patch_json["data"]["todoist"]["has_api_token"], true);

        let disconnect_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/integrations/todoist/disconnect")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(disconnect_resp.status(), StatusCode::OK);
        let disconnect_body = axum::body::to_bytes(disconnect_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let disconnect_json: serde_json::Value = serde_json::from_slice(&disconnect_body).unwrap();
        assert_eq!(disconnect_json["data"]["todoist"]["configured"], false);
        assert_eq!(disconnect_json["data"]["todoist"]["connected"], false);
        assert_eq!(disconnect_json["data"]["todoist"]["has_api_token"], false);
        assert_eq!(
            disconnect_json["data"]["todoist"]["last_sync_status"],
            "disconnected"
        );
    }

    #[tokio::test]
    async fn integrations_google_calendar_selection_persists_and_disconnect_clears_connection() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(
                "integration_google_calendar",
                &serde_json::json!({
                    "client_id": "gcal-client",
                    "calendars": [
                        { "id": "cal_a", "summary": "Primary", "primary": true, "selected": true },
                        { "id": "cal_b", "summary": "Work", "primary": false, "selected": false }
                    ],
                    "all_calendars_selected": false
                }),
            )
            .await
            .unwrap();
        storage
            .set_setting(
                "integration_google_calendar_secrets",
                &serde_json::json!({
                    "client_secret": "gcal-secret",
                    "refresh_token": "refresh-token",
                    "access_token": "access-token"
                }),
            )
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let patch_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/google-calendar")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"selected_calendar_ids":["cal_b"],"all_calendars_selected":false}"#
                            .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let patch_body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let patch_json: serde_json::Value = serde_json::from_slice(&patch_body).unwrap();
        let calendars = patch_json["data"]["google_calendar"]["calendars"]
            .as_array()
            .expect("calendars should be an array");
        assert_eq!(calendars[0]["selected"], false);
        assert_eq!(calendars[1]["selected"], true);
        assert_eq!(
            patch_json["data"]["google_calendar"]["all_calendars_selected"],
            false
        );
        assert_eq!(patch_json["data"]["google_calendar"]["connected"], true);

        let disconnect_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/integrations/google-calendar/disconnect")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(disconnect_resp.status(), StatusCode::OK);
        let disconnect_body = axum::body::to_bytes(disconnect_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let disconnect_json: serde_json::Value = serde_json::from_slice(&disconnect_body).unwrap();
        assert_eq!(
            disconnect_json["data"]["google_calendar"]["connected"],
            false
        );
        assert_eq!(
            disconnect_json["data"]["google_calendar"]["has_client_id"],
            true
        );
        assert_eq!(
            disconnect_json["data"]["google_calendar"]["has_client_secret"],
            true
        );
        let calendars = disconnect_json["data"]["google_calendar"]["calendars"]
            .as_array()
            .expect("calendars should be an array");
        assert_eq!(calendars[1]["selected"], true);
    }

    #[tokio::test]
    async fn integrations_get_includes_local_adapter_statuses() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let record_result =
            crate::services::integrations::record_sync_success(&storage, "notes", 2).await;
        assert!(record_result.is_ok());
        let config = AppConfig {
            activity_snapshot_path: Some("/tmp/activity.json".to_string()),
            git_snapshot_path: Some("/tmp/git.json".to_string()),
            messaging_snapshot_path: Some("/tmp/messaging.json".to_string()),
            notes_path: Some("/tmp/notes".to_string()),
            transcript_snapshot_path: Some("/tmp/transcripts.json".to_string()),
            ..Default::default()
        };
        let app = build_app(storage, config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/integrations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["data"]["activity"]["configured"], true);
        assert_eq!(
            json["data"]["activity"]["source_path"],
            "/tmp/activity.json"
        );
        assert_eq!(json["data"]["notes"]["configured"], true);
        assert_eq!(json["data"]["notes"]["source_path"], "/tmp/notes");
        assert_eq!(json["data"]["notes"]["last_sync_status"], "ok");
        assert_eq!(json["data"]["notes"]["last_item_count"], 2);
    }

    #[tokio::test]
    async fn sync_notes_updates_integrations_status_and_sync_messaging_records_error() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let dir = std::env::temp_dir().join(format!("vel_notes_{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("plan.md"), "# plan\n").unwrap();
        let config = AppConfig {
            notes_path: Some(dir.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let notes_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/notes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(notes_resp.status(), StatusCode::OK);

        let integrations_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/integrations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let integrations_body = axum::body::to_bytes(integrations_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let integrations_json: serde_json::Value =
            serde_json::from_slice(&integrations_body).unwrap();
        assert_eq!(integrations_json["data"]["notes"]["last_sync_status"], "ok");
        assert_eq!(integrations_json["data"]["notes"]["last_item_count"], 1);

        let failing_app = build_app(
            storage.clone(),
            AppConfig {
                notes_path: Some(dir.to_string_lossy().to_string()),
                messaging_snapshot_path: Some(
                    dir.join("missing.json").to_string_lossy().to_string(),
                ),
                ..Default::default()
            },
            test_policy_config(),
            None,
            None,
        );
        let messaging_resp = failing_app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/messaging")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(messaging_resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let integrations_resp = build_app(
            storage,
            AppConfig {
                notes_path: Some(dir.to_string_lossy().to_string()),
                messaging_snapshot_path: Some(
                    dir.join("missing.json").to_string_lossy().to_string(),
                ),
                ..Default::default()
            },
            test_policy_config(),
            None,
            None,
        )
        .oneshot(
            Request::builder()
                .uri("/api/integrations")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        let integrations_body = axum::body::to_bytes(integrations_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let integrations_json: serde_json::Value =
            serde_json::from_slice(&integrations_body).unwrap();
        assert_eq!(
            integrations_json["data"]["messaging"]["last_sync_status"],
            "error"
        );
        assert!(integrations_json["data"]["messaging"]["last_error"]
            .as_str()
            .is_some());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn integrations_logs_endpoint_lists_recent_sync_history() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        crate::services::integrations::record_sync_success(&storage, "notes", 2)
            .await
            .unwrap();
        crate::services::integrations::record_sync_error(
            &storage,
            "notes",
            "notes snapshot missing",
        )
        .await
        .unwrap();
        let app = build_app(
            storage,
            AppConfig {
                notes_path: Some("/tmp/notes".to_string()),
                ..Default::default()
            },
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/integrations/notes/logs?limit=5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let entries = json["data"].as_array().expect("integration logs array");
        assert_eq!(entries.len(), 2);
        assert!(entries
            .iter()
            .any(|entry| entry["integration_id"] == "notes"
                && entry["status"] == "error"
                && entry["message"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("notes snapshot missing")));
        assert!(entries
            .iter()
            .any(|entry| entry["status"] == "ok" && entry["payload"]["item_count"] == 2));
    }

    #[tokio::test]
    async fn list_components_returns_all_known_components() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/components")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = json["data"].as_array().expect("components array");
        assert_eq!(data.len(), 8);
        let ids: Vec<&str> = data
            .iter()
            .map(|entry| entry["id"].as_str().unwrap_or_default())
            .collect();
        assert!(ids.contains(&"google-calendar"));
        assert!(ids.contains(&"todoist"));
        assert!(ids.contains(&"activity"));
        assert!(ids.contains(&"git"));
        assert!(ids.contains(&"messaging"));
        assert!(ids.contains(&"notes"));
        assert!(ids.contains(&"transcripts"));
        assert!(ids.contains(&"evaluate"));
        assert_eq!(json["data"][0]["status"], "idle");
    }

    #[tokio::test]
    async fn restart_unknown_component_returns_404() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/components/does-not-exist/restart")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["ok"], false);
        assert_eq!(json["error"]["code"], "not_found");
    }

    #[tokio::test]
    async fn restart_evaluate_emits_status_and_logs() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let restart_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/components/evaluate/restart")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(restart_resp.status(), StatusCode::OK);
        let restart_body = axum::body::to_bytes(restart_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let restart_json: serde_json::Value = serde_json::from_slice(&restart_body).unwrap();
        assert_eq!(restart_json["ok"], true);
        assert_eq!(restart_json["data"]["id"], "evaluate");
        assert_eq!(restart_json["data"]["status"], "ok");
        assert_eq!(restart_json["data"]["restart_count"], 1);
        assert!(
            restart_json["data"]["last_restarted_at"]
                .as_i64()
                .unwrap_or(0)
                > 0
        );

        let logs_resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/components/evaluate/logs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(logs_resp.status(), StatusCode::OK);
        let logs_body = axum::body::to_bytes(logs_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let logs_json: serde_json::Value = serde_json::from_slice(&logs_body).unwrap();
        let logs = logs_json["data"].as_array().expect("logs array");
        assert!(!logs.is_empty());
        assert!(logs
            .iter()
            .any(|entry| entry["event_name"] == "component.restart.requested"));
        assert!(logs
            .iter()
            .any(|entry| entry["event_name"] == "component.restart.completed"));
    }

    #[tokio::test]
    async fn restart_evaluate_emits_components_updated_websocket_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        let mut rx = broadcast_tx.subscribe();
        let state = crate::state::AppState::new(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        );
        let app = build_app_with_state(state);

        let restart_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/components/evaluate/restart")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(restart_resp.status(), StatusCode::OK);
        let envelope = rx
            .recv()
            .await
            .expect("websocket event should be broadcast");
        assert_eq!(envelope.event_type.to_string(), "components:updated");
        assert_eq!(envelope.payload["id"], "evaluate");
        assert_eq!(envelope.payload["status"], "ok");
        assert_eq!(envelope.payload["restart_count"], 1);
    }

    #[tokio::test]
    async fn chat_intervention_snooze_404_for_nonexistent() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/interventions/intv_nonexistent/snooze")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"minutes":15}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn ws_endpoint_responds_to_get() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(Request::builder().uri("/ws").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert!(!resp.status().is_success());
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }
}
