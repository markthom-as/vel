use time::OffsetDateTime;
use tracing::warn;
use vel_core::PrivacyClass;
use vel_storage::{CaptureInsert, SignalInsert, Storage};

use crate::errors::AppError;

const MOOD_CAPTURE_TYPE: &str = "mood_log";
const PAIN_CAPTURE_TYPE: &str = "pain_log";
const WATCH_SIGNAL_CAPTURE_TYPE_PREFIX: &str = "watch_signal_";

#[derive(Debug, Clone)]
pub struct MoodJournalInput {
    pub score: u8,
    pub label: Option<String>,
    pub note: Option<String>,
    pub source_device: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PainJournalInput {
    pub severity: u8,
    pub location: Option<String>,
    pub note: Option<String>,
    pub source_device: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WatchSignalJournalInput {
    pub signal_type: String,
    pub note: Option<String>,
    pub context: Option<serde_json::Value>,
    pub source_device: Option<String>,
}

#[derive(Debug, Clone)]
pub struct JournalCaptureAccepted {
    pub capture_id: vel_core::CaptureId,
    pub accepted_at: OffsetDateTime,
}

pub async fn record_mood(
    storage: &Storage,
    payload: MoodJournalInput,
) -> Result<JournalCaptureAccepted, AppError> {
    if !(1..=10).contains(&payload.score) {
        return Err(AppError::bad_request("mood score must be between 1 and 10"));
    }

    let capture_text = mood_capture_text(
        payload.score,
        payload.label.as_deref(),
        payload.note.as_deref(),
    );
    let now = OffsetDateTime::now_utc();
    let capture_id = insert_capture(
        storage,
        capture_text,
        MOOD_CAPTURE_TYPE,
        payload.source_device,
        now.unix_timestamp(),
    )
    .await?;
    let signal_payload = serde_json::json!({
        "capture_id": capture_id.to_string(),
        "score": payload.score,
        "label": payload.label,
        "note": payload.note,
    });
    emit_signal(
        storage,
        capture_id.as_ref(),
        "mood_log",
        signal_payload,
        now.unix_timestamp(),
    )
    .await;

    Ok(JournalCaptureAccepted {
        capture_id,
        accepted_at: now,
    })
}

pub async fn record_pain(
    storage: &Storage,
    payload: PainJournalInput,
) -> Result<JournalCaptureAccepted, AppError> {
    if payload.severity > 10 {
        return Err(AppError::bad_request(
            "pain severity must be between 0 and 10",
        ));
    }

    let capture_text = pain_capture_text(
        payload.severity,
        payload.location.as_deref(),
        payload.note.as_deref(),
    );
    let now = OffsetDateTime::now_utc();
    let capture_id = insert_capture(
        storage,
        capture_text,
        PAIN_CAPTURE_TYPE,
        payload.source_device,
        now.unix_timestamp(),
    )
    .await?;
    let signal_payload = serde_json::json!({
        "capture_id": capture_id.to_string(),
        "severity": payload.severity,
        "location": payload.location,
        "note": payload.note,
    });
    emit_signal(
        storage,
        capture_id.as_ref(),
        "pain_log",
        signal_payload,
        now.unix_timestamp(),
    )
    .await;

    Ok(JournalCaptureAccepted {
        capture_id,
        accepted_at: now,
    })
}

pub async fn record_watch_signal(
    storage: &Storage,
    payload: WatchSignalJournalInput,
) -> Result<JournalCaptureAccepted, AppError> {
    let signal_type = watch_signal_type_label(&payload.signal_type)?;
    let capture_text = watch_signal_capture_text(&signal_type, payload.note.as_deref());
    let now = OffsetDateTime::now_utc();
    let capture_id = insert_capture(
        storage,
        capture_text,
        &format!("{WATCH_SIGNAL_CAPTURE_TYPE_PREFIX}{signal_type}"),
        payload.source_device.clone(),
        now.unix_timestamp(),
    )
    .await?;
    let signal_payload = serde_json::json!({
        "capture_id": capture_id.to_string(),
        "signal_type": signal_type,
        "note": payload.note,
        "context": payload.context,
    });
    emit_signal(
        storage,
        capture_id.as_ref(),
        &format!("watch_signal:{signal_type}"),
        signal_payload,
        now.unix_timestamp(),
    )
    .await;

    Ok(JournalCaptureAccepted {
        capture_id,
        accepted_at: now,
    })
}

async fn insert_capture(
    storage: &Storage,
    content_text: String,
    capture_type: &str,
    source_device: Option<String>,
    timestamp: i64,
) -> Result<vel_core::CaptureId, AppError> {
    let capture_id = storage
        .insert_capture(CaptureInsert {
            content_text: content_text.clone(),
            capture_type: capture_type.to_string(),
            source_device,
            privacy_class: PrivacyClass::Private,
        })
        .await?;

    let event_payload = serde_json::json!({ "capture_id": capture_id.to_string() }).to_string();
    if let Err(error) = storage
        .emit_event(
            "CAPTURE_CREATED",
            "capture",
            Some(capture_id.as_ref()),
            &event_payload,
        )
        .await
    {
        warn!(error = %error, "failed to emit CAPTURE_CREATED event");
    }

    let signal_payload = serde_json::json!({
        "capture_id": capture_id.to_string(),
        "content": content_text,
        "tags": [],
    });
    emit_signal(
        storage,
        capture_id.as_ref(),
        "capture_created",
        signal_payload,
        timestamp,
    )
    .await;

    Ok(capture_id)
}

async fn emit_signal(
    storage: &Storage,
    source_ref: &str,
    signal_type: &str,
    payload_json: serde_json::Value,
    timestamp: i64,
) {
    if let Err(error) = storage
        .insert_signal(SignalInsert {
            signal_type: signal_type.to_string(),
            source: "vel".to_string(),
            source_ref: Some(format!("{source_ref}:{signal_type}")),
            timestamp,
            payload_json: Some(payload_json),
        })
        .await
    {
        warn!(error = %error, signal_type, "failed to insert journal signal");
    }
}

fn mood_capture_text(score: u8, label: Option<&str>, note: Option<&str>) -> String {
    let mut text = format!("mood {score}/10");
    if let Some(label) = non_empty_trimmed(label) {
        text.push_str(&format!(" {label}"));
    }
    if let Some(note) = non_empty_trimmed(note) {
        text.push_str(&format!(" - {note}"));
    }
    text
}

fn pain_capture_text(severity: u8, location: Option<&str>, note: Option<&str>) -> String {
    let mut text = format!("pain {severity}/10");
    if let Some(location) = non_empty_trimmed(location) {
        text.push_str(&format!(" in {location}"));
    }
    if let Some(note) = non_empty_trimmed(note) {
        text.push_str(&format!(" - {note}"));
    }
    text
}

fn watch_signal_capture_text(signal_type: &str, note: Option<&str>) -> String {
    let mut text = format!("watch signal: {signal_type}");
    if let Some(note) = non_empty_trimmed(note) {
        text.push_str(&format!(" - {note}"));
    }
    text
}

fn watch_signal_type_label(value: &str) -> Result<String, AppError> {
    let normalized = value.trim().to_lowercase().replace([' ', '-'], "_");
    match normalized.as_str() {
        "drifting" | "on_track" | "need_focus" | "wake" | "waking_up" | "heart_rate" | "motion" => {
            Ok(normalized)
        }
        _ => Err(AppError::bad_request(
            "watch signal_type must be one of drifting, on_track, need_focus, wake, heart_rate, or motion",
        )),
    }
}

fn non_empty_trimmed(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}
