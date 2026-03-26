mod browser_wasm;
mod portable_core;

use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use portable_core::{
    collect_remote_routes, normalize_domain_hint, normalize_pairing_token_input,
    normalize_payload, normalized_optional_trimmed, normalize_positive_minutes,
    prepare_app_shell_feedback_packet, prepare_assistant_entry_fallback_payload,
    prepare_capture_metadata_payload, prepare_linking_feedback_packet,
    prepare_linking_request_packet, prepare_queued_action_packet, prepare_quick_capture_text,
    prepare_thread_draft_packet, prepare_voice_capture_payload,
    prepare_voice_quick_action_packet, trim_text,
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

    let output = if decoded.draft_exists.unwrap_or(false) {
        VoiceContinuitySummaryOutput {
            headline: Some("Voice draft ready to resume.".to_string()),
            detail: Some("Your latest local transcript is still on device and can be resumed without reopening a separate thread.".to_string()),
            ready: true,
        }
    } else if let Some(threaded) = normalized_optional_trimmed(decoded.threaded_transcript) {
        VoiceContinuitySummaryOutput {
            headline: Some("Voice follow-up saved in Threads.".to_string()),
            detail: Some(threaded),
            ready: true,
        }
    } else if decoded.pending_recovery_count.unwrap_or(0) > 0 {
        let count = decoded.pending_recovery_count.unwrap_or(0);
        let detail = if decoded.is_reachable.unwrap_or(false) {
            "Local voice recovery is waiting on canonical replay.".to_string()
        } else {
            format!(
                "Reconnect to merge {count} local voice entr{} back into canonical state.",
                if count == 1 { "y" } else { "ies" }
            )
        };
        VoiceContinuitySummaryOutput {
            headline: Some("Voice recovery pending.".to_string()),
            detail: Some(detail),
            ready: true,
        }
    } else if let Some(merged) = normalized_optional_trimmed(decoded.merged_transcript) {
        VoiceContinuitySummaryOutput {
            headline: Some("Local voice recovery merged.".to_string()),
            detail: Some(merged),
            ready: true,
        }
    } else {
        VoiceContinuitySummaryOutput {
            headline: None,
            detail: None,
            ready: false,
        }
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

    let scenario = trim_text(&decoded.scenario.unwrap_or_default());
    let primary_text = normalized_optional_trimmed(decoded.primary_text);
    let matched_text = normalized_optional_trimmed(decoded.matched_text);
    let options = normalized_optional_trimmed(decoded.options);
    let minutes = decoded.minutes.unwrap_or(10).max(1);
    let is_reachable = decoded.is_reachable.unwrap_or(false);

    let output = match scenario.as_str() {
        "capture_shell" => VoiceOfflineResponseOutput {
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
        "commitment_create_shell" => VoiceOfflineResponseOutput {
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
        "backend_required_shell" => VoiceOfflineResponseOutput {
            summary: Some("This voice action now requires the backend Apple route.".to_string()),
            detail: Some("Reconnect to Vel so the server can interpret and answer it.".to_string()),
            history_status: "backend_required".to_string(),
            error_prefix: "Transcript capture was preserved, but the action needs the backend-owned Apple route.".to_string(),
            ready: true,
        },
        "capture_offline" => VoiceOfflineResponseOutput {
            summary: Some("Voice capture queued for sync.".to_string()),
            detail: primary_text,
            history_status: "queued".to_string(),
            error_prefix: "Transcript capture queued for sync.".to_string(),
            ready: true,
        },
        "commitment_target_missing" => VoiceOfflineResponseOutput {
            summary: Some("Commitment target is missing.".to_string()),
            detail: Some("Try phrasing like \"mark meds done.\"".to_string()),
            history_status: "needs_clarification".to_string(),
            error_prefix: "Commitment target missing.".to_string(),
            ready: true,
        },
        "commitment_no_match" => VoiceOfflineResponseOutput {
            summary: Some("No open commitment matched.".to_string()),
            detail: Some("Transcript capture was queued for sync.".to_string()),
            history_status: "capture_only".to_string(),
            error_prefix: "No local commitment match for offline queueing.".to_string(),
            ready: true,
        },
        "commitment_ambiguous" => VoiceOfflineResponseOutput {
            summary: Some("Ambiguous commitment target.".to_string()),
            detail: options.map(|value| format!("Could match: {value}")),
            history_status: "needs_clarification".to_string(),
            error_prefix: "Commitment target was ambiguous.".to_string(),
            ready: true,
        },
        "commitment_done_queued" => VoiceOfflineResponseOutput {
            summary: Some("Commitment completion queued.".to_string()),
            detail: matched_text,
            history_status: "queued".to_string(),
            error_prefix: "Commitment completion queued for backend replay.".to_string(),
            ready: true,
        },
        "nudge_missing" => VoiceOfflineResponseOutput {
            summary: Some("No active nudge found.".to_string()),
            detail: Some("Transcript capture was queued for sync.".to_string()),
            history_status: "capture_only".to_string(),
            error_prefix: "No active nudge available for offline queueing.".to_string(),
            ready: true,
        },
        "nudge_done_queued" => VoiceOfflineResponseOutput {
            summary: Some("Top nudge resolution queued.".to_string()),
            detail: None,
            history_status: "queued".to_string(),
            error_prefix: "Top nudge resolution queued for backend replay.".to_string(),
            ready: true,
        },
        "nudge_snooze_queued" => VoiceOfflineResponseOutput {
            summary: Some("Top nudge snooze queued.".to_string()),
            detail: Some(format!("{minutes} minutes")),
            history_status: "queued".to_string(),
            error_prefix: "Top nudge snooze queued for backend replay.".to_string(),
            ready: true,
        },
        "backend_required_offline" => VoiceOfflineResponseOutput {
            summary: Some("Unavailable offline.".to_string()),
            detail: Some("This reply is backend-owned and is not synthesized from local Swift cache.".to_string()),
            history_status: "backend_required".to_string(),
            error_prefix: "Transcript capture queued, but this voice reply requires the backend route.".to_string(),
            ready: true,
        },
        _ => VoiceOfflineResponseOutput {
            summary: None,
            detail: None,
            history_status: "capture_only".to_string(),
            error_prefix: String::new(),
            ready: false,
        },
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

    let scenario = trim_text(&decoded.scenario.unwrap_or_default());
    let next_title = normalized_optional_trimmed(decoded.next_title);
    let leave_by = normalized_optional_trimmed(decoded.leave_by);
    let empty_message = normalized_optional_trimmed(decoded.empty_message);
    let cached_now_summary = normalized_optional_trimmed(decoded.cached_now_summary);
    let first_reason = normalized_optional_trimmed(decoded.first_reason);
    let next_commitment_text = normalized_optional_trimmed(decoded.next_commitment_text);
    let next_commitment_due_at = normalized_optional_trimmed(decoded.next_commitment_due_at);
    let behavior_headline = normalized_optional_trimmed(decoded.behavior_headline);
    let behavior_reason = normalized_optional_trimmed(decoded.behavior_reason);

    let output = match scenario.as_str() {
        "schedule_with_event" => VoiceCachedQueryResponseOutput {
            summary: next_title.map(|title| format!("Next event: {title}.")),
            detail: leave_by.or(cached_now_summary).or(empty_message),
            ready: true,
        },
        "schedule_empty" => VoiceCachedQueryResponseOutput {
            summary: Some(
                empty_message.unwrap_or_else(|| "No upcoming schedule is cached.".to_string()),
            ),
            detail: cached_now_summary.or(first_reason),
            ready: true,
        },
        "next_commitment" => VoiceCachedQueryResponseOutput {
            summary: next_commitment_text.map(|text| format!("Next commitment: {text}.")),
            detail: next_commitment_due_at.or(cached_now_summary),
            ready: true,
        },
        "next_commitment_empty" => VoiceCachedQueryResponseOutput {
            summary: Some("No next commitment is cached.".to_string()),
            detail: cached_now_summary.or(empty_message),
            ready: true,
        },
        "behavior_cached" => VoiceCachedQueryResponseOutput {
            summary: behavior_headline,
            detail: behavior_reason,
            ready: true,
        },
        "backend_unavailable" => VoiceCachedQueryResponseOutput {
            summary: Some("Unavailable offline.".to_string()),
            detail: Some("Reconnect to fetch a backend-owned reply.".to_string()),
            ready: true,
        },
        "cached_now_missing" => VoiceCachedQueryResponseOutput {
            summary: Some("Unavailable offline.".to_string()),
            detail: Some("No cached backend /v1/now payload is available yet.".to_string()),
            ready: true,
        },
        "behavior_missing" => VoiceCachedQueryResponseOutput {
            summary: Some("Unavailable offline.".to_string()),
            detail: Some("No cached backend behavior summary is available yet.".to_string()),
            ready: true,
        },
        _ => VoiceCachedQueryResponseOutput {
            summary: None,
            detail: None,
            ready: false,
        },
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
