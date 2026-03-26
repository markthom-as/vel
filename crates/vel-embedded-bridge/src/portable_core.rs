//! Portable transform helpers for `vel-embedded-bridge`.
//!
//! This module is the first extraction point for logic that should be reusable
//! across native Apple FFI and a future browser/WASM adapter.

pub fn normalize_domain_hint(value: String) -> String {
    value
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn normalize_payload(value: &str) -> String {
    value
        .trim()
        .replace('\n', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn prepare_quick_capture_text(value: &str) -> String {
    value
        .lines()
        .flat_map(|line| line.split_whitespace())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

pub fn trim_text(value: &str) -> String {
    value.trim().to_string()
}

pub fn normalized_optional_trimmed(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

pub fn normalize_pairing_token_input(value: &str) -> String {
    let normalized: String = value
        .to_uppercase()
        .chars()
        .filter(|character| character.is_ascii() && character.is_ascii_alphanumeric())
        .take(6)
        .collect();

    if normalized.len() <= 3 {
        return normalized;
    }

    format!("{}-{}", &normalized[..3], &normalized[3..])
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableQueuedActionPacket {
    pub queue_kind: String,
    pub target_id: Option<String>,
    pub text: Option<String>,
    pub minutes: Option<i64>,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableVoiceQuickActionPacket {
    pub queue_kind: String,
    pub target_id: Option<String>,
    pub text: Option<String>,
    pub minutes: Option<i64>,
    pub ready: bool,
}

pub fn normalize_positive_minutes(value: Option<i64>) -> Option<i64> {
    value.map(|value| value.max(1))
}

pub fn prepare_voice_quick_action_packet(
    intent_storage_token: &str,
    primary_text: &str,
    target_id: Option<String>,
    minutes: Option<i64>,
) -> PortableVoiceQuickActionPacket {
    if intent_storage_token == "capture_create" {
        return PortableVoiceQuickActionPacket {
            queue_kind: "capture.create".to_string(),
            target_id: None,
            text: Some(normalize_payload(primary_text)),
            minutes: None,
            ready: true,
        };
    }

    if intent_storage_token == "commitment_create" {
        return PortableVoiceQuickActionPacket {
            queue_kind: "commitment.create".to_string(),
            target_id: None,
            text: Some(normalize_payload(primary_text)),
            minutes: None,
            ready: true,
        };
    }

    if intent_storage_token == "commitment_done" {
        return PortableVoiceQuickActionPacket {
            queue_kind: "commitment.done".to_string(),
            target_id,
            text: None,
            minutes: None,
            ready: true,
        };
    }

    if intent_storage_token == "nudge_done" {
        return PortableVoiceQuickActionPacket {
            queue_kind: "nudge.done".to_string(),
            target_id,
            text: None,
            minutes: None,
            ready: true,
        };
    }

    if intent_storage_token.starts_with("nudge_snooze_") {
        return PortableVoiceQuickActionPacket {
            queue_kind: "nudge.snooze".to_string(),
            target_id,
            text: None,
            minutes,
            ready: true,
        };
    }

    PortableVoiceQuickActionPacket {
        queue_kind: "capture.create".to_string(),
        target_id: None,
        text: Some(normalize_payload(primary_text)),
        minutes: None,
        ready: false,
    }
}

pub fn prepare_queued_action_packet(
    kind: String,
    target_id: Option<String>,
    text: Option<String>,
    minutes: Option<i64>,
) -> PortableQueuedActionPacket {
    let ready = matches!(
        kind.as_str(),
        "capture.create" | "commitment.create" | "commitment.done" | "nudge.done" | "nudge.snooze"
    );

    PortableQueuedActionPacket {
        queue_kind: if ready {
            kind
        } else {
            "capture.create".to_string()
        },
        target_id,
        text,
        minutes,
        ready,
    }
}

pub fn prepare_assistant_entry_fallback_payload(
    text: &str,
    requested_conversation_id: Option<String>,
) -> String {
    [
        "queued_assistant_entry:".to_string(),
        requested_conversation_id
            .map(|value| format!("requested_conversation_id: {value}"))
            .unwrap_or_default(),
        String::new(),
        trim_text(text),
    ]
    .into_iter()
    .filter(|value| !value.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn prepare_capture_metadata_payload(
    text: &str,
    capture_type: &str,
    source_device: &str,
) -> String {
    let text = trim_text(text);
    let capture_type = trim_text(capture_type);
    let source_device = trim_text(source_device);

    if capture_type == "note" && source_device == "apple" {
        text
    } else {
        [
            "queued_capture_metadata:".to_string(),
            format!("requested_capture_type: {capture_type}"),
            format!("requested_source_device: {source_device}"),
            String::new(),
            text,
        ]
        .join("\n")
    }
}
