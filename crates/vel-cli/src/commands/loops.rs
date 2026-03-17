use crate::client::ApiClient;
use vel_api_types::LoopUpdateRequest;

pub async fn run_list(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.list_loops().await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let loops = response.data.expect("list_loops response missing data");
    if loops.is_empty() {
        println!("No loop records yet.");
        return Ok(());
    }

    println!(
        "{:<28} {:<8} {:<10} {:<12} {}",
        "LOOP", "ENABLED", "INTERVAL", "STATUS", "NEXT DUE"
    );
    for loop_data in loops {
        println!(
            "{:<28} {:<8} {:<10} {:<12} {}",
            loop_data.kind,
            if loop_data.enabled { "yes" } else { "no" },
            format!("{}s", loop_data.interval_seconds),
            loop_data.last_status.unwrap_or_else(|| "—".to_string()),
            loop_data
                .next_due_at
                .map(|ts| ts.to_string())
                .unwrap_or_else(|| "—".to_string())
        );
    }
    Ok(())
}

pub async fn run_inspect(client: &ApiClient, kind: &str, json: bool) -> anyhow::Result<()> {
    let response = client.get_loop(kind).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let loop_data = response.data.expect("get_loop response missing data");
    println!("kind:             {}", loop_data.kind);
    println!("enabled:          {}", loop_data.enabled);
    println!("interval_seconds: {}", loop_data.interval_seconds);
    println!(
        "last_started_at:  {}",
        loop_data
            .last_started_at
            .map(|value| value.to_string())
            .unwrap_or_else(|| "—".to_string())
    );
    println!(
        "last_finished_at: {}",
        loop_data
            .last_finished_at
            .map(|value| value.to_string())
            .unwrap_or_else(|| "—".to_string())
    );
    println!(
        "last_status:      {}",
        loop_data.last_status.unwrap_or_else(|| "—".to_string())
    );
    println!(
        "last_error:       {}",
        loop_data.last_error.unwrap_or_else(|| "—".to_string())
    );
    println!(
        "next_due_at:      {}",
        loop_data
            .next_due_at
            .map(|value| value.to_string())
            .unwrap_or_else(|| "—".to_string())
    );
    Ok(())
}

pub async fn run_enable(client: &ApiClient, kind: &str) -> anyhow::Result<()> {
    let response = client
        .update_loop(
            kind,
            &LoopUpdateRequest {
                enabled: Some(true),
                interval_seconds: None,
            },
        )
        .await?;
    let loop_data = response.data.expect("update_loop response missing data");
    println!("Loop {} enabled.", loop_data.kind);
    Ok(())
}

pub async fn run_disable(client: &ApiClient, kind: &str) -> anyhow::Result<()> {
    let response = client
        .update_loop(
            kind,
            &LoopUpdateRequest {
                enabled: Some(false),
                interval_seconds: None,
            },
        )
        .await?;
    let loop_data = response.data.expect("update_loop response missing data");
    println!("Loop {} disabled.", loop_data.kind);
    Ok(())
}
