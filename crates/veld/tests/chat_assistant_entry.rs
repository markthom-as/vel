use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use time::macros::datetime;
use tokio::sync::{broadcast, Mutex};
use tower::ServiceExt;
use vel_config::AppConfig;
use vel_core::{
    DailyLoopPhase, DailyLoopStartMetadata, DailyLoopStartRequest, DailyLoopStartSource,
    DailyLoopSurface, ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRecord,
    ProjectRootRef, ProjectStatus,
};
use vel_llm::{
    FinishReason, LlmError, LlmProvider, LlmRequest, LlmResponse, ModelInfo, ProviderHealth,
    ProviderRegistry, Router, Usage,
};

struct MockEntryProvider {
    requests: Arc<Mutex<Vec<LlmRequest>>>,
}

impl MockEntryProvider {
    fn new() -> (Self, Arc<Mutex<Vec<LlmRequest>>>) {
        let requests = Arc::new(Mutex::new(Vec::new()));
        (
            Self {
                requests: requests.clone(),
            },
            requests,
        )
    }
}

#[async_trait]
impl LlmProvider for MockEntryProvider {
    async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, LlmError> {
        let mut requests = self.requests.lock().await;
        requests.push(req.clone());
        Ok(LlmResponse {
            text: Some("Here is the grounded follow-up.".to_string()),
            tool_calls: vec![],
            usage: Usage::default(),
            finish_reason: FinishReason::Stop,
            raw: serde_json::json!({}),
        })
    }

    async fn health(&self) -> Result<ProviderHealth, LlmError> {
        Ok(ProviderHealth {
            healthy: true,
            details: serde_json::json!({}),
        })
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        Ok(vec![ModelInfo {
            id: "mock-entry".to_string(),
            context_window: Some(32768),
            supports_tools: true,
            supports_json: true,
        }])
    }
}

fn test_state(
    storage: vel_storage::Storage,
    llm_router: Option<Arc<Router>>,
    chat_profile_id: Option<String>,
) -> veld::state::AppState {
    let (broadcast_tx, _) = broadcast::channel(8);
    veld::state::AppState::new(
        storage,
        AppConfig::default(),
        veld::policy_config::PolicyConfig::default(),
        broadcast_tx,
        llm_router,
        chat_profile_id,
    )
}

fn unique_dir(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "vel_chat_assistant_entry_{}_{}",
        label,
        uuid::Uuid::new_v4().simple()
    ));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
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
        name: format!("Project {slug}"),
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

#[tokio::test]
async fn capture_like_assistant_entry_routes_to_inbox_and_surfaces_no_assistant_reply() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"remember to send the project update"}"#.to_string(),
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

    assert_eq!(json["data"]["route_target"], "inbox");
    assert!(json["data"]["assistant_message"].is_null());
    assert!(json["data"]["assistant_error"].is_null());
    assert_eq!(
        json["data"]["user_message"]["content"]["text"],
        "remember to send the project update"
    );
    assert_eq!(
        storage.list_interventions_active(10).await.unwrap().len(),
        1,
        "capture-like entry should surface in Inbox"
    );
    let conversation_id = json["data"]["conversation"]["id"]
        .as_str()
        .expect("conversation id");
    let conversation = storage
        .get_conversation(conversation_id)
        .await
        .unwrap()
        .expect("stored conversation");
    assert!(
        conversation.archived,
        "capture entry conversations stay out of default thread lists"
    );
}

#[tokio::test]
async fn question_assistant_entry_routes_to_threads_and_returns_grounded_reply() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();

    let (provider, requests) = MockEntryProvider::new();
    let mut registry = ProviderRegistry::new();
    registry.register("mock-entry", Arc::new(provider));
    let router = Arc::new(Router::new(registry));
    let app = veld::app::build_app_with_state(test_state(
        storage,
        Some(router),
        Some("mock-entry".to_string()),
    ));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"What should I focus on next?"}"#.to_string(),
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

    assert_eq!(json["data"]["route_target"], "threads");
    assert_eq!(
        json["data"]["assistant_message"]["content"]["text"],
        "Here is the grounded follow-up."
    );
    assert!(json["data"]["assistant_error"].is_null());

    let requests = requests.lock().await;
    assert_eq!(requests.len(), 1);
}

