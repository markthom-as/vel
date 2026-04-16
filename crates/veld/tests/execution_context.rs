use std::{collections::BTreeMap, fs, path::PathBuf};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_config::AppConfig;
use vel_core::{
    ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRecord, ProjectRootRef, ProjectStatus,
};
use vel_storage::Storage;
use veld::{app::build_app_with_state, policy_config::PolicyConfig, state::AppState};

fn unique_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "vel_execution_context_{}_{}",
        label,
        uuid::Uuid::new_v4().simple()
    ));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

async fn test_state() -> AppState {
    let storage = Storage::connect(":memory:").await.expect("storage");
    storage.migrate().await.expect("migrations");
    let (broadcast_tx, _) = broadcast::channel(16);
    AppState::new(
        storage,
        AppConfig::default(),
        PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    )
}

fn test_project(id: &str, slug: &str, root: &PathBuf) -> ProjectRecord {
    let now = time::OffsetDateTime::now_utc();
    let repo = root.join("repo");
    let notes = root.join("notes");
    fs::create_dir_all(&repo).expect("repo root");
    fs::create_dir_all(&notes).expect("notes root");

    ProjectRecord {
        id: ProjectId::from(id.to_string()),
        slug: slug.to_string(),
        name: format!("Project {}", slug),
        family: ProjectFamily::Work,
        status: ProjectStatus::Active,
        primary_repo: ProjectRootRef {
            path: repo.to_string_lossy().to_string(),
            label: slug.to_string(),
            kind: "repo".to_string(),
        },
        primary_notes_root: ProjectRootRef {
            path: notes.to_string_lossy().to_string(),
            label: format!("{slug}-notes"),
            kind: "notes_root".to_string(),
        },
        secondary_repos: Vec::new(),
        secondary_notes_roots: Vec::new(),
        upstream_ids: BTreeMap::new(),
        pending_provision: ProjectProvisionRequest::default(),
        created_at: now,
        updated_at: now,
        archived_at: None,
    }
}

fn request(method: &str, uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request")
}

async fn decode_json(response: axum::response::Response) -> serde_json::Value {
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    serde_json::from_slice(&body).unwrap_or_else(|error| {
        panic!(
            "expected JSON body for status {}: {} ({})",
            status,
            String::from_utf8_lossy(&body),
            error
        )
    })
}

#[tokio::test]
async fn execution_context_route_maps_request_validation_to_bad_request() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());
    let root = unique_dir("validation");
    let project = test_project(
        "proj_exec_context_validation",
        "exec-context-validation",
        &root,
    );
    state.storage.create_project(project.clone()).await.unwrap();

    let response = app
        .oneshot(request(
            "POST",
            &format!("/v1/execution/projects/{}/context", project.id.as_ref()),
            json!({
                "objective": "   ",
                "repo_brief": "keep this rejected at the request boundary"
            }),
        ))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let payload = decode_json(response).await;
    assert_eq!(payload["ok"], false);
    assert_eq!(payload["error"]["code"], "validation_error");
}

#[tokio::test]
async fn execution_context_route_maps_absent_context_to_not_found() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());
    let root = unique_dir("missing");
    let project = test_project("proj_exec_context_missing", "exec-context-missing", &root);
    state.storage.create_project(project.clone()).await.unwrap();

    let response = app
        .oneshot(request(
            "GET",
            &format!("/v1/execution/projects/{}/context", project.id.as_ref()),
            json!({}),
        ))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let payload = decode_json(response).await;
    assert_eq!(payload["ok"], false);
    assert_eq!(payload["error"]["code"], "not_found");
}

#[tokio::test]
async fn execution_context_route_maps_stored_shape_failure_to_internal_error() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());
    let root = unique_dir("corrupted");
    let project = test_project(
        "proj_exec_context_corrupted",
        "exec-context-corrupted",
        &root,
    );
    state.storage.create_project(project.clone()).await.unwrap();
    state
        .storage
        .upsert_project_execution_context(
            &project.id,
            &json!({
                "repo_brief": "missing objective is persisted corruption, not request validation"
            }),
            time::OffsetDateTime::now_utc(),
        )
        .await
        .unwrap();

    let response = app
        .oneshot(request(
            "GET",
            &format!("/v1/execution/projects/{}/context", project.id.as_ref()),
            json!({}),
        ))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let payload = decode_json(response).await;
    assert_eq!(payload["ok"], false);
    assert_eq!(payload["error"]["code"], "internal_error");
}
