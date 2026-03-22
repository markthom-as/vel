use uuid::Uuid;
use vel_storage::{InterventionInsert, InterventionRecord};

use crate::{
    errors::AppError,
    services::chat::events::{
        broadcast_chat_ws_event, emit_chat_event, WS_EVENT_INTERVENTIONS_NEW,
        WS_EVENT_INTERVENTIONS_UPDATED,
    },
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

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct InterventionInboxItem {
    pub id: String,
    pub message_id: String,
    pub kind: String,
    pub state: String,
    pub surfaced_at: i64,
    pub snoozed_until: Option<i64>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct InterventionAction {
    pub id: String,
    pub state: String,
}

fn intervention_kind_for_message(message: &InterventionMessageInput) -> Option<&'static str> {
    if message
        .content
        .get("proposal")
        .and_then(serde_json::Value::as_object)
        .is_some()
    {
        return Some("assistant_proposal");
    }
    if message.role == "user"
        && message.kind == "text"
        && message
            .content
            .get("entry_route")
            .and_then(serde_json::Value::as_str)
            == Some("inbox")
    {
        return Some("capture");
    }
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
    broadcast_chat_ws_event(state, WS_EVENT_INTERVENTIONS_NEW, ws_payload);

    Ok(Some(data))
}

async fn sync_assistant_proposal_thread(
    state: &AppState,
    intervention: &InterventionRecord,
    thread_status: &str,
    transition: &str,
    snoozed_until: Option<i64>,
) -> Result<(), AppError> {
    if intervention.kind != "assistant_proposal" {
        return Ok(());
    }

    let thread_id = intervention
        .source_json
        .as_deref()
        .and_then(|value| serde_json::from_str::<serde_json::Value>(value).ok())
        .and_then(|value| {
            value
                .get("proposal")
                .and_then(|value| value.get("thread_route"))
                .and_then(|value| value.get("thread_id"))
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
        })
        .unwrap_or_else(|| {
            crate::services::chat::messages::assistant_proposal_thread_id(
                intervention.message_id.as_ref(),
            )
        });

    let Some((_, _, _, _, metadata_json, _, _)) =
        state.storage.get_thread_by_id(&thread_id).await?
    else {
        return Ok(());
    };
    let mut metadata = serde_json::from_str::<serde_json::Value>(&metadata_json)
        .unwrap_or_else(|_| serde_json::json!({}));
    let Some(object) = metadata.as_object_mut() else {
        return Ok(());
    };

    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let previous_state = object
        .get("proposal_state")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("staged")
        .to_string();
    let previous_follow_through = object
        .get("follow_through")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));

    match transition {
        "snoozed" => {
            object.insert(
                "follow_through".to_string(),
                serde_json::json!({
                    "kind": "snoozed",
                    "previous_kind": previous_follow_through
                        .get("kind")
                        .and_then(serde_json::Value::as_str),
                    "intervention_id": intervention.id.as_ref(),
                    "snoozed_until": snoozed_until,
                }),
            );
        }
        "acknowledged" | "resolved" => {
            object.insert(
                "proposal_state".to_string(),
                serde_json::Value::String("applied".to_string()),
            );
            object.insert("applied_at".to_string(), serde_json::json!(now));
            object.insert(
                "applied_via".to_string(),
                serde_json::Value::String(format!("intervention_{transition}")),
            );
            object.insert(
                "follow_through".to_string(),
                serde_json::json!({
                    "kind": "applied",
                    "previous_kind": previous_follow_through
                        .get("kind")
                        .and_then(serde_json::Value::as_str),
                    "intervention_id": intervention.id.as_ref(),
                    "applied_at": now,
                    "applied_via": format!("intervention_{transition}"),
                }),
            );
            object.insert(
                "reversal".to_string(),
                serde_json::json!({
                    "supported": true,
                    "dismiss_path": format!("/api/interventions/{}/dismiss", intervention.id.as_ref()),
                    "note": "Dismissal reverses Vel's assistant proposal state only. External systems keep their own reversal rules.",
                }),
            );
        }
        "reactivated" => {
            object.insert(
                "proposal_state".to_string(),
                serde_json::Value::String("staged".to_string()),
            );
            object.insert(
                "follow_through".to_string(),
                serde_json::json!({
                    "kind": "reactivated",
                    "intervention_id": intervention.id.as_ref(),
                    "reactivated_at": now,
                    "previous_state": previous_state,
                }),
            );
        }
        "dismissed" => {
            let next_state = if previous_state == "applied" {
                "reversed"
            } else {
                "failed"
            };
            object.insert(
                "proposal_state".to_string(),
                serde_json::Value::String(next_state.to_string()),
            );
            object.insert(
                "follow_through".to_string(),
                serde_json::json!({
                    "kind": next_state,
                    "previous_state": previous_state,
                    "previous_kind": previous_follow_through
                        .get("kind")
                        .and_then(serde_json::Value::as_str),
                    "intervention_id": intervention.id.as_ref(),
                    "changed_at": now,
                    "changed_via": "intervention_dismiss",
                }),
            );
            if next_state == "reversed" {
                object.insert("reversed_at".to_string(), serde_json::json!(now));
                object.insert(
                    "reversed_via".to_string(),
                    serde_json::Value::String("intervention_dismiss".to_string()),
                );
                object.insert(
                    "reversal".to_string(),
                    serde_json::json!({
                        "supported": false,
                        "note": "Proposal reversal has already been recorded.",
                    }),
                );
            } else {
                object.insert("failed_at".to_string(), serde_json::json!(now));
                object.insert(
                    "failed_via".to_string(),
                    serde_json::Value::String("intervention_dismiss".to_string()),
                );
            }
        }
        _ => {}
    }

    if let Some(lineage) = object
        .entry("lineage".to_string())
        .or_insert_with(|| serde_json::json!({}))
        .as_object_mut()
    {
        lineage.insert(
            "last_transition".to_string(),
            serde_json::Value::String(transition.to_string()),
        );
        lineage.insert(
            "intervention_id".to_string(),
            serde_json::json!(intervention.id.as_ref()),
        );
        lineage.insert(
            "intervention_state".to_string(),
            serde_json::json!(intervention.state),
        );
    }

    state
        .storage
        .update_thread_metadata(&thread_id, &metadata.to_string())
        .await?;
    state
        .storage
        .update_thread_status(&thread_id, thread_status)
        .await?;
    Ok(())
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
    let updated = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    sync_assistant_proposal_thread(state, &updated, "snoozed", "snoozed", Some(until_ts)).await?;
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
    broadcast_chat_ws_event(
        state,
        WS_EVENT_INTERVENTIONS_UPDATED,
        serde_json::to_value(&payload).unwrap_or_else(|_| serde_json::json!({ "id": id })),
    );
    Ok(payload)
}

