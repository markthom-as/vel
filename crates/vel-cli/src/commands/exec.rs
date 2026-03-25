use anyhow::anyhow;

use crate::client::{
    ApiClient, ExecutionArtifactPackData, ExecutionArtifactRequestData, ExecutionContextData,
    ExecutionContextSaveRequestData, ExecutionExportResultData, ExecutionHandoffRecordData,
    ExecutionLaunchPreviewData, LaunchExecutionHandoffRequestData,
    ReviewExecutionHandoffRequestData,
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
    println!(
        "\nnext: review persisted handoffs with `vel exec review --project-id {}`",
        project_id
    );
    Ok(())
}

pub async fn run_review_handoffs(
    client: &ApiClient,
    project_id: Option<&str>,
    state: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client.list_execution_handoffs(project_id, state).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let handoffs = response
        .data
        .ok_or_else(|| anyhow!("execution handoffs response missing data"))?;
    if handoffs.is_empty() {
        println!("No execution handoffs.");
        return Ok(());
    }

    for (index, handoff) in handoffs.iter().enumerate() {
        if index > 0 {
            println!();
        }
        println!("{}", format_execution_handoff(handoff));
    }
    Ok(())
}

pub async fn run_preview_handoff_launch(
    client: &ApiClient,
    handoff_id: &str,
    json: bool,
) -> anyhow::Result<()> {
    let response = client.preview_execution_handoff_launch(handoff_id).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let preview = response
        .data
        .ok_or_else(|| anyhow!("execution handoff launch preview missing data"))?;
    println!("{}", format_execution_launch_preview(&preview));
    Ok(())
}

pub async fn run_approve_handoff(
    client: &ApiClient,
    handoff_id: &str,
    reviewed_by: &str,
    decision_reason: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .approve_execution_handoff(
            handoff_id,
            &ReviewExecutionHandoffRequestData {
                reviewed_by: reviewed_by.to_string(),
                decision_reason,
            },
        )
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let handoff = response
        .data
        .ok_or_else(|| anyhow!("execution handoff approval missing data"))?;
    println!("{}", format_execution_handoff(&handoff));
    Ok(())
}

pub async fn run_reject_handoff(
    client: &ApiClient,
    handoff_id: &str,
    reviewed_by: &str,
    decision_reason: Option<String>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .reject_execution_handoff(
            handoff_id,
            &ReviewExecutionHandoffRequestData {
                reviewed_by: reviewed_by.to_string(),
                decision_reason,
            },
        )
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let handoff = response
        .data
        .ok_or_else(|| anyhow!("execution handoff rejection missing data"))?;
    println!("{}", format_execution_handoff(&handoff));
    Ok(())
}

