//! Browser/WASM scaffold for `vel-embedded-bridge`.
//!
//! This file exposes the browser adapter seam where portable packet-shaping
//! logic from `portable_core` can be surfaced to JS/WASM callers without the
//! native Apple FFI loader path.

use crate::portable_core::{
    collect_remote_routes, normalize_domain_hint, normalize_pairing_token_input,
    normalize_positive_minutes, normalize_semantic_label, normalized_optional_trimmed,
    prepare_app_shell_feedback_packet, prepare_assistant_entry_fallback_payload,
    prepare_capture_metadata_payload, prepare_linking_feedback_packet,
    prepare_linking_request_packet, prepare_queued_action_packet, prepare_thread_draft_packet,
    prepare_voice_cached_query_response_packet, prepare_voice_capture_payload,
    prepare_voice_continuity_summary_packet, prepare_voice_offline_response_packet,
    prepare_voice_quick_action_packet, trim_text,
};
#[cfg(feature = "browser-wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserPacketResponse {
    pub kind: &'static str,
    pub payload_json: String,
}

pub struct BrowserWasmScaffold;

impl BrowserWasmScaffold {
    pub const STATUS: &'static str = "scaffold_only";

    pub fn intended_packet_families() -> &'static [&'static str] {
        &[
            "cached_now_hydration",
            "offline_request_packaging",
            "deterministic_domain_helpers",
            "thread_draft_packaging",
            "voice_capture_packaging",
            "voice_quick_action_packaging",
            "voice_continuity_packaging",
            "queued_action_packaging",
            "linking_settings_normalization",
            "assistant_entry_fallback_packaging",
            "linking_request_packaging",
            "capture_metadata_packaging",
            "voice_continuity_summary_packaging",
            "voice_offline_response_packaging",
            "voice_cached_query_packaging",
            "linking_feedback_packaging",
            "app_shell_feedback_packaging",
        ]
    }

    pub fn normalize_pairing_token_packet(input: &str) -> BrowserPacketResponse {
        BrowserPacketResponse {
            kind: "linking_settings_normalization",
            payload_json: format!(
                "{{\"tokenCode\":\"{}\"}}",
                normalize_pairing_token_input(input)
            ),
        }
    }

    pub fn normalize_domain_hint_packet(input: &str) -> BrowserPacketResponse {
        BrowserPacketResponse {
            kind: "deterministic_domain_helpers",
            payload_json: format!(
                "{{\"normalized\":\"{}\"}}",
                normalize_domain_hint(input.to_string())
            ),
        }
    }

    pub fn normalize_semantic_label_packet(input: &str) -> BrowserPacketResponse {
        BrowserPacketResponse {
            kind: "deterministic_domain_helpers",
            payload_json: format!("{{\"normalized\":\"{}\"}}", normalize_semantic_label(input)),
        }
    }

    pub fn queued_action_packet(
        kind: String,
        target_id: Option<String>,
        text: Option<String>,
        minutes: Option<i64>,
    ) -> BrowserPacketResponse {
        let packet = prepare_queued_action_packet(
            trim_text(&kind),
            normalized_optional_trimmed(target_id),
            normalized_optional_trimmed(text),
            normalize_positive_minutes(minutes),
        );

        BrowserPacketResponse {
            kind: "queued_action_packaging",
            payload_json: format!(
                "{{\"queueKind\":\"{}\",\"targetId\":{},\"text\":{},\"minutes\":{},\"ready\":{}}}",
                packet.queue_kind,
                option_json(packet.target_id),
                option_json(packet.text),
                option_number_json(packet.minutes),
                bool_json(packet.ready)
            ),
        }
    }

    pub fn voice_quick_action_packet(
        intent_storage_token: String,
        primary_text: String,
        target_id: Option<String>,
        minutes: Option<i64>,
    ) -> BrowserPacketResponse {
        let packet = prepare_voice_quick_action_packet(
            &trim_text(&intent_storage_token),
            &primary_text,
            normalized_optional_trimmed(target_id),
            normalize_positive_minutes(minutes),
        );

        BrowserPacketResponse {
            kind: "voice_quick_action_packaging",
            payload_json: format!(
                "{{\"queueKind\":\"{}\",\"targetId\":{},\"text\":{},\"minutes\":{},\"ready\":{}}}",
                packet.queue_kind,
                option_json(packet.target_id),
                option_json(packet.text),
                option_number_json(packet.minutes),
                bool_json(packet.ready)
            ),
        }
    }

    pub fn assistant_entry_fallback_packet(
        text: String,
        requested_conversation_id: Option<String>,
    ) -> BrowserPacketResponse {
        let payload = prepare_assistant_entry_fallback_payload(
            &text,
            normalized_optional_trimmed(requested_conversation_id),
        );

        BrowserPacketResponse {
            kind: "assistant_entry_fallback_packaging",
            payload_json: format!("{{\"payload\":{}}}", string_json(&payload)),
        }
    }

    pub fn capture_metadata_packet(
        text: String,
        capture_type: String,
        source_device: String,
    ) -> BrowserPacketResponse {
        let payload = prepare_capture_metadata_payload(&text, &capture_type, &source_device);

        BrowserPacketResponse {
            kind: "capture_metadata_packaging",
            payload_json: format!("{{\"payload\":{}}}", string_json(&payload)),
        }
    }

    pub fn thread_draft_packet(
        text: String,
        requested_conversation_id: Option<String>,
    ) -> BrowserPacketResponse {
        let packet = prepare_thread_draft_packet(&text, requested_conversation_id);

        BrowserPacketResponse {
            kind: "thread_draft_packaging",
            payload_json: format!(
                "{{\"payload\":{},\"requestedConversationId\":{}}}",
                string_json(&packet.payload),
                option_json(packet.requested_conversation_id)
            ),
        }
    }

    pub fn voice_capture_packet(
        transcript: String,
        intent_storage_token: String,
    ) -> BrowserPacketResponse {
        let payload = prepare_voice_capture_payload(&transcript, &intent_storage_token);

        BrowserPacketResponse {
            kind: "voice_capture_packaging",
            payload_json: format!("{{\"payload\":{}}}", string_json(&payload)),
        }
    }

    pub fn linking_request_packet(
        token_code: Option<String>,
        target_base_url: Option<String>,
    ) -> BrowserPacketResponse {
        let packet = prepare_linking_request_packet(token_code, target_base_url);

        BrowserPacketResponse {
            kind: "linking_request_packaging",
            payload_json: format!(
                "{{\"tokenCode\":{},\"targetBaseUrl\":{}}}",
                option_json(packet.token_code),
                option_json(packet.target_base_url)
            ),
        }
    }

    pub fn linking_feedback_packet(
        scenario: String,
        node_display_name: Option<String>,
    ) -> BrowserPacketResponse {
        let packet = prepare_linking_feedback_packet(&scenario, node_display_name)
            .unwrap_or_else(|| panic!("unsupported linking feedback scenario"));

        BrowserPacketResponse {
            kind: "linking_feedback_packaging",
            payload_json: format!("{{\"message\":{}}}", string_json(&packet.message)),
        }
    }

    pub fn app_shell_feedback_packet(
        scenario: String,
        detail: Option<String>,
    ) -> BrowserPacketResponse {
        let packet = prepare_app_shell_feedback_packet(&scenario, detail)
            .unwrap_or_else(|| panic!("unsupported app shell feedback scenario"));

        BrowserPacketResponse {
            kind: "app_shell_feedback_packaging",
            payload_json: format!("{{\"message\":{}}}", string_json(&packet.message)),
        }
    }

    pub fn voice_continuity_summary_packet(
        draft_exists: Option<bool>,
        threaded_transcript: Option<String>,
        pending_recovery_count: Option<i64>,
        is_reachable: Option<bool>,
        merged_transcript: Option<String>,
    ) -> BrowserPacketResponse {
        let packet = prepare_voice_continuity_summary_packet(
            draft_exists,
            threaded_transcript,
            pending_recovery_count,
            is_reachable,
            merged_transcript,
        );

        BrowserPacketResponse {
            kind: "voice_continuity_summary_packaging",
            payload_json: format!(
                "{{\"headline\":{},\"detail\":{},\"ready\":{}}}",
                option_json(packet.headline),
                option_json(packet.detail),
                bool_json(packet.ready)
            ),
        }
    }

    pub fn voice_offline_response_packet(
        scenario: String,
        primary_text: Option<String>,
        matched_text: Option<String>,
        options: Option<String>,
        minutes: Option<i64>,
        is_reachable: Option<bool>,
    ) -> BrowserPacketResponse {
        let packet = prepare_voice_offline_response_packet(
            &scenario,
            primary_text,
            matched_text,
            options,
            minutes,
            is_reachable,
        );

        BrowserPacketResponse {
            kind: "voice_offline_response_packaging",
            payload_json: format!(
                "{{\"summary\":{},\"detail\":{},\"historyStatus\":{},\"errorPrefix\":{},\"ready\":{}}}",
                option_json(packet.summary),
                option_json(packet.detail),
                string_json(&packet.history_status),
                string_json(&packet.error_prefix),
                bool_json(packet.ready)
            ),
        }
    }

    pub fn voice_cached_query_response_packet(
        scenario: String,
        next_title: Option<String>,
        leave_by: Option<String>,
        empty_message: Option<String>,
        cached_now_summary: Option<String>,
        first_reason: Option<String>,
        next_commitment_text: Option<String>,
        next_commitment_due_at: Option<String>,
        behavior_headline: Option<String>,
        behavior_reason: Option<String>,
    ) -> BrowserPacketResponse {
        let packet = prepare_voice_cached_query_response_packet(
            &scenario,
            next_title,
            leave_by,
            empty_message,
            cached_now_summary,
            first_reason,
            next_commitment_text,
            next_commitment_due_at,
            behavior_headline,
            behavior_reason,
        );

        BrowserPacketResponse {
            kind: "voice_cached_query_packaging",
            payload_json: format!(
                "{{\"summary\":{},\"detail\":{},\"ready\":{}}}",
                option_json(packet.summary),
                option_json(packet.detail),
                bool_json(packet.ready)
            ),
        }
    }

    pub fn remote_routes_packet(
        sync_base_url: Option<String>,
        tailscale_base_url: Option<String>,
        lan_base_url: Option<String>,
        public_base_url: Option<String>,
    ) -> BrowserPacketResponse {
        let routes = collect_remote_routes(
            sync_base_url,
            tailscale_base_url,
            lan_base_url,
            public_base_url,
        );

        let payload_json = format!(
            "[{}]",
            routes
                .into_iter()
                .map(|route| format!(
                    "{{\"label\":{},\"baseUrl\":{}}}",
                    string_json(&route.label),
                    string_json(&route.base_url)
                ))
                .collect::<Vec<_>>()
                .join(",")
        );

        BrowserPacketResponse {
            kind: "linking_settings_normalization",
            payload_json,
        }
    }
}

