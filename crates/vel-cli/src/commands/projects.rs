//! vel project — list and inspect project records.

use crate::client::ApiClient;
use anyhow::Context;
use std::collections::BTreeMap;
use vel_api_types::{
    ProjectCreateRequestData, ProjectFamilyData, ProjectProvisionRequestData, ProjectRecordData,
    ProjectRootRefData, ProjectStatusData,
};

fn family_label(family: ProjectFamilyData) -> &'static str {
    match family {
        ProjectFamilyData::Personal => "personal",
        ProjectFamilyData::Creative => "creative",
        ProjectFamilyData::Work => "work",
    }
}

fn status_label(status: ProjectStatusData) -> &'static str {
    match status {
        ProjectStatusData::Active => "active",
        ProjectStatusData::Paused => "paused",
        ProjectStatusData::Archived => "archived",
    }
}

fn summarize_project(project: &ProjectRecordData) -> String {
    format!(
        "{}  {}  {}  {}  {}",
        project.id,
        project.slug,
        family_label(project.family),
        status_label(project.status),
        project.name
    )
}

fn detail_lines(project: &ProjectRecordData) -> Vec<String> {
    let mut lines = vec![
        format!("id:                  {}", project.id),
        format!("slug:                {}", project.slug),
        format!("name:                {}", project.name),
        format!("family:              {}", family_label(project.family)),
        format!("status:              {}", status_label(project.status)),
        format!("primary_repo:        {}", project.primary_repo.path),
        format!("primary_notes_root:  {}", project.primary_notes_root.path),
        format!("created_at:          {}", project.created_at),
        format!("updated_at:          {}", project.updated_at),
        format!(
            "archived_at:         {}",
            project
                .archived_at
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".to_string())
        ),
        format!(
            "pending_create_repo: {}",
            project.pending_provision.create_repo
        ),
        format!(
            "pending_notes_root:  {}",
            project.pending_provision.create_notes_root
        ),
    ];

    if !project.secondary_repos.is_empty() {
        lines.push("secondary_repos:".to_string());
        for root in &project.secondary_repos {
            lines.push(format!("  - {} ({})", root.path, root.label));
        }
    }
    if !project.secondary_notes_roots.is_empty() {
        lines.push("secondary_notes_roots:".to_string());
        for root in &project.secondary_notes_roots {
            lines.push(format!("  - {} ({})", root.path, root.label));
        }
    }
    if !project.upstream_ids.is_empty() {
        lines.push("upstream_ids:".to_string());
        for (provider, id) in &project.upstream_ids {
            lines.push(format!("  - {}={}", provider, id));
        }
    }

    lines
}

