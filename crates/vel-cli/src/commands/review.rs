//! `vel review` — daily and weekly review views.

use crate::client::ApiClient;

const TRUNCATE: usize = 50;

pub async fn run_today(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let captures_resp = client.list_captures_recent(20, true).await?;
    let captures = captures_resp.data.expect("list_captures_recent missing data");
    let latest_ctx = client.get_artifact_latest("context_brief").await.ok().and_then(|r| r.data);

    if json {
        let out = serde_json::json!({
            "captures_today": captures.len(),
            "captures": captures,
            "latest_context_artifact": latest_ctx
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("=== Review: today ===\n");
    println!("Captures today: {}", captures.len());
    if !captures.is_empty() {
        for c in &captures {
            let content = if c.content_text.len() > TRUNCATE {
                format!("{}...", &c.content_text[..TRUNCATE])
            } else {
                c.content_text.clone()
            };
            println!("  {}  {}  {}", c.capture_id, c.occurred_at, content);
        }
    }
    println!();
    if let Some(Some(ref a)) = latest_ctx {
        println!("Latest context artifact: {}  ({})", a.artifact_id, a.storage_uri);
    } else {
        println!("Latest context artifact: (none)");
    }
    Ok(())
}

pub async fn run_week(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let captures_resp = client.list_captures_recent(50, false).await?;
    let captures = captures_resp.data.expect("list_captures_recent missing data");
    let latest_ctx = client.get_artifact_latest("context_brief").await.ok().and_then(|r| r.data);

    if json {
        let out = serde_json::json!({
            "captures_recent": captures.len(),
            "captures": captures,
            "latest_context_artifact": latest_ctx
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!("=== Review: week (recent {}) ===\n", captures.len());
    println!("Recent captures: {}", captures.len());
    if !captures.is_empty() {
        for c in &captures {
            let content = if c.content_text.len() > TRUNCATE {
                format!("{}...", &c.content_text[..TRUNCATE])
            } else {
                c.content_text.clone()
            };
            println!("  {}  {}  {}", c.capture_id, c.occurred_at, content);
        }
    }
    println!();
    if let Some(Some(ref a)) = latest_ctx {
        println!("Latest context artifact: {}  ({})", a.artifact_id, a.storage_uri);
    } else {
        println!("Latest context artifact: (none)");
    }
    Ok(())
}
