use anyhow::anyhow;
use std::time::Duration;
use vel_api_types::{
    ConnectAttachData, ConnectInstanceCapabilityManifestData, ConnectInstanceData,
};

use crate::client::{
    ApiClient, ConnectHeartbeatRequestData, ConnectLaunchRequestData, ConnectStdinRequestData,
    ConnectTerminateRequestData,
};

pub async fn run_list_instances(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.list_connect_instances().await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let instances = response
        .data
        .ok_or_else(|| anyhow!("connect instances response missing data"))?;
    if instances.is_empty() {
        println!("No connect instances.");
        println!(
            "Launches follow the repo-local workflow: `vel exec export`, `vel exec review`, then authenticated `POST /v1/connect/instances`."
        );
        return Ok(());
    }

    println!(
        "{:<14} {:<20} {:<12} {:<12} RUNTIMES",
        "INSTANCE ID", "DISPLAY", "STATUS", "REACHABILITY"
    );
    for instance in instances {
        println!(
            "{:<14} {:<20} {:<12} {:<12} {}",
            instance.id,
            truncate(&instance.display_name, 20),
            instance.status,
            instance.reachability,
            format_launchable_runtimes(&instance.manifest)
        );
    }
    println!(
        "\nUse `vel exec review` to inspect pending handoffs before trusting runtime launches."
    );
    Ok(())
}

pub async fn run_inspect_instance(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let response = client.get_connect_instance(id).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let instance = response
        .data
        .ok_or_else(|| anyhow!("connect inspect response missing data"))?;
    print_instance(&instance);
    Ok(())
}

pub async fn run_attach_instance(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let response = client.attach_connect_instance(id).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }
    let attach = response
        .data
        .ok_or_else(|| anyhow!("connect attach response missing data"))?;
    print_attach(&attach);
    Ok(())
}

pub async fn run_launch_instance(
    client: &ApiClient,
    payload: ConnectLaunchRequestData,
    json: bool,
) -> anyhow::Result<()> {
    let response = client.launch_connect_instance(&payload).await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let instance = response
        .data
        .ok_or_else(|| anyhow!("connect launch response missing data"))?;
    print_instance(&instance);
    Ok(())
}

pub async fn run_heartbeat_instance(
    client: &ApiClient,
    id: &str,
    status: &str,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .heartbeat_connect_instance(
            id,
            &ConnectHeartbeatRequestData {
                status: status.to_string(),
            },
        )
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let ack = response
        .data
        .ok_or_else(|| anyhow!("connect heartbeat response missing data"))?;
    println!("id: {}", ack.id);
    println!("status: {}", ack.status);
    println!("lease_expires_at: {}", ack.lease_expires_at);
    println!("trace_id: {}", ack.trace_id);
    Ok(())
}

pub async fn run_terminate_instance(
    client: &ApiClient,
    id: &str,
    reason: &str,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .terminate_connect_instance(
            id,
            &ConnectTerminateRequestData {
                reason: reason.to_string(),
            },
        )
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let instance = response
        .data
        .ok_or_else(|| anyhow!("connect terminate response missing data"))?;
    print_instance(&instance);
    Ok(())
}

pub async fn run_stdin_instance(
    client: &ApiClient,
    id: &str,
    input: &str,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .write_connect_instance_stdin(
            id,
            &ConnectStdinRequestData {
                input: input.to_string(),
            },
        )
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }
    let ack = response
        .data
        .ok_or_else(|| anyhow!("connect stdin response missing data"))?;
    println!("run_id: {}", ack.run_id);
    println!("accepted_bytes: {}", ack.accepted_bytes);
    println!("event_id: {}", ack.event_id);
    println!("trace_id: {}", ack.trace_id.as_deref().unwrap_or("—"));
    Ok(())
}

pub async fn run_events_instance(
    client: &ApiClient,
    id: &str,
    after_id: Option<i64>,
    limit: Option<u32>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .list_connect_instance_events(id, after_id, limit)
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }
    let events = response
        .data
        .ok_or_else(|| anyhow!("connect events response missing data"))?;
    if events.is_empty() {
        println!("No connect events.");
        return Ok(());
    }
    for event in events {
        print_event_line(event.id, event.created_at, &event.stream, &event.chunk);
    }
    Ok(())
}

