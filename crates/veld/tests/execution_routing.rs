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
        "vel_execution_routing_{}_{}",
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
    let config = AppConfig {
        artifact_root: unique_dir("artifacts").to_string_lossy().to_string(),
        node_id: Some("vel-authority".to_string()),
        node_display_name: Some("Vel Authority".to_string()),
        ..Default::default()
    };
    AppState::new(
        storage,
        config,
        PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    )
}

fn test_project(id: &str, slug: &str, repo_root: &PathBuf) -> ProjectRecord {
    let now = time::OffsetDateTime::now_utc();
    let repo = repo_root.join("repo");
    let notes = repo_root.join("notes");
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

async fn decode_json<T: serde::de::DeserializeOwned>(response: axum::response::Response) -> T {
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body");
    serde_json::from_slice::<T>(&body).unwrap_or_else(|error| {
        panic!(
            "expected JSON body for status {}: {} ({})",
            status,
            String::from_utf8_lossy(&body),
            error
        )
    })
}

async fn create_pending_handoff(
    app: &axum::Router,
    project: &ProjectRecord,
    objective: &str,
) -> String {
    let response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/execution/handoffs",
            json!({
                "project_id": project.id.as_ref(),
                "from_agent": "operator",
                "to_agent": "codex-local",
                "origin_kind": "human_to_agent",
                "objective": objective,
                "task_kind": "implementation",
                "read_scopes": [project.primary_repo.path, project.primary_notes_root.path],
                "write_scopes": [project.primary_repo.path],
                "allowed_tools": ["rg", "cargo test"],
                "expected_output_schema": { "artifacts": ["patch"] }
            }),
        ))
        .await
        .expect("create handoff");
    assert_eq!(response.status(), StatusCode::OK);
    let payload: serde_json::Value = decode_json(response).await;
    payload["data"]["id"]
        .as_str()
        .expect("handoff id")
        .to_string()
}

#[tokio::test]
async fn execution_routing_rejects_handoff_without_scopes_or_expected_output() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());
    let project_root = unique_dir("invalid");
    let project = test_project("proj_exec_invalid", "exec-invalid", &project_root);
    state.storage.create_project(project.clone()).await.unwrap();

    let response = app
        .oneshot(request(
            "POST",
            "/v1/execution/handoffs",
            json!({
                "project_id": project.id.as_ref(),
                "from_agent": "operator",
                "to_agent": "codex-local",
                "origin_kind": "human_to_agent",
                "objective": "Prepare a supervised coding handoff",
                "read_scopes": [],
                "write_scopes": [],
                "allowed_tools": ["rg"],
                "expected_output_schema": {}
            }),
        ))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let payload: serde_json::Value = decode_json(response).await;
    assert_eq!(payload["ok"], false);
    assert!(
        payload["error"]["message"]
            .as_str()
            .unwrap_or_default()
            .contains("expected output")
            || payload["error"]["message"]
                .as_str()
                .unwrap_or_default()
                .contains("scope"),
        "unexpected error payload: {payload:?}"
    );
}

#[tokio::test]
async fn execution_routing_persists_typed_reasons_and_launch_preview() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());
    let project_root = unique_dir("persisted");
    let project = test_project("proj_exec_routing", "exec-routing", &project_root);
    state.storage.create_project(project.clone()).await.unwrap();

    let create_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/execution/handoffs",
            json!({
                "project_id": project.id.as_ref(),
                "from_agent": "operator",
                "to_agent": "codex-local",
                "origin_kind": "human_to_agent",
                "objective": "Implement Phase 08 review routing",
                "task_kind": "implementation",
                "read_scopes": [project.primary_repo.path, project.primary_notes_root.path],
                "write_scopes": [project.primary_repo.path],
                "allowed_tools": ["rg", "cargo test"],
                "constraints": ["do not widen permissions"],
                "expected_output_schema": {
                  "artifacts": ["patch", "summary"]
                }
            }),
        ))
        .await
        .expect("create response");
    assert_eq!(create_response.status(), StatusCode::OK);
    let created: serde_json::Value = decode_json(create_response).await;
    let handoff_id = created["data"]["id"]
        .as_str()
        .expect("handoff id")
        .to_string();
    assert_eq!(created["data"]["review_state"], "pending_review");
    assert_eq!(created["data"]["handoff"]["task_kind"], "implementation");
    assert_eq!(created["data"]["routing"]["task_kind"], "implementation");
    assert_eq!(
        created["data"]["routing"]["review_gate"],
        "operator_approval"
    );
    assert!(created["data"]["routing"]["reasons"]
        .as_array()
        .expect("reasons array")
        .iter()
        .any(|reason| reason["code"] == "write_scope_requires_approval"));

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/execution/handoffs?state=pending_review")
                .body(Body::empty())
                .expect("list request"),
        )
        .await
        .expect("list response");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_payload: serde_json::Value = decode_json(list_response).await;
    assert_eq!(
        list_payload["data"].as_array().expect("handoff list").len(),
        1
    );

    let preview_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/v1/execution/handoffs/{handoff_id}/launch-preview"
                ))
                .body(Body::empty())
                .expect("preview request"),
        )
        .await
        .expect("preview response");
    assert_eq!(preview_response.status(), StatusCode::OK);
    let preview_payload: serde_json::Value = decode_json(preview_response).await;
    assert_eq!(preview_payload["data"]["handoff_id"], handoff_id);
    assert_eq!(preview_payload["data"]["launch_ready"], false);
    assert!(preview_payload["data"]["blockers"]
        .as_array()
        .expect("preview blockers")
        .iter()
        .any(|blocker| blocker == "handoff review is still pending"));
}

