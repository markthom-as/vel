use uuid::Uuid;
use vel_api_types::{InboxItemData, MessageData, WsEventType};
use vel_storage::InterventionInsert;

use crate::{
    broadcast::WsEnvelope, errors::AppError, services::chat::events::emit_chat_event,
    state::AppState,
};

fn intervention_kind_for_message(message: &MessageData) -> Option<&'static str> {
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
    message: &MessageData,
) -> Result<Option<InboxItemData>, AppError> {
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

    let data = InboxItemData {
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
        .send(WsEnvelope::new(WsEventType::InterventionsNew, ws_payload));

    Ok(Some(data))
}
