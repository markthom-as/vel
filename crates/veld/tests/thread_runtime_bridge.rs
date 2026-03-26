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
        "vel_thread_runtime_bridge_{}_{}",
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

fn json_request(method: &str, uri: &str, body: serde_json::Value) -> Request<Body> {
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
    serde_json::from_slice::<serde_json::Value>(&body).unwrap_or_else(|error| {
        panic!(
            "expected JSON body for status {}: {} ({})",
            status,
            String::from_utf8_lossy(&body),
            error
        )
    })
}

#[tokio::test]
async fn launched_handoff_attaches_connect_run_to_origin_thread() {
    let state = test_state().await;
    let repo_root = unique_dir("project");
    let project = test_project("proj_thread_bridge", "thread-bridge", &repo_root);
    state
        .storage
        .create_project(project.clone())
        .await
        .expect("project");
    let app = build_app_with_state(state);

    let handoff_response = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/v1/execution/handoffs",
            json!({
                "project_id": project.id.as_ref(),
                "from_agent": "operator",
                "to_agent": "codex-local",
                "origin_kind": "human_to_agent",
                "objective": "Reply through the linked runtime thread",
                "task_kind": "implementation",
                "read_scopes": [project.primary_repo.path, project.primary_notes_root.path],
                "write_scopes": [project.primary_repo.path],
                "allowed_tools": ["rg"],
                "expected_output_schema": { "artifacts": ["patch"] }
            }),
        ))
        .await
        .expect("handoff response");
    assert_eq!(handoff_response.status(), StatusCode::OK);
    let handoff_json = decode_json(handoff_response).await;
    let handoff_id = handoff_json["data"]["id"].as_str().expect("handoff id");

    let thread_response = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/v1/threads",
            json!({
                "thread_type": "assistant_proposal",
                "title": "Agent bridge thread",
                "metadata_json": {
                    "proposal_state": "approved"
                }
            }),
        ))
        .await
        .expect("thread response");
    assert_eq!(thread_response.status(), StatusCode::OK);
    let thread_json = decode_json(thread_response).await;
    let thread_id = thread_json["data"]["id"].as_str().expect("thread id");

    let add_link_response = app
        .clone()
        .oneshot(json_request(
            "POST",
            &format!("/v1/threads/{thread_id}/links"),
            json!({
                "entity_type": "execution_handoff",
                "entity_id": handoff_id,
                "relation_type": "approves"
            }),
        ))
        .await
        .expect("thread link response");
    assert_eq!(add_link_response.status(), StatusCode::OK);

    let approve_response = app
        .clone()
        .oneshot(json_request(
            "POST",
            &format!("/v1/execution/handoffs/{handoff_id}/approve"),
            json!({
                "reviewed_by": "operator",
                "decision_reason": "thread bridge test"
            }),
        ))
        .await
        .expect("approve response");
    assert_eq!(approve_response.status(), StatusCode::OK);

    let launch_response = app
        .clone()
        .oneshot(json_request(
            "POST",
            &format!("/v1/execution/handoffs/{handoff_id}/launch"),
            json!({
                "runtime_kind": "local_command",
                "command": ["/bin/sh", "-lc", "printf 'bridge-ready\\n'; sleep 5"],
                "working_dir": project.primary_repo.path,
                "writable_roots": [project.primary_repo.path]
            }),
        ))
        .await
        .expect("launch response");
    assert_eq!(launch_response.status(), StatusCode::OK);
    let launch_json = decode_json(launch_response).await;
    let run_id = launch_json["data"]["id"].as_str().expect("run id");

    let inspect_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/threads/{thread_id}"))
                .body(Body::empty())
                .expect("inspect request"),
        )
        .await
        .expect("inspect response");
    assert_eq!(inspect_response.status(), StatusCode::OK);
    let inspect_json = decode_json(inspect_response).await;
    assert_eq!(
        inspect_json["data"]["metadata"]["active_connect_run_id"],
        run_id
    );
    assert_eq!(
        inspect_json["data"]["metadata"]["connect_attach_path"],
        format!("/v1/connect/instances/{run_id}/attach")
    );
    assert!(inspect_json["data"]["links"]
        .as_array()
        .expect("thread links")
        .iter()
        .any(|link| {
            link["entity_type"] == "connect_run"
                && link["entity_id"] == run_id
                && link["relation_type"] == "attached"
        }));

    let terminate_response = app
        .clone()
        .oneshot(json_request(
            "POST",
            &format!("/v1/connect/instances/{run_id}/terminate"),
            json!({ "reason": "test cleanup" }),
        ))
        .await
        .expect("terminate response");
    assert_eq!(terminate_response.status(), StatusCode::OK);
}
