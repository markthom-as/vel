use axum::{routing::{get, post}, Router};
use tower_http::trace::TraceLayer;
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::{routes, state::AppState};

pub fn build_app(storage: Storage, config: AppConfig) -> Router {
    let state = AppState::new(storage, config);

    Router::new()
        .route("/v1/health", get(routes::health::health))
        .route("/v1/doctor", get(routes::doctor::doctor))
        .route("/v1/captures", post(routes::captures::create_capture))
        .route("/v1/captures/:id", get(routes::captures::get_capture))
        .route("/v1/artifacts", post(routes::artifacts::create_artifact))
        .route("/v1/artifacts/:id", get(routes::artifacts::get_artifact))
        .route("/v1/runs", get(routes::runs::list_runs))
        .route("/v1/runs/:id", get(routes::runs::get_run))
        .route("/v1/context/today", get(routes::context::today))
        .route("/v1/context/morning", get(routes::context::morning))
        .route("/v1/context/end-of-day", get(routes::context::end_of_day))
        .route("/v1/search", get(routes::search::search))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage, AppConfig::default());

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
        let app = build_app(storage, AppConfig::default());

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
        let app = build_app(storage, AppConfig::default());

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
        let app = build_app(storage, AppConfig::default());

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

    #[tokio::test]
    async fn end_of_day_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(storage, AppConfig::default());

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
        let app = build_app(storage, AppConfig::default());

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
}
