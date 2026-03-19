use std::collections::HashMap;

use time::{Duration, OffsetDateTime};
use vel_api_types::{LinkScopeData, LinkTargetSuggestionData, LinkingPromptData};
use vel_core::{LinkScope, LinkStatus, LinkedNodeRecord, PairingTokenRecord};

use crate::{errors::AppError, state::AppState};

const DEFAULT_TOKEN_TTL_SECONDS: i64 = 900;
const MAX_TOKEN_TTL_SECONDS: i64 = 3600;
const LINKING_PROMPTS_SETTINGS_KEY: &str = "linking_prompts";

pub async fn issue_pairing_token(
    state: &AppState,
    request: IssuePairingTokenInput,
) -> Result<PairingTokenRecord, AppError> {
    let issued_by_node_id = request.issued_by_node_id.trim();
    if issued_by_node_id.is_empty() {
        return Err(AppError::bad_request("issued_by_node_id must not be empty"));
    }

    let ttl_seconds = request.ttl_seconds.unwrap_or(DEFAULT_TOKEN_TTL_SECONDS);
    if ttl_seconds <= 0 {
        return Err(AppError::bad_request("ttl_seconds must be positive"));
    }
    if ttl_seconds > MAX_TOKEN_TTL_SECONDS {
        return Err(AppError::bad_request("ttl_seconds exceeds maximum"));
    }

    let scopes = scope_from_data(request.scopes);
    validate_requested_scopes(&scopes)?;

    let issued_at = OffsetDateTime::now_utc();
    let record = PairingTokenRecord {
        token_id: format!("pair_{}", uuid::Uuid::new_v4().simple()),
        token_code: format!("paircode_{}", uuid::Uuid::new_v4().simple()),
        issued_at,
        expires_at: issued_at + Duration::seconds(ttl_seconds),
        issued_by_node_id: issued_by_node_id.to_string(),
        scopes,
    };

    let issued = state.storage.issue_pairing_token(&record).await?;
    if let Some(target_node_id) = request
        .target_node_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        save_linking_prompt(
            state,
            target_node_id,
            LinkingPromptData {
                target_node_id: target_node_id.to_string(),
                target_node_display_name: request
                    .target_node_display_name
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToString::to_string),
                issued_by_node_id: issued_by_node_id.to_string(),
                issued_by_node_display_name: issuer_display_name(state, issued_by_node_id).await?,
                issued_at: issued.issued_at,
                expires_at: issued.expires_at,
                scopes: issued.scopes.into(),
            },
        )
        .await?;
    }

    Ok(issued)
}

pub async fn redeem_pairing_token(
    state: &AppState,
    request: RedeemPairingTokenInput,
) -> Result<LinkedNodeRecord, AppError> {
    let token_code = request.token_code.trim();
    if token_code.is_empty() {
        return Err(AppError::bad_request("token_code must not be empty"));
    }
    let node_id = request.node_id.trim();
    if node_id.is_empty() {
        return Err(AppError::bad_request("node_id must not be empty"));
    }
    let node_display_name = request.node_display_name.trim();
    if node_display_name.is_empty() {
        return Err(AppError::bad_request("node_display_name must not be empty"));
    }

    let Some((token, redeemed_at)) = state.storage.get_pairing_token_by_code(token_code).await?
    else {
        return Err(AppError::bad_request(
            "pairing token is malformed or unknown",
        ));
    };
    if redeemed_at.is_some() {
        return Err(AppError::bad_request(
            "pairing token has already been redeemed",
        ));
    }

    let now = OffsetDateTime::now_utc();
    if token.expires_at < now {
        return Err(AppError::bad_request("pairing token has expired"));
    }

    let requested_scopes = request
        .requested_scopes
        .map(scope_from_data)
        .unwrap_or_else(|| token.scopes.clone());
    validate_requested_scopes(&requested_scopes)?;
    if !scopes_within(&requested_scopes, &token.scopes) {
        return Err(AppError::bad_request(
            "requested scopes are out of scope for token",
        ));
    }

    let record = LinkedNodeRecord {
        node_id: node_id.to_string(),
        node_display_name: node_display_name.to_string(),
        status: LinkStatus::Linked,
        scopes: requested_scopes,
        linked_at: now,
        last_seen_at: Some(now),
        transport_hint: request
            .transport_hint
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty()),
    };

    let marked = state
        .storage
        .mark_pairing_token_redeemed(token_code, now)
        .await?;
    if !marked {
        return Err(AppError::bad_request(
            "pairing token has already been redeemed",
        ));
    }

    clear_linking_prompt(state, &record.node_id).await?;
    state
        .storage
        .upsert_linked_node(&record)
        .await
        .map_err(Into::into)
}

