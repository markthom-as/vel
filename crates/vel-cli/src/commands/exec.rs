use anyhow::anyhow;

use crate::client::{
    ApiClient, ExecutionArtifactPackData, ExecutionArtifactRequestData, ExecutionContextData,
    ExecutionContextSaveRequestData, ExecutionExportResultData,
};

pub async fn run_show_context(
    client: &ApiClient,
    project_id: &str,
    json: bool,
) -> anyhow::Result<()> {
    let response = client.get_execution_context(project_id).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let context = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("execution context response missing data"))?;
    println!("{}", format_execution_context(context));
    Ok(())
}

pub async fn run_save_context(
    client: &ApiClient,
    project_id: &str,
    payload: ExecutionContextSaveRequestData,
    json: bool,
) -> anyhow::Result<()> {
    let response = client.save_execution_context(project_id, &payload).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let context = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("execution context response missing data"))?;
    println!("{}", format_execution_context(context));
    Ok(())
}

pub async fn run_preview_context(
    client: &ApiClient,
    project_id: &str,
    output_dir: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .preview_execution_artifacts(
            project_id,
            &ExecutionArtifactRequestData {
                output_dir: output_dir.map(ToString::to_string),
            },
        )
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let pack = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("execution artifact preview missing data"))?;
    println!("{}", format_artifact_pack(pack));
    Ok(())
}

pub async fn run_export_context(
    client: &ApiClient,
    project_id: &str,
    output_dir: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .export_execution_artifacts(
            project_id,
            &ExecutionArtifactRequestData {
                output_dir: output_dir.map(ToString::to_string),
            },
        )
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let exported = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("execution artifact export missing data"))?;
    println!("{}", format_export_result(exported));
    Ok(())
}

fn format_execution_context(context: &ExecutionContextData) -> String {
    let mut lines = vec![
        format!("project: {} ({})", context.project_name, context.project_id),
        format!("slug: {}", context.project_slug),
        format!("objective: {}", context.objective),
    ];

    if !context.repo_brief.is_empty() {
        lines.push(format!("repo_brief: {}", context.repo_brief));
    }
    if !context.notes_brief.is_empty() {
        lines.push(format!("notes_brief: {}", context.notes_brief));
    }

    if !context.repo_roots.is_empty() {
        lines.push("repo_roots:".to_string());
        for root in &context.repo_roots {
            lines.push(format!("- {} [{}] ({})", root.path, root.kind, root.label));
        }
    }

    if !context.notes_roots.is_empty() {
        lines.push("notes_roots:".to_string());
        for root in &context.notes_roots {
            lines.push(format!("- {} [{}] ({})", root.path, root.kind, root.label));
        }
    }

    if !context.constraints.is_empty() {
        lines.push("constraints:".to_string());
        for item in &context.constraints {
            lines.push(format!("- {}", item));
        }
    }

    if !context.expected_outputs.is_empty() {
        lines.push("expected_outputs:".to_string());
        for item in &context.expected_outputs {
            lines.push(format!("- {}", item));
        }
    }

    lines.push(format!("updated_at: {}", context.updated_at));
    lines.join("\n")
}

fn format_artifact_pack(pack: &ExecutionArtifactPackData) -> String {
    let mut lines = vec![
        format!("project_id: {}", pack.project_id),
        format!("project_slug: {}", pack.project_slug),
        format!("repo_root: {}", pack.repo_root),
        format!("output_dir: {}", pack.output_dir),
    ];

    for file in &pack.files {
        lines.push(String::new());
        lines.push(format!("== {} ==", file.relative_path));
        lines.push(file.contents.clone());
    }

    lines.join("\n")
}

fn format_export_result(exported: &ExecutionExportResultData) -> String {
    let mut lines = vec![
        format!("project_id: {}", exported.pack.project_id),
        format!("repo_root: {}", exported.pack.repo_root),
        format!("output_dir: {}", exported.pack.output_dir),
        "written_paths:".to_string(),
    ];

    for path in &exported.written_paths {
        lines.push(format!("- {}", path));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{ExecutionArtifactFileData, ExecutionRootData};

    #[test]
    fn exec_format_context_lists_roots_and_constraints() {
        let text = format_execution_context(&ExecutionContextData {
            project_id: "proj_exec_cli".to_string(),
            project_slug: "vel".to_string(),
            project_name: "Vel".to_string(),
            objective: "Ship execution context".to_string(),
            repo_brief: "Keep changes local".to_string(),
            notes_brief: String::new(),
            constraints: vec!["sidecar only".to_string()],
            expected_outputs: vec![".planning/vel/gsd-handoff.md".to_string()],
            repo_roots: vec![ExecutionRootData {
                path: "/tmp/vel".to_string(),
                label: "vel".to_string(),
                kind: "repo".to_string(),
            }],
            notes_roots: vec![ExecutionRootData {
                path: "/tmp/vel/notes".to_string(),
                label: "vel-notes".to_string(),
                kind: "notes_root".to_string(),
            }],
            created_at: "2026-03-19T00:00:00Z".to_string(),
            updated_at: "2026-03-19T00:00:00Z".to_string(),
        });

        assert!(text.contains("project: Vel (proj_exec_cli)"));
        assert!(text.contains("/tmp/vel [repo] (vel)"));
        assert!(text.contains("sidecar only"));
    }

    #[test]
    fn exec_format_artifact_pack_keeps_sidecar_paths_visible() {
        let text = format_artifact_pack(&ExecutionArtifactPackData {
            project_id: "proj_exec_cli".to_string(),
            project_slug: "vel".to_string(),
            repo_root: "/tmp/vel".to_string(),
            output_dir: ".planning/vel".to_string(),
            files: vec![
                ExecutionArtifactFileData {
                    relative_path: ".planning/vel/execution-context.md".to_string(),
                    contents: "# Context".to_string(),
                },
                ExecutionArtifactFileData {
                    relative_path: ".planning/vel/gsd-handoff.md".to_string(),
                    contents: "# Handoff".to_string(),
                },
            ],
        });

        assert!(text.contains("== .planning/vel/execution-context.md =="));
        assert!(text.contains("== .planning/vel/gsd-handoff.md =="));
    }
}
