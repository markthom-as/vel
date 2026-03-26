use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

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
    let payload = parse_cached_now_context(raw_context.as_deref())
        .unwrap_or_else(|_| CachedNowContext {
            mode: Some("unknown".to_string()),
            next_event_title: None,
            nudge_count: Some(0),
        });

    let mut lines = Vec::new();
    lines.push(format!("Mode: {}", payload.mode.unwrap_or_else(|| "unknown".to_string())));
    lines.push(format!(
        "Next: {}",
        payload.next_event_title.unwrap_or_else(|| "none".to_string())
    ));
    lines.push(format!("Nudges: {}", payload.nudge_count.unwrap_or_default()));

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

    let trimmed = raw_input
        .lines()
        .flat_map(|line| line.split_whitespace())
        .collect::<Vec<_>>()
        .join(" ");

    let payload = trimmed.trim();
    to_owned_c_string(payload)
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
        Err(error) => {
            (
                "unknown".to_string(),
                normalize_payload(&raw_payload),
                false,
                Some(format!("invalid payload: {error}")),
            )
        }
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

fn parse_offline_request(raw_json: Option<&str>) -> Result<OfflineRequestInputDecoded, serde_json::Error> {
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

fn parse_thread_draft(raw_json: Option<&str>) -> Result<ThreadDraftInputDecoded, serde_json::Error> {
    let input = raw_json.unwrap_or("{}");
    let decoded: ThreadDraftInput = serde_json::from_str(input)?;

    Ok(ThreadDraftInputDecoded {
        text: decoded.text.unwrap_or_default(),
        requested_conversation_id: decoded.requested_conversation_id.and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() { None } else { Some(trimmed) }
        }),
    })
}

