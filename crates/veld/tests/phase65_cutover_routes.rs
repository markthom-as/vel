use axum::{
    body::Body,
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
async fn legacy_provider_write_routes_are_removed_during_cutover() {
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

    assert_eq!(todoist.status(), StatusCode::NOT_FOUND);
    assert_eq!(notes.status(), StatusCode::NOT_FOUND);
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
