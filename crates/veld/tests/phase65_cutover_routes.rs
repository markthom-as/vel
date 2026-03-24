use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;
use vel_config::AppConfig;
use vel_storage::Storage;
use veld::{app::build_app, policy_config::PolicyConfig};

const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";

fn request(method: &str, uri: &str, body: Body) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(OPERATOR_AUTH_HEADER, "operator-secret")
        .header("content-type", "application/json")
        .body(body)
        .unwrap()
}

#[tokio::test]
async fn legacy_provider_write_routes_are_quarantined_during_cutover() {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = build_app(
        storage,
        AppConfig::default(),
        PolicyConfig::default(),
        None,
        None,
    );

    let todoist = app
        .clone()
        .oneshot(request(
            "POST",
            "/api/integrations/todoist/create-task",
            Body::from(
                r#"{"content":"Morning review","project_id":"proj_1","priority":1}"#.to_string(),
            ),
        ))
        .await
        .unwrap();
    let notes = app
        .oneshot(request(
            "POST",
            "/api/integrations/notes/create-note",
            Body::from(r#"{"path":"daily.md","content":"hello"}"#.to_string()),
        ))
        .await
        .unwrap();

    assert_eq!(todoist.status(), StatusCode::GONE);
    assert_eq!(notes.status(), StatusCode::GONE);

    let todoist_json: serde_json::Value =
        serde_json::from_slice(&to_bytes(todoist.into_body(), usize::MAX).await.unwrap()).unwrap();
    let notes_json: serde_json::Value =
        serde_json::from_slice(&to_bytes(notes.into_body(), usize::MAX).await.unwrap()).unwrap();

    assert_eq!(todoist_json["error"]["code"], "deprecated");
    assert!(todoist_json["error"]["message"]
        .as_str()
        .unwrap()
        .contains("/api/integrations/todoist/write-intent"));
    assert!(notes_json["error"]["message"]
        .as_str()
        .unwrap()
        .contains("outside the 0.5 proving-adapter scope"));
}

#[tokio::test]
async fn read_side_integration_surface_remains_available_while_legacy_writes_are_retired() {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = build_app(
        storage,
        AppConfig::default(),
        PolicyConfig::default(),
        None,
        None,
    );

    let response = app
        .oneshot(request("GET", "/api/integrations", Body::empty()))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
