use crate::client::ApiClient;
use anyhow::Context;
use vel_api_types::{ComponentData, ComponentLogEventData};

fn summarize_component(component: &ComponentData) -> String {
    format!(
        "{}  {}  {}  restarts={}  {}",
        component.id,
        component.status,
        component.name,
        component.restart_count,
        component.description
    )
}

fn summarize_component_log(event: &ComponentLogEventData) -> String {
    format!(
        "{}  {}  {}  {}",
        event.created_at, event.status, event.event_name, event.message
    )
}

pub async fn run_list(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.list_components().await.context("list components")?;
    let components = response
        .data
        .ok_or_else(|| anyhow::anyhow!("component list missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&components)?);
        return Ok(());
    }
    if components.is_empty() {
        println!("No components.");
        return Ok(());
    }
    for component in &components {
        println!("{}", summarize_component(component));
    }
    Ok(())
}

pub async fn run_logs(client: &ApiClient, id: &str, limit: u32, json: bool) -> anyhow::Result<()> {
    let response = client
        .get_component_logs(id, Some(limit))
        .await
        .with_context(|| format!("get component logs for {}", id))?;
    let events = response
        .data
        .ok_or_else(|| anyhow::anyhow!("component logs missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&events)?);
        return Ok(());
    }
    if events.is_empty() {
        println!("No component logs for {id}.");
        return Ok(());
    }
    for event in &events {
        println!("{}", summarize_component_log(event));
    }
    Ok(())
}

pub async fn run_restart(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let response = client
        .restart_component(id)
        .await
        .with_context(|| format!("restart component {}", id))?;
    let component = response
        .data
        .ok_or_else(|| anyhow::anyhow!("component restart missing data"))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&component)?);
        return Ok(());
    }
    println!("{}", summarize_component(&component));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{summarize_component, summarize_component_log};
    use vel_api_types::{ComponentData, ComponentLogEventData};

    #[test]
    fn summarize_component_includes_status_and_restart_count() {
        let component = ComponentData {
            id: "evaluate".to_string(),
            name: "Evaluate".to_string(),
            description: "Inference evaluator".to_string(),
            status: "healthy".to_string(),
            last_restarted_at: Some(1_742_927_200),
            last_error: None,
            restart_count: 3,
        };

        let rendered = summarize_component(&component);

        assert!(rendered.contains("evaluate"));
        assert!(rendered.contains("healthy"));
        assert!(rendered.contains("restarts=3"));
    }

    #[test]
    fn summarize_component_log_includes_status_event_and_message() {
        let event = ComponentLogEventData {
            id: "log_1".to_string(),
            component_id: "evaluate".to_string(),
            event_name: "restart".to_string(),
            status: "ok".to_string(),
            message: "component restarted".to_string(),
            payload: serde_json::json!({}),
            created_at: 1_742_927_200,
        };

        let rendered = summarize_component_log(&event);

        assert!(rendered.contains("ok"));
        assert!(rendered.contains("restart"));
        assert!(rendered.contains("component restarted"));
    }
}
