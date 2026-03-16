//! Notes adapter: ingest markdown/plaintext files into captures with replay-safe deterministic IDs.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use vel_config::AppConfig;
use vel_core::{CaptureId, PrivacyClass};
use vel_storage::{CaptureInsert, SignalInsert, Storage};

pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let Some(notes_path) = &config.notes_path else {
        return Ok(0);
    };

    let root = PathBuf::from(notes_path);
    let base_dir = if root.is_dir() {
        root.clone()
    } else {
        root.parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    };
    let files = collect_note_files(&root).await?;
    let mut count = 0u32;

    for file_path in files {
        let content = tokio::fs::read_to_string(&file_path).await.map_err(|e| {
            crate::errors::AppError::internal(format!(
                "read notes file {}: {}",
                file_path.display(),
                e
            ))
        })?;
        let content = content.trim();
        if content.is_empty() {
            continue;
        }

        let metadata = tokio::fs::metadata(&file_path).await.map_err(|e| {
            crate::errors::AppError::internal(format!(
                "stat notes file {}: {}",
                file_path.display(),
                e
            ))
        })?;
        let modified_at = metadata
            .modified()
            .ok()
            .and_then(|value| value.duration_since(UNIX_EPOCH).ok())
            .map(|value| value.as_secs() as i64)
            .unwrap_or(0);
        let relative_path = normalize_relative_path(&base_dir, &file_path);
        let title = extract_title(content, &file_path);
        let capture_id = CaptureId::from(stable_capture_id(&relative_path, modified_at, content));
        let inserted = storage
            .insert_capture_with_id(
                capture_id.clone(),
                CaptureInsert {
                    content_text: content.to_string(),
                    capture_type: "note_document".to_string(),
                    source_device: Some("notes-sync".to_string()),
                    privacy_class: PrivacyClass::Private,
                },
            )
            .await
            .map_err(crate::errors::AppError::from)?;
        if !inserted {
            continue;
        }

        storage
            .insert_signal(SignalInsert {
                signal_type: "note_document".to_string(),
                source: "notes".to_string(),
                timestamp: modified_at,
                payload_json: Some(serde_json::json!({
                    "capture_id": capture_id.to_string(),
                    "path": relative_path,
                    "title": title,
                    "modified_at": modified_at,
                })),
            })
            .await
            .map_err(crate::errors::AppError::from)?;
        count += 1;
    }

    Ok(count)
}

async fn collect_note_files(root: &Path) -> Result<Vec<PathBuf>, crate::errors::AppError> {
    let metadata = tokio::fs::metadata(root).await.map_err(|e| {
        crate::errors::AppError::internal(format!("stat notes path {}: {}", root.display(), e))
    })?;
    if metadata.is_file() {
        return Ok(if is_supported_note_file(root) {
            vec![root.to_path_buf()]
        } else {
            Vec::new()
        });
    }

    if !metadata.is_dir() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&dir).await.map_err(|e| {
            crate::errors::AppError::internal(format!("read notes dir {}: {}", dir.display(), e))
        })?;
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            crate::errors::AppError::internal(format!("read dir entry {}: {}", dir.display(), e))
        })? {
            let path = entry.path();
            let entry_type = entry.file_type().await.map_err(|e| {
                crate::errors::AppError::internal(format!(
                    "stat notes entry {}: {}",
                    path.display(),
                    e
                ))
            })?;
            if entry_type.is_dir() {
                stack.push(path);
            } else if entry_type.is_file() && is_supported_note_file(&path) {
                files.push(path);
            }
        }
    }

    files.sort();
    Ok(files)
}

fn is_supported_note_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_ascii_lowercase()),
        Some(ext) if ext == "md" || ext == "markdown" || ext == "txt"
    )
}

fn normalize_relative_path(base_dir: &Path, path: &Path) -> String {
    path.strip_prefix(base_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn extract_title(content: &str, path: &Path) -> String {
    for line in content.lines() {
        let line = line.trim();
        if let Some(title) = line
            .strip_prefix("# ")
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return title.to_string();
        }
    }

    path.file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("note")
        .to_string()
}

fn stable_capture_id(relative_path: &str, modified_at: i64, content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(relative_path.as_bytes());
    hasher.update(b"|");
    hasher.update(modified_at.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(content.as_bytes());
    let digest = hasher.finalize();
    format!("cap_note_{}", hex::encode(&digest[..8]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_capture_id_is_deterministic() {
        let a = stable_capture_id("daily/today.md", 1700000000, "# Today");
        let b = stable_capture_id("daily/today.md", 1700000000, "# Today");
        assert_eq!(a, b);
        assert!(a.starts_with("cap_note_"));
    }
}