pub(crate) async fn acknowledge_intervention(
    state: &AppState,
    id: &str,
) -> Result<InterventionAction, AppError> {
    let id = id.trim();
    let _ = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    state.storage.acknowledge_intervention(id).await?;
    let updated = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    sync_assistant_proposal_thread(state, &updated, "resolved", "acknowledged", None).await?;
    emit_chat_event(
        state,
        "intervention.acknowledged",
        "intervention",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let payload = InterventionAction {
        id: id.to_string(),
        state: "acknowledged".to_string(),
    };
    broadcast_chat_ws_event(
        state,
        WS_EVENT_INTERVENTIONS_UPDATED,
        serde_json::to_value(&payload).unwrap_or_else(|_| serde_json::json!({ "id": id })),
    );
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
    let updated = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    sync_assistant_proposal_thread(state, &updated, "resolved", "resolved", None).await?;
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
    broadcast_chat_ws_event(
        state,
        WS_EVENT_INTERVENTIONS_UPDATED,
        serde_json::to_value(&payload).unwrap_or_else(|_| serde_json::json!({ "id": id })),
    );
    Ok(payload)
}

pub(crate) async fn reactivate_intervention(
    state: &AppState,
    id: &str,
) -> Result<InterventionAction, AppError> {
    let id = id.trim();
    let _ = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    state.storage.reactivate_intervention(id).await?;
    let updated = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    sync_assistant_proposal_thread(state, &updated, "open", "reactivated", None).await?;
    emit_chat_event(
        state,
        "intervention.reactivated",
        "intervention",
        id,
        serde_json::json!({ "id": id }),
    )
    .await;
    let payload = InterventionAction {
        id: id.to_string(),
        state: "active".to_string(),
    };
    broadcast_chat_ws_event(
        state,
        WS_EVENT_INTERVENTIONS_UPDATED,
        serde_json::to_value(&payload).unwrap_or_else(|_| serde_json::json!({ "id": id })),
    );
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
    let updated = state
        .storage
        .get_intervention(id)
        .await?
        .ok_or_else(|| AppError::not_found("intervention not found"))?;
    sync_assistant_proposal_thread(state, &updated, "dismissed", "dismissed", None).await?;
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
    broadcast_chat_ws_event(
        state,
        WS_EVENT_INTERVENTIONS_UPDATED,
        serde_json::to_value(&payload).unwrap_or_else(|_| serde_json::json!({ "id": id })),
    );
    Ok(payload)
}
