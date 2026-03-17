use vel_api_types::IntegrationLogEventData;
use vel_storage::EventLogRecord;

const INTEGRATION_LOG_LIMIT_DEFAULT: u32 = 10;

pub(crate) fn integration_log_limit(limit: Option<u32>) -> u32 {
    limit.unwrap_or(INTEGRATION_LOG_LIMIT_DEFAULT)
}

pub(crate) fn canonical_integration_id(integration_id: &str) -> Option<&'static str> {
    match integration_id {
        "google-calendar" | "calendar" | "google_calendar" => Some("google-calendar"),
        "todoist" => Some("todoist"),
        "activity" => Some("activity"),
        "git" => Some("git"),
        "messaging" => Some("messaging"),
        "notes" => Some("notes"),
        "transcripts" => Some("transcripts"),
        _ => None,
    }
}

pub(crate) fn map_integration_log_event(
    event: EventLogRecord,
    fallback_integration_id: &str,
) -> IntegrationLogEventData {
    let payload =
        serde_json::from_str(&event.payload_json).unwrap_or_else(|_| serde_json::json!({}));
    let event_name = event.event_name;
    let status = payload
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| integration_log_status(&event_name))
        .to_string();

    IntegrationLogEventData {
        id: event.id.to_string(),
        integration_id: event
            .aggregate_id
            .unwrap_or_else(|| fallback_integration_id.to_string()),
        event_name: event_name.clone(),
        status,
        message: integration_log_message(&event_name, &payload),
        payload,
        created_at: event.created_at,
    }
}

fn integration_log_status(event_name: &str) -> &'static str {
    match event_name {
        "integration.sync.succeeded" => "ok",
        "integration.sync.failed" => "error",
        _ => "info",
    }
}

fn integration_log_message(event_name: &str, payload: &serde_json::Value) -> String {
    let item_count = payload
        .get("item_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let integration_id = payload
        .get("integration_id")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("integration");

    match event_name {
        "integration.sync.succeeded" => {
            format!("{integration_id} sync completed with {item_count} items.")
        }
        "integration.sync.failed" => {
            let error = payload
                .get("error")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown error");
            format!("{integration_id} sync failed: {error}")
        }
        _ => event_name.to_string(),
    }
}
