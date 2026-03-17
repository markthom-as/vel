use crate::client::ApiClient;
use anyhow::Context;
use vel_api_types::{ClusterBootstrapData, SyncClusterStateData};

pub async fn run_calendar(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_calendar().await.context("sync calendar")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("calendar: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_todoist(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_todoist().await.context("sync todoist")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("todoist: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_activity(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_activity().await.context("sync activity")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("activity: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_health(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_health().await.context("sync health")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("health: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_git(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_git().await.context("sync git")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("git: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_notes(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_notes().await.context("sync notes")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("notes: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_transcripts(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client
        .sync_transcripts()
        .await
        .context("sync transcripts")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("transcripts: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_messaging(client: &ApiClient) -> anyhow::Result<()> {
    let resp = client.sync_messaging().await.context("sync messaging")?;
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    println!("messaging: {} signals ingested", d.signals_ingested);
    Ok(())
}

pub async fn run_bootstrap(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client.sync_bootstrap().await.context("sync bootstrap")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }
    let d = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    print_bootstrap_cluster(&d.cluster);
    println!("nudges: {}", d.nudges.len());
    println!("commitments: {}", d.commitments.len());
    println!(
        "current_context: {}",
        if d.current_context.is_some() {
            "present"
        } else {
            "missing"
        }
    );
    Ok(())
}

pub async fn run_cluster(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client
        .cluster_bootstrap()
        .await
        .context("cluster bootstrap")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }
    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    print_bootstrap_cluster(data);
    Ok(())
}

pub async fn run_workers(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let resp = client
        .sync_cluster_state()
        .await
        .context("sync workers (requires GET /v1/sync/cluster)")?;
    if json {
        println!("{}", serde_json::to_string_pretty(&resp)?);
        return Ok(());
    }

    let data = resp
        .data
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("no data"))?;
    print_cluster_state_summary(data);
    Ok(())
}

fn print_bootstrap_cluster(cluster: &ClusterBootstrapData) {
    println!("node_id: {}", cluster.node_id);
    println!("node_display_name: {}", cluster.node_display_name);
    println!("authority_node_id: {}", cluster.active_authority_node_id);
    println!("authority_epoch: {}", cluster.active_authority_epoch);
    println!("sync_transport: {}", cluster.sync_transport);
    println!("sync_base_url: {}", cluster.sync_base_url);
    println!(
        "tailscale_base_url: {}",
        cluster.tailscale_base_url.as_deref().unwrap_or("-")
    );
    println!(
        "lan_base_url: {}",
        cluster.lan_base_url.as_deref().unwrap_or("-")
    );
    println!(
        "localhost_base_url: {}",
        cluster.localhost_base_url.as_deref().unwrap_or("-")
    );
    if cluster.capabilities.is_empty() {
        println!("capabilities: -");
    } else {
        println!("capabilities: {}", cluster.capabilities.join(", "));
    }
    match &cluster.branch_sync {
        Some(branch_sync) => {
            println!("branch_sync: enabled");
            println!("branch_sync_repo_root: {}", branch_sync.repo_root);
            println!("branch_sync_default_remote: {}", branch_sync.default_remote);
            println!("branch_sync_supports_fetch: {}", branch_sync.supports_fetch);
            println!("branch_sync_supports_pull: {}", branch_sync.supports_pull);
            println!("branch_sync_supports_push: {}", branch_sync.supports_push);
        }
        None => println!("branch_sync: unavailable"),
    }
    println!(
        "validation_profiles_count: {}",
        cluster.validation_profiles.len()
    );
    for profile in &cluster.validation_profiles {
        println!(
            "validation_profile: {} [{}] env={} cmd={}",
            profile.profile_id, profile.label, profile.environment, profile.command_hint
        );
    }
}

fn print_cluster_state_summary(state: &SyncClusterStateData) {
    let authority_node_id = state
        .cluster
        .as_ref()
        .map(|cluster| cluster.active_authority_node_id.as_str())
        .or(state.authority_node_id.as_deref())
        .unwrap_or("-");
    let authority_epoch = state
        .cluster
        .as_ref()
        .map(|cluster| cluster.active_authority_epoch.to_string())
        .or_else(|| state.authority_epoch.map(|epoch| epoch.to_string()))
        .unwrap_or_else(|| "-".to_string());
    let sync_transport = state
        .cluster
        .as_ref()
        .map(|cluster| cluster.sync_transport.as_str())
        .or(state.sync_transport.as_deref())
        .unwrap_or("-");

    println!("authority_node_id: {}", authority_node_id);
    println!("authority_epoch: {}", authority_epoch);
    println!("sync_transport: {}", sync_transport);
    println!(
        "cluster_view_version: {}",
        state
            .cluster_view_version
            .map(|version| version.to_string())
            .unwrap_or_else(|| "-".to_string())
    );
    if let Some(cluster) = &state.cluster {
        println!(
            "tailscale_base_url: {}",
            cluster.tailscale_base_url.as_deref().unwrap_or("-")
        );
        println!(
            "lan_base_url: {}",
            cluster.lan_base_url.as_deref().unwrap_or("-")
        );
        println!(
            "localhost_base_url: {}",
            cluster.localhost_base_url.as_deref().unwrap_or("-")
        );
    }
    println!("nodes_count: {}", state.nodes.len());
    println!("workers_count: {}", state.workers.len());

    if !state.nodes.is_empty() {
        println!();
        println!(
            "{:<24} {:<12} {:<14} {:<6} {}",
            "NODE_ID", "CLASS", "REACHABILITY", "TAILNET", "LAST_SEEN"
        );
        for node in &state.nodes {
            let tailnet = if node.tailscale_base_url.is_some() {
                "yes"
            } else {
                "no"
            };
            println!(
                "{:<24} {:<12} {:<14} {:<6} {}",
                node.node_id,
                node.node_class.as_deref().unwrap_or("-"),
                node.reachability.as_deref().unwrap_or("-"),
                tailnet,
                node.last_seen_at
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "-".to_string())
            );
        }
    }

    if state.workers.is_empty() {
        println!();
        println!("No worker metadata published yet.");
        return;
    }

    let total_load: u32 = state
        .workers
        .iter()
        .filter_map(|worker| worker.current_load)
        .sum();
    let total_capacity: u32 = state
        .workers
        .iter()
        .filter_map(|worker| worker.max_concurrency)
        .sum();

    println!();
    println!(
        "{:<24} {:<16} {:<12} {:<10} {:<14} {:<8} {}",
        "WORKER_ID", "NODE_ID", "CLASS", "STATUS", "LOAD", "TAILNET", "FAIL_RATE"
    );
    for worker in &state.workers {
        let load = format_worker_load(worker.current_load, worker.max_concurrency);
        let fail_rate = worker
            .recent_failure_rate
            .map(|value| format!("{value:.3}"))
            .unwrap_or_else(|| "-".to_string());
        println!(
            "{:<24} {:<16} {:<12} {:<10} {:<14} {:<8} {}",
            worker.worker_id,
            worker.node_id.as_deref().unwrap_or("-"),
            worker.worker_class.as_deref().unwrap_or("-"),
            worker.status.as_deref().unwrap_or("-"),
            load,
            match worker.tailscale_preferred {
                Some(true) => "yes",
                Some(false) => "no",
                None => "-",
            },
            fail_rate
        );
    }

    if total_capacity > 0 {
        let utilization = (total_load as f64 / total_capacity as f64) * 100.0;
        println!("cluster_load: {total_load}/{total_capacity} ({utilization:.1}%)");
    }
}

fn format_worker_load(current: Option<u32>, max: Option<u32>) -> String {
    match (current, max) {
        (Some(current), Some(max)) => format!("{current}/{max}"),
        (Some(current), None) => format!("{current}/-"),
        (None, Some(max)) => format!("-/{max}"),
        (None, None) => "-".to_string(),
    }
}