pub async fn run_tail_instance(
    client: &ApiClient,
    id: &str,
    mut after_id: Option<i64>,
    limit: u32,
    poll_ms: u64,
    once: bool,
) -> anyhow::Result<()> {
    if limit == 0 {
        return Err(anyhow!("`--limit` must be greater than 0"));
    }
    if poll_ms == 0 {
        return Err(anyhow!("`--poll-ms` must be greater than 0"));
    }

    if once {
        return poll_events_once(client, id, &mut after_id, limit).await;
    }

    println!(
        "Tailing events for {id}. Press Ctrl-C to stop. (after_id={}, limit={}, poll_ms={poll_ms})",
        after_id
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        limit
    );
    loop {
        poll_events_once(client, id, &mut after_id, limit).await?;
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("Stopped tail for {id}.");
                return Ok(());
            }
            _ = tokio::time::sleep(Duration::from_millis(poll_ms)) => {}
        }
    }
}

pub async fn run_stream_instance(
    client: &ApiClient,
    id: &str,
    after_id: Option<i64>,
    limit: u32,
    poll_ms: u64,
    max_events: Option<u32>,
) -> anyhow::Result<()> {
    if limit == 0 {
        return Err(anyhow!("`--limit` must be greater than 0"));
    }
    if poll_ms == 0 {
        return Err(anyhow!("`--poll-ms` must be greater than 0"));
    }

    let response = client
        .stream_connect_instance_events(id, after_id, Some(limit), Some(poll_ms), max_events)
        .await?;
    let mut response = response;
    let mut buffer = String::new();

    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| anyhow!("stream read failed: {error}"))?
    {
        let text = String::from_utf8_lossy(&chunk);
        buffer.push_str(&text);

        while let Some(idx) = buffer.find("\n\n") {
            let frame = buffer[..idx].to_string();
            buffer.drain(..idx + 2);
            if let Some((event_name, data)) = parse_sse_frame(&frame) {
                match event_name.as_deref() {
                    Some("connect_event") | None => {
                        if let Ok(event) =
                            serde_json::from_str::<crate::client::ConnectRunEventData>(&data)
                        {
                            print_event_line(
                                event.id,
                                event.created_at,
                                &event.stream,
                                &event.chunk,
                            );
                        } else {
                            println!("{data}");
                        }
                    }
                    Some("connect_error") => {
                        return Err(anyhow!("connect stream error: {data}"));
                    }
                    Some(_) => {}
                }
            }
        }
    }

    Ok(())
}

async fn poll_events_once(
    client: &ApiClient,
    id: &str,
    after_id: &mut Option<i64>,
    limit: u32,
) -> anyhow::Result<()> {
    let response = client
        .list_connect_instance_events(id, *after_id, Some(limit))
        .await?;
    let events = response
        .data
        .ok_or_else(|| anyhow!("connect events response missing data"))?;
    for event in events {
        print_event_line(event.id, event.created_at, &event.stream, &event.chunk);
        *after_id = Some(after_id.map_or(event.id, |current| current.max(event.id)));
    }
    Ok(())
}

fn print_event_line(id: i64, created_at: i64, stream: &str, chunk: &str) {
    println!("[{id}] {created_at} {stream} {chunk}");
}

fn print_attach(attach: &ConnectAttachData) {
    println!(
        "latest_event_id: {}",
        attach
            .latest_event_id
            .map_or("—".to_string(), |v| v.to_string())
    );
    println!("stream_path: {}", attach.stream_path);
    print_instance(&attach.instance);
}

