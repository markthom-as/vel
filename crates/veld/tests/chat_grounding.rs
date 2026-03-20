use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use time::OffsetDateTime;
use tokio::sync::{broadcast, Mutex};
use tower::ServiceExt;
use vel_config::AppConfig;
use vel_llm::{
    FinishReason, LlmError, LlmProvider, LlmRequest, LlmResponse, ModelInfo, ProviderHealth,
    ProviderRegistry, Router, ToolCall, Usage,
};

struct MockChatProvider {
    requests: Arc<Mutex<Vec<LlmRequest>>>,
}

impl MockChatProvider {
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

struct MockCloseoutProvider {
    requests: Arc<Mutex<Vec<LlmRequest>>>,
}

impl MockCloseoutProvider {
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
impl LlmProvider for MockCloseoutProvider {
    async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, LlmError> {
        let mut requests = self.requests.lock().await;
        requests.push(req.clone());
        let call_index = requests.len();

        match call_index {
            1 => {
                assert!(req
                    .tools
                    .iter()
                    .any(|tool| tool.name == "vel_get_context_brief"));
                Ok(LlmResponse {
                    text: None,
                    tool_calls: vec![ToolCall {
                        id: "call_closeout_1".to_string(),
                        name: "vel_get_context_brief".to_string(),
                        arguments: serde_json::json!({
                            "kind": "end_of_day",
                        }),
                    }],
                    usage: Usage::default(),
                    finish_reason: FinishReason::ToolCall,
                    raw: serde_json::json!({}),
                })
            }
            2 => {
                let last_message = req.messages.last().expect("tool result message");
                assert!(last_message.content.contains("\"kind\": \"end_of_day\""));
                assert!(last_message.content.contains("Shipped closeout support"));
                Ok(LlmResponse {
                    text: Some("Vel's end-of-day brief shows shipped closeout support and no remaining blockers.".to_string()),
                    tool_calls: vec![],
                    usage: Usage::default(),
                    finish_reason: FinishReason::Stop,
                    raw: serde_json::json!({}),
                })
            }
            _ => panic!("unexpected extra provider call"),
        }
    }

    async fn health(&self) -> Result<ProviderHealth, LlmError> {
        Ok(ProviderHealth {
            healthy: true,
            details: serde_json::json!({}),
        })
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        Ok(vec![ModelInfo {
            id: "mock-closeout".to_string(),
            context_window: Some(32768),
            supports_tools: true,
            supports_json: true,
        }])
    }
}

#[async_trait]
impl LlmProvider for MockChatProvider {
    async fn generate(&self, req: &LlmRequest) -> Result<LlmResponse, LlmError> {
        let mut requests = self.requests.lock().await;
        requests.push(req.clone());
        let call_index = requests.len();

        match call_index {
            1 => {
                assert!(req.system.contains("Agent Grounding"));
                assert!(req
                    .tools
                    .iter()
                    .any(|tool| tool.name == "vel_search_memory"));
                assert!(req
                    .tools
                    .iter()
                    .any(|tool| tool.name == "vel_get_daily_loop_status"));
                assert!(req.tools.iter().any(|tool| tool.name == "vel_list_threads"));
                Ok(LlmResponse {
                    text: None,
                    tool_calls: vec![ToolCall {
                        id: "call_1".to_string(),
                        name: "vel_search_memory".to_string(),
                        arguments: serde_json::json!({
                            "query": "accountant follow up",
                            "limit": 3,
                        }),
                    }],
                    usage: Usage::default(),
                    finish_reason: FinishReason::ToolCall,
                    raw: serde_json::json!({}),
                })
            }
            2 => {
                let last_message = req.messages.last().expect("tool result message");
                assert!(last_message.content.contains("Vel tool results"));
                assert!(last_message.content.contains("accountant"));
                Ok(LlmResponse {
                    text: Some(
                        "Vel shows a saved note about accountant follow up on the quarterly estimate."
                            .to_string(),
                    ),
                    tool_calls: vec![],
                    usage: Usage::default(),
                    finish_reason: FinishReason::Stop,
                    raw: serde_json::json!({}),
                })
            }
            _ => panic!("unexpected extra provider call"),
        }
    }

    async fn health(&self) -> Result<ProviderHealth, LlmError> {
        Ok(ProviderHealth {
            healthy: true,
            details: serde_json::json!({}),
        })
    }

    async fn models(&self) -> Result<Vec<ModelInfo>, LlmError> {
        Ok(vec![ModelInfo {
            id: "mock-chat".to_string(),
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

#[tokio::test]
async fn chat_assistant_uses_vel_tools_to_answer_from_persisted_data() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
        .upsert_note_semantic_record(
            "projects/tax/accountant.md",
            "Accountant follow up",
            "Need accountant follow up on the quarterly estimate this week.",
            "cap_seed",
            OffsetDateTime::now_utc().unix_timestamp(),
        )
        .await
        .unwrap();

    let (provider, requests) = MockChatProvider::new();
    let mut registry = ProviderRegistry::new();
    registry.register("mock-chat", Arc::new(provider));
    let router = Arc::new(Router::new(registry));

    let app = veld::app::build_app_with_state(test_state(
        storage,
        Some(router),
        Some("mock-chat".to_string()),
    ));

    let create_conv = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/conversations")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"title":"Assistant","kind":"general"}"#.to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_conv.status(), StatusCode::OK);
    let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
        .await
        .unwrap();
    let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
    let conv_id = conv_json["data"]["id"].as_str().unwrap();

    let message_resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/conversations/{conv_id}/messages"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"role":"user","kind":"text","content":{"text":"What do I need to remember about the accountant?"}}"#
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(message_resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(message_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json["data"]["assistant_message"]["content"]["text"],
        "Vel shows a saved note about accountant follow up on the quarterly estimate."
    );
    assert!(json["data"]["assistant_error"].is_null());

    let requests = requests.lock().await;
    assert_eq!(requests.len(), 2);
}

#[tokio::test]
async fn chat_assistant_can_ground_closeout_from_end_of_day_tool() {
    let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
    storage.migrate().await.unwrap();
    storage
        .insert_capture(vel_storage::CaptureInsert {
            content_text: "Shipped closeout support".to_string(),
            capture_type: "quick_note".to_string(),
            source_device: Some("desktop".to_string()),
            privacy_class: vel_core::PrivacyClass::Private,
        })
        .await
        .unwrap();

    let (provider, requests) = MockCloseoutProvider::new();
    let mut registry = ProviderRegistry::new();
    registry.register("mock-closeout", Arc::new(provider));
    let router = Arc::new(Router::new(registry));

    let app = veld::app::build_app_with_state(test_state(
        storage,
        Some(router),
        Some("mock-closeout".to_string()),
    ));

    let create_conv = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/conversations")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"title":"Closeout","kind":"general"}"#.to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_conv.status(), StatusCode::OK);
    let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
        .await
        .unwrap();
    let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
    let conv_id = conv_json["data"]["id"].as_str().unwrap();

    let message_resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/conversations/{conv_id}/messages"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"role":"user","kind":"text","content":{"text":"Help me close out today."}}"#
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(message_resp.status(), StatusCode::OK);
    let body = axum::body::to_bytes(message_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json["data"]["assistant_message"]["content"]["text"],
        "Vel's end-of-day brief shows shipped closeout support and no remaining blockers."
    );

    let requests = requests.lock().await;
    assert_eq!(requests.len(), 2);
}
