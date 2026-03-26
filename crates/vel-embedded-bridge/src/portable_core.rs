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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableRemoteRoute {
    pub label: String,
    pub base_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableThreadDraftPacket {
    pub payload: String,
    pub requested_conversation_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableLinkingRequestPacket {
    pub token_code: Option<String>,
    pub target_base_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableLinkingFeedbackPacket {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableAppShellFeedbackPacket {
    pub message: String,
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

pub fn prepare_thread_draft_packet(
    text: &str,
    requested_conversation_id: Option<String>,
) -> PortableThreadDraftPacket {
    PortableThreadDraftPacket {
        payload: normalize_payload(text),
        requested_conversation_id: normalized_optional_trimmed(requested_conversation_id),
    }
}

pub fn prepare_voice_capture_payload(
    transcript: &str,
    intent_storage_token: &str,
) -> String {
    [
        "voice_transcript:".to_string(),
        trim_text(transcript),
        String::new(),
        format!("intent_candidate: {}", normalize_payload(intent_storage_token)),
        "client_surface: ios_voice".to_string(),
    ]
    .join("\n")
}

pub fn prepare_linking_request_packet(
    token_code: Option<String>,
    target_base_url: Option<String>,
) -> PortableLinkingRequestPacket {
    PortableLinkingRequestPacket {
        token_code: normalized_optional_trimmed(token_code),
        target_base_url: normalized_optional_trimmed(target_base_url),
    }
}

pub fn prepare_linking_feedback_packet(
    scenario: &str,
    node_display_name: Option<String>,
) -> Option<PortableLinkingFeedbackPacket> {
    let message = match scenario {
        "issue_without_target" => "Pair nodes code created.".to_string(),
        "issue_with_target" => format!(
            "Pair nodes code created. {} has been prompted to enter it on that client.",
            node_display_name.unwrap_or_else(|| "Remote client".to_string())
        ),
        "redeem_empty_token" => "Enter the pairing token shown on the issuing node.".to_string(),
        "redeem_success" => format!(
            "Linked as {}. The link has been saved locally and the issuing client has been notified.",
            node_display_name.unwrap_or_else(|| "linked node".to_string())
        ),
        "renegotiate_success" => format!(
            "Pair nodes code created for {}. That client has been prompted to approve the new access.",
            node_display_name.unwrap_or_else(|| "linked node".to_string())
        ),
        "unpair_success" => format!(
            "Unpaired {}.",
            node_display_name.unwrap_or_else(|| "linked node".to_string())
        ),
        _ => return None,
    };

    Some(PortableLinkingFeedbackPacket { message })
}

pub fn prepare_app_shell_feedback_packet(
    scenario: &str,
    detail: Option<String>,
) -> Option<PortableAppShellFeedbackPacket> {
    let message = match scenario {
        "offline_cache_in_use" => match normalized_optional_trimmed(detail) {
            Some(value) => format!("Offline cache in use. {value}"),
            None => "Offline cache in use.".to_string(),
        },
        "no_reachable_endpoint" => {
            "No reachable Vel endpoint. Configure vel_tailscale_url or vel_base_url.".to_string()
        }
        "refresh_signals_failed" => match normalized_optional_trimmed(detail) {
            Some(value) => format!("Could not refresh activity feed. {value}"),
            None => "Could not refresh activity feed.".to_string(),
        },
        "queued_nudge_done" => "Queued nudge completion for sync.".to_string(),
        "queued_nudge_snooze" => "Queued nudge snooze for sync.".to_string(),
        "queued_commitment_done" => "Queued commitment completion for sync.".to_string(),
        "queued_commitment_create" => "Queued commitment for sync.".to_string(),
        "queued_capture_create" => "Queued capture for sync.".to_string(),
        "assistant_entry_queued" => "Assistant message queued for sync.".to_string(),
        _ => return None,
    };

    Some(PortableAppShellFeedbackPacket { message })
}

pub fn collect_remote_routes(
    sync_base_url: Option<String>,
    tailscale_base_url: Option<String>,
    lan_base_url: Option<String>,
    public_base_url: Option<String>,
) -> Vec<PortableRemoteRoute> {
    let entries = [
        ("primary", sync_base_url),
        ("tailscale", tailscale_base_url),
        ("lan", lan_base_url),
        ("public", public_base_url),
    ];

    let mut seen: Vec<String> = Vec::new();
    let mut routes: Vec<PortableRemoteRoute> = Vec::new();

    for (label, value) in entries {
        let Some(trimmed) = normalized_optional_trimmed(value) else {
            continue;
        };
        if trimmed.contains("127.0.0.1")
            || trimmed.contains("localhost")
            || seen.iter().any(|existing| existing == &trimmed)
        {
            continue;
        }
        seen.push(trimmed.clone());
        routes.push(PortableRemoteRoute {
            label: label.to_string(),
            base_url: trimmed,
        });
    }

    routes
}
