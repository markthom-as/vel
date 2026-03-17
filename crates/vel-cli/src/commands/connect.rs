use crate::client::ApiClient;
use anyhow::Context;

pub async fn run_list_instances(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client
        .list_connect_instances()
        .await
        .context("list connect instances")?;
    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;

    if json {
        println!("{}", serde_json::to_string_pretty(data)?);
        return Ok(());
    }

    if data.is_empty() {
        println!("No connect instances.");
        return Ok(());
    }

    for instance in data {
        println!(
            "{}  {}  {}  {}",
            instance.id, instance.status, instance.reachability, instance.display_name
        );
        if let Some(sync_base_url) = instance.sync_base_url.as_deref() {
            println!(
                "  sync: {} ({})",
                sync_base_url,
                instance.sync_transport.as_deref().unwrap_or("unknown")
            );
        }
        if instance.manifest.supports_agent_launch {
            let runtimes = instance
                .manifest
                .launchable_runtimes
                .iter()
                .map(|runtime| runtime.runtime_id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            println!("  launchable_runtimes: {}", runtimes);
        } else {
            println!("  launchable_runtimes: -");
        }
    }

    Ok(())
}

pub async fn run_inspect_instance(client: &ApiClient, id: &str, json: bool) -> anyhow::Result<()> {
    let resp = client
        .get_connect_instance(id)
        .await
        .context("get connect instance")?;
    let instance = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;

    if json {
        println!("{}", serde_json::to_string_pretty(instance)?);
        return Ok(());
    }

    println!("id:             {}", instance.id);
    println!("node_id:        {}", instance.node_id);
    println!("display_name:   {}", instance.display_name);
    println!("status:         {}", instance.status);
    println!("reachability:   {}", instance.reachability);
    println!(
        "sync:           {} ({})",
        instance.sync_base_url.as_deref().unwrap_or("-"),
        instance.sync_transport.as_deref().unwrap_or("-")
    );
    println!(
        "tailscale_url:  {}",
        instance.tailscale_base_url.as_deref().unwrap_or("-")
    );
    println!(
        "lan_url:        {}",
        instance.lan_base_url.as_deref().unwrap_or("-")
    );
    println!(
        "localhost_url:  {}",
        instance.localhost_base_url.as_deref().unwrap_or("-")
    );
    println!(
        "last_seen_at:   {}",
        instance
            .last_seen_at
            .map(|value| value.to_string())
            .unwrap_or_else(|| "-".to_string())
    );
    println!("worker_ids:     {}", instance.worker_ids.join(", "));
    println!("worker_classes: {}", instance.worker_classes.join(", "));
    println!(
        "supports_launch: {}",
        instance.manifest.supports_agent_launch
    );
    if instance.manifest.launchable_runtimes.is_empty() {
        println!("launchable_runtimes: []");
    } else {
        println!("launchable_runtimes:");
        for runtime in &instance.manifest.launchable_runtimes {
            println!(
                "  - {} ({}) launch={} followup={} native_open={} host_control={}",
                runtime.runtime_id,
                runtime.display_name,
                runtime.supports_launch,
                runtime.supports_interactive_followup,
                runtime.supports_native_open,
                runtime.supports_host_agent_control
            );
        }
    }
    println!(
        "capabilities:   {}",
        if instance.manifest.capabilities.is_empty() {
            "-".to_string()
        } else {
            instance.manifest.capabilities.join(", ")
        }
    );
    println!(
        "metadata:       {}",
        serde_json::to_string_pretty(&instance.metadata)?
    );

    Ok(())
}