pub async fn run_list(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client.list_projects().await.context("list projects")?;
    let projects = resp
        .data
        .ok_or_else(|| anyhow::anyhow!("project list missing data"))?
        .projects;
    if json {
        println!("{}", serde_json::to_string_pretty(&projects)?);
        return Ok(());
    }
    if projects.is_empty() {
        println!("No projects.");
        return Ok(());
    }
    for project in &projects {
        println!("{}", summarize_project(project));
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let resp = client.get_project(id).await.context("get project")?;
    let project = resp
        .data
        .ok_or_else(|| anyhow::anyhow!("project inspect missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&project)?);
        return Ok(());
    }
    for line in detail_lines(&project) {
        println!("{line}");
    }
    Ok(())
}

pub async fn run_families(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client
        .list_project_families()
        .await
        .context("list project families")?;
    let families = resp
        .data
        .ok_or_else(|| anyhow::anyhow!("project families missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&families)?);
        return Ok(());
    }
    for family in families {
        println!("{}", family_label(family));
    }
    Ok(())
}

pub async fn run_create(
    client: &ApiClient,
    slug: &str,
    name: &str,
    family: &str,
    status: Option<&str>,
    repo_path: &str,
    notes_path: &str,
    create_repo: bool,
    create_notes_root: bool,
    json: bool,
) -> anyhow::Result<()> {
    let family = parse_family(family)?;
    let status = status.map(parse_status).transpose()?;
    let payload = ProjectCreateRequestData {
        slug: slug.to_string(),
        name: name.to_string(),
        family,
        status,
        primary_repo: ProjectRootRefData {
            path: repo_path.to_string(),
            label: "repo".to_string(),
            kind: "repo".to_string(),
        },
        primary_notes_root: ProjectRootRefData {
            path: notes_path.to_string(),
            label: "notes".to_string(),
            kind: "notes_root".to_string(),
        },
        secondary_repos: Vec::new(),
        secondary_notes_roots: Vec::new(),
        upstream_ids: BTreeMap::new(),
        pending_provision: ProjectProvisionRequestData {
            create_repo,
            create_notes_root,
        },
    };

    let resp = client
        .create_project(&payload)
        .await
        .context("create project")?;
    let project = resp
        .data
        .ok_or_else(|| anyhow::anyhow!("project create missing data"))?
        .project;

    if json {
        println!("{}", serde_json::to_string_pretty(&project)?);
        return Ok(());
    }

    println!("project_id:          {}", project.id);
    println!("slug:                {}", project.slug);
    println!("name:                {}", project.name);
    println!("family:              {}", family_label(project.family));
    println!("status:              {}", status_label(project.status));
    Ok(())
}

fn parse_family(value: &str) -> anyhow::Result<ProjectFamilyData> {
    match value.trim().to_ascii_lowercase().as_str() {
        "personal" => Ok(ProjectFamilyData::Personal),
        "creative" => Ok(ProjectFamilyData::Creative),
        "work" => Ok(ProjectFamilyData::Work),
        other => Err(anyhow::anyhow!(
            "unsupported project family '{}'; use personal|creative|work",
            other
        )),
    }
}

fn parse_status(value: &str) -> anyhow::Result<ProjectStatusData> {
    match value.trim().to_ascii_lowercase().as_str() {
        "active" => Ok(ProjectStatusData::Active),
        "paused" => Ok(ProjectStatusData::Paused),
        "archived" => Ok(ProjectStatusData::Archived),
        other => Err(anyhow::anyhow!(
            "unsupported project status '{}'; use active|paused|archived",
            other
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{detail_lines, parse_family, parse_status, summarize_project};
    use time::OffsetDateTime;
    use vel_api_types::{
        ProjectFamilyData, ProjectProvisionRequestData, ProjectRecordData, ProjectRootRefData,
        ProjectStatusData,
    };
    use vel_core::ProjectId;

    fn sample_project() -> ProjectRecordData {
        ProjectRecordData {
            id: ProjectId::from("proj_cli_1".to_string()),
            slug: "vel".to_string(),
            name: "Vel".to_string(),
            family: ProjectFamilyData::Creative,
            status: ProjectStatusData::Active,
            primary_repo: ProjectRootRefData {
                path: "/code/vel".to_string(),
                label: "repo".to_string(),
                kind: "repo".to_string(),
            },
            primary_notes_root: ProjectRootRefData {
                path: "/notes/vel".to_string(),
                label: "notes".to_string(),
                kind: "notes_root".to_string(),
            },
            secondary_repos: Vec::new(),
            secondary_notes_roots: Vec::new(),
            upstream_ids: [("github".to_string(), "vel/vel".to_string())]
                .into_iter()
                .collect(),
            pending_provision: ProjectProvisionRequestData {
                create_repo: false,
                create_notes_root: true,
            },
            created_at: OffsetDateTime::from_unix_timestamp(1_742_927_200).unwrap(),
            updated_at: OffsetDateTime::from_unix_timestamp(1_742_927_260).unwrap(),
            archived_at: None,
        }
    }

    #[test]
    fn summary_uses_slug_family_status_and_name() {
        let summary = summarize_project(&sample_project());
        assert!(summary.contains("proj_cli_1"));
        assert!(summary.contains("vel"));
        assert!(summary.contains("creative"));
        assert!(summary.contains("active"));
        assert!(summary.contains("Vel"));
    }

    #[test]
    fn detail_lines_surface_roots_and_pending_provision() {
        let lines = detail_lines(&sample_project()).join("\n");
        assert!(lines.contains("primary_repo:        /code/vel"));
        assert!(lines.contains("primary_notes_root:  /notes/vel"));
        assert!(lines.contains("pending_notes_root:  true"));
        assert!(lines.contains("upstream_ids:"));
        assert!(lines.contains("github=vel/vel"));
    }

    #[test]
    fn family_parser_rejects_unknown_values() {
        assert!(parse_family("creative").is_ok());
        assert!(parse_family("unknown").is_err());
    }

    #[test]
    fn status_parser_rejects_unknown_values() {
        assert!(parse_status("active").is_ok());
        assert!(parse_status("retired").is_err());
    }
}
