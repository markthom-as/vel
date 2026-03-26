use crate::client::ApiClient;
use anyhow::Context;
use vel_api_types::{
    GoogleCalendarIntegrationData, IntegrationLogEventData, IntegrationsData, LocalIntegrationData,
    TodoistIntegrationData,
};

fn summarize_google_calendar(data: &GoogleCalendarIntegrationData) -> String {
    format!(
        "google_calendar  configured={} connected={} calendars={} last_sync_status={} last_item_count={}",
        data.configured,
        data.connected,
        data.calendars.len(),
        data.last_sync_status.as_deref().unwrap_or("-"),
        data
            .last_item_count
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string())
    )
}

fn summarize_todoist(data: &TodoistIntegrationData) -> String {
    format!(
        "todoist  configured={} connected={} last_sync_status={} last_item_count={}",
        data.configured,
        data.connected,
        data.last_sync_status.as_deref().unwrap_or("-"),
        data.last_item_count
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string())
    )
}

fn summarize_local(name: &str, data: &LocalIntegrationData) -> String {
    format!(
        "{}  configured={} source_kind={} last_sync_status={} last_item_count={}",
        name,
        data.configured,
        data.source_kind,
        data.last_sync_status.as_deref().unwrap_or("-"),
        data.last_item_count
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string())
    )
}

fn summarize_log(event: &IntegrationLogEventData) -> String {
    format!(
        "{}  {}  {}  {}",
        event.created_at, event.status, event.event_name, event.message
    )
}

pub async fn run_show(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_integrations()
        .await
        .context("get integrations")?;
    let data = resp
        .data
        .ok_or_else(|| anyhow::anyhow!("integrations response missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }
    for line in integrations_lines(&data) {
        println!("{line}");
    }
    Ok(())
}

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
            println!(
                "  - {}  {}  {}",
                event.id, event.event_type, event.timestamp
            );
            println!("    {}", serde_json::to_string_pretty(&event.payload)?);
        }
    }

    Ok(())
}

pub async fn run_logs(client: &ApiClient, id: &str, limit: u32, json: bool) -> anyhow::Result<()> {
    let resp = client
        .list_integration_logs(id, Some(limit))
        .await
        .with_context(|| format!("list integration logs for {}", id))?;
    let events = resp
        .data
        .ok_or_else(|| anyhow::anyhow!("integration log response missing data"))?;

    if json {
        println!("{}", serde_json::to_string_pretty(&events)?);
        return Ok(());
    }

    if events.is_empty() {
        println!("No integration logs for {id}.");
        return Ok(());
    }

    for event in &events {
        println!("{}", summarize_log(event));
    }

    Ok(())
}

fn integrations_lines(data: &IntegrationsData) -> Vec<String> {
    vec![
        summarize_google_calendar(&data.google_calendar),
        summarize_todoist(&data.todoist),
        summarize_local("activity", &data.activity),
        summarize_local("health", &data.health),
        summarize_local("git", &data.git),
        summarize_local("messaging", &data.messaging),
        summarize_local("reminders", &data.reminders),
        summarize_local("notes", &data.notes),
        summarize_local("transcripts", &data.transcripts),
    ]
}

#[cfg(test)]
mod tests {
    use super::{integrations_lines, summarize_log};
    use vel_api_types::{
        GoogleCalendarIntegrationData, IntegrationCalendarData, IntegrationLogEventData,
        IntegrationsData, LocalIntegrationData, TodoistIntegrationData,
        TodoistWriteCapabilitiesData,
    };

    fn local(source_kind: &str) -> LocalIntegrationData {
        LocalIntegrationData {
            configured: true,
            guidance: None,
            source_path: Some(format!("/tmp/{source_kind}.json")),
            selected_paths: Vec::new(),
            available_paths: Vec::new(),
            internal_paths: Vec::new(),
            suggested_paths: Vec::new(),
            source_kind: source_kind.to_string(),
            last_sync_at: None,
            last_sync_status: Some("ok".to_string()),
            last_error: None,
            last_item_count: Some(2),
        }
    }

    #[test]
    fn integrations_lines_cover_provider_and_local_surfaces() {
        let data = IntegrationsData {
            google_calendar: GoogleCalendarIntegrationData {
                configured: true,
                connected: true,
                has_client_id: true,
                has_client_secret: true,
                calendars: vec![IntegrationCalendarData {
                    id: "cal_1".to_string(),
                    summary: "Primary".to_string(),
                    primary: true,
                    sync_enabled: true,
                    display_enabled: true,
                }],
                all_calendars_selected: false,
                last_sync_at: None,
                last_sync_status: Some("ok".to_string()),
                last_error: None,
                last_item_count: Some(8),
                guidance: None,
            },
            todoist: TodoistIntegrationData {
                configured: true,
                connected: false,
                has_api_token: true,
                last_sync_at: None,
                last_sync_status: Some("warn".to_string()),
                last_error: Some("reauth needed".to_string()),
                last_item_count: Some(4),
                guidance: None,
                write_capabilities: TodoistWriteCapabilitiesData {
                    completion_status: true,
                    due_date: true,
                    tags: false,
                },
            },
            activity: local("activity"),
            health: local("health"),
            git: local("git"),
            messaging: local("messaging"),
            reminders: local("reminders"),
            notes: local("notes"),
            transcripts: local("transcripts"),
        };

        let lines = integrations_lines(&data);

        assert!(lines.iter().any(|line| line.contains("google_calendar")));
        assert!(lines.iter().any(|line| line.contains("todoist")));
        assert!(lines.iter().any(|line| line.contains("messaging")));
    }

    #[test]
    fn summarize_log_includes_status_event_name_and_message() {
        let event = IntegrationLogEventData {
            id: "ilog_1".to_string(),
            integration_id: "todoist".to_string(),
            event_name: "sync".to_string(),
            status: "error".to_string(),
            message: "token expired".to_string(),
            payload: serde_json::json!({}),
            created_at: 1_742_927_200,
        };

        let rendered = summarize_log(&event);

        assert!(rendered.contains("error"));
        assert!(rendered.contains("sync"));
        assert!(rendered.contains("token expired"));
    }
}
