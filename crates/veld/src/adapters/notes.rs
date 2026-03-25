//! Notes adapter: ingest markdown/plaintext files into captures with replay-safe deterministic IDs.
#![allow(dead_code)] // Scoped note writeback is staged but not yet reachable from active flows.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use vel_config::AppConfig;
use vel_core::{CaptureId, PrivacyClass, ProjectId, ProjectRecord};
use vel_storage::{CaptureInsert, SignalInsert, Storage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AllowedNotesRoot {
    pub path: PathBuf,
    pub label: String,
    pub project_id: Option<ProjectId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScopedNoteWriteResult {
    pub root_path: PathBuf,
    pub relative_path: String,
    pub absolute_path: PathBuf,
    pub bytes_written: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ScopedNoteWriteError {
    NoAllowedRoots,
    InvalidPath(String),
    Blocked(String),
    Io(String),
}

impl std::fmt::Display for ScopedNoteWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoAllowedRoots => f.write_str("notes writeback has no allowed roots"),
            Self::InvalidPath(message) | Self::Blocked(message) | Self::Io(message) => {
                f.write_str(message)
            }
        }
    }
}

pub async fn ingest(storage: &Storage, config: &AppConfig) -> Result<u32, crate::errors::AppError> {
    let Some(notes_path) = &config.notes_path else {
        return Ok(0);
    };
    match tokio::fs::try_exists(notes_path).await {
        Ok(true) => {}
        Ok(false) if vel_config::is_default_local_source_path("notes", notes_path) => return Ok(0),
        Ok(false) => {
            return Err(crate::errors::AppError::internal(format!(
                "stat notes path {}: No such file or directory",
                notes_path
            )));
        }
        Err(error) => {
            return Err(crate::errors::AppError::internal(format!(
                "stat notes path {}: {}",
                notes_path, error
            )));
        }
    }

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
                source_ref: Some(format!("notes:{}", capture_id)),
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
        storage
            .upsert_note_semantic_record(
                &relative_path,
                &title,
                content,
                capture_id.as_ref(),
                modified_at,
            )
            .await
            .map_err(crate::errors::AppError::from)?;
        count += 1;
    }

    Ok(count)
}

pub(crate) fn allowed_write_roots(
    config: &AppConfig,
    projects: &[ProjectRecord],
) -> Vec<AllowedNotesRoot> {
    let mut roots = Vec::new();
    if let Some(notes_path) = config
        .notes_path
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        roots.push(AllowedNotesRoot {
            path: PathBuf::from(notes_path),
            label: "notes_path".to_string(),
            project_id: None,
        });
    }

    for project in projects {
        roots.push(AllowedNotesRoot {
            path: PathBuf::from(&project.primary_notes_root.path),
            label: format!("project:{}:primary_notes_root", project.id),
            project_id: Some(project.id.clone()),
        });
        roots.extend(
            project
                .secondary_notes_roots
                .iter()
                .enumerate()
                .map(|(index, root)| AllowedNotesRoot {
                    path: PathBuf::from(&root.path),
                    label: format!("project:{}:secondary_notes_roots:{}", project.id, index),
                    project_id: Some(project.id.clone()),
                }),
        );
    }

    roots
}