#[tokio::test]
async fn question_assistant_entry_can_stage_a_bounded_action_proposal() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"remember to send the project update"}"#.to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"What should I do next?"}"#.to_string(),
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

    assert_eq!(json["data"]["route_target"], "threads");
    assert_eq!(json["data"]["proposal"]["state"], "staged");
    assert_eq!(json["data"]["proposal"]["kind"], "intervention");
    assert_eq!(json["data"]["proposal"]["permission_mode"], "user_confirm");
    assert_eq!(json["data"]["proposal"]["title"], "Inbox intervention");
    assert_eq!(
        json["data"]["proposal"]["thread_route"]["target"],
        "existing_thread"
    );
    assert_eq!(
        json["data"]["proposal"]["thread_route"]["thread_type"],
        "assistant_proposal"
    );
    let thread_id = json["data"]["proposal"]["thread_route"]["thread_id"]
        .as_str()
        .expect("thread id");
    let thread_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/v1/threads/{thread_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(thread_response.status(), StatusCode::OK);
    let thread_body = axum::body::to_bytes(thread_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let thread_json: serde_json::Value = serde_json::from_slice(&thread_body).unwrap();
    assert_eq!(
        thread_json["data"]["metadata"]["source"],
        "assistant_proposal"
    );
    assert_eq!(thread_json["data"]["metadata"]["proposal_state"], "staged");
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["kind"],
        "action_confirmation"
    );
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["action_item_id"],
        json["data"]["proposal"]["action_item_id"]
    );
    assert_eq!(
        storage.list_interventions_active(10).await.unwrap().len(),
        2
    );
}

#[tokio::test]
async fn mutation_like_assistant_proposal_fails_closed_when_safe_mode_is_enabled() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"remember to send the project update"}"#.to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"Can you send the project update now?"}"#.to_string(),
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

    assert_eq!(json["data"]["proposal"]["state"], "staged");
    assert_eq!(json["data"]["proposal"]["permission_mode"], "blocked");
    assert!(json["data"]["proposal"]["summary"]
        .as_str()
        .unwrap()
        .contains("safe mode"));
    let thread_id = json["data"]["proposal"]["thread_route"]["thread_id"]
        .as_str()
        .expect("thread id");
    let thread_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/v1/threads/{thread_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(thread_response.status(), StatusCode::OK);
    let thread_body = axum::body::to_bytes(thread_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let thread_json: serde_json::Value = serde_json::from_slice(&thread_body).unwrap();
    assert_eq!(thread_json["data"]["metadata"]["proposal_state"], "staged");
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["kind"],
        "gated"
    );
}

