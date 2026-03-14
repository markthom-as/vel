//! `vel recent` — list recent captures.

use crate::client::ApiClient;

const TRUNCATE_LEN: usize = 60;

pub async fn run(
    client: &ApiClient,
    limit: u32,
    today: bool,
    json: bool,
) -> anyhow::Result<()> {
    let response = client.list_captures_recent(limit, today).await?;
    let captures = response.data.expect("list_captures_recent response missing data");
    if json {
        println!("{}", serde_json::to_string_pretty(&captures)?);
        return Ok(());
    }
    if captures.is_empty() {
        println!("No captures.");
        return Ok(());
    }
    println!(
        "{:<20} {:<12} {:<12} {:<20} {}",
        "CAPTURE ID", "TYPE", "SOURCE", "TIME", "CONTENT"
    );
    for c in &captures {
        let content = if c.content_text.len() > TRUNCATE_LEN {
            format!("{}...", &c.content_text[..TRUNCATE_LEN])
        } else {
            c.content_text.clone()
        };
        let source = c.source_device.as_deref().unwrap_or("—");
        println!(
            "{:<20} {:<12} {:<12} {:<20} {}",
            c.capture_id.to_string(),
            c.capture_type,
            source,
            c.occurred_at.to_string(),
            content
        );
    }
    Ok(())
}
