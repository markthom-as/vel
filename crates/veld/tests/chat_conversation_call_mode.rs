use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tokio::sync::broadcast;
use tower::ServiceExt;
use vel_config::AppConfig;

fn test_state(storage: vel_storage::Storage) -> veld::state::AppState {
    let (broadcast_tx, _) = broadcast::channel(8);
    veld::state::AppState::new(
        storage,
        AppConfig::default(),
        veld::policy_config::PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    )
}

#[tokio::test]
async fn conversation_patch_persists_call_mode_state() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = veld::app::build_app_with_state(test_state(storage.clone()));

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/conversations")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"title":"Call thread","kind":"general"}"#.to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let create_json: serde_json::Value = serde_json::from_slice(&create_body).unwrap();
    let conversation_id = create_json["data"]["id"].as_str().unwrap();
    assert_eq!(create_json["data"]["call_mode_active"], false);

    let patch_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::PATCH)
                .uri(format!("/api/conversations/{conversation_id}"))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"call_mode_active":true}"#.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(patch_response.status(), StatusCode::OK);
    let patch_body = axum::body::to_bytes(patch_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let patch_json: serde_json::Value = serde_json::from_slice(&patch_body).unwrap();
    assert_eq!(patch_json["data"]["call_mode_active"], true);

    let stored = storage
        .get_conversation(conversation_id)
        .await
        .unwrap()
        .expect("stored conversation");
    assert!(stored.call_mode_active);
}