#[tokio::test]
async fn assistant_repo_write_proposal_links_thread_to_pending_execution_review() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let project_root = unique_dir("assistant_review");
    let project = test_project("proj_assistant_review", "assistant-review", &project_root);
    storage.create_project(project.clone()).await.unwrap();
    let (broadcast_tx, _) = broadcast::channel(8);
    let state = veld::state::AppState::new(
        storage.clone(),
        AppConfig {
            artifact_root: unique_dir("artifacts").to_string_lossy().to_string(),
            node_id: Some("vel-authority".to_string()),
            node_display_name: Some("Vel Authority".to_string()),
            ..Default::default()
        },
        veld::policy_config::PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    );
    let app = veld::app::build_app_with_state(state);

    let handoff_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/execution/handoffs")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "project_id": project.id.as_ref(),
                        "from_agent": "operator",
                        "to_agent": "codex-local",
                        "origin_kind": "human_to_agent",
                        "objective": "Reply to the project update thread",
                        "task_kind": "implementation",
                        "read_scopes": [project.primary_repo.path, project.primary_notes_root.path],
                        "write_scopes": [project.primary_repo.path],
                        "allowed_tools": ["rg"],
                        "constraints": ["do not widen permissions"],
                        "expected_output_schema": {
                            "artifacts": ["patch", "summary"]
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(handoff_response.status(), StatusCode::OK);
    let handoff_body = axum::body::to_bytes(handoff_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let handoff_json: serde_json::Value = serde_json::from_slice(&handoff_body).unwrap();
    let handoff_id = handoff_json["data"]["id"].as_str().expect("handoff id");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"Can you edit the code and send the project update now?"}"#
                        .to_string(),
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

    assert_eq!(json["data"]["proposal"]["state"], "staged");
    assert_eq!(json["data"]["proposal"]["permission_mode"], "unavailable");
    let thread_id = json["data"]["proposal"]["thread_route"]["thread_id"]
        .as_str()
        .expect("thread id");
    let thread_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/v1/threads/{thread_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(thread_response.status(), StatusCode::OK);
    let thread_body = axum::body::to_bytes(thread_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let thread_json: serde_json::Value = serde_json::from_slice(&thread_body).unwrap();
    assert_eq!(thread_json["data"]["metadata"]["proposal_state"], "staged");
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["kind"],
        "execution_handoff_review"
    );
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["handoff_id"],
        handoff_id
    );
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["approve_path"],
        format!("/v1/execution/handoffs/{handoff_id}/approve")
    );
    assert!(thread_json["data"]["links"]
        .as_array()
        .expect("thread links")
        .iter()
        .any(|link| {
            link["entity_type"] == "execution_handoff"
                && link["entity_id"] == handoff_id
                && link["relation_type"] == "approves"
        }));
}

#[tokio::test]
async fn assistant_repo_write_proposal_becomes_approved_when_write_scope_is_already_approved() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    let project_root = unique_dir("assistant_approved");
    let project = test_project(
        "proj_assistant_approved",
        "assistant-approved",
        &project_root,
    );
    storage.create_project(project.clone()).await.unwrap();
    let (broadcast_tx, _) = broadcast::channel(8);
    let state = veld::state::AppState::new(
        storage.clone(),
        AppConfig {
            artifact_root: unique_dir("artifacts-approved")
                .to_string_lossy()
                .to_string(),
            node_id: Some("vel-authority".to_string()),
            node_display_name: Some("Vel Authority".to_string()),
            ..Default::default()
        },
        veld::policy_config::PolicyConfig::default(),
        broadcast_tx,
        None,
        None,
    );
    let app = veld::app::build_app_with_state(state);

    let handoff_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/v1/execution/handoffs")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "project_id": project.id.as_ref(),
                        "from_agent": "operator",
                        "to_agent": "codex-local",
                        "origin_kind": "human_to_agent",
                        "objective": "Reply to the project update thread",
                        "task_kind": "implementation",
                        "read_scopes": [project.primary_repo.path, project.primary_notes_root.path],
                        "write_scopes": [project.primary_repo.path],
                        "allowed_tools": ["rg"],
                        "expected_output_schema": { "artifacts": ["patch"] }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let handoff_body = axum::body::to_bytes(handoff_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let handoff_json: serde_json::Value = serde_json::from_slice(&handoff_body).unwrap();
    let handoff_id = handoff_json["data"]["id"].as_str().expect("handoff id");

    let approve_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/v1/execution/handoffs/{handoff_id}/approve"))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "reviewed_by": "operator",
                        "decision_reason": "Scoped repo work is approved."
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(approve_response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"Can you edit the code and send the project update now?"}"#
                        .to_string(),
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

    assert_eq!(json["data"]["proposal"]["state"], "approved");
    assert_eq!(json["data"]["proposal"]["permission_mode"], "user_confirm");
    let thread_id = json["data"]["proposal"]["thread_route"]["thread_id"]
        .as_str()
        .expect("thread id");
    let thread_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/v1/threads/{thread_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let thread_body = axum::body::to_bytes(thread_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let thread_json: serde_json::Value = serde_json::from_slice(&thread_body).unwrap();
    assert_eq!(
        thread_json["data"]["metadata"]["proposal_state"],
        "approved"
    );
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["kind"],
        "execution_handoff_ready"
    );
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["handoff_id"],
        handoff_id
    );
}