fn string_json(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('\"', "\\\"")
            .replace('\n', "\\n")
    )
}

fn option_json(value: Option<String>) -> String {
    value
        .map(|value| string_json(&value))
        .unwrap_or_else(|| "null".to_string())
}

fn option_number_json(value: Option<i64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string())
}

fn bool_json(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedBrowserStatus)]
pub fn vel_embedded_browser_status() -> String {
    BrowserWasmScaffold::STATUS.to_string()
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedNormalizePairingTokenPacket)]
pub fn vel_embedded_normalize_pairing_token_packet(input: String) -> String {
    BrowserWasmScaffold::normalize_pairing_token_packet(&input).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedNormalizeDomainHintPacket)]
pub fn vel_embedded_normalize_domain_hint_packet(input: String) -> String {
    BrowserWasmScaffold::normalize_domain_hint_packet(&input).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedNormalizeSemanticLabelPacket)]
pub fn vel_embedded_normalize_semantic_label_packet(input: String) -> String {
    BrowserWasmScaffold::normalize_semantic_label_packet(&input).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedQueuedActionPacket)]
pub fn vel_embedded_queued_action_packet(
    kind: String,
    target_id: Option<String>,
    text: Option<String>,
    minutes: Option<i64>,
) -> String {
    BrowserWasmScaffold::queued_action_packet(kind, target_id, text, minutes).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedVoiceQuickActionPacket)]
