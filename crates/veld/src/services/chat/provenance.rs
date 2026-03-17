use vel_api_types::MessageData;
use vel_core::normalize_risk_level;

pub(crate) fn build_linked_objects(
    message: &MessageData,
    interventions: &[vel_storage::InterventionRecord],
) -> Vec<serde_json::Value> {
    let mut linked_objects = vec![serde_json::json!({
        "kind": "message",
        "id": message.id,
        "conversation_id": message.conversation_id,
        "role": message.role,
        "message_kind": message.kind,
    })];

    for intervention in interventions {
        let mut object = serde_json::json!({
            "kind": "intervention",
            "id": intervention.id.as_ref(),
            "message_id": intervention.message_id.as_ref(),
            "intervention_kind": intervention.kind,
            "state": intervention.state,
            "surfaced_at": intervention.surfaced_at,
            "snoozed_until": intervention.snoozed_until,
            "confidence": intervention.confidence,
        });
        if let Some(source) = parse_optional_json_str(intervention.source_json.as_deref()) {
            object["source"] = source;
        }
        if let Some(provenance) = parse_optional_json_str(intervention.provenance_json.as_deref()) {
            object["provenance"] = provenance;
        }
        linked_objects.push(object);
    }

    linked_objects
}

pub(crate) fn build_provenance_signals(
    message: &MessageData,
    interventions: &[vel_storage::InterventionRecord],
) -> Vec<serde_json::Value> {
    let mut signals = Vec::new();

    if let Some(signal) = message_signal_summary(message) {
        signals.push(signal);
    }

    for intervention in interventions {
        if let Some(source) = parse_optional_json_str(intervention.source_json.as_deref()) {
            signals.push(serde_json::json!({
                "kind": "intervention_source",
                "intervention_id": intervention.id.as_ref(),
                "intervention_kind": intervention.kind,
                "payload": source,
            }));
        }
        if let Some(provenance) = parse_optional_json_str(intervention.provenance_json.as_deref()) {
            signals.push(serde_json::json!({
                "kind": "intervention_provenance",
                "intervention_id": intervention.id.as_ref(),
                "payload": provenance,
            }));
        }
    }

    signals
}

pub(crate) fn build_policy_decisions(
    message: &MessageData,
    interventions: &[vel_storage::InterventionRecord],
) -> Vec<serde_json::Value> {
    let mut policy_decisions = Vec::new();

    if let Some(decision) = message_policy_summary(message) {
        policy_decisions.push(decision);
    }

    for intervention in interventions {
        policy_decisions.push(serde_json::json!({
            "kind": "intervention_state",
            "intervention_id": intervention.id.as_ref(),
            "intervention_kind": intervention.kind,
            "state": intervention.state,
            "confidence": intervention.confidence,
        }));
    }

    policy_decisions
}

pub(crate) fn message_signal_summary(message: &MessageData) -> Option<serde_json::Value> {
    match message.kind.as_str() {
        "reminder_card" => Some(serde_json::json!({
            "kind": "message_content",
            "message_kind": message.kind,
            "title": message.content.get("title").and_then(|value| value.as_str()),
            "reason": message.content.get("reason").and_then(|value| value.as_str()),
            "confidence": message.content.get("confidence").and_then(|value| value.as_f64()),
        })),
        "risk_card" => Some(serde_json::json!({
            "kind": "message_content",
            "message_kind": message.kind,
            "commitment_title": message.content.get("commitment_title").and_then(|value| value.as_str()),
            "risk_level": message
                .content
                .get("risk_level")
                .and_then(|value| value.as_str())
                .map(normalize_risk_level),
            "top_drivers": message.content.get("top_drivers").cloned().unwrap_or(serde_json::Value::Null),
            "proposed_next_step": message.content.get("proposed_next_step").and_then(|value| value.as_str()),
        })),
        "suggestion_card" => Some(serde_json::json!({
            "kind": "message_content",
            "message_kind": message.kind,
            "suggestion_text": message.content.get("suggestion_text").and_then(|value| value.as_str()),
            "expected_benefit": message.content.get("expected_benefit").and_then(|value| value.as_str()),
            "linked_goal": message.content.get("linked_goal").and_then(|value| value.as_str()),
        })),
        _ => None,
    }
}

pub(crate) fn message_policy_summary(message: &MessageData) -> Option<serde_json::Value> {
    match message.kind.as_str() {
        "reminder_card" => Some(serde_json::json!({
            "kind": "message_policy",
            "message_kind": message.kind,
            "reason": message.content.get("reason").and_then(|value| value.as_str()),
            "confidence": message.content.get("confidence").and_then(|value| value.as_f64()),
        })),
        "risk_card" => Some(serde_json::json!({
            "kind": "message_policy",
            "message_kind": message.kind,
            "risk_level": message
                .content
                .get("risk_level")
                .and_then(|value| value.as_str())
                .map(normalize_risk_level),
            "top_drivers": message.content.get("top_drivers").cloned().unwrap_or(serde_json::Value::Null),
            "proposed_next_step": message.content.get("proposed_next_step").and_then(|value| value.as_str()),
        })),
        "suggestion_card" => Some(serde_json::json!({
            "kind": "message_policy",
            "message_kind": message.kind,
            "expected_benefit": message.content.get("expected_benefit").and_then(|value| value.as_str()),
            "linked_goal": message.content.get("linked_goal").and_then(|value| value.as_str()),
        })),
        _ => None,
    }
}

fn parse_optional_json_str(value: Option<&str>) -> Option<serde_json::Value> {
    value.and_then(|raw| serde_json::from_str::<serde_json::Value>(raw).ok())
}
