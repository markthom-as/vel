//! Chat API: conversations, messages, inbox, interventions.
//! Tickets 014–017. Routes under /api/.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vel_api_types::ApiResponse;
use vel_storage::{ConversationInsert, EventLogInsert, MessageInsert};

use crate::broadcast::WsEnvelope;
use crate::{errors::AppError, state::AppState};

// --- DTOs (product-shaped, not raw DB rows) ---

#[derive(Debug, Clone, Serialize)]
pub struct ConversationData {
    pub id: String,
    pub title: Option<String>,
    pub kind: String,
    pub pinned: bool,
    pub archived: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct ConversationCreateRequest {
    pub title: Option<String>,
    #[serde(default = "default_kind")]
    pub kind: String,
}

fn default_kind() -> String {
    "general".to_string()
}

#[derive(Debug, Deserialize)]
pub struct ConversationUpdateRequest {
    pub title: Option<String>,
    pub pinned: Option<bool>,
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageData {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub kind: String,
    pub content: serde_json::Value,
    pub status: Option<String>,
    pub importance: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct MessageCreateRequest {
    pub role: String,
    pub kind: String,
    pub content: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct InboxItemData {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: i64,
    pub snoozed_until: Option<i64>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InterventionActionData {
    pub id: String,
    pub state: String,
}

// --- Helpers: append event and map storage -> DTO ---

async fn emit_chat_event(
    state: &AppState,
    event_name: &str,
    aggregate_type: &str,
    aggregate_id: &str,
    payload: serde_json::Value,
) {
    let _ = state
        .storage
        .append_event(EventLogInsert {
            id: None,
            event_name: event_name.to_string(),
            aggregate_type: Some(aggregate_type.to_string()),
            aggregate_id: Some(aggregate_id.to_string()),
            payload_json: payload.to_string(),
        })
        .await;
}

fn conversation_record_to_data(r: vel_storage::ConversationRecord) -> ConversationData {
    ConversationData {
        id: r.id.as_ref().to_string(),
        title: r.title,
        kind: r.kind,
        pinned: r.pinned,
        archived: r.archived,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }
}

fn message_record_to_data(r: vel_storage::MessageRecord) -> Result<MessageData, AppError> {
    let content: serde_json::Value = serde_json::from_str(&r.content_json)
        .unwrap_or_else(|_| serde_json::json!({ "raw": r.content_json }));
    Ok(MessageData {
        id: r.id.as_ref().to_string(),
        conversation_id: r.conversation_id.as_ref().to_string(),
        role: r.role,
        kind: r.kind,
        content,
        status: r.status,
        importance: r.importance,
        created_at: r.created_at,
        updated_at: r.updated_at,
    })
}

fn intervention_record_to_inbox_item(r: vel_storage::InterventionRecord) -> InboxItemData {
    InboxItemData {
        id: r.id.as_ref().to_string(),
        message_id: r.message_id.as_ref().to_string(),
        kind: r.kind,
        state: r.state,
        surfaced_at: r.surfaced_at,
        snoozed_until: r.snoozed_until,
        confidence: r.confidence,
    }
}

// --- Conversation handlers ---

pub async fn list_conversations(
    State(state): State<AppState>,
    Query(q): Query<ListConversationsQuery>,
) -> Result<Json<ApiResponse<Vec<ConversationData>>>, AppError> {
    let limit = q.limit.unwrap_or(100).min(500);
    let list = state
        .storage
        .list_conversations(q.archived, limit)
        .await?;
    let data: Vec<ConversationData> = list.into_iter().map(conversation_record_to_data).collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct ListConversationsQuery {
    pub archived: Option<bool>,
    pub limit: Option<u32>,
}

pub async fn create_conversation(
    State(state): State<AppState>,
    Json(payload): Json<ConversationCreateRequest>,
) -> Result<Json<ApiResponse<ConversationData>>, AppError> {
    let id = format!("conv_{}", Uuid::new_v4().simple());
    let kind = payload.kind.clone();
    state
        .storage
        .create_conversation(ConversationInsert {
            id: id.clone(),
            title: payload.title,
            kind: kind.clone(),
            pinned: false,
            archived: false,
        })
        .await?;
    emit_chat_event(
        &state,
        "conversation.created",
        "conversation",
        &id,
        serde_json::json!({ "id": id, "kind": kind }),
    )
    .await;
    let conv = state
        .storage
        .get_conversation(&id)
        .await?
        .ok_or_else(|| AppError::internal("conversation not found after create"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        conversation_record_to_data(conv),
        request_id,
    )))
}

pub async fn get_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ConversationData>>, AppError> {
    let conv = state
        .storage
        .get_conversation(id.trim())
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        conversation_record_to_data(conv),
        request_id,
    )))
}

pub async fn update_conversation(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ConversationUpdateRequest>,
) -> Result<Json<ApiResponse<ConversationData>>, AppError> {
    let id = id.trim();
    if let Some(title) = payload.title {
        state.storage.rename_conversation(id, &title).await?;
    }
    if let Some(pinned) = payload.pinned {
        state.storage.pin_conversation(id, pinned).await?;
    }
    if let Some(archived) = payload.archived {
        state.storage.archive_conversation(id, archived).await?;
    }
    emit_chat_event(
        &state,
        "conversation.updated",
        "conversation",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let conv = state
        .storage
        .get_conversation(id)
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        conversation_record_to_data(conv),
        request_id,
    )))
}

