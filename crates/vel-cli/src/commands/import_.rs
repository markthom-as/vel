//! `vel import` — import file, lines from stdin, capture URL, or a codex-workspace vault.

use crate::client::ApiClient;
use anyhow::{bail, Context};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};
use vel_api_types::CaptureCreateRequest;
use vel_config::AppConfig;
use vel_core::{
    CaptureId, PrivacyClass, ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectRecord,
    ProjectRootRef, ProjectStatus,
};
use vel_storage::{CaptureInsert, SignalInsert, Storage};

const CODEX_WORKSPACE_SOURCE_DEVICE: &str = "codex-workspace-import";
const CODEX_WORKSPACE_SIGNAL_SOURCE: &str = "codex_workspace";
const NOTE_CAPTURE_TYPE: &str = "note_document";
const ROUTINE_CAPTURE_TYPE: &str = "routine_document";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct WorkspaceImportSummary {
    projects_created: u32,
    projects_skipped: u32,
    note_captures_created: u32,
    note_captures_skipped: u32,
    routine_captures_created: u32,
    routine_captures_skipped: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectRegistryEntry {
    project: String,
    todoist: Option<String>,
    brain_path: Option<String>,
    status: Option<String>,
    project_type: Option<String>,
    repo: Option<String>,
}

fn read_stdin() -> anyhow::Result<String> {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}

pub async fn run_file(client: &ApiClient, path: &str, capture_type: &str) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(Path::new(path))?;
    let content = content.trim();
    if content.is_empty() {
        bail!("file is empty");
    }
    let request = CaptureCreateRequest {
        content_text: content.to_string(),
        capture_type: capture_type.to_string(),
        source_device: Some("import-file".to_string()),
    };
    let response = client.capture(request).await?;
    let data = response.data.expect("capture response missing data");
    println!("capture_id: {}", data.capture_id);
    Ok(())
}

pub async fn run_lines(client: &ApiClient, capture_type: &str) -> anyhow::Result<()> {
    let stdin = read_stdin()?;
    let lines: Vec<&str> = stdin
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if lines.is_empty() {
        bail!("no non-empty lines on stdin");
    }
    let mut ids = Vec::new();
    for line in lines {
        let request = CaptureCreateRequest {
            content_text: line.to_string(),
            capture_type: capture_type.to_string(),
            source_device: Some("import-lines".to_string()),
        };
        let response = client.capture(request).await?;
        let data = response.data.expect("capture response missing data");
        ids.push(data.capture_id.to_string());
    }
    println!("Created {} capture(s): {}", ids.len(), ids.join(", "));
    Ok(())
}

pub async fn run_capture_url(client: &ApiClient, url: &str) -> anyhow::Result<()> {
    let request = CaptureCreateRequest {
        content_text: url.trim().to_string(),
        capture_type: "url".to_string(),
        source_device: Some("vel-cli".to_string()),
    };
    let response = client.capture(request).await?;
    let data = response.data.expect("capture response missing data");
    println!("capture_id: {}", data.capture_id);
    Ok(())
}

pub async fn run_codex_workspace(config: &AppConfig, path: &str) -> anyhow::Result<()> {
    let root = std::fs::canonicalize(path)
        .with_context(|| format!("resolving codex-workspace path {}", path))?;
    if !root.is_dir() {
        bail!("codex-workspace path must be a directory");
    }

    let storage = Storage::connect(&config.db_path)
        .await
        .with_context(|| format!("connecting db {}", config.db_path))?;
    storage.migrate().await.context("running migrations")?;

    let summary = import_codex_workspace(&storage, &root).await?;
    println!("projects created: {}", summary.projects_created);
    println!("projects skipped: {}", summary.projects_skipped);
    println!("note captures created: {}", summary.note_captures_created);
    println!("note captures skipped: {}", summary.note_captures_skipped);
    println!(
        "routine captures created: {}",
        summary.routine_captures_created
    );
    println!(
        "routine captures skipped: {}",
        summary.routine_captures_skipped
    );
    Ok(())
}

async fn import_codex_workspace(
    storage: &Storage,
    root: &Path,
) -> anyhow::Result<WorkspaceImportSummary> {
    let mut summary = WorkspaceImportSummary::default();
    import_projects(storage, root, &mut summary).await?;
    import_documents(
        storage,
        root,
        &workspace_note_files(root),
        NOTE_CAPTURE_TYPE,
        &mut summary.note_captures_created,
        &mut summary.note_captures_skipped,
    )
    .await?;
    import_documents(
        storage,
        root,
        &workspace_routine_files(root),
        ROUTINE_CAPTURE_TYPE,
        &mut summary.routine_captures_created,
        &mut summary.routine_captures_skipped,
    )
    .await?;
    Ok(summary)
}