pub async fn run_launch_handoff(
    client: &ApiClient,
    handoff_id: &str,
    payload: LaunchExecutionHandoffRequestData,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .launch_execution_handoff(handoff_id, &payload)
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let launched = response
        .data
        .ok_or_else(|| anyhow!("execution handoff launch missing connect instance data"))?;
    println!("connect_instance_id: {}", launched.id);
    println!("display_name: {}", launched.display_name);
    println!("status: {}", launched.status);
    println!("reachability: {}", launched.reachability);
    println!("node_id: {}", launched.node_id);
    println!(
        "trace_id: {}",
        launched
            .metadata
            .get("trace_id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("—")
    );
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

fn format_execution_handoff(handoff: &ExecutionHandoffRecordData) -> String {
    let mut lines = vec![
        format!("handoff_id: {}", handoff.id),
        format!("project_id: {}", handoff.project_id),
        format!("review_state: {}", handoff.review_state),
        format!("origin_kind: {}", handoff.origin_kind),
        format!(
            "route: {} · {} · {} · {}",
            handoff.routing.task_kind,
            handoff.routing.agent_profile,
            handoff.routing.token_budget,
            handoff.routing.review_gate
        ),
        format!(
            "agents: {} -> {}",
            handoff.handoff.handoff.from_agent, handoff.handoff.handoff.to_agent
        ),
        format!("objective: {}", handoff.handoff.handoff.objective),
    ];

    if !handoff.routing.read_scopes.is_empty() {
        lines.push(format!(
            "read_scopes: {}",
            handoff.routing.read_scopes.join(", ")
        ));
    }
    if !handoff.routing.write_scopes.is_empty() {
        lines.push(format!(
            "write_scopes: {}",
            handoff.routing.write_scopes.join(", ")
        ));
    }
    if !handoff.routing.allowed_tools.is_empty() {
        lines.push(format!(
            "allowed_tools: {}",
            handoff.routing.allowed_tools.join(", ")
        ));
    }
    if !handoff.routing.reasons.is_empty() {
        lines.push("routing_reasons:".to_string());
        for reason in &handoff.routing.reasons {
            lines.push(format!("- {}: {}", reason.code, reason.message));
        }
    }
    if let Some(reason) = &handoff.decision_reason {
        lines.push(format!("decision_reason: {}", reason));
    }
    if let Some(reviewed_by) = &handoff.reviewed_by {
        lines.push(format!("reviewed_by: {}", reviewed_by));
    }
    if let Some(reviewed_at) = &handoff.reviewed_at {
        lines.push(format!("reviewed_at: {}", reviewed_at));
    }
    lines.push(format!("updated_at: {}", handoff.updated_at));
    lines.join("\n")
}

fn format_execution_launch_preview(preview: &ExecutionLaunchPreviewData) -> String {
    let mut lines = vec![
        format!("handoff_id: {}", preview.handoff_id),
        format!("review_state: {}", preview.review_state),
        format!("launch_ready: {}", preview.launch_ready),
    ];
    if !preview.blockers.is_empty() {
        lines.push("blockers:".to_string());
        for blocker in &preview.blockers {
            lines.push(format!("- {}", blocker));
        }
    }
    lines.push(format!("objective: {}", preview.handoff.handoff.objective));
    if !preview.routing.write_scopes.is_empty() {
        lines.push(format!(
            "write_scopes: {}",
            preview.routing.write_scopes.join(", ")
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{
        ExecutionArtifactFileData, ExecutionRootData, ExecutionRoutingDecisionData,
        ExecutionRoutingReasonData,
    };
    use vel_api_types::{
        AgentProfileData, ExecutionHandoffData, ExecutionReviewGateData, ExecutionTaskKindData,
        HandoffEnvelopeData, ProjectRootRefData, RepoWorktreeRefData, TokenBudgetClassData,
    };
    use vel_core::{ProjectId, TraceId};

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

    #[test]
    fn exec_format_handoff_surfaces_review_and_routing_state() {
        let text = format_execution_handoff(&ExecutionHandoffRecordData {
            id: "xho_123".to_string(),
            project_id: "proj_exec_cli".to_string(),
            origin_kind: "human_to_agent".to_string(),
            review_state: "pending_review".to_string(),
            handoff: sample_handoff(),
            routing: sample_routing(),
            manifest_id: Some("local_coder".to_string()),
            requested_by: "operator_shell".to_string(),
            reviewed_by: None,
            decision_reason: None,
            reviewed_at: None,
            launched_at: None,
            created_at: "2026-03-19T00:00:00Z".to_string(),
            updated_at: "2026-03-19T00:00:00Z".to_string(),
        });

        assert!(text.contains("handoff_id: xho_123"));
        assert!(text.contains("review_state: pending_review"));
        assert!(text.contains("route: implementation · quality · large · operator_approval"));
        assert!(text.contains("routing_reasons:"));
    }

    #[test]
    fn exec_format_launch_preview_lists_blockers() {
        let text = format_execution_launch_preview(&ExecutionLaunchPreviewData {
            handoff_id: "xho_123".to_string(),
            review_state: "pending_review".to_string(),
            launch_ready: false,
            blockers: vec!["handoff review is still pending".to_string()],
            handoff: sample_handoff(),
            routing: sample_routing(),
        });

        assert!(text.contains("launch_ready: false"));
        assert!(text.contains("handoff review is still pending"));
    }

    fn sample_handoff() -> ExecutionHandoffData {
        ExecutionHandoffData {
            handoff: HandoffEnvelopeData {
                task_id: "handoff_123".to_string(),
                trace_id: TraceId::new(),
                from_agent: "operator".to_string(),
                to_agent: "codex-local".to_string(),
                objective: "Ship the next safe slice".to_string(),
                inputs: serde_json::json!({ "ticket": "08-04" }),
                constraints: vec!["sidecar only".to_string()],
                read_scopes: vec!["/tmp/vel".to_string()],
                write_scopes: vec!["/tmp/vel".to_string()],
                project_id: Some(ProjectId::from("proj_exec_cli".to_string())),
                task_kind: Some(ExecutionTaskKindData::Implementation),
                agent_profile: Some(AgentProfileData::Quality),
                token_budget: Some(TokenBudgetClassData::Large),
                review_gate: Some(ExecutionReviewGateData::OperatorApproval),
                repo_root: Some(RepoWorktreeRefData {
                    path: "/tmp/vel".to_string(),
                    label: "vel".to_string(),
                    branch: Some("main".to_string()),
                    head_rev: Some("abc123".to_string()),
                }),
                allowed_tools: vec!["rg".to_string(), "cargo test".to_string()],
                capability_scope: serde_json::json!({
                    "read_scopes": ["/tmp/vel"],
                    "write_scopes": ["/tmp/vel"]
                }),
                deadline: None,
                expected_output_schema: serde_json::json!({ "artifacts": ["patch"] }),
            },
            project_id: ProjectId::from("proj_exec_cli".to_string()),
            task_kind: ExecutionTaskKindData::Implementation,
            agent_profile: AgentProfileData::Quality,
            token_budget: TokenBudgetClassData::Large,
            review_gate: ExecutionReviewGateData::OperatorApproval,
            repo: RepoWorktreeRefData {
                path: "/tmp/vel".to_string(),
                label: "vel".to_string(),
                branch: Some("main".to_string()),
                head_rev: Some("abc123".to_string()),
            },
            notes_root: ProjectRootRefData {
                path: "/tmp/vel/notes".to_string(),
                label: "vel-notes".to_string(),
                kind: "notes_root".to_string(),
            },
            manifest_id: Some("local_coder".to_string()),
        }
    }

    fn sample_routing() -> ExecutionRoutingDecisionData {
        ExecutionRoutingDecisionData {
            task_kind: "implementation".to_string(),
            agent_profile: "quality".to_string(),
            token_budget: "large".to_string(),
            review_gate: "operator_approval".to_string(),
            read_scopes: vec!["/tmp/vel".to_string()],
            write_scopes: vec!["/tmp/vel".to_string()],
            allowed_tools: vec!["rg".to_string(), "cargo test".to_string()],
            reasons: vec![ExecutionRoutingReasonData {
                code: "write_scope_requires_approval".to_string(),
                message: "write scopes require explicit operator approval before launch"
                    .to_string(),
            }],
        }
    }
}
