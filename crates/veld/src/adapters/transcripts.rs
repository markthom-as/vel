//! Transcript adapter: ingest assistant/chat transcript snapshots into assistant_transcripts and signals.

use sha2::{Digest, Sha256};
use vel_config::AppConfig;
use vel_storage::{AssistantTranscriptInsert, SignalInsert, Storage};

pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let path = match &config.transcript_snapshot_path {
        Some(path) => path,
        None => return Ok(0),
    };

    let content = tokio::fs::read_to_string(path).await.map_err(|e| {
        crate::errors::AppError::internal(format!("read transcript snapshot {}: {}", path, e))
    })?;
    let snapshot: TranscriptSnapshot = serde_json::from_str(&content).map_err(|e| {
        crate::errors::AppError::internal(format!("parse transcript snapshot: {}", e))
    })?;

    let snapshot_source = snapshot.source();
    let snapshot_conversation_id = snapshot.conversation_id();
    let mut signals_count = 0u32;
    for message in snapshot.messages() {
        let conversation_id = message
            .conversation_id
            .clone()
            .or_else(|| snapshot_conversation_id.clone())
            .ok_or_else(|| {
                crate::errors::AppError::bad_request("transcript message missing conversation_id")
            })?;
        let source = message.source.clone().unwrap_or_else(|| {
            snapshot_source
                .clone()
                .unwrap_or_else(|| "transcript".to_string())
        });
        let timestamp = message.timestamp;
        let role = message.role.trim().to_string();
        let content = message.content.trim().to_string();
        if role.is_empty() || content.is_empty() {
            continue;
        }

        let transcript_id = message.id.clone().unwrap_or_else(|| {
            stable_transcript_id(&source, &conversation_id, timestamp, &role, &content)
        });
        let metadata = message.metadata.unwrap_or_else(|| serde_json::json!({}));
        let inserted = storage
            .insert_assistant_transcript(AssistantTranscriptInsert {
                id: transcript_id.clone(),
                source: source.clone(),
                conversation_id: conversation_id.clone(),
                timestamp,
                role: role.clone(),
                content: content.clone(),
                metadata_json: metadata.clone(),
            })
            .await
            .map_err(crate::errors::AppError::from)?;

        if !inserted {
            continue;
        }

        storage
            .insert_signal(SignalInsert {
                signal_type: "assistant_message".to_string(),
                source: source.clone(),
                source_ref: Some(transcript_id.clone()),
                timestamp,
                payload_json: Some(serde_json::json!({
                    "transcript_id": transcript_id,
                    "conversation_id": conversation_id,
                    "role": role,
                    "content": content,
                    "metadata": metadata,
                })),
            })
            .await
            .map_err(crate::errors::AppError::from)?;
        signals_count += 1;
    }

    Ok(signals_count)
}

fn stable_transcript_id(
    source: &str,
    conversation_id: &str,
    timestamp: i64,
    role: &str,
    content: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(source.as_bytes());
    hasher.update(b"|");
    hasher.update(conversation_id.as_bytes());
    hasher.update(b"|");
    hasher.update(timestamp.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(role.as_bytes());
    hasher.update(b"|");
    hasher.update(content.as_bytes());
    let digest = hasher.finalize();
    format!("tr_{}", hex::encode(&digest[..8]))
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum TranscriptSnapshot {
    Envelope(TranscriptEnvelope),
    Messages(Vec<TranscriptMessage>),
}

impl TranscriptSnapshot {
    fn source(&self) -> Option<String> {
        match self {
            TranscriptSnapshot::Envelope(envelope) => envelope.source.clone(),
            TranscriptSnapshot::Messages(_) => None,
        }
    }

    fn conversation_id(&self) -> Option<String> {
        match self {
            TranscriptSnapshot::Envelope(envelope) => envelope.conversation_id.clone(),
            TranscriptSnapshot::Messages(_) => None,
        }
    }

    fn messages(self) -> Vec<TranscriptMessage> {
        match self {
            TranscriptSnapshot::Envelope(envelope) => envelope.messages,
            TranscriptSnapshot::Messages(messages) => messages,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct TranscriptEnvelope {
    source: Option<String>,
    conversation_id: Option<String>,
    #[serde(default)]
    messages: Vec<TranscriptMessage>,
}

#[derive(Debug, serde::Deserialize)]
struct TranscriptMessage {
    id: Option<String>,
    source: Option<String>,
    conversation_id: Option<String>,
    timestamp: i64,
    role: String,
    content: String,
    metadata: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_id_is_deterministic() {
        let a = stable_transcript_id("chatgpt", "conv_1", 100, "user", "hello");
        let b = stable_transcript_id("chatgpt", "conv_1", 100, "user", "hello");
        assert_eq!(a, b);
        assert!(a.starts_with("tr_"));
    }
}