async fn import_projects(
    storage: &Storage,
    root: &Path,
    summary: &mut WorkspaceImportSummary,
) -> anyhow::Result<()> {
    let registry_path = root.join("schemas/project-registry.md");
    if !registry_path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&registry_path)
        .with_context(|| format!("reading {}", registry_path.display()))?;
    for entry in parse_project_registry(&content) {
        let slug = slugify_project_name(&entry.project);
        if storage
            .get_project_by_slug(&slug)
            .await
            .with_context(|| format!("checking existing project {}", slug))?
            .is_some()
        {
            summary.projects_skipped += 1;
            continue;
        }

        let notes_path = entry
            .brain_path
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(|value| workspace_join(root, value))
            .unwrap_or_else(|| root.join("Projects").join(&entry.project));
        let repo_path = entry
            .repo
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(|value| workspace_join(root, value))
            .unwrap_or_else(|| notes_path.clone());

        let record = ProjectRecord {
            id: ProjectId::new(),
            slug,
            name: entry.project.trim().to_string(),
            family: infer_project_family(&entry),
            status: map_project_status(entry.status.as_deref()),
            primary_repo: root_ref(repo_path, "repo", &entry.project),
            primary_notes_root: root_ref(notes_path, "notes_root", &entry.project),
            secondary_repos: vec![],
            secondary_notes_roots: vec![],
            upstream_ids: BTreeMap::new(),
            pending_provision: ProjectProvisionRequest {
                create_repo: false,
                create_notes_root: false,
            },
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
            archived_at: None,
        };

        storage
            .create_project(record)
            .await
            .with_context(|| format!("creating project {}", entry.project.trim()))?;
        summary.projects_created += 1;
    }

    Ok(())
}

async fn import_documents(
    storage: &Storage,
    root: &Path,
    files: &[PathBuf],
    capture_type: &str,
    created: &mut u32,
    skipped: &mut u32,
) -> anyhow::Result<()> {
    for file_path in files {
        let content = std::fs::read_to_string(file_path)
            .with_context(|| format!("reading {}", file_path.display()))?;
        let trimmed = content.trim();
        if trimmed.is_empty() {
            continue;
        }

        let modified_at = std::fs::metadata(file_path)
            .with_context(|| format!("stat {}", file_path.display()))?
            .modified()
            .ok()
            .and_then(|value| value.duration_since(UNIX_EPOCH).ok())
            .map(|value| value.as_secs() as i64)
            .unwrap_or(0);
        let relative_path = normalize_relative_path(root, file_path);
        let capture_id = CaptureId::from(stable_capture_id(
            capture_type,
            &relative_path,
            modified_at,
            trimmed,
        ));
        let inserted = storage
            .insert_capture_with_id(
                capture_id.clone(),
                CaptureInsert {
                    content_text: trimmed.to_string(),
                    capture_type: capture_type.to_string(),
                    source_device: Some(CODEX_WORKSPACE_SOURCE_DEVICE.to_string()),
                    privacy_class: PrivacyClass::Private,
                },
            )
            .await
            .with_context(|| format!("inserting capture for {}", relative_path))?;

        if !inserted {
            *skipped += 1;
            continue;
        }

        storage
            .insert_signal(SignalInsert {
                signal_type: capture_type.to_string(),
                source: CODEX_WORKSPACE_SIGNAL_SOURCE.to_string(),
                source_ref: Some(format!("{}:{}", CODEX_WORKSPACE_SIGNAL_SOURCE, capture_id)),
                timestamp: modified_at,
                payload_json: Some(serde_json::json!({
                    "capture_id": capture_id.to_string(),
                    "path": relative_path,
                    "title": extract_title(trimmed, file_path),
                    "modified_at": modified_at,
                })),
            })
            .await
            .with_context(|| format!("inserting signal for {}", file_path.display()))?;
        *created += 1;
    }

    Ok(())
}

