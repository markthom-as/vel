use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::OffsetDateTime;
use tokio::fs;
use vel_api_types::AgentInspectData;
use vel_core::{ProjectRecord, ProjectRootRef};

use crate::{errors::AppError, state::AppState};

const DEFAULT_EXPORT_DIR: &str = ".planning/vel";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextInput {
    pub objective: String,
    #[serde(default)]
    pub repo_brief: String,
    #[serde(default)]
    pub notes_brief: String,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub expected_outputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionRootData {
    pub path: String,
    pub label: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionContextData {
    pub project_id: String,
    pub project_slug: String,
    pub project_name: String,
    pub objective: String,
    pub repo_brief: String,
    pub notes_brief: String,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub expected_outputs: Vec<String>,
    #[serde(default)]
    pub repo_roots: Vec<ExecutionRootData>,
    #[serde(default)]
    pub notes_roots: Vec<ExecutionRootData>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionArtifactFileData {
    pub relative_path: String,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionArtifactPackData {
    pub project_id: String,
    pub project_slug: String,
    pub repo_root: String,
    pub output_dir: String,
    #[serde(default)]
    pub files: Vec<ExecutionArtifactFileData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionExportResultData {
    pub pack: ExecutionArtifactPackData,
    #[serde(default)]
    pub written_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct StoredExecutionContext {
    pub objective: String,
    #[serde(default)]
    pub repo_brief: String,
    #[serde(default)]
    pub notes_brief: String,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub expected_outputs: Vec<String>,
}

pub async fn get_execution_context(
    state: &AppState,
    project_id: &str,
) -> Result<Option<ExecutionContextData>, AppError> {
    let project = match state.storage.get_project(project_id).await? {
        Some(project) => project,
        None => return Ok(None),
    };
    let stored = match state
        .storage
        .get_project_execution_context(project_id)
        .await?
    {
        Some(value) => value,
        None => return Ok(None),
    };

    build_execution_context(project, stored.0, stored.1, stored.2)
        .map(Some)
        .map_err(AppError::internal)
}

pub async fn save_execution_context(
    state: &AppState,
    project_id: &str,
    input: ExecutionContextInput,
) -> Result<ExecutionContextData, AppError> {
    let project = load_project(state, project_id).await?;
    let stored = normalize_input(input)?;
    let now = OffsetDateTime::now_utc();
    let context_json =
        serde_json::to_value(&stored).map_err(|error| AppError::internal(error.to_string()))?;

    state
        .storage
        .upsert_project_execution_context(&project.id, &context_json, now)
        .await?;

    let (context_json, created_at, updated_at) = state
        .storage
        .get_project_execution_context(project.id.as_ref())
        .await?
        .ok_or_else(|| AppError::internal("execution context missing after upsert"))?;

    build_execution_context(project, context_json, created_at, updated_at)
        .map_err(AppError::internal)
}

pub async fn preview_gsd_artifacts(
    state: &AppState,
    project_id: &str,
    output_dir: Option<&str>,
) -> Result<ExecutionArtifactPackData, AppError> {
    let context = load_execution_context(state, project_id).await?;
    let inspect = crate::services::agent_grounding::build_agent_inspect(state).await?;
    render_gsd_artifacts(&context, &inspect, output_dir)
}

pub async fn export_gsd_artifacts(
    state: &AppState,
    project_id: &str,
    output_dir: Option<&str>,
) -> Result<ExecutionExportResultData, AppError> {
    let context = load_execution_context(state, project_id).await?;
    let inspect = crate::services::agent_grounding::build_agent_inspect(state).await?;
    let pack = render_gsd_artifacts(&context, &inspect, output_dir)?;
    let repo_root = PathBuf::from(&pack.repo_root);

    let mut written_paths = Vec::with_capacity(pack.files.len());
    for file in &pack.files {
        let destination = repo_root.join(&file.relative_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|error| AppError::internal(error.to_string()))?;
        }
        fs::write(&destination, &file.contents)
            .await
            .map_err(|error| AppError::internal(error.to_string()))?;
        written_paths.push(destination.to_string_lossy().into_owned());
    }

    Ok(ExecutionExportResultData {
        pack,
        written_paths,
    })
}

pub fn render_gsd_artifacts(
    context: &ExecutionContextData,
    inspect: &AgentInspectData,
    output_dir: Option<&str>,
) -> Result<ExecutionArtifactPackData, AppError> {
    let output_dir = normalize_output_dir(output_dir)?;
    let inspect_json = serde_json::to_string_pretty(inspect)
        .map_err(|error| AppError::internal(error.to_string()))?;

    Ok(ExecutionArtifactPackData {
        project_id: context.project_id.clone(),
        project_slug: context.project_slug.clone(),
        repo_root: context
            .repo_roots
            .first()
            .map(|root| root.path.clone())
            .ok_or_else(|| AppError::internal("execution context missing primary repo root"))?,
        output_dir: output_dir.clone(),
        files: vec![
            ExecutionArtifactFileData {
                relative_path: join_relative(&output_dir, "execution-context.md"),
                contents: render_execution_context_markdown(context),
            },
            ExecutionArtifactFileData {
                relative_path: join_relative(&output_dir, "gsd-handoff.md"),
                contents: render_handoff_markdown(context),
            },
            ExecutionArtifactFileData {
                relative_path: join_relative(&output_dir, "agent-grounding.md"),
                contents: crate::services::agent_grounding::render_agent_grounding_markdown(
                    inspect,
                ),
            },
            ExecutionArtifactFileData {
                relative_path: join_relative(&output_dir, "agent-inspect.json"),
                contents: inspect_json,
            },
        ],
    })
}

async fn load_project(state: &AppState, project_id: &str) -> Result<ProjectRecord, AppError> {
    state
        .storage
        .get_project(project_id)
        .await?
        .ok_or_else(|| AppError::not_found("project not found"))
}

async fn load_execution_context(
    state: &AppState,
    project_id: &str,
) -> Result<ExecutionContextData, AppError> {
    get_execution_context(state, project_id)
        .await?
        .ok_or_else(|| AppError::not_found("execution context not found"))
}

fn build_execution_context(
    project: ProjectRecord,
    context_json: JsonValue,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
) -> Result<ExecutionContextData, String> {
    let stored: StoredExecutionContext =
        serde_json::from_value(context_json).map_err(|error| error.to_string())?;
    let repo_roots = project_repo_roots(&project);
    let notes_roots = project_notes_roots(&project);

    Ok(ExecutionContextData {
        project_id: project.id.to_string(),
        project_slug: project.slug,
        project_name: project.name,
        objective: stored.objective,
        repo_brief: stored.repo_brief,
        notes_brief: stored.notes_brief,
        constraints: stored.constraints,
        expected_outputs: stored.expected_outputs,
        repo_roots,
        notes_roots,
        created_at,
        updated_at,
    })
}

fn normalize_input(input: ExecutionContextInput) -> Result<StoredExecutionContext, AppError> {
    let objective = input.objective.trim();
    if objective.is_empty() {
        return Err(AppError::bad_request(
            "execution objective must not be empty",
        ));
    }

    Ok(StoredExecutionContext {
        objective: objective.to_string(),
        repo_brief: input.repo_brief.trim().to_string(),
        notes_brief: input.notes_brief.trim().to_string(),
        constraints: normalize_lines(input.constraints),
        expected_outputs: normalize_lines(input.expected_outputs),
    })
}

fn normalize_lines(items: Vec<String>) -> Vec<String> {
    let mut normalized = Vec::new();
    for item in items {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !normalized.iter().any(|existing| existing == trimmed) {
            normalized.push(trimmed.to_string());
        }
    }
    normalized
}

fn project_repo_roots(project: &ProjectRecord) -> Vec<ExecutionRootData> {
    let mut roots = vec![root_data(&project.primary_repo)];
    roots.extend(project.secondary_repos.iter().map(root_data));
    roots
}

fn project_notes_roots(project: &ProjectRecord) -> Vec<ExecutionRootData> {
    let mut roots = vec![root_data(&project.primary_notes_root)];
    roots.extend(project.secondary_notes_roots.iter().map(root_data));
    roots
}

fn root_data(root: &ProjectRootRef) -> ExecutionRootData {
    ExecutionRootData {
        path: root.path.clone(),
        label: root.label.clone(),
        kind: root.kind.clone(),
    }
}

fn normalize_output_dir(output_dir: Option<&str>) -> Result<String, AppError> {
    let candidate = output_dir
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_EXPORT_DIR);
    let path = Path::new(candidate);
    if path.is_absolute() {
        return Err(AppError::bad_request(
            "execution artifact output must stay inside the primary repo root",
        ));
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => normalized.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(AppError::bad_request(
                    "execution artifact output must stay inside the primary repo root",
                ));
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Ok(DEFAULT_EXPORT_DIR.to_string());
    }

    Ok(normalized.to_string_lossy().into_owned())
}

fn join_relative(base: &str, leaf: &str) -> String {
    Path::new(base).join(leaf).to_string_lossy().into_owned()
}

fn render_execution_context_markdown(context: &ExecutionContextData) -> String {
    let mut lines = vec![
        format!("# Execution Context: {}", context.project_name),
        String::new(),
        format!("- project_id: {}", context.project_id),
        format!("- project_slug: {}", context.project_slug),
        format!("- objective: {}", context.objective),
    ];

    if !context.repo_brief.is_empty() {
        lines.push(String::new());
        lines.push("## Repo Brief".to_string());
        lines.push(context.repo_brief.clone());
    }

    if !context.notes_brief.is_empty() {
        lines.push(String::new());
        lines.push("## Notes Brief".to_string());
        lines.push(context.notes_brief.clone());
    }

    lines.push(String::new());
    lines.push("## Repo Roots".to_string());
    lines.extend(
        context
            .repo_roots
            .iter()
            .map(|root| format!("- {} ({}) [{}]", root.path, root.label, root.kind)),
    );

    lines.push(String::new());
    lines.push("## Notes Roots".to_string());
    lines.extend(
        context
            .notes_roots
            .iter()
            .map(|root| format!("- {} ({}) [{}]", root.path, root.label, root.kind)),
    );

    if !context.constraints.is_empty() {
        lines.push(String::new());
        lines.push("## Constraints".to_string());
        lines.extend(context.constraints.iter().map(|item| format!("- {}", item)));
    }

    if !context.expected_outputs.is_empty() {
        lines.push(String::new());
        lines.push("## Expected Outputs".to_string());
        lines.extend(
            context
                .expected_outputs
                .iter()
                .map(|item| format!("- {}", item)),
        );
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_handoff_markdown(context: &ExecutionContextData) -> String {
    let primary_repo = context
        .repo_roots
        .first()
        .map(|root| root.path.as_str())
        .unwrap_or("");

    let mut lines = vec![
        format!("# GSD Handoff: {}", context.project_name),
        String::new(),
        "## Objective".to_string(),
        context.objective.clone(),
        String::new(),
        "## Writable Scope".to_string(),
        format!("- primary_repo_root: {}", primary_repo),
        "- review_gate: operator approval required before execution widens beyond this sidecar pack".to_string(),
        String::new(),
        "## Expected Outputs".to_string(),
    ];

    if context.expected_outputs.is_empty() {
        lines.push("- execution-context.md".to_string());
        lines.push("- gsd-handoff.md".to_string());
        lines.push("- agent-grounding.md".to_string());
        lines.push("- agent-inspect.json".to_string());
    } else {
        lines.extend(
            context
                .expected_outputs
                .iter()
                .map(|item| format!("- {}", item)),
        );
    }

    if !context.constraints.is_empty() {
        lines.push(String::new());
        lines.push("## Constraints".to_string());
        lines.extend(context.constraints.iter().map(|item| format!("- {}", item)));
    }

    lines.push(String::new());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;
    use vel_core::{ProjectFamily, ProjectId, ProjectProvisionRequest, ProjectStatus};

    fn test_state(storage: vel_storage::Storage) -> AppState {
        let (broadcast_tx, _) = broadcast::channel(8);
        AppState::new(
            storage,
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        )
    }

    fn test_project(id: &str, slug: &str, primary_repo: &str) -> ProjectRecord {
        let now = OffsetDateTime::now_utc();
        ProjectRecord {
            id: ProjectId::from(id.to_string()),
            slug: slug.to_string(),
            name: format!("Project {}", slug),
            family: ProjectFamily::Work,
            status: ProjectStatus::Active,
            primary_repo: ProjectRootRef {
                path: primary_repo.to_string(),
                label: slug.to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRef {
                path: format!("{primary_repo}/notes"),
                label: format!("{slug}-notes"),
                kind: "notes_root".to_string(),
            },
            secondary_repos: vec![ProjectRootRef {
                path: format!("{primary_repo}/tools"),
                label: format!("{slug}-tools"),
                kind: "repo".to_string(),
            }],
            secondary_notes_roots: vec![ProjectRootRef {
                path: format!("{primary_repo}/notes/shared"),
                label: format!("{slug}-shared-notes"),
                kind: "notes_root".to_string(),
            }],
            upstream_ids: BTreeMap::new(),
            pending_provision: ProjectProvisionRequest::default(),
            created_at: now,
            updated_at: now,
            archived_at: None,
        }
    }

    fn unique_temp_path(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "vel-execution-context-{label}-{}",
            uuid::Uuid::new_v4().simple()
        ))
    }

    fn test_agent_inspect() -> vel_api_types::AgentInspectData {
        vel_api_types::AgentInspectData {
            grounding: vel_api_types::AgentGroundingPackData {
                generated_at: 1_700_000_000,
                now: vel_api_types::NowData {
                    computed_at: 1_700_000_000,
                    timezone: "America/Denver".to_string(),
                    overview: vel_api_types::NowOverviewData {
                        dominant_action: None,
                        today_timeline: Vec::new(),
                        visible_nudge: None,
                        why_state: Vec::new(),
                        suggestions: Vec::new(),
                        decision_options: vec![
                            "accept".to_string(),
                            "choose".to_string(),
                            "thread".to_string(),
                            "close".to_string(),
                        ],
                    },
                    summary: vel_api_types::NowSummaryData {
                        mode: vel_api_types::NowLabelData {
                            key: "focused".to_string(),
                            label: "Focused".to_string(),
                        },
                        phase: vel_api_types::NowLabelData {
                            key: "engaged".to_string(),
                            label: "Engaged".to_string(),
                        },
                        meds: vel_api_types::NowLabelData {
                            key: "done".to_string(),
                            label: "Done".to_string(),
                        },
                        risk: vel_api_types::NowRiskSummaryData {
                            level: "low".to_string(),
                            score: Some(0.2),
                            label: "Low".to_string(),
                        },
                    },
                    schedule: vel_api_types::NowScheduleData {
                        empty_message: Some("No upcoming events.".to_string()),
                        next_event: None,
                        upcoming_events: Vec::new(),
                    },
                    tasks: vel_api_types::NowTasksData {
                        todoist: Vec::new(),
                        other_open: Vec::new(),
                        next_commitment: None,
                    },
                    attention: vel_api_types::NowAttentionData {
                        state: vel_api_types::NowLabelData {
                            key: "on_task".to_string(),
                            label: "On task".to_string(),
                        },
                        drift: vel_api_types::NowLabelData {
                            key: "none".to_string(),
                            label: "None".to_string(),
                        },
                        severity: vel_api_types::NowLabelData {
                            key: "low".to_string(),
                            label: "Low".to_string(),
                        },
                        confidence: Some(0.9),
                        reasons: Vec::new(),
                    },
                    sources: vel_api_types::NowSourcesData {
                        git_activity: None,
                        health: None,
                        mood: None,
                        pain: None,
                        note_document: None,
                        assistant_message: None,
                    },
                    freshness: vel_api_types::NowFreshnessData {
                        overall_status: "fresh".to_string(),
                        sources: Vec::new(),
                    },
                    trust_readiness: vel_api_types::TrustReadinessData {
                        level: "ok".to_string(),
                        headline: "Ready".to_string(),
                        summary: "Backup, freshness, and review pressure look healthy enough for normal operation."
                            .to_string(),
                        backup: vel_api_types::TrustReadinessFacetData {
                            level: "ok".to_string(),
                            label: "Backup".to_string(),
                            detail: "Backup trust is healthy.".to_string(),
                        },
                        freshness: vel_api_types::TrustReadinessFacetData {
                            level: "ok".to_string(),
                            label: "Freshness".to_string(),
                            detail: "Current context and integrations look fresh enough to trust."
                                .to_string(),
                        },
                        review: vel_api_types::TrustReadinessReviewData {
                            open_action_count: 0,
                            pending_execution_reviews: 0,
                            pending_writeback_count: 0,
                            conflict_count: 0,
                        },
                        guidance: vec!["Backup trust is healthy.".to_string()],
                        follow_through: Vec::new(),
                    },
                    check_in: None,
                    planning_profile_summary: None,
                    commitment_scheduling_summary: None,
                    day_plan: None,
                    reflow: None,
                    reflow_status: None,
                    action_items: Vec::new(),
                    review_snapshot: vel_api_types::ReviewSnapshotData::default(),
                    pending_writebacks: Vec::new(),
                    conflicts: Vec::new(),
                    people: Vec::new(),
                    reasons: vec!["test grounding".to_string()],
                    debug: vel_api_types::NowDebugData {
                        raw_context: serde_json::json!({}),
                        signals_used: Vec::new(),
                        commitments_used: Vec::new(),
                        risk_used: Vec::new(),
                    },
                },
                current_context: None,
                projects: Vec::new(),
                people: Vec::new(),
                commitments: Vec::new(),
                review: vel_api_types::AgentReviewObligationsData {
                    review_snapshot: vel_api_types::ReviewSnapshotData::default(),
                    pending_writebacks: Vec::new(),
                    conflicts: Vec::new(),
                    pending_execution_handoffs: Vec::new(),
                },
            },
            capabilities: vel_api_types::AgentCapabilitySummaryData { groups: Vec::new() },
            blockers: Vec::new(),
            explainability: vel_api_types::AgentInspectExplainabilityData {
                persisted_record_kinds: vec!["now".to_string()],
                supporting_paths: vec!["/v1/agent/inspect".to_string()],
                raw_context_json_supporting_only: true,
            },
        }
    }

    #[tokio::test]
    async fn execution_context_service_persists_by_project_id() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let project = test_project(
            "proj_exec_service_1",
            "exec-service-1",
            "/tmp/exec-service-1",
        );
        let other = test_project(
            "proj_exec_service_2",
            "exec-service-2",
            "/tmp/exec-service-2",
        );
        storage.create_project(project.clone()).await.unwrap();
        storage.create_project(other.clone()).await.unwrap();

        let state = test_state(storage);
        let saved = save_execution_context(
            &state,
            project.id.as_ref(),
            ExecutionContextInput {
                objective: "Ship bounded execution context".to_string(),
                repo_brief: "Repo-local sidecars only.".to_string(),
                notes_brief: "Capture weekly decisions in notes.".to_string(),
                constraints: vec!["Do not mutate arbitrary repo files".to_string()],
                expected_outputs: vec![".planning/vel/gsd-handoff.md".to_string()],
            },
        )
        .await
        .unwrap();

        assert_eq!(saved.project_id, project.id.as_ref());
        assert_eq!(saved.objective, "Ship bounded execution context");
        assert!(get_execution_context(&state, other.id.as_ref())
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn execution_context_render_only_uses_declared_project_roots() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let project = test_project("proj_exec_render_1", "exec-render-1", "/tmp/exec-render-1");
        storage.create_project(project.clone()).await.unwrap();
        let state = test_state(storage);

        save_execution_context(
            &state,
            project.id.as_ref(),
            ExecutionContextInput {
                objective: "Prepare a GSD-ready handoff".to_string(),
                repo_brief: "Stay inside declared roots.".to_string(),
                notes_brief: "Undeclared roots must not leak into rendered artifacts.".to_string(),
                constraints: vec!["No rogue roots".to_string()],
                expected_outputs: vec![".planning/vel/execution-context.md".to_string()],
            },
        )
        .await
        .unwrap();

        let pack = preview_gsd_artifacts(&state, project.id.as_ref(), None)
            .await
            .unwrap();
        let rendered = pack
            .files
            .iter()
            .map(|file| file.contents.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("/tmp/exec-render-1"));
        assert!(rendered.contains("/tmp/exec-render-1/notes"));
        assert!(rendered.contains("/tmp/exec-render-1/tools"));
        assert!(rendered.contains("/tmp/exec-render-1/notes/shared"));
        assert!(!rendered.contains("/tmp/not-owned"));
        assert!(pack
            .files
            .iter()
            .any(|file| file.relative_path.ends_with("agent-grounding.md")));
        assert!(pack
            .files
            .iter()
            .any(|file| file.relative_path.ends_with("agent-inspect.json")));
    }

    #[test]
    fn execution_context_render_includes_grounding_artifacts() {
        let context = ExecutionContextData {
            project_id: "proj_exec_render".to_string(),
            project_slug: "exec-render".to_string(),
            project_name: "Project exec-render".to_string(),
            objective: "Prepare a grounded handoff".to_string(),
            repo_brief: "Repo-local only".to_string(),
            notes_brief: "Keep notes in scope".to_string(),
            constraints: vec!["Stay inside the repo".to_string()],
            expected_outputs: Vec::new(),
            repo_roots: vec![ExecutionRootData {
                path: "/tmp/exec-render".to_string(),
                label: "exec-render".to_string(),
                kind: "repo".to_string(),
            }],
            notes_roots: vec![ExecutionRootData {
                path: "/tmp/exec-render/notes".to_string(),
                label: "exec-render-notes".to_string(),
                kind: "notes_root".to_string(),
            }],
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc(),
        };

        let pack = render_gsd_artifacts(&context, &test_agent_inspect(), None).unwrap();

        assert_eq!(pack.files.len(), 4);
        assert!(pack
            .files
            .iter()
            .any(|file| file.relative_path.ends_with("agent-grounding.md")));
        assert!(pack
            .files
            .iter()
            .any(|file| file.relative_path.ends_with("agent-inspect.json")));
    }

    #[tokio::test]
    async fn execution_context_export_rejects_output_outside_primary_repo_root() {
        let repo_root = unique_temp_path("repo");
        std::fs::create_dir_all(&repo_root).unwrap();

        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let project = test_project(
            "proj_exec_export_1",
            "exec-export-1",
            &repo_root.to_string_lossy(),
        );
        storage.create_project(project.clone()).await.unwrap();
        let state = test_state(storage);

        save_execution_context(
            &state,
            project.id.as_ref(),
            ExecutionContextInput {
                objective: "Export a bounded sidecar pack".to_string(),
                repo_brief: String::new(),
                notes_brief: String::new(),
                constraints: vec!["Reject parent traversal".to_string()],
                expected_outputs: vec![".planning/vel/gsd-handoff.md".to_string()],
            },
        )
        .await
        .unwrap();

        let error = export_gsd_artifacts(&state, project.id.as_ref(), Some("../outside"))
            .await
            .unwrap_err();
        assert!(error
            .to_string()
            .contains("must stay inside the primary repo root"));

        std::fs::remove_dir_all(&repo_root).unwrap();
    }
}
