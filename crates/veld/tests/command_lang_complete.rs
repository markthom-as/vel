use std::sync::Arc;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tokio::sync::broadcast;
use tower::ServiceExt;
use vel_config::AppConfig;

const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";

fn test_state(storage: vel_storage::Storage) -> veld::state::AppState {
    let (broadcast_tx, _) = broadcast::channel(8);
    veld::state::AppState::new(
        storage,
        AppConfig::default(),
        veld::policy_config::PolicyConfig::default(),
        broadcast_tx,
        None::<Arc<vel_llm::Router>>,
        None,
    )
}

#[tokio::test]
async fn command_complete_returns_partial_hints_without_parse_success() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = veld::app::build_app_with_state(test_state(storage));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/command/complete")
                .header("content-type", "application/json")
                .header(OPERATOR_AUTH_HEADER, "operator-secret")
                .body(Body::from(r#"{"input":["should","plan"]}"#.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json["data"]["completion_hints"],
        serde_json::json!(["<goal>", "for", "with"])
    );
    assert_eq!(json["data"]["parsed"], serde_json::Value::Null);
    assert!(json["data"]["parse_error"]
        .as_str()
        .expect("parse error")
        .contains("require a verb and a target"));
}

#[tokio::test]
async fn command_complete_returns_resolution_for_valid_input() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = veld::app::build_app_with_state(test_state(storage));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/command/complete")
                .header("content-type", "application/json")
                .header(OPERATOR_AUTH_HEADER, "operator-secret")
                .body(Body::from(
                    r#"{"input":["should","delegate","queue","cleanup"]}"#.to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["data"]["parsed"]["family"], "should");
    assert_eq!(json["data"]["parsed"]["verb"], "delegate");
    assert_eq!(
        json["data"]["resolved_command"]["targets"][0]["kind"],
        "delegation_plan"
    );
    assert_eq!(
        json["data"]["intent_hints"]["suggestions"],
        serde_json::json!(["worker split", "ownership", "review gate"])
    );
    assert!(json["data"]["local_preview"]
        .as_str()
        .expect("preview")
        .contains("kind=delegation_plan"));
}

#[tokio::test]
async fn command_complete_returns_vel_root_hints() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = veld::app::build_app_with_state(test_state(storage));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/command/complete")
                .header("content-type", "application/json")
                .header(OPERATOR_AUTH_HEADER, "operator-secret")
                .body(Body::from(r#"{"input":[]}"#.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let hints = json["data"]["completion_hints"]
        .as_array()
        .expect("completion hints");
    for expected in ["morning", "standup", "checkin", "should", "project"] {
        assert!(
            hints.iter().any(|value| value == expected),
            "missing completion hint: {expected}"
        );
    }
    assert_eq!(json["data"]["local_preview"], "Vel command mode");
}

#[tokio::test]
async fn command_complete_returns_daily_loop_metadata_for_vel_morning() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = veld::app::build_app_with_state(test_state(storage));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/command/complete")
                .header("content-type", "application/json")
                .header(OPERATOR_AUTH_HEADER, "operator-secret")
                .body(Body::from(r#"{"input":["morning"]}"#.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["data"]["parsed"]["family"], "vel");
    assert_eq!(json["data"]["parsed"]["verb"], "morning");
    assert_eq!(
        json["data"]["intent_hints"]["target_kind"],
        "daily_loop_session"
    );
    assert_eq!(json["data"]["intent_hints"]["mode"], "execute");
}

#[tokio::test]
async fn command_complete_accepts_raw_text_for_shell_aliases() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = veld::app::build_app_with_state(test_state(storage));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/command/complete")
                .header("content-type", "application/json")
                .header(OPERATOR_AUTH_HEADER, "operator-secret")
                .body(Body::from(r#"{"text":"slash check in"}"#.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["data"]["input"], serde_json::json!(["checkin"]));
    assert_eq!(json["data"]["parsed"]["verb"], "checkin");
    assert_eq!(json["data"]["parsed"]["source_text"], "vel checkin");
}