fn workspace_note_files(root: &Path) -> Vec<PathBuf> {
    let mut files = BTreeSet::new();
    for relative in [
        "daily",
        "inbox",
        "Projects",
        "docs",
        "workflows",
        "reports",
        "schemas",
        "data/Correspondence",
        "data/astro",
    ] {
        collect_files_with_extensions(
            &workspace_join(root, relative),
            &["md", "markdown", "txt"],
            &mut files,
        );
    }

    for relative in ["README.md", "dashboard.md", "index.md", "todo.md"] {
        let path = workspace_join(root, relative);
        if path.is_file() && has_supported_extension(&path, &["md", "markdown", "txt"]) {
            files.insert(path);
        }
    }

    files.into_iter().collect()
}

fn workspace_routine_files(root: &Path) -> Vec<PathBuf> {
    let mut files = BTreeSet::new();
    collect_files_with_extensions(
        &workspace_join(root, "schedules"),
        &["yaml", "yml"],
        &mut files,
    );
    files.into_iter().collect()
}

fn collect_files_with_extensions(root: &Path, extensions: &[&str], files: &mut BTreeSet<PathBuf>) {
    if !root.exists() {
        return;
    }
    if root.is_file() {
        if has_supported_extension(root, extensions) {
            files.insert(root.to_path_buf());
        }
        return;
    }

    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files_with_extensions(&path, extensions, files);
        } else if path.is_file() && has_supported_extension(&path, extensions) {
            files.insert(path);
        }
    }
}

fn has_supported_extension(path: &Path, extensions: &[&str]) -> bool {
    let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
        return false;
    };
    extensions
        .iter()
        .any(|candidate| extension.eq_ignore_ascii_case(candidate))
}

fn parse_project_registry(content: &str) -> Vec<ProjectRegistryEntry> {
    let mut rows = content
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|'))
        .filter(|line| !line.contains("---"));

    let Some(header_line) = rows.next() else {
        return Vec::new();
    };
    let headers = split_markdown_row(header_line);
    let mut entries = Vec::new();

    for row in rows {
        let values = split_markdown_row(row);
        if values.iter().all(|value| value.is_empty()) {
            continue;
        }
        let value = |name: &str| -> Option<String> {
            headers
                .iter()
                .position(|header| header.eq_ignore_ascii_case(name))
                .and_then(|index| values.get(index))
                .map(|item| item.trim())
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
        };

        let Some(project) = value("Project") else {
            continue;
        };
        entries.push(ProjectRegistryEntry {
            project,
            todoist: value("Todoist"),
            brain_path: value("Brain Path"),
            status: value("Status"),
            project_type: value("Type"),
            repo: value("Repo"),
        });
    }

    entries
}

fn split_markdown_row(line: &str) -> Vec<String> {
    line.trim_matches('|')
        .split('|')
        .map(|value| value.trim().to_string())
        .collect()
}

fn slugify_project_name(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;
    for ch in name.trim().chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_alphanumeric() || ch == '.' {
            slug.push(ch);
            previous_dash = false;
        } else if (ch.is_ascii_whitespace() || ch == '-' || ch == '_') && !previous_dash {
            slug.push('-');
            previous_dash = true;
        }
    }
    slug.trim_matches('-').to_string()
}

fn infer_project_family(entry: &ProjectRegistryEntry) -> ProjectFamily {
    match entry.project_type.as_deref().map(str::trim) {
        Some(value) if value.eq_ignore_ascii_case("personal") => ProjectFamily::Personal,
        Some(value) if value.eq_ignore_ascii_case("creative") => ProjectFamily::Creative,
        Some(value) if value.eq_ignore_ascii_case("work") => ProjectFamily::Work,
        _ => {
            let lower = entry.project.trim().to_ascii_lowercase();
            match lower.as_str() {
                "personal" | "inbox" | "disability" => ProjectFamily::Personal,
                "creative" | "magic" | "memoir" | "occulted" | "spirit supply"
                | "wetware gallery" | "witchaid" | "materia" => ProjectFamily::Creative,
                _ => ProjectFamily::Work,
            }
        }
    }
}

fn map_project_status(status: Option<&str>) -> ProjectStatus {
    match status.map(str::trim) {
        Some(value) if value.eq_ignore_ascii_case("paused") => ProjectStatus::Paused,
        Some(value) if value.eq_ignore_ascii_case("archived") => ProjectStatus::Archived,
        _ => ProjectStatus::Active,
    }
}

fn workspace_join(root: &Path, value: &str) -> PathBuf {
    let candidate = Path::new(value.trim());
    if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        root.join(candidate)
    }
}