fn normalize_domain_hint(value: String) -> String {
    value
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_payload(value: &str) -> String {
    value
        .trim()
        .replace("\n", " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn trim_text(value: &str) -> String {
    value.trim().to_string()
}

fn normalized_optional_trimmed(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_voice_capture_payload(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<VoiceCaptureInput>(&raw).unwrap_or(VoiceCaptureInput {
        transcript: Some(raw.clone()),
        intent_storage_token: Some("capture".to_string()),
    });

    let transcript = decoded.transcript.unwrap_or_default().trim().to_string();
    let intent_storage_token = decoded.intent_storage_token.unwrap_or_else(|| "capture".to_string());

    let payload = [
        "voice_transcript:".to_string(),
        transcript,
        String::new(),
        format!("intent_candidate: {}", normalize_payload(&intent_storage_token)),
        "client_surface: ios_voice".to_string(),
    ]
    .join("\n");

    to_owned_c_string(&payload)
}

#[no_mangle]
pub extern "C" fn vel_embedded_package_voice_quick_action(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<VoiceQuickActionInput>(&raw).unwrap_or(VoiceQuickActionInput {
        intent_storage_token: Some("capture_create".to_string()),
        primary_text: Some(raw.clone()),
        target_id: None,
        minutes: None,
    });

    let intent_storage_token = decoded.intent_storage_token.unwrap_or_else(|| "capture_create".to_string());
    let primary_text = decoded.primary_text.unwrap_or_default();
    let target_id = decoded.target_id.and_then(|value| {
        let trimmed = value.trim().to_string();
        if trimmed.is_empty() { None } else { Some(trimmed) }
    });
    let minutes = decoded.minutes.map(|value| value.max(1));

    let output = if intent_storage_token == "capture_create" {
        VoiceQuickActionOutput {
            queue_kind: "capture.create".to_string(),
            target_id: None,
            text: Some(normalize_payload(&primary_text)),
            minutes: None,
            ready: true,
        }
    } else if intent_storage_token == "commitment_create" {
        VoiceQuickActionOutput {
            queue_kind: "commitment.create".to_string(),
            target_id: None,
            text: Some(normalize_payload(&primary_text)),
            minutes: None,
            ready: true,
        }
    } else if intent_storage_token == "commitment_done" {
        VoiceQuickActionOutput {
            queue_kind: "commitment.done".to_string(),
            target_id,
            text: None,
            minutes: None,
            ready: true,
        }
    } else if intent_storage_token == "nudge_done" {
        VoiceQuickActionOutput {
            queue_kind: "nudge.done".to_string(),
            target_id,
            text: None,
            minutes: None,
            ready: true,
        }
    } else if intent_storage_token.starts_with("nudge_snooze_") {
        VoiceQuickActionOutput {
            queue_kind: "nudge.snooze".to_string(),
            target_id,
            text: None,
            minutes,
            ready: true,
        }
    } else {
        VoiceQuickActionOutput {
            queue_kind: "capture.create".to_string(),
            target_id: None,
            text: Some(normalize_payload(&primary_text)),
            minutes: None,
            ready: false,
        }
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
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
pub extern "C" fn vel_embedded_prepare_voice_continuity_entry(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded =
        serde_json::from_str::<VoiceContinuityEntryInput>(&raw).unwrap_or(VoiceContinuityEntryInput {
            transcript: Some(raw.clone()),
            suggested_intent_storage_token: Some("capture".to_string()),
            committed_intent_storage_token: None,
            status: Some("pending_review".to_string()),
            thread_id: None,
        });

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
        status: trim_text(&decoded.status.unwrap_or_else(|| "pending_review".to_string())),
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

    let kind = trim_text(&decoded.kind.unwrap_or_else(|| "capture.create".to_string()));
    let target_id = normalized_optional_trimmed(decoded.target_id);
    let text = normalized_optional_trimmed(decoded.text);
    let minutes = decoded.minutes.map(|value| value.max(1));

    let ready = matches!(
        kind.as_str(),
        "capture.create" | "commitment.create" | "commitment.done" | "nudge.done" | "nudge.snooze"
    );

    let output = QueuedActionOutput {
        queue_kind: if ready { kind } else { "capture.create".to_string() },
        target_id,
        text,
        minutes,
        ready,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_normalize_pairing_token(value: *const c_char) -> *mut c_char {
    let raw = read_input(value).unwrap_or_default();
    let normalized: String = raw
        .to_uppercase()
        .chars()
        .filter(|character| character.is_ascii() && character.is_ascii_alphanumeric())
        .take(6)
        .collect();

    let output = if normalized.len() <= 3 {
        normalized
    } else {
        format!("{}-{}", &normalized[..3], &normalized[3..])
    };

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

    let entries = [
        ("primary", decoded.sync_base_url),
        ("tailscale", decoded.tailscale_base_url),
        ("lan", decoded.lan_base_url),
        ("public", decoded.public_base_url),
    ];

    let mut seen: Vec<String> = Vec::new();
    let mut routes: Vec<RemoteRouteOutput> = Vec::new();

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
        routes.push(RemoteRouteOutput {
            label: label.to_string(),
            base_url: trimmed,
        });
    }

    let json = serde_json::to_string(&routes).unwrap_or_else(|_| "[]".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_assistant_entry_fallback(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<AssistantEntryFallbackInput>(&raw).unwrap_or(AssistantEntryFallbackInput {
        text: Some(raw.clone()),
        requested_conversation_id: None,
    });

    let requested_conversation_id = normalized_optional_trimmed(decoded.requested_conversation_id);
    let payload = [
        "queued_assistant_entry:".to_string(),
        requested_conversation_id.map(|value| format!("requested_conversation_id: {value}")).unwrap_or_default(),
        String::new(),
        trim_text(&decoded.text.unwrap_or_default()),
    ]
    .into_iter()
    .filter(|value| !value.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n");

    let output = AssistantEntryFallbackOutput { payload, ready: true };
    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_linking_request(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<LinkingRequestInput>(&raw).unwrap_or(LinkingRequestInput {
        token_code: None,
        target_base_url: None,
    });

    let output = LinkingRequestOutput {
        token_code: normalized_optional_trimmed(decoded.token_code),
        target_base_url: normalized_optional_trimmed(decoded.target_base_url),
        ready: true,
    };

    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_capture_metadata(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<CaptureMetadataInput>(&raw).unwrap_or(CaptureMetadataInput {
        text: Some(raw.clone()),
        capture_type: Some("note".to_string()),
        source_device: Some("apple".to_string()),
    });

    let text = trim_text(&decoded.text.unwrap_or_default());
    let capture_type = trim_text(&decoded.capture_type.unwrap_or_else(|| "note".to_string()));
    let source_device = trim_text(&decoded.source_device.unwrap_or_else(|| "apple".to_string()));

    let payload = if capture_type == "note" && source_device == "apple" {
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
    };

    let output = CaptureMetadataOutput { payload, ready: true };
    let json = serde_json::to_string(&output).unwrap_or_else(|_| "{\"ready\":false}".to_string());
    to_owned_c_string(&json)
}

#[no_mangle]
pub extern "C" fn vel_embedded_prepare_pairing_token_issue_request(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<PairingTokenIssueRequestInput>(&raw).unwrap_or(PairingTokenIssueRequestInput {
        issued_by_node_id: Some(String::new()),
        target_node_id: None,
        target_node_display_name: None,
        target_base_url: None,
    });

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
pub extern "C" fn vel_embedded_prepare_pairing_token_redeem_request(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let decoded = serde_json::from_str::<PairingTokenRedeemRequestInput>(&raw).unwrap_or(PairingTokenRedeemRequestInput {
        token_code: Some(String::new()),
        node_id: Some(String::new()),
        node_display_name: Some(String::new()),
        transport_hint: None,
        sync_base_url: None,
        tailscale_base_url: None,
        lan_base_url: None,
        localhost_base_url: None,
        public_base_url: None,
    });

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
pub extern "C" fn vel_embedded_prepare_thread_draft(input_json: *const c_char) -> *mut c_char {
    let raw = read_input(input_json).unwrap_or_default();
    let parsed = parse_thread_draft(Some(&raw)).unwrap_or(ThreadDraftInputDecoded {
        text: raw.clone(),
        requested_conversation_id: None,
    });

    let output = ThreadDraftOutput {
        payload: normalize_payload(&parsed.text),
        requested_conversation_id: parsed.requested_conversation_id,
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