pub(crate) async fn write_scoped_note(
    roots: &[AllowedNotesRoot],
    requested_path: &str,
    preferred_root: Option<&str>,
    content: &str,
    append: bool,
) -> Result<ScopedNoteWriteResult, ScopedNoteWriteError> {
    if roots.is_empty() {
        return Err(ScopedNoteWriteError::NoAllowedRoots);
    }

    let requested = requested_path.trim();
    if requested.is_empty() {
        return Err(ScopedNoteWriteError::InvalidPath(
            "notes path must not be empty".to_string(),
        ));
    }
    let requested_path = PathBuf::from(requested);
    let eligible_roots = filter_roots(roots, preferred_root)?;
    let resolved = if requested_path.is_absolute() {
        resolve_absolute_write_path(&eligible_roots, &requested_path)?
    } else {
        resolve_relative_write_path(&eligible_roots, &requested_path)?
    };

    if !append
        && tokio::fs::try_exists(&resolved.absolute_path)
            .await
            .map_err(|error| ScopedNoteWriteError::Io(error.to_string()))?
    {
        return Err(ScopedNoteWriteError::InvalidPath(format!(
            "notes_create_note refuses to overwrite existing file {}",
            resolved.absolute_path.display()
        )));
    }

    if let Some(parent) = resolved.absolute_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|error| ScopedNoteWriteError::Io(error.to_string()))?;
    }

    let bytes_written = content.len();
    if append {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&resolved.absolute_path)
            .await
            .map_err(|error| ScopedNoteWriteError::Io(error.to_string()))?;
        file.write_all(content.as_bytes())
            .await
            .map_err(|error| ScopedNoteWriteError::Io(error.to_string()))?;
    } else {
        tokio::fs::write(&resolved.absolute_path, content)
            .await
            .map_err(|error| ScopedNoteWriteError::Io(error.to_string()))?;
    }

    Ok(ScopedNoteWriteResult {
        root_path: resolved.root.path.clone(),
        relative_path: resolved.relative_path,
        absolute_path: resolved.absolute_path,
        bytes_written,
    })
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

fn filter_roots<'a>(
    roots: &'a [AllowedNotesRoot],
    preferred_root: Option<&str>,
) -> Result<Vec<&'a AllowedNotesRoot>, ScopedNoteWriteError> {
    let Some(preferred_root) = preferred_root
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Ok(roots.iter().collect());
    };
    let preferred = normalize_path(Path::new(preferred_root)).ok_or_else(|| {
        ScopedNoteWriteError::InvalidPath(format!("invalid notes root {}", preferred_root))
    })?;
    let filtered: Vec<&AllowedNotesRoot> = roots
        .iter()
        .filter(|root| normalize_path(&root.path).as_ref() == Some(&preferred))
        .collect();
    if filtered.is_empty() {
        return Err(ScopedNoteWriteError::Blocked(format!(
            "notes write blocked because {} is outside the configured notes root set",
            preferred_root
        )));
    }
    Ok(filtered)
}

#[derive(Debug, Clone)]
struct ResolvedNotePath<'a> {
    root: &'a AllowedNotesRoot,
    absolute_path: PathBuf,
    relative_path: String,
}

fn resolve_absolute_write_path<'a>(
    roots: &[&'a AllowedNotesRoot],
    requested_path: &Path,
) -> Result<ResolvedNotePath<'a>, ScopedNoteWriteError> {
    let normalized_requested = normalize_path(requested_path).ok_or_else(|| {
        ScopedNoteWriteError::InvalidPath(format!(
            "invalid notes path {}",
            requested_path.display()
        ))
    })?;
    for root in roots {
        let Some(normalized_root) = normalize_path(&root.path) else {
            continue;
        };
        if normalized_requested == normalized_root
            || normalized_requested.starts_with(&normalized_root)
        {
            let relative_path = if normalized_requested == normalized_root {
                normalized_requested
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or("note.md")
                    .to_string()
            } else {
                normalize_relative_path(&normalized_root, &normalized_requested)
            };
            return Ok(ResolvedNotePath {
                root,
                absolute_path: normalized_requested,
                relative_path,
            });
        }
    }
    Err(ScopedNoteWriteError::Blocked(format!(
        "notes write blocked because {} is outside the configured notes root set",
        requested_path.display()
    )))
}