fn root_ref(path: PathBuf, kind: &str, fallback_label: &str) -> ProjectRootRef {
    let label = path
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or(fallback_label)
        .to_string();
    ProjectRootRef {
        path: path.to_string_lossy().to_string(),
        label,
        kind: kind.to_string(),
    }
}

fn normalize_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn extract_title(content: &str, path: &Path) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed
            .strip_prefix("# ")
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return title.to_string();
        }
    }

    path.file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("document")
        .to_string()
}

fn stable_capture_id(
    capture_type: &str,
    relative_path: &str,
    modified_at: i64,
    content: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(capture_type.as_bytes());
    hasher.update(b"|");
    hasher.update(relative_path.as_bytes());
    hasher.update(b"|");
    hasher.update(modified_at.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(content.as_bytes());
    let digest = hasher.finalize();
    format!("cap_ws_{}", hex::encode(&digest[..8]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "vel-import-{name}-{}-{}",
            std::process::id(),
            time::OffsetDateTime::now_utc().unix_timestamp_nanos()
        ));
        std::fs::create_dir_all(&path).expect("temp dir should be created");
        path
    }

    #[test]
    fn parse_project_registry_reads_markdown_table() {
        let entries = parse_project_registry(
            r#"
| Project | Todoist | Brain Path | Status | Type | Description | Website | Repo | Owner | Sync Todoist | Sync Obsidian | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Materia | Materia | Projects/Materia | active | creative |  |  |  | Jove | yes | yes |  |
| Disability | Disability | Projects/Disability | paused | personal |  |  |  | Jove | yes | yes |  |
"#,
        );

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].project, "Materia");
        assert_eq!(entries[0].brain_path.as_deref(), Some("Projects/Materia"));
        assert_eq!(entries[1].status.as_deref(), Some("paused"));
        assert_eq!(entries[1].project_type.as_deref(), Some("personal"));
    }

    #[tokio::test]
    async fn codex_workspace_import_is_idempotent_and_curated() {
        let root = test_dir("codex-workspace");
        std::fs::create_dir_all(root.join("schemas")).unwrap();
        std::fs::create_dir_all(root.join("daily")).unwrap();
        std::fs::create_dir_all(root.join("data/Correspondence")).unwrap();
        std::fs::create_dir_all(root.join("schedules")).unwrap();
        std::fs::create_dir_all(root.join("node_modules/pkg")).unwrap();

        std::fs::write(
            root.join("schemas/project-registry.md"),
            r#"
| Project | Todoist | Brain Path | Status | Type | Description | Website | Repo | Owner | Sync Todoist | Sync Obsidian | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Materia | Materia | Projects/Materia | active | creative |  |  |  | Jove | yes | yes |  |
"#,
        )
        .unwrap();
        std::fs::write(root.join("daily/2026-01-01.md"), "# Daily\nhello world").unwrap();
        std::fs::write(
            root.join("data/Correspondence/chat.md"),
            "# Correspondence\nimportant note",
        )
        .unwrap();
        std::fs::write(
            root.join("schedules/standup.yaml"),
            "id: standup\nschedule: daily",
        )
        .unwrap();
        std::fs::write(
            root.join("node_modules/pkg/README.md"),
            "# Dependency\nshould not import",
        )
        .unwrap();

        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let summary = import_codex_workspace(&storage, &root).await.unwrap();
        assert_eq!(summary.projects_created, 1);
        assert_eq!(summary.note_captures_created, 3);
        assert_eq!(summary.routine_captures_created, 1);
        assert_eq!(storage.capture_count().await.unwrap(), 4);
        assert_eq!(storage.list_projects().await.unwrap().len(), 1);

        let captures = storage.list_captures_recent(10, false).await.unwrap();
        assert!(captures
            .iter()
            .any(|capture| capture.capture_type == NOTE_CAPTURE_TYPE));
        assert!(captures
            .iter()
            .any(|capture| capture.capture_type == ROUTINE_CAPTURE_TYPE));
        assert!(!captures
            .iter()
            .any(|capture| capture.content_text.contains("should not import")));

        let rerun = import_codex_workspace(&storage, &root).await.unwrap();
        assert_eq!(rerun.projects_created, 0);
        assert_eq!(rerun.projects_skipped, 1);
        assert_eq!(rerun.note_captures_skipped, 3);
        assert_eq!(rerun.routine_captures_skipped, 1);
        assert_eq!(storage.capture_count().await.unwrap(), 4);

        let _ = std::fs::remove_dir_all(root);
    }
}