#[tokio::test]
async fn execution_routing_approve_and_reject_are_explicit_and_visible_in_now() {
    let state = test_state().await;
    let app = build_app_with_state(state.clone());
    let project_root = unique_dir("review");
    let project = test_project("proj_exec_review", "exec-review", &project_root);
    state.storage.create_project(project.clone()).await.unwrap();

    let approved_id =
        create_pending_handoff(&app, &project, "Approve this execution handoff").await;
    let rejected_id = create_pending_handoff(&app, &project, "Reject this execution handoff").await;

    let now_before = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/now")
                .body(Body::empty())
                .expect("now request"),
        )
        .await
        .expect("now response");
    assert_eq!(now_before.status(), StatusCode::OK);
    let now_before_payload: serde_json::Value = decode_json(now_before).await;
    assert!(now_before_payload["data"]["action_items"]
        .as_array()
        .expect("action items")
        .iter()
        .any(|item| item["title"]
            .as_str()
            .unwrap_or_default()
            .contains("execution handoff")));

    let approve_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/execution/handoffs/{approved_id}/approve"),
            json!({
                "reviewed_by": "operator",
                "decision_reason": "Objective and scopes are explicit."
            }),
        ))
        .await
        .expect("approve response");
    assert_eq!(approve_response.status(), StatusCode::OK);
    let approve_payload: serde_json::Value = decode_json(approve_response).await;
    assert_eq!(approve_payload["data"]["review_state"], "approved");

    let reject_response = app
        .clone()
        .oneshot(request(
            "POST",
            &format!("/v1/execution/handoffs/{rejected_id}/reject"),
            json!({
                "reviewed_by": "operator",
                "decision_reason": "Writable scope is too broad."
            }),
        ))
        .await
        .expect("reject response");
    assert_eq!(reject_response.status(), StatusCode::OK);
    let reject_payload: serde_json::Value = decode_json(reject_response).await;
    assert_eq!(reject_payload["data"]["review_state"], "rejected");

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/execution/handoffs")
                .body(Body::empty())
                .expect("list request"),
        )
        .await
        .expect("list response");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_payload: serde_json::Value = decode_json(list_response).await;
    let states = list_payload["data"]
        .as_array()
        .expect("handoff list")
        .iter()
        .map(|item| {
            (
                item["id"].as_str().unwrap_or_default().to_string(),
                item["review_state"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            )
        })
        .collect::<std::collections::HashMap<_, _>>();
    assert_eq!(
        states.get(&approved_id).map(String::as_str),
        Some("approved")
    );
    assert_eq!(
        states.get(&rejected_id).map(String::as_str),
        Some("rejected")
    );

    let now_after = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/now")
                .body(Body::empty())
                .expect("now request"),
        )
        .await
        .expect("now response");
    assert_eq!(now_after.status(), StatusCode::OK);
    let now_after_payload: serde_json::Value = decode_json(now_after).await;
    assert!(!now_after_payload["data"]["action_items"]
        .as_array()
        .expect("action items")
        .iter()
        .any(|item| {
            let title = item["title"].as_str().unwrap_or_default();
            title.contains("Approve this execution handoff")
                || title.contains("Reject this execution handoff")
        }));
}
