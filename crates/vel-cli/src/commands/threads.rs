//! vel thread — list, inspect, close, reopen continuity/history threads, including backend-owned resolution follow-through. See docs/api/runtime.md.

use crate::client::ApiClient;
use anyhow::{anyhow, Context};
use vel_api_types::ThreadData;

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
        println!("threads (continuity, archive, and follow-through history):");
        for t in threads {
            println!("{}  {}  {}  {}", t.id, t.thread_type, t.status, t.title);
        }
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, id: &str) -> anyhow::Result<()> {
    let t = load_thread(client, id).await?;
    println!("thread detail:");
    println!("id:          {}", t.id);
    println!("type:        {}", t.thread_type);
    println!("title:       {}", t.title);
    println!("status:      {}", t.status);
    println!("created_at:  {}", t.created_at);
    println!("updated_at:  {}", t.updated_at);
    if let Some(run_id) = linked_connect_run_id(&t) {
        println!("connect_run: {}", run_id);
    }
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

pub async fn run_follow(
    client: &ApiClient,
    id: &str,
    after_id: Option<i64>,
    limit: u32,
    poll_ms: u64,
    once: bool,
) -> anyhow::Result<()> {
    let thread = load_thread(client, id).await?;
    let run_id = resolve_thread_connect_run(thread)?;
    crate::commands::connect::run_tail_instance(client, &run_id, after_id, limit, poll_ms, once)
        .await
}

pub async fn run_reply(
    client: &ApiClient,
    id: &str,
    input: Vec<String>,
    json: bool,
) -> anyhow::Result<()> {
    let thread = load_thread(client, id).await?;
    let run_id = resolve_thread_connect_run(thread)?;
    let message = input.join(" ").trim().to_string();
    if message.is_empty() {
        return Err(anyhow!("reply input must not be empty"));
    }
    crate::commands::connect::run_stdin_instance(client, &run_id, &message, json).await
}

async fn load_thread<'a>(client: &ApiClient, id: &'a str) -> anyhow::Result<ThreadData> {
    let resp = client.get_thread(id).await.context("get thread")?;
    resp.data.ok_or_else(|| anyhow!("no data"))
}

fn resolve_thread_connect_run(thread: ThreadData) -> anyhow::Result<String> {
    if let Some(run_id) = linked_connect_run_id(&thread) {
        return Ok(run_id.to_string());
    }

    if let Some(handoff_id) = linked_execution_handoff_id(&thread) {
        return Err(anyhow!(
            "thread {} is linked to execution handoff {} but no connect runtime is attached yet",
            thread.id,
            handoff_id
        ));
    }

    Err(anyhow!(
        "thread {} has no attached connect runtime",
        thread.id
    ))
}

fn linked_connect_run_id(thread: &ThreadData) -> Option<&str> {
    thread.links.as_ref().and_then(|links| {
        links
            .iter()
            .rev()
            .find(|link| link.entity_type == "connect_run" && link.relation_type == "attached")
            .map(|link| link.entity_id.as_str())
    })
}

fn linked_execution_handoff_id(thread: &ThreadData) -> Option<&str> {
    thread.links.as_ref().and_then(|links| {
        links
            .iter()
            .rev()
            .find(|link| {
                link.entity_type == "execution_handoff" && link.relation_type == "approves"
            })
            .map(|link| link.entity_id.as_str())
    })
}

#[cfg(test)]
mod tests {
    use super::{linked_connect_run_id, linked_execution_handoff_id};
    use serde_json::json;
    use vel_api_types::{ThreadData, ThreadLinkData};

    fn sample_thread(links: Vec<ThreadLinkData>) -> ThreadData {
        ThreadData {
            id: "thr_1".to_string(),
            thread_type: "assistant_proposal".to_string(),
            title: "Proposal thread".to_string(),
            status: "open".to_string(),
            planning_kind: None,
            lifecycle_stage: Some("approved".to_string()),
            created_at: 1,
            updated_at: 2,
            continuation: None,
            metadata: Some(json!({})),
            links: Some(links),
            project_id: None,
            project_label: None,
        }
    }

    #[test]
    fn thread_prefers_latest_attached_connect_run_link() {
        let thread = sample_thread(vec![
            ThreadLinkData {
                id: "tl_1".to_string(),
                entity_type: "connect_run".to_string(),
                entity_id: "run_old".to_string(),
                relation_type: "attached".to_string(),
            },
            ThreadLinkData {
                id: "tl_2".to_string(),
                entity_type: "connect_run".to_string(),
                entity_id: "run_new".to_string(),
                relation_type: "attached".to_string(),
            },
        ]);

        assert_eq!(linked_connect_run_id(&thread), Some("run_new"));
    }

    #[test]
    fn thread_surfaces_linked_execution_handoff_when_runtime_missing() {
        let thread = sample_thread(vec![ThreadLinkData {
            id: "tl_1".to_string(),
            entity_type: "execution_handoff".to_string(),
            entity_id: "handoff_1".to_string(),
            relation_type: "approves".to_string(),
        }]);

        assert_eq!(linked_connect_run_id(&thread), None);
        assert_eq!(linked_execution_handoff_id(&thread), Some("handoff_1"));
    }
}
