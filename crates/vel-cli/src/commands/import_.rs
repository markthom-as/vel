//! `vel import` — import file, lines from stdin, capture URL, or a codex-workspace vault.

use crate::client::ApiClient;
use anyhow::{bail, Context};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};
use vel_api_types::{
    BatchImportCapture, BatchImportItem, BatchImportProject, BatchImportRequest,
    BatchImportSignal, CaptureCreateRequest, ProjectFamilyData, ProjectProvisionRequestData,
    ProjectRootRefData, ProjectStatusData,
};
use vel_core::CaptureId;

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

pub async fn run_codex_workspace(client: &ApiClient, path: &str) -> anyhow::Result<()> {
    let root = std::fs::canonicalize(path)
        .with_context(|| format!("resolving codex-workspace path {}", path))?;
    if !root.is_dir() {
        bail!("codex-workspace path must be a directory");
    }

    let items = build_codex_workspace_items(&root)?;
    if items.is_empty() {
        println!("no items found in workspace");
        return Ok(());
    }

    let response = client
        .import_batch(BatchImportRequest { items })
        .await
        .context("batch import")?;
    let data = response.data.expect("import response missing data");

    let mut summary = WorkspaceImportSummary::default();
    for result in &data.results {
        match (result.kind.as_str(), result.status) {
            ("project", vel_api_types::BatchImportItemStatus::Created) => {
                summary.projects_created += 1
            }
            ("project", vel_api_types::BatchImportItemStatus::Skipped) => {
                summary.projects_skipped += 1
            }
            ("capture", vel_api_types::BatchImportItemStatus::Created) => {
                // Distinguish note vs routine by checking if the capture_id prefix is present
                // We track this based on our item ordering: projects first, then notes, then routines
                // But simpler: just count total captures
                summary.note_captures_created += 1;
            }
            ("capture", vel_api_types::BatchImportItemStatus::Skipped) => {
                summary.note_captures_skipped += 1;
            }
            _ => {}
        }
    }

    println!("projects created: {}", summary.projects_created);
    println!("projects skipped: {}", summary.projects_skipped);
    println!(
        "captures created: {}",
        data.summary.created.saturating_sub(summary.projects_created as usize)
    );
    println!(
        "captures skipped: {}",
        data.summary.skipped.saturating_sub(summary.projects_skipped as usize)
    );
    println!(
        "total: {} created, {} skipped, {} errors",
        data.summary.created, data.summary.skipped, data.summary.errors,
    );
    Ok(())
}

pub fn build_codex_workspace_items(root: &Path) -> anyhow::Result<Vec<BatchImportItem>> {
    let mut items = Vec::new();

    // Projects from registry
    let registry_path = root.join("schemas/project-registry.md");
    if registry_path.exists() {
        let content = std::fs::read_to_string(&registry_path)
            .with_context(|| format!("reading {}", registry_path.display()))?;
        for entry in parse_project_registry(&content) {
            let slug = slugify_project_name(&entry.project);
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

            items.push(BatchImportItem::Project(BatchImportProject {
                slug,
                name: entry.project.trim().to_string(),
                family: infer_project_family_data(&entry),
                status: Some(map_project_status_data(entry.status.as_deref())),
                primary_repo: path_to_root_ref_data(repo_path, "repo", &entry.project),
                primary_notes_root: path_to_root_ref_data(notes_path, "notes_root", &entry.project),
                secondary_repos: vec![],
                secondary_notes_roots: vec![],
                upstream_ids: BTreeMap::new(),
                pending_provision: ProjectProvisionRequestData::default(),
            }));
        }
    }

    // Note documents
    build_document_items(root, &workspace_note_files(root), NOTE_CAPTURE_TYPE, &mut items)?;

    // Routine documents
    build_document_items(
        root,
        &workspace_routine_files(root),
        ROUTINE_CAPTURE_TYPE,
        &mut items,
    )?;

    Ok(items)
}