fn parse_sse_frame(frame: &str) -> Option<(Option<String>, String)> {
    let mut event_name: Option<String> = None;
    let mut data_lines = Vec::new();
    for line in frame.lines() {
        if let Some(value) = line.strip_prefix("event:") {
            event_name = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("data:") {
            data_lines.push(value.trim().to_string());
        }
    }
    if data_lines.is_empty() {
        return None;
    }
    Some((event_name, data_lines.join("\n")))
}

fn print_instance(instance: &ConnectInstanceData) {
    println!("id: {}", instance.id);
    println!("display_name: {}", instance.display_name);
    println!("node_id: {}", instance.node_id);
    println!("status: {}", instance.status);
    println!("reachability: {}", instance.reachability);
    println!(
        "connection_id: {}",
        instance.connection_id.as_deref().unwrap_or("—")
    );
    println!(
        "sync_transport: {}",
        instance.sync_transport.as_deref().unwrap_or("—")
    );
    println!(
        "sync_base_url: {}",
        instance.sync_base_url.as_deref().unwrap_or("—")
    );
    println!(
        "last_seen_at: {}",
        instance
            .last_seen_at
            .map(|value| value.to_string())
            .unwrap_or_else(|| "—".to_string())
    );
    println!("worker_ids: {}", join_or_dash(&instance.worker_ids));
    println!("worker_classes: {}", join_or_dash(&instance.worker_classes));
    println!(
        "manifest.capabilities: {}",
        join_or_dash(&instance.manifest.capabilities)
    );
    println!(
        "manifest.launchable_runtimes: {}",
        format_launchable_runtimes(&instance.manifest)
    );
    println!(
        "manifest.supports_agent_launch: {}",
        instance.manifest.supports_agent_launch
    );
    println!(
        "manifest.supports_interactive_followup: {}",
        instance.manifest.supports_interactive_followup
    );
    println!(
        "manifest.supports_native_open: {}",
        instance.manifest.supports_native_open
    );
    println!(
        "manifest.supports_host_agent_control: {}",
        instance.manifest.supports_host_agent_control
    );
    println!(
        "metadata: {}",
        serde_json::to_string_pretty(&instance.metadata)
            .unwrap_or_else(|_| instance.metadata.to_string())
    );
}

fn join_or_dash(values: &[String]) -> String {
    if values.is_empty() {
        "—".to_string()
    } else {
        values.join(", ")
    }
}

fn format_launchable_runtimes(manifest: &ConnectInstanceCapabilityManifestData) -> String {
    if manifest.launchable_runtimes.is_empty() {
        return "—".to_string();
    }

    manifest
        .launchable_runtimes
        .iter()
        .map(|runtime| runtime.runtime_id.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn truncate(value: &str, max_len: usize) -> String {
    if value.chars().count() <= max_len {
        return value.to_string();
    }
    if max_len <= 3 {
        return ".".repeat(max_len);
    }

    let truncated: String = value.chars().take(max_len - 3).collect();
    format!("{truncated}...")
}

#[cfg(test)]
mod tests {
    use super::{format_launchable_runtimes, join_or_dash, parse_sse_frame, truncate};
    use vel_api_types::{ConnectInstanceCapabilityManifestData, ConnectRuntimeCapabilityData};

    #[test]
    fn connect_join_or_dash_handles_empty_lists() {
        assert_eq!(join_or_dash(&[]), "—");
        assert_eq!(
            join_or_dash(&["agent-a".to_string(), "agent-b".to_string()]),
            "agent-a, agent-b"
        );
    }

    #[test]
    fn connect_launchable_runtimes_are_summarized_by_runtime_id() {
        let manifest = ConnectInstanceCapabilityManifestData {
            worker_classes: vec![],
            capabilities: vec![],
            launchable_runtimes: vec![
                ConnectRuntimeCapabilityData {
                    runtime_id: "local_command".to_string(),
                    display_name: "Local Command".to_string(),
                    supports_launch: true,
                    supports_interactive_followup: false,
                    supports_native_open: false,
                    supports_host_agent_control: true,
                },
                ConnectRuntimeCapabilityData {
                    runtime_id: "other".to_string(),
                    display_name: "Other".to_string(),
                    supports_launch: true,
                    supports_interactive_followup: false,
                    supports_native_open: false,
                    supports_host_agent_control: false,
                },
            ],
            supports_agent_launch: true,
            supports_interactive_followup: false,
            supports_native_open: false,
            supports_host_agent_control: true,
        };

        assert_eq!(
            format_launchable_runtimes(&manifest),
            "local_command, other"
        );
    }

    #[test]
    fn connect_truncate_shortens_long_values() {
        assert_eq!(truncate("short", 8), "short");
        assert_eq!(truncate("abcdefghijkl", 8), "abcde...");
    }

    #[test]
    fn connect_parse_sse_frame_extracts_event_and_data() {
        let frame = "event: connect_event\ndata: {\"id\":1}\n";
        let parsed = parse_sse_frame(frame).expect("frame should parse");
        assert_eq!(parsed.0.as_deref(), Some("connect_event"));
        assert_eq!(parsed.1, "{\"id\":1}");
    }
}
