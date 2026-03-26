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

pub fn normalize_semantic_label(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("_")
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableVoiceContinuitySummaryPacket {
    pub headline: Option<String>,
    pub detail: Option<String>,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableVoiceOfflineResponsePacket {
    pub summary: Option<String>,
    pub detail: Option<String>,
    pub history_status: String,
    pub error_prefix: String,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableVoiceCachedQueryResponsePacket {
    pub summary: Option<String>,
    pub detail: Option<String>,
    pub ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableTaskDisplayNormalizationPacket {
    pub tags: Vec<String>,
    pub project: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableClientKindLabelPacket {
    pub short_label: Option<String>,
}

pub fn normalize_positive_minutes(value: Option<i64>) -> Option<i64> {
    value.map(|value| value.max(1))
}

pub fn normalize_task_display_packet(
    tags: Option<Vec<String>>,
    project: Option<String>,
) -> PortableTaskDisplayNormalizationPacket {
    let project = normalized_optional_trimmed(project);
    let project_key = project.as_ref().map(|value| value.to_lowercase());
    let mut normalized_tags: Vec<String> = Vec::new();

    for tag in tags.unwrap_or_default() {
        let trimmed = trim_text(&tag);
        if trimmed.is_empty() {
            continue;
        }
        let normalized = trimmed.to_lowercase();
        if project_key
            .as_ref()
            .is_some_and(|project_key| project_key == &normalized)
        {
            continue;
        }
        if normalized_tags
            .iter()
            .any(|existing| existing.to_lowercase() == normalized)
        {
            continue;
        }
        normalized_tags.push(trimmed);
    }

    PortableTaskDisplayNormalizationPacket {
        tags: normalized_tags,
        project,
    }
}

pub fn short_client_kind_label_packet(client_kind: Option<String>) -> PortableClientKindLabelPacket {
    let Some(client_kind) = normalized_optional_trimmed(client_kind) else {
        return PortableClientKindLabelPacket { short_label: None };
    };

    let normalized = client_kind.to_lowercase();
    let short_label = if normalized.contains("web") {
        Some("Web".to_string())
    } else if normalized.contains("mac") {
        Some("macOS".to_string())
    } else if normalized.contains("ios")
        || normalized.contains("iphone")
        || normalized.contains("ipad")
    {
        Some("iOS".to_string())
    } else if normalized.contains("watch") {
        Some("watchOS".to_string())
    } else if normalized.contains("veld")
        || normalized.contains("daemon")
        || normalized.contains("server")
    {
        Some("Authority".to_string())
    } else {
        Some(client_kind)
    };

    PortableClientKindLabelPacket { short_label }
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

pub fn prepare_voice_capture_payload(transcript: &str, intent_storage_token: &str) -> String {
    [
        "voice_transcript:".to_string(),
        trim_text(transcript),
        String::new(),
        format!(
            "intent_candidate: {}",
            normalize_payload(intent_storage_token)
        ),
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

pub fn prepare_voice_continuity_summary_packet(
    draft_exists: Option<bool>,
    threaded_transcript: Option<String>,
    pending_recovery_count: Option<i64>,
    is_reachable: Option<bool>,
    merged_transcript: Option<String>,
) -> PortableVoiceContinuitySummaryPacket {
    if draft_exists.unwrap_or(false) {
        return PortableVoiceContinuitySummaryPacket {
            headline: Some("Voice draft ready to resume.".to_string()),
            detail: Some("Your latest local transcript is still on device and can be resumed without reopening a separate thread.".to_string()),
            ready: true,
        };
    }

    if let Some(threaded) = normalized_optional_trimmed(threaded_transcript) {
        return PortableVoiceContinuitySummaryPacket {
            headline: Some("Voice follow-up saved in Threads.".to_string()),
            detail: Some(threaded),
            ready: true,
        };
    }

    if pending_recovery_count.unwrap_or(0) > 0 {
        let count = pending_recovery_count.unwrap_or(0);
        let detail = if is_reachable.unwrap_or(false) {
            "Local voice recovery is waiting on canonical replay.".to_string()
        } else {
            format!(
                "Reconnect to merge {count} local voice entr{} back into canonical state.",
                if count == 1 { "y" } else { "ies" }
            )
        };
        return PortableVoiceContinuitySummaryPacket {
            headline: Some("Voice recovery pending.".to_string()),
            detail: Some(detail),
            ready: true,
        };
    }

    if let Some(merged) = normalized_optional_trimmed(merged_transcript) {
        return PortableVoiceContinuitySummaryPacket {
            headline: Some("Local voice recovery merged.".to_string()),
            detail: Some(merged),
            ready: true,
        };
    }

    PortableVoiceContinuitySummaryPacket {
        headline: None,
        detail: None,
        ready: false,
    }
}

pub fn prepare_voice_offline_response_packet(
    scenario: &str,
    primary_text: Option<String>,
    matched_text: Option<String>,
    options: Option<String>,
    minutes: Option<i64>,
    is_reachable: Option<bool>,
) -> PortableVoiceOfflineResponsePacket {
    let primary_text = normalized_optional_trimmed(primary_text);
    let matched_text = normalized_optional_trimmed(matched_text);
    let options = normalized_optional_trimmed(options);
    let minutes = minutes.unwrap_or(10).max(1);
    let is_reachable = is_reachable.unwrap_or(false);

    match trim_text(scenario).as_str() {
        "capture_shell" => PortableVoiceOfflineResponsePacket {
            summary: Some(if is_reachable {
                "Saved voice capture.".to_string()
            } else {
                "Voice capture queued for sync.".to_string()
            }),
            detail: primary_text,
            history_status: if is_reachable { "submitted" } else { "queued" }.to_string(),
            error_prefix: if is_reachable {
                String::new()
            } else {
                "Voice transcript queued for sync.".to_string()
            },
            ready: true,
        },
        "commitment_create_shell" => PortableVoiceOfflineResponsePacket {
            summary: Some(if is_reachable {
                "Created commitment.".to_string()
            } else {
                "Commitment queued for sync.".to_string()
            }),
            detail: primary_text,
            history_status: if is_reachable { "submitted" } else { "queued" }.to_string(),
            error_prefix: if is_reachable {
                String::new()
            } else {
                "Commitment request queued for sync.".to_string()
            },
            ready: true,
        },
        "backend_required_shell" => PortableVoiceOfflineResponsePacket {
            summary: Some("This voice action now requires the backend Apple route.".to_string()),
            detail: Some("Reconnect to Vel so the server can interpret and answer it.".to_string()),
            history_status: "backend_required".to_string(),
            error_prefix: "Transcript capture was preserved, but the action needs the backend-owned Apple route.".to_string(),
            ready: true,
        },
        "capture_offline" => PortableVoiceOfflineResponsePacket {
            summary: Some("Voice capture queued for sync.".to_string()),
            detail: primary_text,
            history_status: "queued".to_string(),
            error_prefix: "Transcript capture queued for sync.".to_string(),
            ready: true,
        },
        "commitment_target_missing" => PortableVoiceOfflineResponsePacket {
            summary: Some("Commitment target is missing.".to_string()),
            detail: Some("Try phrasing like \"mark meds done.\"".to_string()),
            history_status: "needs_clarification".to_string(),
            error_prefix: "Commitment target missing.".to_string(),
            ready: true,
        },
        "commitment_no_match" => PortableVoiceOfflineResponsePacket {
            summary: Some("No open commitment matched.".to_string()),
            detail: Some("Transcript capture was queued for sync.".to_string()),
            history_status: "capture_only".to_string(),
            error_prefix: "No local commitment match for offline queueing.".to_string(),
            ready: true,
        },
        "commitment_ambiguous" => PortableVoiceOfflineResponsePacket {
            summary: Some("Ambiguous commitment target.".to_string()),
            detail: options.map(|value| format!("Could match: {value}")),
            history_status: "needs_clarification".to_string(),
            error_prefix: "Commitment target was ambiguous.".to_string(),
            ready: true,
        },
        "commitment_done_queued" => PortableVoiceOfflineResponsePacket {
            summary: Some("Commitment completion queued.".to_string()),
            detail: matched_text,
            history_status: "queued".to_string(),
            error_prefix: "Commitment completion queued for backend replay.".to_string(),
            ready: true,
        },
        "nudge_missing" => PortableVoiceOfflineResponsePacket {
            summary: Some("No active nudge found.".to_string()),
            detail: Some("Transcript capture was queued for sync.".to_string()),
            history_status: "capture_only".to_string(),
            error_prefix: "No active nudge available for offline queueing.".to_string(),
            ready: true,
        },
        "nudge_done_queued" => PortableVoiceOfflineResponsePacket {
            summary: Some("Top nudge resolution queued.".to_string()),
            detail: None,
            history_status: "queued".to_string(),
            error_prefix: "Top nudge resolution queued for backend replay.".to_string(),
            ready: true,
        },
        "nudge_snooze_queued" => PortableVoiceOfflineResponsePacket {
            summary: Some("Top nudge snooze queued.".to_string()),
            detail: Some(format!("{minutes} minutes")),
            history_status: "queued".to_string(),
            error_prefix: "Top nudge snooze queued for backend replay.".to_string(),
            ready: true,
        },
        "backend_required_offline" => PortableVoiceOfflineResponsePacket {
            summary: Some("Unavailable offline.".to_string()),
            detail: Some(
                "This reply is backend-owned and is not synthesized from local Swift cache."
                    .to_string(),
            ),
            history_status: "backend_required".to_string(),
            error_prefix:
                "Transcript capture queued, but this voice reply requires the backend route."
                    .to_string(),
            ready: true,
        },
        _ => PortableVoiceOfflineResponsePacket {
            summary: None,
            detail: None,
            history_status: "capture_only".to_string(),
            error_prefix: String::new(),
            ready: false,
        },
    }
}

pub fn prepare_voice_cached_query_response_packet(
    scenario: &str,
    next_title: Option<String>,
    leave_by: Option<String>,
    empty_message: Option<String>,
    cached_now_summary: Option<String>,
    first_reason: Option<String>,
    next_commitment_text: Option<String>,
    next_commitment_due_at: Option<String>,
    behavior_headline: Option<String>,
    behavior_reason: Option<String>,
) -> PortableVoiceCachedQueryResponsePacket {
    let next_title = normalized_optional_trimmed(next_title);
    let leave_by = normalized_optional_trimmed(leave_by);
    let empty_message = normalized_optional_trimmed(empty_message);
    let cached_now_summary = normalized_optional_trimmed(cached_now_summary);
    let first_reason = normalized_optional_trimmed(first_reason);
    let next_commitment_text = normalized_optional_trimmed(next_commitment_text);
    let next_commitment_due_at = normalized_optional_trimmed(next_commitment_due_at);
    let behavior_headline = normalized_optional_trimmed(behavior_headline);
    let behavior_reason = normalized_optional_trimmed(behavior_reason);

    match trim_text(scenario).as_str() {
        "schedule_with_event" => PortableVoiceCachedQueryResponsePacket {
            summary: next_title.map(|title| format!("Next event: {title}.")),
            detail: leave_by.or(cached_now_summary).or(empty_message),
            ready: true,
        },
        "schedule_empty" => PortableVoiceCachedQueryResponsePacket {
            summary: Some(
                empty_message.unwrap_or_else(|| "No upcoming schedule is cached.".to_string()),
            ),
            detail: cached_now_summary.or(first_reason),
            ready: true,
        },
        "next_commitment" => PortableVoiceCachedQueryResponsePacket {
            summary: next_commitment_text.map(|text| format!("Next commitment: {text}.")),
            detail: next_commitment_due_at.or(cached_now_summary),
            ready: true,
        },
        "next_commitment_empty" => PortableVoiceCachedQueryResponsePacket {
            summary: Some("No next commitment is cached.".to_string()),
            detail: cached_now_summary.or(empty_message),
            ready: true,
        },
        "behavior_cached" => PortableVoiceCachedQueryResponsePacket {
            summary: behavior_headline,
            detail: behavior_reason,
            ready: true,
        },
        "backend_unavailable" => PortableVoiceCachedQueryResponsePacket {
            summary: Some("Unavailable offline.".to_string()),
            detail: Some("Reconnect to fetch a backend-owned reply.".to_string()),
            ready: true,
        },
        "cached_now_missing" => PortableVoiceCachedQueryResponsePacket {
            summary: Some("Unavailable offline.".to_string()),
            detail: Some("No cached backend /v1/now payload is available yet.".to_string()),
            ready: true,
        },
        "behavior_missing" => PortableVoiceCachedQueryResponsePacket {
            summary: Some("Unavailable offline.".to_string()),
            detail: Some("No cached backend behavior summary is available yet.".to_string()),
            ready: true,
        },
        _ => PortableVoiceCachedQueryResponsePacket {
            summary: None,
            detail: None,
            ready: false,
        },
    }
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