#[tokio::test]
async fn assistant_mutation_proposal_becomes_approved_when_writeback_is_enabled() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
        .set_setting("writeback_enabled", &serde_json::json!(true))
        .await
        .unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"remember to send the project update"}"#.to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"Can you send the project update now?"}"#.to_string(),
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
    assert_eq!(json["data"]["proposal"]["state"], "approved");
    assert_eq!(json["data"]["proposal"]["permission_mode"], "user_confirm");
    let thread_id = json["data"]["proposal"]["thread_route"]["thread_id"]
        .as_str()
        .expect("thread id");
    let thread_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/v1/threads/{thread_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let thread_body = axum::body::to_bytes(thread_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let thread_json: serde_json::Value = serde_json::from_slice(&thread_body).unwrap();
    assert_eq!(
        thread_json["data"]["metadata"]["proposal_state"],
        "approved"
    );
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["kind"],
        "writeback_ready"
    );
}

#[tokio::test]
async fn assistant_proposal_thread_preserves_applied_and_reversed_follow_through() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
        .set_setting("writeback_enabled", &serde_json::json!(true))
        .await
        .unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"remember to send the project update"}"#.to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"Can you send the project update now?"}"#.to_string(),
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
    let thread_id = json["data"]["proposal"]["thread_route"]["thread_id"]
        .as_str()
        .expect("thread id")
        .to_string();

    let intervention_id = storage
        .list_interventions_active(10)
        .await
        .unwrap()
        .into_iter()
        .find(|record| record.kind == "assistant_proposal")
        .map(|record| record.id.to_string())
        .expect("assistant proposal intervention");

    let resolve_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/interventions/{intervention_id}/resolve"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resolve_response.status(), StatusCode::OK);

    let thread_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/v1/threads/{thread_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let thread_body = axum::body::to_bytes(thread_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let thread_json: serde_json::Value = serde_json::from_slice(&thread_body).unwrap();
    assert_eq!(thread_json["data"]["lifecycle_stage"], "applied");
    assert_eq!(thread_json["data"]["metadata"]["proposal_state"], "applied");
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["kind"],
        "applied"
    );
    assert_eq!(
        thread_json["data"]["metadata"]["reversal"]["supported"],
        true
    );

    let dismiss_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/interventions/{intervention_id}/dismiss"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(dismiss_response.status(), StatusCode::OK);

    let thread_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!("/v1/threads/{thread_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let thread_body = axum::body::to_bytes(thread_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let thread_json: serde_json::Value = serde_json::from_slice(&thread_body).unwrap();
    assert_eq!(thread_json["data"]["status"], "dismissed");
    assert_eq!(thread_json["data"]["lifecycle_stage"], "reversed");
    assert_eq!(
        thread_json["data"]["metadata"]["proposal_state"],
        "reversed"
    );
    assert_eq!(
        thread_json["data"]["metadata"]["follow_through"]["kind"],
        "reversed"
    );
}

#[tokio::test]
async fn existing_conversation_and_missing_model_still_persist_user_message_safely() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
        .create_conversation(vel_storage::ConversationInsert {
            id: "conv_existing".to_string(),
            title: Some("Existing".to_string()),
            kind: "general".to_string(),
            pinned: false,
            archived: false,
        })
        .await
        .unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"What did I miss?","conversation_id":"conv_existing"}"#.to_string(),
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

    assert_eq!(json["data"]["route_target"], "threads");
    assert_eq!(json["data"]["conversation"]["id"], "conv_existing");
    assert_eq!(
        json["data"]["assistant_error"],
        "Chat model not configured. Set VEL_LLM_MODEL and run llama-server, or see configs/models/README.md."
    );
    assert!(json["data"]["assistant_message"].is_null());
    assert_eq!(
        storage
            .list_messages_by_conversation("conv_existing", 10)
            .await
            .unwrap()
            .len(),
        1,
        "user message should still be persisted"
    );
}