pub fn vel_embedded_voice_quick_action_packet(
    intent_storage_token: String,
    primary_text: String,
    target_id: Option<String>,
    minutes: Option<i64>,
) -> String {
    BrowserWasmScaffold::voice_quick_action_packet(
        intent_storage_token,
        primary_text,
        target_id,
        minutes,
    )
    .payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedAssistantEntryFallbackPacket)]
pub fn vel_embedded_assistant_entry_fallback_packet(
    text: String,
    requested_conversation_id: Option<String>,
) -> String {
    BrowserWasmScaffold::assistant_entry_fallback_packet(text, requested_conversation_id)
        .payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedCaptureMetadataPacket)]
pub fn vel_embedded_capture_metadata_packet(
    text: String,
    capture_type: String,
    source_device: String,
) -> String {
    BrowserWasmScaffold::capture_metadata_packet(text, capture_type, source_device).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedThreadDraftPacket)]
pub fn vel_embedded_thread_draft_packet(
    text: String,
    requested_conversation_id: Option<String>,
) -> String {
    BrowserWasmScaffold::thread_draft_packet(text, requested_conversation_id).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedVoiceCapturePacket)]
pub fn vel_embedded_voice_capture_packet(
    transcript: String,
    intent_storage_token: String,
) -> String {
    BrowserWasmScaffold::voice_capture_packet(transcript, intent_storage_token).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedLinkingRequestPacket)]
