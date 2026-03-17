use uuid::Uuid;
use vel_storage::InterventionInsert;

use crate::{
    broadcast::WsEnvelope, errors::AppError, services::chat::events::emit_chat_event,
    state::AppState,
};

#[derive(Debug, Clone)]
pub(crate) struct InterventionMessageInput {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub kind: String,
    pub content: serde_json::Value,
}

#[derive(Debug, Clone)]
pub(crate) struct InterventionInboxItem {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: i64,
    pub snoozed_until: Option<i64>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone)]
pub(crate) struct InterventionAction {
    pub id: String,
    pub state: String,
}

fn intervention_kind_for_message(message: &InterventionMessageInput) -> Option<&'static str> {
    if message.role != "assistant" {
        return None;
    }
    match message.kind.as_str() {
        "reminder_card" => Some("reminder"),
        "risk_card" => Some("risk"),
        "suggestion_card" => Some("suggestion"),
        _ => None,
    }
}

pub(crate) async fn create_intervention_for_message_if_needed(
    state: &AppState,
    message: &InterventionMessageInput,
) -> Result<Option<InterventionInboxItem>, AppError> {
    let intervention_kind = match intervention_kind_for_message(message) {
        Some(kind) => kind,
        None => return Ok(None),
    };

    let intervention_id = format!("intv_{}", Uuid::new_v4().simple());
    let surfaced_at = time::OffsetDateTime::now_utc().unix_timestamp();
    state
        .storage
        .create_intervention(InterventionInsert {
            id: intervention_id.clone(),
            message_id: message.id.clone(),
            kind: intervention_kind.to_string(),
            state: "active".to_string(),
            surfaced_at,
            resolved_at: None,
            snoozed_until: None,
            confidence: None,
            source_json: Some(message.content.to_string()),
            provenance_json: Some(
                serde_json::json!({
                    "message_id": message.id,
                    "message_kind": message.kind,
                    "conversation_id": message.conversation_id,
                })
                .to_string(),
            ),
        })
        .await?;

    let data = InterventionInboxItem {
        id: intervention_id.clone(),
        message_id: message.id.clone(),
        kind: intervention_kind.to_string(),
        state: "active".to_string(),
        surfaced_at,
        snoozed_until: None,
        confidence: None,
    };

    emit_chat_event(
        state,
        "intervention.created",
        "intervention",
        &intervention_id,
        serde_json::json!({
            "id": intervention_id,
            "message_id": message.id,
            "kind": intervention_kind,
            "state": "active",
            "conversation_id": message.conversation_id,
        }),
    )
    .await;

    let ws_payload =
        serde_json::to_value(&data).unwrap_or_else(|_| serde_json::json!({ "id": data.id }));
    let _ = state
        .broadcast_tx
        .send(WsEnvelope::new(
            vel_api_types::WsEventType::InterventionsNew,
            ws_payload,
        ));

    Ok(Some(data))
}

pub(crate) async fn snooze_intervention(
    state: &AppState,
    id: &str,
    until_ts: i64,
) -> Result<InterventionAction, AppError> {
    let id = id.trim();
    let _ = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    state.storage.snooze_intervention(id, until_ts).await?;
    emit_chat_event(
        state,
        "intervention.snoozed",
        "intervention",
        id,
        serde_json::json!({ "id": id, "snoozed_until": until_ts }),
    )
    .await;
    let payload = InterventionAction {
        id: id.to_string(),
        state: "snoozed".to_string(),
    };
    let _ = state.broadcast_tx.send(WsEnvelope::new(
        vel_api_types::WsEventType::InterventionsUpdated,
        serde_json::to_value(&payload).unwrap_or_else(|_| serde_json::json!({ "id": id })),
    ));
    Ok(payload)
}

pub(crate) async fn resolve_intervention(
    state: &AppState,
    id: &str,
) -> Result<InterventionAction, AppError> {
    let id = id.trim();
    let _ = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    state.storage.resolve_intervention(id).await?;
    emit_chat_event(
        state,
        "intervention.resolved",
        "intervention",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let payload = InterventionAction {
        id: id.to_string(),
        state: "resolved".to_string(),
    };
    let _ = state.broadcast_tx.send(WsEnvelope::new(
        vel_api_types::WsEventType::InterventionsUpdated,
        serde_json::to_value(&payload).unwrap_or_else(|_| serde_json::json!({ "id": id })),
    ));
    Ok(payload)
}

pub(crate) async fn dismiss_intervention(
    state: &AppState,
    id: &str,
) -> Result<InterventionAction, AppError> {
    let id = id.trim();
    let _ = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    state.storage.dismiss_intervention(id).await?;
    emit_chat_event(
        state,
        "intervention.dismissed",
        "intervention",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let payload = InterventionAction {
        id: id.to_string(),
        state: "dismissed".to_string(),
    };
    let _ = state.broadcast_tx.send(WsEnvelope::new(
        vel_api_types::WsEventType::InterventionsUpdated,
        serde_json::to_value(&payload).unwrap_or_else(|_| serde_json::json!({ "id": id })),
    ));
    Ok(payload)
}
