use crate::client::ApiClient;
use anyhow::{anyhow, bail};
use vel_api_types::{LinkScopeData, LinkStatusData, LinkedNodeData};

/// Handler for `vel node link issue`.
pub async fn run_link_issue(
    client: &ApiClient,
    issued_by_node_id: Option<&str>,
    configured_node_id: Option<&str>,
    expires_seconds: Option<i64>,
    read_context: bool,
    write_safe_actions: bool,
    execute_repo_tasks: bool,
    json: bool,
) -> anyhow::Result<()> {
    let issued_by_node_id = issued_by_node_id
        .or(configured_node_id)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow!("issued_by_node_id is required; pass --issued-by-node-id or configure node_id")
        })?;

    let scopes = LinkScopeData {
        read_context,
        write_safe_actions,
        execute_repo_tasks,
    };
    ensure_any_scope_requested(&scopes)?;

    let response = client
        .issue_pairing_token(issued_by_node_id, expires_seconds, scopes)
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let token = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("link issue response missing data"))?;
    println!("token_code: {}", token.token_code);
    println!("token_id: {}", token.token_id);
    println!("issued_by_node_id: {}", token.issued_by_node_id);
    println!("expires_at: {}", token.expires_at);
    println!("granted scopes: {}", format_scope_summary(&token.scopes));
    if !token.suggested_targets.is_empty() {
        println!("suggested targets:");
        for target in &token.suggested_targets {
            let marker = if target.recommended {
                "recommended"
            } else {
                "fallback"
            };
            println!("- {} [{}]: {}", target.label, marker, target.base_url);
            println!("  transport_hint: {}", target.transport_hint);
            println!("  redeem: {}", target.redeem_command_hint);
        }
    }
    println!(
        "next: vel node link redeem {} --node-id <node_id> --node-display-name <name>",
        token.token_code
    );
    Ok(())
}

/// Handler for `vel node link redeem`.
pub async fn run_link_redeem(
    client: &ApiClient,
    token_code: &str,
    node_id: &str,
    node_display_name: &str,
    transport_hint: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let response = client
        .redeem_pairing_token(token_code, node_id, node_display_name, transport_hint)
        .await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let linked_node = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("link redeem response missing data"))?;
    print_node_record(linked_node);
    Ok(())
}

/// Handler for `vel node status`.
pub async fn run_status(client: &ApiClient, json: bool) -> anyhow::Result<()> {
    let response = client.load_linking_status().await?;
    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let nodes = response
        .data
        .as_ref()
        .ok_or_else(|| anyhow!("node status response missing data"))?;
    if nodes.is_empty() {
        println!("no linked nodes");
        return Ok(());
    }

    for node in nodes {
        print_node_record(node);
        println!();
    }
    Ok(())
}

fn ensure_any_scope_requested(scopes: &LinkScopeData) -> anyhow::Result<()> {
    if !scopes.read_context && !scopes.write_safe_actions && !scopes.execute_repo_tasks {
        bail!(
            "at least one scope must be requested (--read-context, --write-safe-actions, or --execute-repo-tasks)"
        );
    }
    Ok(())
}

fn print_node_record(node: &LinkedNodeData) {
    println!("node: {} ({})", node.node_display_name, node.node_id);
    println!("status: {}", format_link_status(node.status));
    println!("granted scopes: {}", format_scope_summary(&node.scopes));
    println!("linked_at: {}", node.linked_at);
    if let Some(last_seen_at) = node.last_seen_at {
        println!("last_seen_at: {}", last_seen_at);
    }
    if let Some(transport_hint) = node.transport_hint.as_deref() {
        println!("transport_hint: {}", transport_hint);
    }
}

fn format_link_status(status: LinkStatusData) -> &'static str {
    match status {
        LinkStatusData::Pending => "pending",
        LinkStatusData::Linked => "linked",
        LinkStatusData::Revoked => "revoked",
        LinkStatusData::Expired => "expired",
    }
}

fn format_scope_summary(scopes: &LinkScopeData) -> String {
    let mut labels = Vec::new();
    if scopes.read_context {
        labels.push("read_context");
    }
    if scopes.write_safe_actions {
        labels.push("write_safe_actions");
    }
    if scopes.execute_repo_tasks {
        labels.push("execute_repo_tasks");
    }
    if labels.is_empty() {
        "none".to_string()
    } else {
        labels.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_scope_summary_lists_requested_scopes() {
        let summary = format_scope_summary(&LinkScopeData {
            read_context: true,
            write_safe_actions: false,
            execute_repo_tasks: true,
        });
        assert_eq!(summary, "read_context, execute_repo_tasks");
    }

    #[test]
    fn node_scope_validation_rejects_empty_request() {
        let error = ensure_any_scope_requested(&LinkScopeData::default()).unwrap_err();
        assert!(error
            .to_string()
            .contains("at least one scope must be requested"));
    }
}