fn resolve_relative_write_path<'a>(
    roots: &[&'a AllowedNotesRoot],
    requested_path: &Path,
) -> Result<ResolvedNotePath<'a>, ScopedNoteWriteError> {
    for root in roots {
        let Some(normalized_root) = normalize_path(&root.path) else {
            continue;
        };
        if !normalized_root.is_dir() && root.path.extension().is_some() {
            continue;
        }
        let candidate = normalize_path(&normalized_root.join(requested_path)).ok_or_else(|| {
            ScopedNoteWriteError::InvalidPath(format!(
                "invalid relative notes path {}",
                requested_path.display()
            ))
        })?;
        if candidate.starts_with(&normalized_root) {
            return Ok(ResolvedNotePath {
                root,
                absolute_path: candidate,
                relative_path: requested_path.to_string_lossy().replace('\\', "/"),
            });
        }
    }
    Err(ScopedNoteWriteError::Blocked(format!(
        "notes write blocked because {} escapes the configured notes root set",
        requested_path.display()
    )))
}

fn normalize_path(path: &Path) -> Option<PathBuf> {
    use std::path::Component;

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    return None;
                }
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    Some(normalized)
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
    use std::time::{SystemTime, UNIX_EPOCH};
    use vel_core::{ProjectFamily, ProjectProvisionRequest, ProjectRootRef, ProjectStatus};

    fn unique_temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("vel_{label}_{nanos}"))
    }

    fn sample_project(notes_root: &Path) -> ProjectRecord {
        let now = time::OffsetDateTime::now_utc();
        ProjectRecord {
            id: "proj_notes".to_string().into(),
            slug: "notes".to_string(),
            name: "Notes".to_string(),
            family: ProjectFamily::Work,
            status: ProjectStatus::Active,
            primary_repo: ProjectRootRef {
                path: "/tmp/repo".to_string(),
                label: "repo".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRef {
                path: notes_root.display().to_string(),
                label: "notes".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: vec![],
            secondary_notes_roots: vec![],
            upstream_ids: Default::default(),
            pending_provision: ProjectProvisionRequest {
                create_repo: false,
                create_notes_root: false,
            },
            created_at: now,
            updated_at: now,
            archived_at: None,
        }
    }

    #[test]
    fn stable_capture_id_is_deterministic() {
        let a = stable_capture_id("daily/today.md", 1700000000, "# Today");
        let b = stable_capture_id("daily/today.md", 1700000000, "# Today");
        assert_eq!(a, b);
        assert!(a.starts_with("cap_note_"));
    }

    #[test]
    fn allowed_write_roots_collects_global_and_project_notes_roots() {
        let config = AppConfig {
            notes_path: Some("/tmp/notes".to_string()),
            ..Default::default()
        };
        let project = sample_project(Path::new("/tmp/project-notes"));

        let roots = allowed_write_roots(&config, &[project]);

        assert_eq!(roots.len(), 2);
        assert_eq!(roots[0].label, "notes_path");
        assert!(roots[1].label.contains("primary_notes_root"));
    }

    #[tokio::test]
    async fn write_scoped_note_blocks_escape_outside_allowed_roots() {
        let root = unique_temp_dir("notes_root_block");
        tokio::fs::create_dir_all(&root).await.unwrap();
        let roots = vec![AllowedNotesRoot {
            path: root.clone(),
            label: "notes_path".to_string(),
            project_id: None,
        }];

        let error = write_scoped_note(&roots, "../escape.md", None, "blocked", false)
            .await
            .unwrap_err();

        assert!(matches!(error, ScopedNoteWriteError::Blocked(_)));
        let _ = std::fs::remove_dir_all(root);
    }

    #[tokio::test]
    async fn write_scoped_note_supports_relative_project_writes() {
        let root = unique_temp_dir("notes_root_apply");
        tokio::fs::create_dir_all(&root).await.unwrap();
        let roots = vec![AllowedNotesRoot {
            path: root.clone(),
            label: "project:proj_notes:primary_notes_root".to_string(),
            project_id: Some("proj_notes".to_string().into()),
        }];

        let result = write_scoped_note(&roots, "daily/today.md", None, "# Today\n", false)
            .await
            .unwrap();

        assert_eq!(result.relative_path, "daily/today.md");
        assert_eq!(
            tokio::fs::read_to_string(root.join("daily/today.md"))
                .await
                .unwrap(),
            "# Today\n"
        );
        let _ = std::fs::remove_dir_all(root);
    }
}
