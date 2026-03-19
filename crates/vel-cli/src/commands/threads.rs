//! vel thread — list, inspect, close, reopen threads. See docs/api/runtime.md.

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
        println!("No threads in the continuity/archive lane.");
    } else {
        println!("threads (continuity and archive):");
        for t in threads {
            println!("{}  {}  {}  {}", t.id, t.thread_type, t.status, t.title);
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
    println!("thread detail:");
    println!("id:          {}", t.id);
    println!("type:        {}", t.thread_type);
    println!("title:       {}", t.title);
    println!("status:      {}", t.status);
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

pub async fn run_close(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let _ = client
        .update_thread(id, "closed")
        .await
        .context("close thread")?;
    println!("Thread {} closed.", id);
    Ok(())
}

pub async fn run_reopen(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let _ = client
        .update_thread(id, "open")
        .await
        .context("reopen thread")?;
    println!("Thread {} reopened.", id);
    Ok(())
}