pub async fn list_linked_nodes(state: &AppState) -> Result<Vec<LinkedNodeRecord>, AppError> {
    Ok(state.storage.list_linked_nodes().await?)
}

pub async fn revoke_linked_node(
    state: &AppState,
    node_id: &str,
) -> Result<LinkedNodeRecord, AppError> {
    let revoked_at = OffsetDateTime::now_utc();
    state
        .storage
        .revoke_linked_node(node_id.trim(), revoked_at)
        .await?
        .ok_or_else(|| AppError::not_found("linked node not found"))
}

pub async fn linking_prompts(
    state: &AppState,
) -> Result<HashMap<String, LinkingPromptData>, AppError> {
    let settings = state.storage.get_all_settings().await?;
    let Some(value) = settings.get(LINKING_PROMPTS_SETTINGS_KEY) else {
        return Ok(HashMap::new());
    };
    let mut prompts: HashMap<String, LinkingPromptData> = serde_json::from_value(value.clone())
        .map_err(|error| AppError::internal(format!("invalid linking prompts: {error}")))?;
    let now = OffsetDateTime::now_utc();
    prompts.retain(|_, prompt| prompt.expires_at > now);
    Ok(prompts)
}

pub async fn suggested_targets(
    state: &AppState,
    token_code: &str,
) -> Result<Vec<LinkTargetSuggestionData>, AppError> {
    let bootstrap = crate::services::client_sync::effective_cluster_bootstrap(state).await?;
    Ok(build_suggested_targets(&bootstrap, token_code))
}

fn build_suggested_targets(
    bootstrap: &crate::services::client_sync::ClusterBootstrap,
    token_code: &str,
) -> Vec<LinkTargetSuggestionData> {
    let mut targets = Vec::new();
    push_suggested_target(
        &mut targets,
        "Recommended",
        &bootstrap.sync_base_url,
        &bootstrap.sync_transport,
        true,
        token_code,
    );

    if let Some(url) = bootstrap.tailscale_base_url.as_deref() {
        push_suggested_target(
            &mut targets,
            "Tailscale",
            url,
            "tailscale",
            bootstrap.sync_transport == "tailscale",
            token_code,
        );
    }
    if let Some(url) = bootstrap.lan_base_url.as_deref() {
        push_suggested_target(
            &mut targets,
            "LAN",
            url,
            "lan",
            bootstrap.sync_transport == "lan",
            token_code,
        );
    }
    if let Some(url) = bootstrap.localhost_base_url.as_deref() {
        push_suggested_target(
            &mut targets,
            "Localhost",
            url,
            "localhost",
            bootstrap.sync_transport == "localhost",
            token_code,
        );
    }

    targets
}

fn push_suggested_target(
    targets: &mut Vec<LinkTargetSuggestionData>,
    label: &str,
    base_url: &str,
    transport_hint: &str,
    recommended: bool,
    token_code: &str,
) {
    let trimmed_url = base_url.trim();
    if trimmed_url.is_empty() || targets.iter().any(|target| target.base_url == trimmed_url) {
        return;
    }

    targets.push(LinkTargetSuggestionData {
        label: label.to_string(),
        base_url: trimmed_url.to_string(),
        transport_hint: transport_hint.to_string(),
        recommended,
        redeem_command_hint: format!(
            "vel --base-url {trimmed_url} node link redeem {token_code} --node-id <node_id> --node-display-name <name> --transport-hint {transport_hint}"
        ),
    });
}

fn validate_requested_scopes(scopes: &LinkScope) -> Result<(), AppError> {
    if !scopes.read_context && !scopes.write_safe_actions && !scopes.execute_repo_tasks {
        return Err(AppError::bad_request(
            "at least one linking scope must be requested",
        ));
    }
    Ok(())
}

fn scopes_within(requested: &LinkScope, granted: &LinkScope) -> bool {
    (!requested.read_context || granted.read_context)
        && (!requested.write_safe_actions || granted.write_safe_actions)
        && (!requested.execute_repo_tasks || granted.execute_repo_tasks)
}

