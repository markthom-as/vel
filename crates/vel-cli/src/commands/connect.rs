use anyhow::anyhow;
use vel_api_types::{ConnectInstanceCapabilityManifestData, ConnectInstanceData};

use crate::client::ApiClient;

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
    use super::{format_launchable_runtimes, join_or_dash, truncate};
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
}