// --- Message handlers ---

pub async fn list_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Query(q): Query<ListMessagesQuery>,
) -> Result<Json<ApiResponse<Vec<MessageData>>>, AppError> {
    let limit = q.limit.unwrap_or(200).min(2000);
    let list = state
        .storage
        .list_messages_by_conversation(conversation_id.trim(), limit)
        .await?;
    let mut data = Vec::with_capacity(list.len());
    for r in list {
        data.push(message_record_to_data(r)?);
    }
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct ListMessagesQuery {
    pub limit: Option<u32>,
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<MessageCreateRequest>,
) -> Result<Json<ApiResponse<MessageData>>, AppError> {
    let conversation_id = conversation_id.trim();
    let _ = state
        .storage
        .get_conversation(conversation_id)
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;
    let id = format!("msg_{}", Uuid::new_v4().simple());
    let content_json = serde_json::to_string(&payload.content).map_err(|e| AppError::bad_request(e.to_string()))?;
    let kind = payload.kind.clone();
    state
        .storage
        .create_message(MessageInsert {
            id: id.clone(),
            conversation_id: conversation_id.to_string(),
            role: payload.role,
            kind: kind.clone(),
            content_json,
            status: None,
            importance: None,
        })
        .await?;
    emit_chat_event(
        &state,
        "message.created",
        "message",
        &id,
        serde_json::json!({ "id": id, "conversation_id": conversation_id, "kind": kind }),
    )
    .await;
    let msg = state
        .storage
        .get_message(&id)
        .await?
        .ok_or_else(|| AppError::internal("message not found after create"))?;
    let data = message_record_to_data(msg)?;
    let payload = serde_json::to_value(&data).unwrap_or_else(|_| serde_json::json!({ "id": id }));
    let _ = state.broadcast_tx.send(WsEnvelope::new("messages:new", payload));
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

// --- Inbox handler ---

pub async fn get_inbox(
    State(state): State<AppState>,
    Query(q): Query<InboxQuery>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let limit = q.limit.unwrap_or(100).min(500);
    let list = state.storage.list_interventions_active(limit).await?;
    let data: Vec<InboxItemData> = list.into_iter().map(intervention_record_to_inbox_item).collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct InboxQuery {
    pub limit: Option<u32>,
}

// --- Message interventions (for inline actions) ---

pub async fn list_message_interventions(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<InboxItemData>>>, AppError> {
    let list = state
        .storage
        .get_interventions_by_message(message_id.trim())
        .await?;
    let data: Vec<InboxItemData> = list.into_iter().map(intervention_record_to_inbox_item).collect();
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

// --- Provenance (029) ---

#[derive(Debug, Clone, Serialize)]
pub struct ProvenanceData {
    pub message_id: String,
    pub events: Vec<ProvenanceEvent>,
    pub signals: Vec<serde_json::Value>,
    pub policy_decisions: Vec<serde_json::Value>,
    pub linked_objects: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProvenanceEvent {
    pub id: String,
    pub event_name: String,
    pub created_at: i64,
    pub payload: serde_json::Value,
}

pub async fn get_message_provenance(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
) -> Result<Json<ApiResponse<ProvenanceData>>, AppError> {
    let message_id = message_id.trim();
    let _ = state
        .storage
        .get_message(message_id)
        .await?
        .ok_or_else(|| AppError::not_found("message not found"))?;
    let events = state
        .storage
        .list_events_by_aggregate("message", message_id, 50)
        .await?;
    let events_data: Vec<ProvenanceEvent> = events
        .into_iter()
        .map(|r| {
            let payload: serde_json::Value = serde_json::from_str(&r.payload_json).unwrap_or_else(|_| serde_json::json!({}));
            ProvenanceEvent {
                id: r.id.as_ref().to_string(),
                event_name: r.event_name,
                created_at: r.created_at,
                payload,
            }
        })
        .collect();
    let data = ProvenanceData {
        message_id: message_id.to_string(),
        events: events_data,
        signals: vec![],
        policy_decisions: vec![],
        linked_objects: vec![],
    };
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

// --- Settings (031) ---

pub async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let map = state.storage.get_all_settings().await?;
    let data: serde_json::Value = serde_json::to_value(map).unwrap_or_else(|_| serde_json::json!({}));
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

#[derive(Debug, Deserialize)]
pub struct SettingsUpdateRequest {
    pub quiet_hours: Option<serde_json::Value>,
    pub disable_proactive: Option<bool>,
    pub toggle_risks: Option<bool>,
    pub toggle_reminders: Option<bool>,
}

pub async fn patch_settings(
    State(state): State<AppState>,
    Json(payload): Json<SettingsUpdateRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    if let Some(v) = payload.quiet_hours {
        state.storage.set_setting("quiet_hours", &v).await?;
    }
    if let Some(v) = payload.disable_proactive {
        state.storage.set_setting("disable_proactive", &serde_json::json!(v)).await?;
    }
    if let Some(v) = payload.toggle_risks {
        state.storage.set_setting("toggle_risks", &serde_json::json!(v)).await?;
    }
    if let Some(v) = payload.toggle_reminders {
        state.storage.set_setting("toggle_reminders", &serde_json::json!(v)).await?;
    }
    let map = state.storage.get_all_settings().await?;
    let data: serde_json::Value = serde_json::to_value(map).unwrap_or_else(|_| serde_json::json!({}));
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(data, request_id)))
}

// --- Intervention action handlers ---

pub async fn intervention_snooze(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<SnoozeRequest>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let id = id.trim();
    let until_ts = payload
        .snoozed_until_ts
        .or(payload.minutes.map(|m| time::OffsetDateTime::now_utc().unix_timestamp() + (m as i64) * 60))
        .ok_or_else(|| AppError::bad_request("snoozed_until_ts or minutes required"))?;
    let _ = state.storage.get_intervention(id).await?.ok_or_else(|| AppError::not_found("intervention not found"))?;
    state.storage.snooze_intervention(id, until_ts).await?;
    emit_chat_event(
        &state,
        "intervention.snoozed",
        "intervention",
        id,
        serde_json::json!({ "id": id, "snoozed_until": until_ts }),
    )
    .await;
    let payload = serde_json::json!({ "id": id, "state": "snoozed" });
    let _ = state.broadcast_tx.send(WsEnvelope::new("interventions:updated", payload));
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        InterventionActionData {
            id: id.to_string(),
            state: "snoozed".to_string(),
        },
        request_id,
    )))
}

#[derive(Debug, Deserialize)]
pub struct SnoozeRequest {
    pub snoozed_until_ts: Option<i64>,
    pub minutes: Option<u32>,
}

pub async fn intervention_resolve(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let id = id.trim();
    let _ = state.storage.get_intervention(id).await?.ok_or_else(|| AppError::not_found("intervention not found"))?;
    state.storage.resolve_intervention(id).await?;
    emit_chat_event(
        &state,
        "intervention.resolved",
        "intervention",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let payload = serde_json::json!({ "id": id, "state": "resolved" });
    let _ = state.broadcast_tx.send(WsEnvelope::new("interventions:updated", payload));
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        InterventionActionData {
            id: id.to_string(),
            state: "resolved".to_string(),
        },
        request_id,
    )))
}