#[derive(Debug, Clone)]
pub struct IssuePairingTokenInput {
    pub issued_by_node_id: String,
    pub ttl_seconds: Option<i64>,
    pub scopes: LinkScopeData,
    pub target_node_id: Option<String>,
    pub target_node_display_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RedeemPairingTokenInput {
    pub token_code: String,
    pub node_id: String,
    pub node_display_name: String,
    pub transport_hint: Option<String>,
    pub requested_scopes: Option<LinkScopeData>,
}

fn scope_from_data(value: LinkScopeData) -> LinkScope {
    LinkScope {
        read_context: value.read_context,
        write_safe_actions: value.write_safe_actions,
        execute_repo_tasks: value.execute_repo_tasks,
    }
}

async fn issuer_display_name(
    state: &AppState,
    issued_by_node_id: &str,
) -> Result<Option<String>, AppError> {
    let bootstrap = crate::services::client_sync::effective_cluster_bootstrap(state).await?;
    if bootstrap.node_id == issued_by_node_id {
        return Ok(Some(bootstrap.node_display_name));
    }
    Ok(None)
}

async fn save_linking_prompt(
    state: &AppState,
    target_node_id: &str,
    prompt: LinkingPromptData,
) -> Result<(), AppError> {
    let mut prompts = linking_prompts(state).await?;
    prompts.insert(target_node_id.to_string(), prompt);
    state
        .storage
        .set_setting(
            LINKING_PROMPTS_SETTINGS_KEY,
            &serde_json::to_value(prompts).map_err(|error| {
                AppError::internal(format!("serialize linking prompts: {error}"))
            })?,
        )
        .await?;
    Ok(())
}

async fn clear_linking_prompt(state: &AppState, target_node_id: &str) -> Result<(), AppError> {
    let mut prompts = linking_prompts(state).await?;
    if prompts.remove(target_node_id).is_none() {
        return Ok(());
    }
    state
        .storage
        .set_setting(
            LINKING_PROMPTS_SETTINGS_KEY,
            &serde_json::to_value(prompts).map_err(|error| {
                AppError::internal(format!("serialize linking prompts: {error}"))
            })?,
        )
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::broadcast;
    use vel_config::AppConfig;

    fn test_state(storage: vel_storage::Storage) -> AppState {
        let (broadcast_tx, _) = broadcast::channel(8);
        AppState::new(
            storage,
            AppConfig::default(),
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        )
    }

    #[tokio::test]
    async fn linking_service_issues_and_redeems_scoped_tokens() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);

        let token = issue_pairing_token(
            &state,
            IssuePairingTokenInput {
                issued_by_node_id: "node_alpha".to_string(),
                ttl_seconds: None,
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: true,
                    execute_repo_tasks: false,
                },
                target_node_id: None,
                target_node_display_name: None,
            },
        )
        .await
        .unwrap();

        let linked = redeem_pairing_token(
            &state,
            RedeemPairingTokenInput {
                token_code: token.token_code.clone(),
                node_id: "node_beta".to_string(),
                node_display_name: "Beta".to_string(),
                transport_hint: Some("tailscale".to_string()),
                requested_scopes: Some(LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                }),
            },
        )
        .await
        .unwrap();

        assert_eq!(linked.node_id, "node_beta");
        assert_eq!(linked.status, LinkStatus::Linked);
        assert!(linked.scopes.read_context);
        assert!(!linked.scopes.write_safe_actions);
    }

    #[tokio::test]
    async fn linking_service_fails_closed_for_out_of_scope_redeem() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);

        let token = issue_pairing_token(
            &state,
            IssuePairingTokenInput {
                issued_by_node_id: "node_alpha".to_string(),
                ttl_seconds: Some(900),
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                target_node_id: None,
                target_node_display_name: None,
            },
        )
        .await
        .unwrap();

        let error = redeem_pairing_token(
            &state,
            RedeemPairingTokenInput {
                token_code: token.token_code,
                node_id: "node_beta".to_string(),
                node_display_name: "Beta".to_string(),
                transport_hint: None,
                requested_scopes: Some(LinkScopeData {
                    read_context: true,
                    write_safe_actions: true,
                    execute_repo_tasks: true,
                }),
            },
        )
        .await
        .expect_err("out-of-scope request must fail");

        assert_eq!(
            error.to_string(),
            "requested scopes are out of scope for token"
        );
    }

    #[tokio::test]
    async fn linking_service_builds_suggested_targets_from_cluster_bootstrap() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let mut config = AppConfig::default();
        config.tailscale_base_url = Some("http://vel-desktop.tailnet.ts.net:4130".to_string());
        config.lan_base_url = Some("http://192.168.1.50:4130".to_string());
        let (broadcast_tx, _) = broadcast::channel(8);
        let state = AppState::new(
            storage,
            config,
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );

        let targets = suggested_targets(&state, "VEL-PAIR-123").await.unwrap();

        assert!(!targets.is_empty());
        assert_eq!(targets[0].transport_hint, "tailscale");
        assert!(targets[0].recommended);
        assert!(targets
            .iter()
            .any(|target| target.transport_hint == "lan" && !target.recommended));
        assert!(targets[0]
            .redeem_command_hint
            .contains("node link redeem VEL-PAIR-123"));
    }
}
