//! vel thread — list, inspect, and update thread lifecycle state.
//! See docs/specs/vel-thread-graph-spec.md.

use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_list(
    client: &ApiClient,
    status: Option<&str>,
    limit: u32,
    json: bool,
) -> anyhow::Result<()> {
    let resp = client
        .list_threads(status, limit)
        .await
        .context("list threads")?;
    let threads = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(threads)?);
    } else if threads.is_empty() {
        println!("No threads.");
    } else {
        for t in threads {
            match (&t.planning_kind, &t.lifecycle_stage) {
                (Some(kind), Some(stage)) => {
                    println!(
                        "{}  {}  {}  {}  {}",
                        t.id, t.thread_type, kind, stage, t.title
                    );
                }
                _ => {
                    println!("{}  {}  {}  {}", t.id, t.thread_type, t.status, t.title);
                }
            }
        }
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let resp = client.get_thread(id).await.context("get thread")?;
    let t = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("id:          {}", t.id);
    println!("type:        {}", t.thread_type);
    println!("title:       {}", t.title);
    println!("status:      {}", t.status);
    if let Some(ref planning_kind) = t.planning_kind {
        println!("planning:    {}", planning_kind);
    }
    if let Some(ref lifecycle_stage) = t.lifecycle_stage {
        println!("lifecycle:   {}", lifecycle_stage);
    }
    println!("created_at:  {}", t.created_at);
    println!("updated_at:  {}", t.updated_at);
    if let Some(ref links) = t.links {
        if !links.is_empty() {
            println!("links:");
            for l in links.iter() {
                println!(
                    "  {}  {}  {}  {}",
                    l.entity_type, l.entity_id, l.relation_type, l.id
                );
            }
        }
    }
    Ok(())
}

pub async fn run_status(client: &ApiClient, id: &str, status: &str) -> anyhow::Result<()> {
    let _ = client
        .update_thread(id, status)
        .await
        .with_context(|| format!("set thread {} to {}", id, status))?;
    println!("Thread {} -> {}", id, status);
    Ok(())
}

pub async fn run_close(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    run_status(client, id, "closed").await
}

pub async fn run_reopen(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    run_status(client, id, "open").await
}
