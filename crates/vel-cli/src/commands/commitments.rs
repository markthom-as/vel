//! CLI commands for commitments: list, add, done, cancel, inspect.

use anyhow::Context;
use vel_api_types::CommitmentCreateRequest;

use crate::client::ApiClient;

/// List commitments; default status filter is "open".
pub async fn run_list(
    client: &ApiClient,
    status: Option<&str>,
    project: Option<&str>,
    limit: u32,
    json: bool,
) -> anyhow::Result<()> {
    let status = status.unwrap_or("open");
    let resp = client
        .list_commitments(Some(status), project, None, limit)
        .await
        .context("list commitments")?;
    let data = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }
    if data.is_empty() {
        println!("No commitments.");
        return Ok(());
    }
    for c in data {
        let resolved = c
            .resolved_at
            .map(|t| t.to_string())
            .unwrap_or_else(|| "—".to_string());
        println!("{}  {}  {}  {}", c.id, c.status, c.text, resolved);
    }
    Ok(())
}

pub async fn run_add(
    client: &ApiClient,
    text: &str,
    kind: Option<&str>,
    project: Option<&str>,
) -> anyhow::Result<()> {
    let req = CommitmentCreateRequest {
        text: text.to_string(),
        source_type: "manual".to_string(),
        source_id: None,
        due_at: None,
        project: project.map(String::from),
        commitment_kind: kind.map(String::from),
        metadata: serde_json::json!({}),
    };
    let resp = client.create_commitment(req).await.context("create commitment")?;
    let data = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("{}  {}  {}", data.id, data.status, data.text);
    Ok(())
}

pub async fn run_done(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client
        .update_commitment(
            id,
            vel_api_types::CommitmentUpdateRequest {
                status: Some("done".to_string()),
                ..Default::default()
            },
        )
        .await
        .context("mark commitment done")?;
    let data = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("{}  done  {}", data.id, data.text);
    Ok(())
}

pub async fn run_cancel(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client
        .update_commitment(
            id,
            vel_api_types::CommitmentUpdateRequest {
                status: Some("cancelled".to_string()),
                ..Default::default()
            },
        )
        .await
        .context("cancel commitment")?;
    let data = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("{}  cancelled  {}", data.id, data.text);
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client.get_commitment(id).await.context("get commitment")?;
    let c = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("id:          {}", c.id);
    println!("text:        {}", c.text);
    println!("status:      {}", c.status);
    println!("source:      {}  {}", c.source_type, c.source_id.as_deref().unwrap_or("—"));
    println!("kind:        {}", c.commitment_kind.as_deref().unwrap_or("—"));
    println!("project:     {}", c.project.as_deref().unwrap_or("—"));
    println!("due_at:      {}", c.due_at.map(|t| t.to_string()).unwrap_or_else(|| "—".to_string()));
    println!("created_at:  {}", c.created_at);
    println!(
        "resolved_at: {}",
        c.resolved_at.map(|t| t.to_string()).unwrap_or_else(|| "—".to_string())
    );
    Ok(())
}

pub async fn run_dependencies(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client.list_commitment_dependencies(id).await.context("list dependencies")?;
    let deps = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    if deps.is_empty() {
        println!("No dependencies for commitment {}.", id);
        return Ok(());
    }
    for d in deps {
        println!("{}  {}  {}  {}", d.id, d.child_commitment_id, d.dependency_type, d.created_at);
    }
    Ok(())
}

pub async fn run_add_dependency(
    client: &ApiClient,
    parent_id: &str,
    child_id: &str,
    dependency_type: &str,
) -> anyhow::Result<()> {
    let resp = client
        .add_commitment_dependency(parent_id, child_id, dependency_type)
        .await
        .context("add dependency")?;
    let d = resp.data.as_ref().ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("{}  {} -> {}  {}", d.id, parent_id, child_id, d.dependency_type);
    Ok(())
}
