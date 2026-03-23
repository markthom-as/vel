use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;
use vel_config::AppConfig;
use vel_storage::{Storage, list_runtime_records};
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
async fn commitment_patch_route_records_write_intent_runtime_history() {
    let storage = Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let app = build_app(
        storage.clone(),
        AppConfig::default(),
        PolicyConfig::default(),
        None,
        None,
    );

    let create = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/commitments",
            Body::from(
                r#"{"text":"Finish phase 69 proof","source_type":"manual","source_id":"local"}"#
                    .to_string(),
            ),
        ))
        .await
        .unwrap();

    assert_eq!(create.status(), StatusCode::OK);
    let create_json: serde_json::Value =
        serde_json::from_slice(&to_bytes(create.into_body(), usize::MAX).await.unwrap()).unwrap();
    let commitment_id = create_json["data"]["id"].as_str().unwrap().to_string();

    let patch = app
        .oneshot(request(
            "PATCH",
            &format!("/v1/commitments/{commitment_id}"),
            Body::from(r#"{"status":"done"}"#.to_string()),
        ))
        .await
        .unwrap();

    assert_eq!(patch.status(), StatusCode::OK);

    let write_intents = list_runtime_records(storage.sql_pool(), "write_intent")
        .await
        .unwrap();
    assert_eq!(write_intents.len(), 3);
    assert!(write_intents.iter().any(|record| record.status == "approved"));
    assert!(write_intents.iter().any(|record| record.status == "executing"));
    assert!(write_intents.iter().any(|record| record.status == "succeeded"));
}
