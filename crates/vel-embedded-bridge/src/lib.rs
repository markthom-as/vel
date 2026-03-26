mod browser_wasm;
mod portable_core;

use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use portable_core::{
    collect_remote_routes, normalize_domain_hint, normalize_pairing_token_input, normalize_payload,
    normalize_positive_minutes, normalized_optional_trimmed, prepare_app_shell_feedback_packet,
    prepare_assistant_entry_fallback_payload, prepare_capture_metadata_payload,
    prepare_linking_feedback_packet, prepare_linking_request_packet, prepare_queued_action_packet,
    prepare_quick_capture_text, prepare_thread_draft_packet, prepare_voice_capture_payload,
    prepare_voice_cached_query_response_packet, prepare_voice_continuity_summary_packet,
    prepare_voice_offline_response_packet, prepare_voice_quick_action_packet, trim_text,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CachedNowContext {
    mode: Option<String>,
    next_event_title: Option<String>,
    nudge_count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OfflineRequestInput {
    kind: Option<String>,
    payload: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OfflineRequestInputDecoded {
    kind: String,
    payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DomainHintInput {
    kind: Option<String>,
    input: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DomainHintInputDecoded {
    kind: String,
    input: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ThreadDraftInput {
    text: Option<String>,
    requested_conversation_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ThreadDraftInputDecoded {
    text: String,
    requested_conversation_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OfflineRequestOutput {
    kind: String,
    payload: String,
    ready: bool,
    reason: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DomainHintOutput {
    kind: String,
    normalized: String,
    ready: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ThreadDraftOutput {
    payload: String,
    requested_conversation_id: Option<String>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoiceCaptureInput {
    transcript: Option<String>,
    intent_storage_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoiceQuickActionInput {
    intent_storage_token: Option<String>,
    primary_text: Option<String>,
    target_id: Option<String>,
    minutes: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceQuickActionOutput {
    queue_kind: String,
    target_id: Option<String>,
    text: Option<String>,
    minutes: Option<i64>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoiceDraftInput {
    transcript: Option<String>,
    suggested_intent_storage_token: Option<String>,
    suggested_text: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceDraftOutput {
    transcript: String,
    suggested_intent_storage_token: String,
    suggested_text: String,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoiceContinuityEntryInput {
    transcript: Option<String>,
    suggested_intent_storage_token: Option<String>,
    committed_intent_storage_token: Option<String>,
    status: Option<String>,
    thread_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceContinuityEntryOutput {
    transcript: String,
    suggested_intent_storage_token: String,
    committed_intent_storage_token: Option<String>,
    status: String,
    thread_id: Option<String>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QueuedActionInput {
    kind: Option<String>,
    target_id: Option<String>,
    text: Option<String>,
    minutes: Option<i64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QueuedActionOutput {
    queue_kind: String,
    target_id: Option<String>,
    text: Option<String>,
    minutes: Option<i64>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoteRoutesInput {
    sync_base_url: Option<String>,
    tailscale_base_url: Option<String>,
    lan_base_url: Option<String>,
    public_base_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RemoteRouteOutput {
    label: String,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssistantEntryFallbackInput {
    text: Option<String>,
    requested_conversation_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AssistantEntryFallbackOutput {
    payload: String,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkingRequestInput {
    token_code: Option<String>,
    target_base_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LinkingRequestOutput {
    token_code: Option<String>,
    target_base_url: Option<String>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CaptureMetadataInput {
    text: Option<String>,
    capture_type: Option<String>,
    source_device: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CaptureMetadataOutput {
    payload: String,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PairingTokenIssueRequestInput {
    issued_by_node_id: Option<String>,
    target_node_id: Option<String>,
    target_node_display_name: Option<String>,
    target_base_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PairingTokenIssueRequestOutput {
    issued_by_node_id: String,
    target_node_id: Option<String>,
    target_node_display_name: Option<String>,
    target_base_url: Option<String>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PairingTokenRedeemRequestInput {
    token_code: Option<String>,
    node_id: Option<String>,
    node_display_name: Option<String>,
    transport_hint: Option<String>,
    sync_base_url: Option<String>,
    tailscale_base_url: Option<String>,
    lan_base_url: Option<String>,
    localhost_base_url: Option<String>,
    public_base_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PairingTokenRedeemRequestOutput {
    token_code: String,
    node_id: String,
    node_display_name: String,
    transport_hint: Option<String>,
    sync_base_url: Option<String>,
    tailscale_base_url: Option<String>,
    lan_base_url: Option<String>,
    localhost_base_url: Option<String>,
    public_base_url: Option<String>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoiceContinuitySummaryInput {
    draft_exists: Option<bool>,
    threaded_transcript: Option<String>,
    pending_recovery_count: Option<i64>,
    is_reachable: Option<bool>,
    merged_transcript: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceContinuitySummaryOutput {
    headline: Option<String>,
    detail: Option<String>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoiceOfflineResponseInput {
    scenario: Option<String>,
    primary_text: Option<String>,
    matched_text: Option<String>,
    options: Option<String>,
    minutes: Option<i64>,
    is_reachable: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceOfflineResponseOutput {
    summary: Option<String>,
    detail: Option<String>,
    history_status: String,
    error_prefix: String,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VoiceCachedQueryResponseInput {
    scenario: Option<String>,
    next_title: Option<String>,
    leave_by: Option<String>,
    empty_message: Option<String>,
    cached_now_summary: Option<String>,
    first_reason: Option<String>,
    next_commitment_text: Option<String>,
    next_commitment_due_at: Option<String>,
    behavior_headline: Option<String>,
    behavior_reason: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct VoiceCachedQueryResponseOutput {
    summary: Option<String>,
    detail: Option<String>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LinkingFeedbackInput {
    scenario: Option<String>,
    node_display_name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LinkingFeedbackOutput {
    message: Option<String>,
    ready: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppShellFeedbackInput {
    scenario: Option<String>,
    detail: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AppShellFeedbackOutput {
    message: Option<String>,
    ready: bool,
}

fn read_input(pointer: *const c_char) -> Option<String> {
    if pointer.is_null() {
        return None;
    }

    let c_string = unsafe { CStr::from_ptr(pointer) };
    Some(c_string.to_string_lossy().to_string())
}

fn to_owned_c_string(value: &str) -> *mut c_char {
    match CString::new(value) {
        Ok(value) => value.into_raw(),
        Err(_) => CString::new("[]").unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn vel_embedded_cached_now_summary(context_json: *const c_char) -> *mut c_char {
    let raw_context = read_input(context_json);
    let payload =
        parse_cached_now_context(raw_context.as_deref()).unwrap_or_else(|_| CachedNowContext {
            mode: Some("unknown".to_string()),
            next_event_title: None,
            nudge_count: Some(0),
        });

    let mut lines = Vec::new();
    lines.push(format!(
        "Mode: {}",
        payload.mode.unwrap_or_else(|| "unknown".to_string())
    ));
    lines.push(format!(
        "Next: {}",
        payload
            .next_event_title
            .unwrap_or_else(|| "none".to_string())
    ));
    lines.push(format!(
        "Nudges: {}",
        payload.nudge_count.unwrap_or_default()
    ));

    let json = serde_json::to_string(&lines).unwrap_or_else(|_| "[]".to_string());
    to_owned_c_string(&json)
}

fn parse_cached_now_context(raw_json: Option<&str>) -> Result<CachedNowContext, serde_json::Error> {
    let input = raw_json.unwrap_or("{}");
    serde_json::from_str(input)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_quick_capture(text: *const c_char) -> *mut c_char {
    let raw_input = read_input(text).unwrap_or_default();
    let payload = prepare_quick_capture_text(&raw_input);
    to_owned_c_string(&payload)
}

#[no_mangle]
pub extern "C" fn vel_embedded_package_offline_request(payload_json: *const c_char) -> *mut c_char {
    let raw_payload = read_input(payload_json).unwrap_or_default();
    let request = parse_offline_request(Some(&raw_payload));

    let (kind, payload, ready, reason) = match request {
        Ok(request) => {
            let kind = request.kind.unwrap_or_else(|| "unknown".to_string());
            let payload = request
                .payload
                .map(|value| normalize_payload(&value))
                .unwrap_or_else(String::new);

            (kind, payload, true, None)
        }
        Err(error) => (
            "unknown".to_string(),
            normalize_payload(&raw_payload),
            false,
            Some(format!("invalid payload: {error}")),
        ),
    };

    let output = OfflineRequestOutput {
        kind,
        payload,
        ready,
        reason,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

fn parse_offline_request(
    raw_json: Option<&str>,
) -> Result<OfflineRequestInputDecoded, serde_json::Error> {
    let input = raw_json.unwrap_or("{}");
    let decoded: OfflineRequestInput = serde_json::from_str(input)?;

    Ok(OfflineRequestInputDecoded {
        kind: decoded.kind.unwrap_or_else(|| "unknown".to_string()),
        payload: decoded.payload.unwrap_or_default(),
    })
}

#[no_mangle]
pub extern "C" fn vel_embedded_normalize_domain_helpers(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = parse_domain_input(&raw);

    let output = DomainHintOutput {
        kind: decoded.kind,
        normalized: normalize_domain_hint(decoded.input),
        ready: true,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

fn parse_domain_input(raw_json: &str) -> DomainHintInputDecoded {
    serde_json::from_str::<DomainHintInput>(raw_json)
        .map(|value| DomainHintInputDecoded {
            kind: value.kind.unwrap_or_else(|| "unknown".to_string()),
            input: value.input.unwrap_or_default(),
        })
        .unwrap_or_else(|_| DomainHintInputDecoded {
            kind: "text".to_string(),
            input: raw_json.to_string(),
        })
}

fn parse_thread_draft(
    raw_json: Option<&str>,
) -> Result<ThreadDraftInputDecoded, serde_json::Error> {
    let input = raw_json.unwrap_or("{}");
    let decoded: ThreadDraftInput = serde_json::from_str(input)?;

    Ok(ThreadDraftInputDecoded {
        text: decoded.text.unwrap_or_default(),
        requested_conversation_id: decoded.requested_conversation_id.and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        }),
    })
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_voice_capture_payload(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<VoiceCaptureInput>(&raw).unwrap_or(VoiceCaptureInput {
        transcript: Some(raw.clone()),
        intent_storage_token: Some("capture".to_string()),
    });

    let payload = prepare_voice_capture_payload(
        &decoded.transcript.unwrap_or_default(),
        &decoded
            .intent_storage_token
            .unwrap_or_else(|| "capture".to_string()),
    );

    to_owned_c_string(&payload)
}

#[no_mangle]
pub extern "C" fn vel_embedded_package_voice_quick_action(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded =
        serde_json::from_str::<VoiceQuickActionInput>(&raw).unwrap_or(VoiceQuickActionInput {
            intent_storage_token: Some("capture_create".to_string()),
            primary_text: Some(raw.clone()),
            target_id: None,
            minutes: None,
        });

    let intent_storage_token = decoded
        .intent_storage_token
        .unwrap_or_else(|| "capture_create".to_string());
    let primary_text = decoded.primary_text.unwrap_or_default();
    let output = prepare_voice_quick_action_packet(
        &intent_storage_token,
        &primary_text,
        normalized_optional_trimmed(decoded.target_id),
        normalize_positive_minutes(decoded.minutes),
    );

    let json = serde_json::to_string(&VoiceQuickActionOutput {
        queue_kind: output.queue_kind,
        target_id: output.target_id,
        text: output.text,
        minutes: output.minutes,
        ready: output.ready,
    })
    .unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_voice_draft(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<VoiceDraftInput>(&raw).unwrap_or(VoiceDraftInput {
        transcript: Some(raw.clone()),
        suggested_intent_storage_token: Some("capture".to_string()),
        suggested_text: Some(String::new()),
    });

    let output = VoiceDraftOutput {
        transcript: trim_text(&decoded.transcript.unwrap_or_default()),
        suggested_intent_storage_token: trim_text(
            &decoded
                .suggested_intent_storage_token
                .unwrap_or_else(|| "capture".to_string()),
        ),
        suggested_text: trim_text(&decoded.suggested_text.unwrap_or_default()),
        ready: true,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_voice_continuity_entry(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<VoiceContinuityEntryInput>(&raw).unwrap_or(
        VoiceContinuityEntryInput {
            transcript: Some(raw.clone()),
            suggested_intent_storage_token: Some("capture".to_string()),
            committed_intent_storage_token: None,
            status: Some("pending_review".to_string()),
            thread_id: None,
        },
    );

    let thread_id = decoded.thread_id.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    });

    let committed_intent_storage_token = decoded.committed_intent_storage_token.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    });

    let output = VoiceContinuityEntryOutput {
        transcript: trim_text(&decoded.transcript.unwrap_or_default()),
        suggested_intent_storage_token: trim_text(
            &decoded
                .suggested_intent_storage_token
                .unwrap_or_else(|| "capture".to_string()),
        ),
        committed_intent_storage_token,
        status: trim_text(
            &decoded
                .status
                .unwrap_or_else(|| "pending_review".to_string()),
        ),
        thread_id,
        ready: true,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_package_queued_action(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<QueuedActionInput>(&raw).unwrap_or(QueuedActionInput {
        kind: Some("capture.create".to_string()),
        target_id: None,
        text: Some(raw.clone()),
        minutes: None,
    });

    let output = prepare_queued_action_packet(
        trim_text(&decoded.kind.unwrap_or_else(|| "capture.create".to_string())),
        normalized_optional_trimmed(decoded.target_id),
        normalized_optional_trimmed(decoded.text),
        normalize_positive_minutes(decoded.minutes),
    );

    let json = serde_json::to_string(&QueuedActionOutput {
        queue_kind: output.queue_kind,
        target_id: output.target_id,
        text: output.text,
        minutes: output.minutes,
        ready: output.ready,
    })
    .unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_normalize_pairing_token(value: *const c_char) -> *mut c_char {
    let raw = read_input(value).unwrap_or_default();
    let output = normalize_pairing_token_input(&raw);

    to_owned_c_string(&output)
}

#[no_mangle]
pub extern "C" fn vel_embedded_collect_remote_routes(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<RemoteRoutesInput>(&raw).unwrap_or(RemoteRoutesInput {
        sync_base_url: None,
        tailscale_base_url: None,
        lan_base_url: None,
        public_base_url: None,
    });

    let routes = collect_remote_routes(
        decoded.sync_base_url,
        decoded.tailscale_base_url,
        decoded.lan_base_url,
        decoded.public_base_url,
    )
    .into_iter()
    .map(|route| RemoteRouteOutput {
        label: route.label,
        base_url: route.base_url,
    })
    .collect::<Vec<_>>();

    let json = serde_json::to_string(&routes).unwrap_or_else(|_| "[]".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_assistant_entry_fallback(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<AssistantEntryFallbackInput>(&raw).unwrap_or(
        AssistantEntryFallbackInput {
            text: Some(raw.clone()),
            requested_conversation_id: None,
        },
    );

    let payload = prepare_assistant_entry_fallback_payload(
        &decoded.text.unwrap_or_default(),
        normalized_optional_trimmed(decoded.requested_conversation_id),
    );

    let output = AssistantEntryFallbackOutput {
        payload,
        ready: true,
    };
    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_linking_request(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded =
        serde_json::from_str::<LinkingRequestInput>(&raw).unwrap_or(LinkingRequestInput {
            token_code: None,
            target_base_url: None,
        });
    let packet = prepare_linking_request_packet(decoded.token_code, decoded.target_base_url);

    let output = LinkingRequestOutput {
        token_code: packet.token_code,
        target_base_url: packet.target_base_url,
        ready: true,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_capture_metadata(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded =
        serde_json::from_str::<CaptureMetadataInput>(&raw).unwrap_or(CaptureMetadataInput {
            text: Some(raw.clone()),
            capture_type: Some("note".to_string()),
            source_device: Some("apple".to_string()),
        });

    let payload = prepare_capture_metadata_payload(
        &decoded.text.unwrap_or_default(),
        &decoded.capture_type.unwrap_or_else(|| "note".to_string()),
        &decoded.source_device.unwrap_or_else(|| "apple".to_string()),
    );

    let output = CaptureMetadataOutput {
        payload,
        ready: true,
    };
    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_pairing_token_issue_request(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<PairingTokenIssueRequestInput>(&raw).unwrap_or(
        PairingTokenIssueRequestInput {
            issued_by_node_id: Some(String::new()),
            target_node_id: None,
            target_node_display_name: None,
            target_base_url: None,
        },
    );

    let output = PairingTokenIssueRequestOutput {
        issued_by_node_id: trim_text(&decoded.issued_by_node_id.unwrap_or_default()),
        target_node_id: normalized_optional_trimmed(decoded.target_node_id),
        target_node_display_name: normalized_optional_trimmed(decoded.target_node_display_name),
        target_base_url: normalized_optional_trimmed(decoded.target_base_url),
        ready: true,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_pairing_token_redeem_request(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<PairingTokenRedeemRequestInput>(&raw).unwrap_or(
        PairingTokenRedeemRequestInput {
            token_code: Some(String::new()),
            node_id: Some(String::new()),
            node_display_name: Some(String::new()),
            transport_hint: None,
            sync_base_url: None,
            tailscale_base_url: None,
            lan_base_url: None,
            localhost_base_url: None,
            public_base_url: None,
        },
    );

    let output = PairingTokenRedeemRequestOutput {
        token_code: trim_text(&decoded.token_code.unwrap_or_default()),
        node_id: trim_text(&decoded.node_id.unwrap_or_default()),
        node_display_name: trim_text(&decoded.node_display_name.unwrap_or_default()),
        transport_hint: normalized_optional_trimmed(decoded.transport_hint),
        sync_base_url: normalized_optional_trimmed(decoded.sync_base_url),
        tailscale_base_url: normalized_optional_trimmed(decoded.tailscale_base_url),
        lan_base_url: normalized_optional_trimmed(decoded.lan_base_url),
        localhost_base_url: normalized_optional_trimmed(decoded.localhost_base_url),
        public_base_url: normalized_optional_trimmed(decoded.public_base_url),
        ready: true,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_voice_continuity_summary(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<VoiceContinuitySummaryInput>(&raw).unwrap_or(
        VoiceContinuitySummaryInput {
            draft_exists: Some(false),
            threaded_transcript: None,
            pending_recovery_count: Some(0),
            is_reachable: Some(false),
            merged_transcript: None,
        },
    );

    let packet = prepare_voice_continuity_summary_packet(
        decoded.draft_exists,
        decoded.threaded_transcript,
        decoded.pending_recovery_count,
        decoded.is_reachable,
        decoded.merged_transcript,
    );
    let output = VoiceContinuitySummaryOutput {
        headline: packet.headline,
        detail: packet.detail,
        ready: packet.ready,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_voice_offline_response(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<VoiceOfflineResponseInput>(&raw).unwrap_or(
        VoiceOfflineResponseInput {
            scenario: None,
            primary_text: None,
            matched_text: None,
            options: None,
            minutes: None,
            is_reachable: Some(false),
        },
    );

    let packet = prepare_voice_offline_response_packet(
        &decoded.scenario.unwrap_or_default(),
        decoded.primary_text,
        decoded.matched_text,
        decoded.options,
        decoded.minutes,
        decoded.is_reachable,
    );
    let output = VoiceOfflineResponseOutput {
        summary: packet.summary,
        detail: packet.detail,
        history_status: packet.history_status,
        error_prefix: packet.error_prefix,
        ready: packet.ready,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_voice_cached_query_response(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<VoiceCachedQueryResponseInput>(&raw).unwrap_or(
        VoiceCachedQueryResponseInput {
            scenario: None,
            next_title: None,
            leave_by: None,
            empty_message: None,
            cached_now_summary: None,
            first_reason: None,
            next_commitment_text: None,
            next_commitment_due_at: None,
            behavior_headline: None,
            behavior_reason: None,
        },
    );

    let packet = prepare_voice_cached_query_response_packet(
        &decoded.scenario.unwrap_or_default(),
        decoded.next_title,
        decoded.leave_by,
        decoded.empty_message,
        decoded.cached_now_summary,
        decoded.first_reason,
        decoded.next_commitment_text,
        decoded.next_commitment_due_at,
        decoded.behavior_headline,
        decoded.behavior_reason,
    );
    let output = VoiceCachedQueryResponseOutput {
        summary: packet.summary,
        detail: packet.detail,
        ready: packet.ready,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_linking_feedback(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded =
        serde_json::from_str::<LinkingFeedbackInput>(&raw).unwrap_or(LinkingFeedbackInput {
            scenario: None,
            node_display_name: None,
        });

    let scenario = trim_text(&decoded.scenario.unwrap_or_default());
    let output = match prepare_linking_feedback_packet(
        &scenario,
        normalized_optional_trimmed(decoded.node_display_name),
    ) {
        Some(packet) => LinkingFeedbackOutput {
            message: Some(packet.message),
            ready: true,
        },
        None => LinkingFeedbackOutput {
            message: None,
            ready: false,
        },
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_app_shell_feedback(
    input_json: *const c_char,
) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded =
        serde_json::from_str::<AppShellFeedbackInput>(&raw).unwrap_or(AppShellFeedbackInput {
            scenario: None,
            detail: None,
        });

    let scenario = trim_text(&decoded.scenario.unwrap_or_default());
    let output = match prepare_app_shell_feedback_packet(
        &scenario,
        normalized_optional_trimmed(decoded.detail),
    ) {
        Some(packet) => AppShellFeedbackOutput {
            message: Some(packet.message),
            ready: true,
        },
        None => AppShellFeedbackOutput {
            message: None,
            ready: false,
        },
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_thread_draft(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let parsed = parse_thread_draft(Some(&raw)).unwrap_or(ThreadDraftInputDecoded {
        text: raw.clone(),
        requested_conversation_id: None,
    });
    let packet = prepare_thread_draft_packet(&parsed.text, parsed.requested_conversation_id);

    let output = ThreadDraftOutput {
        payload: packet.payload,
        requested_conversation_id: packet.requested_conversation_id,
        ready: true,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_free_buffer(pointer: *mut c_char) {
    if pointer.is_null() {
        return;
    }

    unsafe {
        drop(CString::from_raw(pointer));
    }
}
