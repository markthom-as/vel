use std::{collections::BTreeMap, fs, path::PathBuf};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde::de::DeserializeOwned;
use serde_json::json;
use time::OffsetDateTime;
use tokio::sync::broadcast;
use tower::util::ServiceExt;
use vel_api_types::ApiResponse;
use vel_config::AppConfig;
use vel_core::{
    CurrentContextV1, PersonId, ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRecord,
    ProjectRootRef, ProjectStatus,
};
use vel_storage::{CommitmentInsert, PersonRecord, Storage};
use veld::{app::build_app_with_state, policy_config::PolicyConfig, state::AppState};

const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";

fn unique_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "vel_agent_grounding_{}_{}",
        label,
        uuid::Uuid::new_v4().simple()
    ));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

fn current_context_json(context: CurrentContextV1) -> String {
    serde_json::to_string(&context).expect("context json")
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
    let now = OffsetDateTime::now_utc();
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

fn request(
    method: &str,
    uri: &str,
    token: Option<&str>,
    body: Option<serde_json::Value>,
) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(token) = token {
        builder = builder.header(OPERATOR_AUTH_HEADER, token);
    }
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    builder
        .body(match body {
            Some(body) => Body::from(body.to_string()),
            None => Body::empty(),
        })
        .expect("request")
}