pub fn vel_embedded_linking_request_packet(
    token_code: Option<String>,
    target_base_url: Option<String>,
) -> String {
    BrowserWasmScaffold::linking_request_packet(token_code, target_base_url).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedLinkingFeedbackPacket)]
pub fn vel_embedded_linking_feedback_packet(
    scenario: String,
    node_display_name: Option<String>,
) -> String {
    BrowserWasmScaffold::linking_feedback_packet(scenario, node_display_name).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedAppShellFeedbackPacket)]
pub fn vel_embedded_app_shell_feedback_packet(scenario: String, detail: Option<String>) -> String {
    BrowserWasmScaffold::app_shell_feedback_packet(scenario, detail).payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedCollectRemoteRoutesPacket)]
pub fn vel_embedded_collect_remote_routes_packet(
    sync_base_url: Option<String>,
    tailscale_base_url: Option<String>,
    lan_base_url: Option<String>,
    public_base_url: Option<String>,
) -> String {
    BrowserWasmScaffold::remote_routes_packet(
        sync_base_url,
        tailscale_base_url,
        lan_base_url,
        public_base_url,
    )
    .payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedVoiceContinuitySummaryPacket)]
pub fn vel_embedded_voice_continuity_summary_packet(
    draft_exists: Option<bool>,
    threaded_transcript: Option<String>,
    pending_recovery_count: Option<i64>,
    is_reachable: Option<bool>,
    merged_transcript: Option<String>,
) -> String {
    BrowserWasmScaffold::voice_continuity_summary_packet(
        draft_exists,
        threaded_transcript,
        pending_recovery_count,
        is_reachable,
        merged_transcript,
    )
    .payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedVoiceOfflineResponsePacket)]
pub fn vel_embedded_voice_offline_response_packet(
    scenario: String,
    primary_text: Option<String>,
    matched_text: Option<String>,
    options: Option<String>,
    minutes: Option<i64>,
    is_reachable: Option<bool>,
) -> String {
    BrowserWasmScaffold::voice_offline_response_packet(
        scenario,
        primary_text,
        matched_text,
        options,
        minutes,
        is_reachable,
    )
    .payload_json
}

#[cfg(feature = "browser-wasm")]
#[wasm_bindgen(js_name = velEmbeddedVoiceCachedQueryResponsePacket)]
pub fn vel_embedded_voice_cached_query_response_packet(
    scenario: String,
    next_title: Option<String>,
    leave_by: Option<String>,
    empty_message: Option<String>,
    cached_now_summary: Option<String>,
    first_reason: Option<String>,
    next_commitment_text: Option<String>,
    next_commitment_due_at: Option<String>,
    behavior_headline: Option<String>,
    behavior_reason: Option<String>,
) -> String {
    BrowserWasmScaffold::voice_cached_query_response_packet(
        scenario,
        next_title,
        leave_by,
        empty_message,
        cached_now_summary,
        first_reason,
        next_commitment_text,
        next_commitment_due_at,
        behavior_headline,
        behavior_reason,
    )
    .payload_json
}
