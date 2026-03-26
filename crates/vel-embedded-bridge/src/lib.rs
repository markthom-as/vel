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
            if trimmed.is_empty() { nil } else { Some(trimmed) }
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
        if trimmed.is_empty() { nil } else { Some(trimmed) }
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