pub async fn intervention_dismiss(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<InterventionActionData>>, AppError> {
    let id = id.trim();
    let _ = state.storage.get_intervention(id).await?.ok_or_else(|| AppError::not_found("intervention not found"))?;
    state.storage.dismiss_intervention(id).await?;
    emit_chat_event(
        &state,
        "intervention.dismissed",
        "intervention",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let payload = serde_json::json!({ "id": id, "state": "dismissed" });
    let _ = state.broadcast_tx.send(WsEnvelope::new("interventions:updated", payload));
    let request_id = format!("req_{}", Uuid::new_v4().simple());
    Ok(Json(ApiResponse::success(
        InterventionActionData {
            id: id.to_string(),
            state: "dismissed".to_string(),
        },
        request_id,
    )))
}

pub fn chat_routes() -> Router<AppState> {
    Router::new()
        .route("/api/conversations", get(list_conversations).post(create_conversation))
        .route("/api/conversations/:id", get(get_conversation).patch(update_conversation))
        .route(
            "/api/conversations/:id/messages",
            get(list_messages).post(create_message),
        )
        .route("/api/inbox", get(get_inbox))
        .route("/api/messages/:id/interventions", get(list_message_interventions))
        .route("/api/messages/:id/provenance", get(get_message_provenance))
        .route("/api/settings", get(get_settings).patch(patch_settings))
        .route("/api/interventions/:id/snooze", post(intervention_snooze))
        .route("/api/interventions/:id/resolve", post(intervention_resolve))
        .route("/api/interventions/:id/dismiss", post(intervention_dismiss))
}