fn build_document_items(
    root: &Path,
    files: &[PathBuf],
    capture_type: &str,
    items: &mut Vec<BatchImportItem>,
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

        items.push(BatchImportItem::Capture(BatchImportCapture {
            capture_id: capture_id.to_string(),
            content_text: trimmed.to_string(),
            capture_type: capture_type.to_string(),
            source_device: Some(CODEX_WORKSPACE_SOURCE_DEVICE.to_string()),
        }));

        items.push(BatchImportItem::Signal(BatchImportSignal {
            signal_type: capture_type.to_string(),
            source: CODEX_WORKSPACE_SIGNAL_SOURCE.to_string(),
            source_ref: Some(format!("{}:{}", CODEX_WORKSPACE_SIGNAL_SOURCE, capture_id)),
            timestamp: Some(modified_at),
            payload: serde_json::json!({
                "capture_id": capture_id.to_string(),
                "path": relative_path,
                "title": extract_title(trimmed, file_path),
                "modified_at": modified_at,
            }),
        }));
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

fn infer_project_family_data(entry: &ProjectRegistryEntry) -> ProjectFamilyData {
    match entry.project_type.as_deref().map(str::trim) {
        Some(value) if value.eq_ignore_ascii_case("personal") => ProjectFamilyData::Personal,
        Some(value) if value.eq_ignore_ascii_case("creative") => ProjectFamilyData::Creative,
        Some(value) if value.eq_ignore_ascii_case("work") => ProjectFamilyData::Work,
        _ => {
            let lower = entry.project.trim().to_ascii_lowercase();
            match lower.as_str() {
                "personal" | "inbox" | "disability" => ProjectFamilyData::Personal,
                "creative" | "magic" | "memoir" | "occulted" | "spirit supply"
                | "wetware gallery" | "witchaid" | "materia" => ProjectFamilyData::Creative,
                _ => ProjectFamilyData::Work,
            }
        }
    }
}

fn map_project_status_data(status: Option<&str>) -> ProjectStatusData {
    match status.map(str::trim) {
        Some(value) if value.eq_ignore_ascii_case("paused") => ProjectStatusData::Paused,
        Some(value) if value.eq_ignore_ascii_case("archived") => ProjectStatusData::Archived,
        _ => ProjectStatusData::Active,
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

fn path_to_root_ref_data(path: PathBuf, kind: &str, fallback_label: &str) -> ProjectRootRefData {
    let label = path
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or(fallback_label)
        .to_string();
    ProjectRootRefData {
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

    #[test]
    fn build_codex_workspace_items_produces_correct_types() {
        let root = test_dir("codex-items");
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

        let items = build_codex_workspace_items(&root).unwrap();

        // 1 project + 3 notes (daily, correspondence, project-registry schema) × 2 (capture+signal each) + 1 routine × 2
        let project_count = items
            .iter()
            .filter(|i| matches!(i, BatchImportItem::Project(_)))
            .count();
        let capture_count = items
            .iter()
            .filter(|i| matches!(i, BatchImportItem::Capture(_)))
            .count();
        let signal_count = items
            .iter()
            .filter(|i| matches!(i, BatchImportItem::Signal(_)))
            .count();

        assert_eq!(project_count, 1, "should have 1 project");
        assert!(capture_count >= 3, "should have at least 3 captures (notes + routine), got {}", capture_count);
        assert_eq!(capture_count, signal_count, "each capture should have a paired signal");

        // node_modules content should not appear
        let has_node_modules = items.iter().any(|i| match i {
            BatchImportItem::Capture(c) => c.content_text.contains("should not import"),
            _ => false,
        });
        assert!(!has_node_modules, "node_modules should be excluded");

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn stable_capture_ids_are_deterministic() {
        let id1 = stable_capture_id("note_document", "daily/2026-01-01.md", 1000, "hello");
        let id2 = stable_capture_id("note_document", "daily/2026-01-01.md", 1000, "hello");
        assert_eq!(id1, id2);

        let id3 = stable_capture_id("note_document", "daily/2026-01-01.md", 1001, "hello");
        assert_ne!(id1, id3);
    }
}