#[tokio::test]
async fn voice_assistant_entry_persists_explicit_voice_provenance() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "text": "What matters right now?",
                        "voice": {
                            "surface": "desktop_push_to_talk",
                            "source_device": "linux-desktop",
                            "locale": "en-US",
                            "transcript_origin": "speech_recognition",
                            "recorded_at": datetime!(2026-03-20 03:00:00 UTC),
                        }
                    })
                    .to_string(),
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

    assert_eq!(json["data"]["route_target"], "threads");
    assert_eq!(
        json["data"]["user_message"]["content"]["input_mode"],
        "voice"
    );
    assert_eq!(
        json["data"]["user_message"]["content"]["voice_provenance"]["surface"],
        "desktop_push_to_talk"
    );
    assert_eq!(
        json["data"]["user_message"]["content"]["voice_provenance"]["transcript_origin"],
        "speech_recognition"
    );
}

#[tokio::test]
async fn morning_assistant_entry_starts_shared_daily_loop_inline() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
        .set_setting("timezone", &serde_json::json!("UTC"))
        .await
        .unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"Good morning, start my day."}"#.to_string(),
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

    assert_eq!(json["data"]["route_target"], "inline");
    assert_eq!(
        json["data"]["daily_loop_session"]["phase"],
        "morning_overview"
    );
    assert_eq!(
        json["data"]["daily_loop_session"]["start"]["surface"],
        "web"
    );
    assert!(json["data"]["assistant_message"]["content"]["text"]
        .as_str()
        .unwrap_or_default()
        .contains("Morning overview ready"));

    let session_date = time::OffsetDateTime::now_utc().date();
    let session_date = format!(
        "{:04}-{:02}-{:02}",
        session_date.year(),
        u8::from(session_date.month()),
        session_date.day()
    );
    let record = storage
        .get_active_daily_session_for_date(&session_date, DailyLoopPhase::MorningOverview)
        .await
        .unwrap()
        .expect("active morning session");
    assert_eq!(record.session.phase, DailyLoopPhase::MorningOverview);
    assert_eq!(record.session.start.surface, DailyLoopSurface::Web);
}

#[tokio::test]
async fn standup_assistant_entry_resumes_existing_shared_session() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
        .set_setting("timezone", &serde_json::json!("UTC"))
        .await
        .unwrap();

    let session_date = format!(
        "{:04}-{:02}-{:02}",
        time::OffsetDateTime::now_utc().year(),
        u8::from(time::OffsetDateTime::now_utc().month()),
        time::OffsetDateTime::now_utc().day()
    );
    let existing = veld::services::daily_loop::start_session(
        &storage,
        &AppConfig::default(),
        DailyLoopStartRequest {
            session_date,
            phase: DailyLoopPhase::Standup,
            start: DailyLoopStartMetadata {
                source: DailyLoopStartSource::Manual,
                surface: DailyLoopSurface::Cli,
            },
        },
    )
    .await
    .unwrap();

    let app = veld::app::build_app_with_state(test_state(storage, None, None));
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"text":"Resume standup."}"#.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["data"]["route_target"], "inline");
    assert_eq!(json["data"]["daily_loop_session"]["phase"], "standup");
    assert_eq!(
        json["data"]["daily_loop_session"]["id"],
        existing.id.to_string()
    );
}

#[tokio::test]
async fn end_of_day_assistant_entry_returns_typed_closeout_inline() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
        .insert_capture(vel_storage::CaptureInsert {
            content_text: "Shipped the grounded assistant path".to_string(),
            capture_type: "quick_note".to_string(),
            source_device: Some("desktop".to_string()),
            privacy_class: vel_core::PrivacyClass::Private,
        })
        .await
        .unwrap();

    let app = veld::app::build_app_with_state(test_state(storage.clone(), None, None));
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/assistant/entry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"text":"Help me close out today."}"#.to_string(),
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

    assert_eq!(json["data"]["route_target"], "inline");
    assert_eq!(
        json["data"]["end_of_day"]["what_was_done"][0]["content_text"],
        "Shipped the grounded assistant path"
    );
    assert!(json["data"]["assistant_message"]["content"]["text"]
        .as_str()
        .unwrap_or_default()
        .contains("End-of-day closeout ready"));
    assert!(json["data"]["daily_loop_session"].is_null());
}
