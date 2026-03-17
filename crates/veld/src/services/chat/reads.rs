use vel_api_types::{InboxItemData, ProvenanceData, ProvenanceEvent};

use crate::{
    errors::AppError,
    services::chat::{
        mapping::{intervention_record_to_inbox_item, message_record_to_data},
        provenance::{build_linked_objects, build_policy_decisions, build_provenance_signals},
    },
    state::AppState,
};

pub(crate) async fn list_inbox_items(
    state: &AppState,
    limit: u32,
) -> Result<Vec<InboxItemData>, AppError> {
    let list = state.storage.list_interventions_active(limit).await?;
    Ok(list
        .into_iter()
        .map(intervention_record_to_inbox_item)
        .collect())
}

pub(crate) async fn list_conversation_intervention_items(
    state: &AppState,
    conversation_id: &str,
) -> Result<Vec<InboxItemData>, AppError> {
    let conversation_id = conversation_id.trim();
    let _ = state
        .storage
        .get_conversation(conversation_id)
        .await?
        .ok_or_else(|| AppError::not_found("conversation not found"))?;
    let list = state
        .storage
        .get_interventions_by_conversation(conversation_id)
        .await?;
    Ok(list
        .into_iter()
        .map(intervention_record_to_inbox_item)
        .collect())
}

pub(crate) async fn list_message_intervention_items(
    state: &AppState,
    message_id: &str,
) -> Result<Vec<InboxItemData>, AppError> {
    let list = state
        .storage
        .get_interventions_by_message(message_id.trim())
        .await?;
    Ok(list
        .into_iter()
        .map(intervention_record_to_inbox_item)
        .collect())
}

pub(crate) async fn build_message_provenance_data(
    state: &AppState,
    message_id: &str,
) -> Result<ProvenanceData, AppError> {
    let message_id = message_id.trim();
    let message = state
        .storage
        .get_message(message_id)
        .await?
        .ok_or_else(|| AppError::not_found("message not found"))?;
    let message = message_record_to_data(message)?;
    let interventions = state
        .storage
        .get_interventions_by_message(message_id)
        .await?;
    let events = state
        .storage
        .list_events_by_aggregate("message", message_id, 50)
        .await?;
    let events_data: Vec<ProvenanceEvent> = events
        .into_iter()
        .map(|record| {
            let payload: serde_json::Value = serde_json::from_str(&record.payload_json)
                .unwrap_or_else(|_| serde_json::json!({}));
            ProvenanceEvent {
                id: record.id.as_ref().to_string(),
                event_name: record.event_name,
                created_at: record.created_at,
                payload,
            }
        })
        .collect();
    let linked_objects = build_linked_objects(&message, &interventions);
    let signals = build_provenance_signals(&message, &interventions);
    let policy_decisions = build_policy_decisions(&message, &interventions);

    Ok(ProvenanceData {
        message_id: message_id.to_string(),
        events: events_data,
        signals,
        policy_decisions,
        linked_objects,
    })
}
