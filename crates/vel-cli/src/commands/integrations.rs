use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_list_connections(
    client: &ApiClient,
    family: Option<&str>,
    provider_key: Option<&str>,
    include_disabled: bool,
    json: bool,
) -> anyhow::Result<()> {
    let resp = client
        .list_integration_connections(family, provider_key, include_disabled)
        .await
        .context("list integration connections")?;
    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }
    if data.is_empty() {
        println!("No integration connections.");
        return Ok(());
    }
    for connection in data {
        println!(
            "{}  {}:{}  {}  {}",
            connection.id,
            connection.family,
            connection.provider_key,
            connection.status,
            connection.display_name
        );
        if let Some(account_ref) = connection.account_ref.as_deref() {
            println!("  account_ref: {}", account_ref);
        }
        if !connection.setting_refs.is_empty() {
            let keys = connection
                .setting_refs
                .iter()
                .map(|item| item.setting_key.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            println!("  settings: {}", keys);
        }
    }
    Ok(())
}

pub async fn run_inspect_connection(
    client: &ApiClient,
    id: &str,
    events_limit: u32,
    json: bool,
) -> anyhow::Result<()> {
    let connection = client
        .get_integration_connection(id)
        .await
        .context("get integration connection")?;
    let connection = connection
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no connection data"))?;
    let events = client
        .list_integration_connection_events(id, Some(events_limit))
        .await
        .context("list integration connection events")?;
    let events = events
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no connection event data"))?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "connection": connection,
                "events": events,
            }))?
        );
        return Ok(());
    }

    println!("id:           {}", connection.id);
    println!(
        "family:       {}:{}",
        connection.family, connection.provider_key
    );
    println!("status:       {}", connection.status);
    println!("display_name: {}", connection.display_name);
    println!(
        "account_ref:  {}",
        connection.account_ref.as_deref().unwrap_or("-")
    );
    println!("created_at:   {}", connection.created_at);
    println!("updated_at:   {}", connection.updated_at);
    println!(
        "metadata:     {}",
        serde_json::to_string_pretty(&connection.metadata)?
    );
    if connection.setting_refs.is_empty() {
        println!("setting_refs: []");
    } else {
        println!("setting_refs:");
        for item in &connection.setting_refs {
            println!(
                "  - {}={} @ {}",
                item.setting_key, item.setting_value, item.created_at
            );
        }
    }
    if events.is_empty() {
        println!("events:       []");
    } else {
        println!("events:");
        for event in events {
            println!("  - {}  {}  {}", event.id, event.event_type, event.timestamp);
            println!("    {}", serde_json::to_string_pretty(&event.payload)?);
        }
    }

    Ok(())
}