async fn decode_json<T: DeserializeOwned>(response: axum::response::Response) -> T {
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

async fn seed_grounding_fixture(state: &AppState) -> ProjectRecord {
    let project_root = unique_dir("project");
    let project = test_project("proj_agent_grounding", "agent-grounding", &project_root);
    state.storage.create_project(project.clone()).await.unwrap();
    state
        .storage
        .create_person(PersonRecord {
            id: PersonId::from("per_agent_grounding".to_string()),
            display_name: "Annie Case".to_string(),
            given_name: Some("Annie".to_string()),
            family_name: Some("Case".to_string()),
            relationship_context: Some("Design review".to_string()),
            birthday: None,
            last_contacted_at: Some(OffsetDateTime::now_utc()),
            aliases: Vec::new(),
            links: Vec::new(),
        })
        .await
        .unwrap();
    state
        .storage
        .insert_commitment(CommitmentInsert {
            text: "Ship agent grounding".to_string(),
            source_type: "todoist".to_string(),
            source_id: "todo_agent_grounding".to_string(),
            status: vel_core::CommitmentStatus::Open,
            due_at: Some(OffsetDateTime::now_utc() + time::Duration::days(1)),
            project: Some(project.id.to_string()),
            commitment_kind: Some("must".to_string()),
            metadata_json: Some(json!({ "priority": 1 })),
        })
        .await
        .unwrap();
    state
        .storage
        .set_current_context(
            OffsetDateTime::now_utc().unix_timestamp(),
            &current_context_json(CurrentContextV1 {
                computed_at: OffsetDateTime::now_utc().unix_timestamp(),
                mode: "focused".to_string(),
                morning_state: "engaged".to_string(),
                meds_status: "done".to_string(),
                global_risk_level: "low".to_string(),
                attention_state: "on_task".to_string(),
                attention_reasons: vec!["Testing grounded inspect route".to_string()],
                ..CurrentContextV1::default()
            }),
        )
        .await
        .unwrap();
    state
        .storage
        .set_setting("writeback_enabled", &json!(false))
        .await
        .unwrap();
    project
}

#[tokio::test]
async fn agent_grounding_inspect_returns_typed_grounding_and_explicit_blockers() {
    unsafe {
        std::env::set_var("VEL_OPERATOR_API_TOKEN", "operator-secret");
    }
    let state = test_state().await;
    let project = seed_grounding_fixture(&state).await;
    let app = build_app_with_state(state.clone());

    let create_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/v1/execution/handoffs",
            Some("operator-secret"),
            Some(json!({
                "project_id": project.id.as_ref(),
                "from_agent": "operator",
                "to_agent": "codex-local",
                "origin_kind": "human_to_agent",
                "objective": "Implement the grounded inspect route",
                "task_kind": "implementation",
                "read_scopes": [project.primary_repo.path, project.primary_notes_root.path],
                "write_scopes": [project.primary_repo.path],
                "allowed_tools": ["rg", "cargo test"],
                "expected_output_schema": { "artifacts": ["patch"] }
            })),
        ))
        .await
        .expect("create handoff");
    assert_eq!(create_response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(request(
            "GET",
            "/v1/agent/inspect",
            Some("operator-secret"),
            None,
        ))
        .await
        .expect("inspect response");
    assert_eq!(response.status(), StatusCode::OK);
    let payload: ApiResponse<serde_json::Value> = decode_json(response).await;
    let data = payload.data.expect("inspect payload");

    assert_eq!(data["grounding"]["projects"].as_array().unwrap().len(), 1);
    assert_eq!(data["grounding"]["people"].as_array().unwrap().len(), 1);
    assert_eq!(
        data["grounding"]["commitments"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        data["grounding"]["review"]["pending_execution_handoffs"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        data["grounding"]["current_context"]["mode"].as_str(),
        Some("focused")
    );

    let mutation_entries = data["capabilities"]["groups"]
        .as_array()
        .unwrap()
        .iter()
        .find(|group| group["kind"] == "mutation_actions")
        .expect("mutation group")["entries"]
        .as_array()
        .unwrap()
        .clone();
    let writeback_entry = mutation_entries
        .iter()
        .find(|entry| entry["key"] == "integration_writeback")
        .expect("writeback entry");
    assert_eq!(writeback_entry["available"], false);
    assert_eq!(
        writeback_entry["blocked_reason"]["code"].as_str(),
        Some("safe_mode_enabled")
    );

    let repo_entry = mutation_entries
        .iter()
        .find(|entry| entry["key"] == "repo_local_write_scope")
        .expect("repo scope entry");
    assert_eq!(repo_entry["available"], false);
    assert_eq!(
        repo_entry["blocked_reason"]["code"].as_str(),
        Some("handoff_review_pending")
    );

    assert!(data["blockers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|blocker| blocker["code"] == "writeback_disabled"));
}

#[tokio::test]
async fn agent_grounding_inspect_requires_operator_auth_when_token_policy_is_configured() {
    unsafe {
        std::env::set_var("VEL_OPERATOR_API_TOKEN", "operator-secret");
    }
    let state = test_state().await;
    seed_grounding_fixture(&state).await;
    let app = build_app_with_state(state);

    let response = app
        .oneshot(request("GET", "/v1/agent/inspect", None, None))
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn agent_grounding_inspect_reports_no_matching_write_grant_when_none_exists() {
    unsafe {
        std::env::set_var("VEL_OPERATOR_API_TOKEN", "operator-secret");
    }
    let state = test_state().await;
    seed_grounding_fixture(&state).await;
    let app = build_app_with_state(state);

    let response = app
        .oneshot(request(
            "GET",
            "/v1/agent/inspect",
            Some("operator-secret"),
            None,
        ))
        .await
        .expect("inspect response");
    assert_eq!(response.status(), StatusCode::OK);
    let payload: ApiResponse<serde_json::Value> = decode_json(response).await;
    let data = payload.data.expect("inspect payload");

    let repo_entry = data["capabilities"]["groups"]
        .as_array()
        .unwrap()
        .iter()
        .find(|group| group["kind"] == "mutation_actions")
        .expect("mutation group")["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["key"] == "repo_local_write_scope")
        .expect("repo scope entry");
    assert_eq!(repo_entry["available"], false);
    assert_eq!(
        repo_entry["blocked_reason"]["code"].as_str(),
        Some("no_matching_write_grant")
    );
    assert!(data["blockers"]
        .as_array()
        .unwrap()
        .iter()
        .any(|blocker| blocker["code"] == "no_matching_write_grant"));
}

#[tokio::test]
async fn execution_context_preview_and_export_include_agent_grounding_artifacts() {
    unsafe {
        std::env::remove_var("VEL_OPERATOR_API_TOKEN");
    }
    let state = test_state().await;
    let project = seed_grounding_fixture(&state).await;
    veld::services::execution_context::save_execution_context(
        &state,
        project.id.as_ref(),
        veld::services::execution_context::ExecutionContextInput {
            objective: "Ship Phase 11 backend inspect route".to_string(),
            repo_brief: "Runtime route and service work".to_string(),
            notes_brief: "Keep the export bounded.".to_string(),
            constraints: vec!["stay inside declared repo".to_string()],
            expected_outputs: vec![],
        },
    )
    .await
    .unwrap();

    let preview =
        veld::services::execution_context::preview_gsd_artifacts(&state, project.id.as_ref(), None)
            .await
            .unwrap();
    assert!(preview
        .files
        .iter()
        .any(|file| file.relative_path.ends_with("agent-grounding.md")));
    assert!(preview
        .files
        .iter()
        .any(|file| file.relative_path.ends_with("agent-inspect.json")));

    let exported =
        veld::services::execution_context::export_gsd_artifacts(&state, project.id.as_ref(), None)
            .await
            .unwrap();
    let inspect_path = exported
        .written_paths
        .iter()
        .find(|path| path.ends_with("agent-inspect.json"))
        .expect("inspect path");
    let inspect_json = fs::read_to_string(inspect_path).expect("inspect json");
    let inspect: serde_json::Value = serde_json::from_str(&inspect_json).expect("inspect value");
    assert_eq!(
        inspect["grounding"]["projects"][0]["id"].as_str(),
        Some(project.id.as_ref())
    );
    let markdown_path = exported
        .written_paths
        .iter()
        .find(|path| path.ends_with("agent-grounding.md"))
        .expect("grounding markdown");
    let markdown = fs::read_to_string(markdown_path).expect("grounding markdown");
    assert!(markdown.contains("# Agent Grounding"));
}
