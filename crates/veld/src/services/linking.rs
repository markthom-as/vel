use std::collections::HashMap;

use time::{Duration, OffsetDateTime};
use tracing::warn;
use vel_api_types::{
    ApiResponse, LinkScopeData, LinkTargetSuggestionData, LinkedNodeData, LinkingPromptData,
};
use vel_core::{
    trusted_node_endpoint_inventory_from_urls, LinkScope, LinkStatus, LinkedNodeRecord,
    PairingTokenRecord, TrustedNodeReachability,
};

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

    let local_bootstrap = crate::services::client_sync::effective_cluster_bootstrap(state).await?;
    if issued_by_node_id != local_bootstrap.node_id {
        return Err(AppError::bad_request(
            "issued_by_node_id must match the local node",
        ));
    }

    if request
        .target_node_id
        .as_deref()
        .map(str::trim)
        .is_some_and(|value| value == issued_by_node_id)
    {
        return Err(AppError::bad_request(
            "target_node_id must not match the issuing node",
        ));
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
        token_code: generate_unique_pairing_token_code(state).await?,
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
        let prompt = LinkingPromptData {
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
            issuer_sync_base_url: local_bootstrap.sync_base_url.clone(),
            issuer_sync_transport: local_bootstrap.sync_transport.clone(),
            issuer_tailscale_base_url: local_bootstrap.tailscale_base_url.clone(),
            issuer_lan_base_url: local_bootstrap.lan_base_url.clone(),
            issuer_localhost_base_url: local_bootstrap.localhost_base_url.clone(),
            issuer_public_base_url: None,
        };
        save_linking_prompt(state, target_node_id, prompt.clone()).await?;
        if let Some(target_base_url) = request
            .target_base_url
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            push_linking_prompt(target_base_url, &prompt).await?;
        }
    }

    Ok(issued)
}

pub async fn redeem_pairing_token(
    state: &AppState,
    request: RedeemPairingTokenInput,
) -> Result<LinkedNodeRecord, AppError> {
    let token_code = normalize_pairing_token_code(&request.token_code);
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

    let local_token = state
        .storage
        .get_pairing_token_by_code(&format_pairing_token_code(&token_code))
        .await?
        .or(state
            .storage
            .get_pairing_token_by_code(token_code.as_str())
            .await?);
    if let Some((token, redeemed_at)) = local_token {
        return redeem_local_pairing_token(state, request, token_code, token, redeemed_at).await;
    }

    redeem_pairing_token_via_prompt(state, request, token_code.as_str()).await
}

async fn redeem_local_pairing_token(
    state: &AppState,
    request: RedeemPairingTokenInput,
    _token_code: String,
    token: PairingTokenRecord,
    redeemed_at: Option<OffsetDateTime>,
) -> Result<LinkedNodeRecord, AppError> {
    let node_id = request.node_id.trim();
    let node_display_name = request.node_display_name.trim();
    if node_id == token.issued_by_node_id {
        return Err(AppError::bad_request(
            "node_id must not match the issuing node",
        ));
    }
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

    let existing = state
        .storage
        .list_linked_nodes()
        .await?
        .into_iter()
        .find(|record| record.node_id == node_id);
    let record = merge_linked_node(
        existing.as_ref(),
        linked_node_from_request(
            node_id,
            node_display_name,
            requested_scopes,
            now,
            request.transport_hint.as_deref(),
            request.sync_base_url.as_deref(),
            request.tailscale_base_url.as_deref(),
            request.lan_base_url.as_deref(),
            request.localhost_base_url.as_deref(),
            request.public_base_url.as_deref(),
        ),
    );

    let marked = state
        .storage
        .mark_pairing_token_redeemed(&token.token_code, now)
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

async fn redeem_pairing_token_via_prompt(
    state: &AppState,
    request: RedeemPairingTokenInput,
    token_code: &str,
) -> Result<LinkedNodeRecord, AppError> {
    let node_id = request.node_id.trim();
    let prompt = linking_prompts(state)
        .await?
        .remove(node_id)
        .ok_or_else(|| AppError::bad_request("pairing token is malformed or unknown"))?;
    if prompt.expires_at <= OffsetDateTime::now_utc() {
        clear_linking_prompt(state, node_id).await?;
        return Err(AppError::bad_request("pairing prompt has expired"));
    }
    if prompt.issued_by_node_id == node_id {
        clear_linking_prompt(state, node_id).await?;
        return Err(AppError::bad_request(
            "pairing prompt issuer must not match the redeeming node",
        ));
    }

    let redeem_base_url = prompt_redeem_base_url(&prompt).ok_or_else(|| {
        AppError::bad_request("pairing prompt does not advertise a redeemable address")
    })?;
    let requested_scopes = request.requested_scopes.unwrap_or(prompt.scopes);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(1500))
        .build()
        .map_err(|error| AppError::internal(format!("build pairing redeem client: {error}")))?;
    let response = client
        .post(format!(
            "{}/v1/linking/public-redeem",
            redeem_base_url.trim_end_matches('/')
        ))
        .json(&serde_json::json!({
            "token_code": token_code,
            "node_id": request.node_id,
            "node_display_name": request.node_display_name,
            "transport_hint": request.transport_hint,
            "requested_scopes": requested_scopes,
            "sync_base_url": request.sync_base_url,
            "tailscale_base_url": request.tailscale_base_url,
            "lan_base_url": request.lan_base_url,
            "localhost_base_url": request.localhost_base_url,
            "public_base_url": request.public_base_url,
        }))
        .send()
        .await
        .map_err(|error| AppError::internal(format!("forward pairing redeem: {error}")))?;
    if !response.status().is_success() {
        return Err(AppError::bad_request(format!(
            "issuer rejected pairing token with status {}",
            response.status()
        )));
    }
    let payload: ApiResponse<LinkedNodeData> = response
        .json()
        .await
        .map_err(|error| AppError::internal(format!("decode pairing redeem response: {error}")))?;
    let linked = payload
        .data
        .ok_or_else(|| AppError::internal("issuer pairing response omitted data"))?;
    let now = OffsetDateTime::now_utc();
    let existing = state
        .storage
        .list_linked_nodes()
        .await?
        .into_iter()
        .find(|record| record.node_id == prompt.issued_by_node_id);
    let local_record = merge_linked_node(
        existing.as_ref(),
        LinkedNodeRecord {
            node_id: prompt.issued_by_node_id,
            node_display_name: prompt
                .issued_by_node_display_name
                .unwrap_or(linked.node_display_name),
            status: LinkStatus::Linked,
            scopes: scope_from_data(linked.scopes),
            linked_at: now,
            last_seen_at: Some(now),
            transport_hint: Some(prompt.issuer_sync_transport),
            sync_base_url: Some(prompt.issuer_sync_base_url.clone()),
            tailscale_base_url: prompt.issuer_tailscale_base_url.clone(),
            lan_base_url: prompt.issuer_lan_base_url.clone(),
            localhost_base_url: prompt.issuer_localhost_base_url.clone(),
            public_base_url: prompt.issuer_public_base_url.clone(),
            endpoint_inventory: trusted_node_endpoint_inventory_from_urls(
                Some(prompt.issuer_sync_base_url.as_str()),
                prompt.issuer_tailscale_base_url.as_deref(),
                prompt.issuer_lan_base_url.as_deref(),
                prompt.issuer_localhost_base_url.as_deref(),
                prompt.issuer_public_base_url.as_deref(),
            ),
            reachability: TrustedNodeReachability::Unknown,
        },
    );
    clear_linking_prompt(state, node_id).await?;
    state.storage.upsert_linked_node(&local_record).await?;
    Ok(local_record)
}

pub async fn list_linked_nodes(state: &AppState) -> Result<Vec<LinkedNodeRecord>, AppError> {
    Ok(state.storage.list_linked_nodes().await?)
}

pub async fn revoke_linked_node(
    state: &AppState,
    node_id: &str,
) -> Result<LinkedNodeRecord, AppError> {
    let revoked_at = OffsetDateTime::now_utc();
    let existing = state
        .storage
        .list_linked_nodes()
        .await?
        .into_iter()
        .find(|record| record.node_id == node_id.trim())
        .ok_or_else(|| AppError::not_found("linked node not found"))?;
    let revoked = state
        .storage
        .revoke_linked_node(node_id.trim(), revoked_at)
        .await?
        .ok_or_else(|| AppError::not_found("linked node not found"))?;

    if let Some(target_base_url) = linked_node_target_base_url(&existing) {
        let local_node_id = crate::services::client_sync::effective_cluster_bootstrap(state)
            .await?
            .node_id;
        if let Err(error) = push_remote_revoke(target_base_url, &local_node_id).await {
            warn!(
                target_node_id = %existing.node_id,
                target_base_url = %target_base_url,
                error = %error,
                "remote linked-node revoke failed"
            );
        }
    }

    Ok(revoked)
}

pub async fn receive_remote_revoke(state: &AppState, node_id: &str) -> Result<(), AppError> {
    let revoked_at = OffsetDateTime::now_utc();
    let _ = state
        .storage
        .revoke_linked_node(node_id.trim(), revoked_at)
        .await?;
    Ok(())
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
    push_suggested_target(
        &mut targets,
        "Configured",
        &bootstrap.configured_base_url,
        "configured",
        bootstrap.sync_transport == "configured",
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
    pub target_base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RedeemPairingTokenInput {
    pub token_code: String,
    pub node_id: String,
    pub node_display_name: String,
    pub transport_hint: Option<String>,
    pub requested_scopes: Option<LinkScopeData>,
    pub sync_base_url: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub public_base_url: Option<String>,
}

fn scope_from_data(value: LinkScopeData) -> LinkScope {
    LinkScope {
        read_context: value.read_context,
        write_safe_actions: value.write_safe_actions,
        execute_repo_tasks: value.execute_repo_tasks,
    }
}

fn linked_node_from_request(
    node_id: &str,
    node_display_name: &str,
    scopes: LinkScope,
    now: OffsetDateTime,
    transport_hint: Option<&str>,
    sync_base_url: Option<&str>,
    tailscale_base_url: Option<&str>,
    lan_base_url: Option<&str>,
    localhost_base_url: Option<&str>,
    public_base_url: Option<&str>,
) -> LinkedNodeRecord {
    let sync_base_url = sync_base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);
    let tailscale_base_url = tailscale_base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);
    let lan_base_url = lan_base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);
    let localhost_base_url = localhost_base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);
    let public_base_url = public_base_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);

    LinkedNodeRecord {
        node_id: node_id.to_string(),
        node_display_name: node_display_name.to_string(),
        status: LinkStatus::Linked,
        scopes,
        linked_at: now,
        last_seen_at: Some(now),
        transport_hint: transport_hint
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string),
        sync_base_url: sync_base_url.clone(),
        tailscale_base_url: tailscale_base_url.clone(),
        lan_base_url: lan_base_url.clone(),
        localhost_base_url: localhost_base_url.clone(),
        public_base_url: public_base_url.clone(),
        endpoint_inventory: trusted_node_endpoint_inventory_from_urls(
            sync_base_url.as_deref(),
            tailscale_base_url.as_deref(),
            lan_base_url.as_deref(),
            localhost_base_url.as_deref(),
            public_base_url.as_deref(),
        ),
        reachability: TrustedNodeReachability::Unknown,
    }
}

fn merge_linked_node(
    existing: Option<&LinkedNodeRecord>,
    incoming: LinkedNodeRecord,
) -> LinkedNodeRecord {
    let Some(existing) = existing else {
        return incoming;
    };

    incoming.merge_trust_state_from(existing)
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

pub async fn receive_linking_prompt(
    state: &AppState,
    prompt: LinkingPromptData,
) -> Result<(), AppError> {
    let target_node_id = prompt.target_node_id.trim().to_string();
    if target_node_id.is_empty() {
        return Err(AppError::bad_request("target_node_id must not be empty"));
    }
    if prompt.expires_at <= OffsetDateTime::now_utc() {
        return Err(AppError::bad_request("linking prompt has already expired"));
    }
    if prompt.issued_by_node_id.trim().is_empty() {
        return Err(AppError::bad_request("issued_by_node_id must not be empty"));
    }
    if prompt.issued_by_node_id == target_node_id {
        return Err(AppError::bad_request(
            "linking prompt issuer must not match the target node",
        ));
    }
    if prompt_redeem_base_url(&prompt).is_none() {
        return Err(AppError::bad_request(
            "linking prompt does not advertise a redeemable address",
        ));
    }
    let local_node_id = crate::services::client_sync::effective_cluster_bootstrap(state)
        .await?
        .node_id;
    if local_node_id != target_node_id {
        return Err(AppError::bad_request(
            "linking prompt target does not match this node",
        ));
    }
    save_linking_prompt(state, &target_node_id, prompt).await
}

async fn generate_unique_pairing_token_code(state: &AppState) -> Result<String, AppError> {
    for _ in 0..8 {
        let token_code = generate_pairing_token_code();
        if state
            .storage
            .get_pairing_token_by_code(&token_code)
            .await?
            .is_none()
        {
            return Ok(token_code);
        }
    }

    Err(AppError::internal(
        "failed to allocate a unique pairing token code",
    ))
}

async fn push_linking_prompt(
    target_base_url: &str,
    prompt: &LinkingPromptData,
) -> Result<(), AppError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(1000))
        .build()
        .map_err(|error| AppError::internal(format!("build linking prompt client: {error}")))?;
    let response = client
        .post(format!(
            "{}/v1/linking/prompts",
            target_base_url.trim_end_matches('/')
        ))
        .json(prompt)
        .send()
        .await
        .map_err(|error| AppError::internal(format!("send linking prompt: {error}")))?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(AppError::internal(format!(
            "target node rejected linking prompt with status {}",
            response.status()
        )))
    }
}

async fn push_remote_revoke(target_base_url: &str, node_id: &str) -> Result<(), AppError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(1000))
        .build()
        .map_err(|error| AppError::internal(format!("build remote revoke client: {error}")))?;
    let response = client
        .post(format!(
            "{}/v1/linking/public-revoke",
            target_base_url.trim_end_matches('/')
        ))
        .json(&serde_json::json!({ "node_id": node_id }))
        .send()
        .await
        .map_err(|error| AppError::internal(format!("send remote revoke: {error}")))?;
    if response.status().is_success() {
        Ok(())
    } else {
        Err(AppError::internal(format!(
            "target node rejected remote revoke with status {}",
            response.status()
        )))
    }
}

fn generate_pairing_token_code() -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let bytes = *uuid::Uuid::new_v4().as_bytes();
    let mut token = String::with_capacity(7);
    for byte in bytes.iter().take(6) {
        if token.len() == 3 {
            token.push('-');
        }
        token.push(ALPHABET[(*byte as usize) % ALPHABET.len()] as char);
    }
    token
}

fn normalize_pairing_token_code(value: &str) -> String {
    value
        .trim()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_uppercase())
        .collect()
}

fn format_pairing_token_code(value: &str) -> String {
    let normalized = value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .take(6)
        .collect::<String>();
    if normalized.len() <= 3 {
        normalized
    } else {
        format!("{}-{}", &normalized[..3], &normalized[3..])
    }
}

fn linked_node_target_base_url(record: &LinkedNodeRecord) -> Option<&str> {
    [
        record.sync_base_url.as_deref(),
        record.tailscale_base_url.as_deref(),
        record.lan_base_url.as_deref(),
        record.public_base_url.as_deref(),
        record.localhost_base_url.as_deref(),
    ]
    .into_iter()
    .flatten()
    .map(str::trim)
    .find(|value| !value.is_empty())
}

fn prompt_redeem_base_url(prompt: &LinkingPromptData) -> Option<&str> {
    [
        Some(prompt.issuer_sync_base_url.as_str()),
        prompt.issuer_tailscale_base_url.as_deref(),
        prompt.issuer_lan_base_url.as_deref(),
        prompt.issuer_public_base_url.as_deref(),
        prompt.issuer_localhost_base_url.as_deref(),
    ]
    .into_iter()
    .flatten()
    .map(str::trim)
    .find(|value| !value.is_empty())
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

    async fn local_node_id(state: &AppState) -> String {
        crate::services::client_sync::effective_cluster_bootstrap(state)
            .await
            .unwrap()
            .node_id
    }

    #[tokio::test]
    async fn linking_service_issues_and_redeems_scoped_tokens() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);
        let issued_by_node_id = local_node_id(&state).await;

        let token = issue_pairing_token(
            &state,
            IssuePairingTokenInput {
                issued_by_node_id,
                ttl_seconds: None,
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: true,
                    execute_repo_tasks: false,
                },
                target_node_id: None,
                target_node_display_name: None,
                target_base_url: None,
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
                sync_base_url: Some("http://node-beta.tailnet.ts.net:4130".to_string()),
                tailscale_base_url: Some("http://node-beta.tailnet.ts.net:4130".to_string()),
                lan_base_url: Some("http://192.168.1.55:4130".to_string()),
                localhost_base_url: None,
                public_base_url: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(linked.node_id, "node_beta");
        assert_eq!(linked.status, LinkStatus::Linked);
        assert!(linked.scopes.read_context);
        assert!(!linked.scopes.write_safe_actions);
        assert_eq!(
            linked.sync_base_url.as_deref(),
            Some("http://node-beta.tailnet.ts.net:4130")
        );
    }

    #[tokio::test]
    async fn linking_service_fails_closed_for_out_of_scope_redeem() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);
        let issued_by_node_id = local_node_id(&state).await;

        let token = issue_pairing_token(
            &state,
            IssuePairingTokenInput {
                issued_by_node_id,
                ttl_seconds: Some(900),
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                target_node_id: None,
                target_node_display_name: None,
                target_base_url: None,
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
                sync_base_url: None,
                tailscale_base_url: None,
                lan_base_url: None,
                localhost_base_url: None,
                public_base_url: None,
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
        config.base_url = "http://vel-desktop.example:4130".to_string();
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
        assert!(targets
            .iter()
            .any(|target| target.transport_hint == "configured"
                && target.base_url == "http://vel-desktop.example:4130"));
        assert!(targets[0]
            .redeem_command_hint
            .contains("node link redeem VEL-PAIR-123"));
    }

    #[tokio::test]
    async fn linking_service_receives_prompt_for_local_node() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let mut config = AppConfig::default();
        config.node_id = Some("vel-desktop".to_string());
        let (broadcast_tx, _) = broadcast::channel(8);
        let state = AppState::new(
            storage,
            config,
            crate::policy_config::PolicyConfig::default(),
            broadcast_tx,
            None,
            None,
        );
        let local_node_id = crate::services::client_sync::effective_cluster_bootstrap(&state)
            .await
            .unwrap()
            .node_id;

        receive_linking_prompt(
            &state,
            LinkingPromptData {
                target_node_id: local_node_id.clone(),
                target_node_display_name: Some("Vel Desktop".to_string()),
                issued_by_node_id: "node_remote".to_string(),
                issued_by_node_display_name: Some("Remote Mac".to_string()),
                issued_at: OffsetDateTime::now_utc(),
                expires_at: OffsetDateTime::now_utc() + Duration::seconds(300),
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                issuer_sync_base_url: "http://remote.tailnet.ts.net:4130".to_string(),
                issuer_sync_transport: "tailscale".to_string(),
                issuer_tailscale_base_url: Some("http://remote.tailnet.ts.net:4130".to_string()),
                issuer_lan_base_url: Some("http://192.168.1.60:4130".to_string()),
                issuer_localhost_base_url: None,
                issuer_public_base_url: None,
            },
        )
        .await
        .unwrap();

        let prompts = linking_prompts(&state).await.unwrap();
        assert!(prompts.contains_key(&local_node_id));
        assert_eq!(
            prompts[&local_node_id].issuer_sync_base_url,
            "http://remote.tailnet.ts.net:4130"
        );
    }

    #[tokio::test]
    async fn linking_service_remote_revoke_revokes_local_link() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);
        let now = OffsetDateTime::now_utc();
        state
            .storage
            .upsert_linked_node(&LinkedNodeRecord {
                node_id: "node_remote".to_string(),
                node_display_name: "Remote Mac".to_string(),
                status: LinkStatus::Linked,
                scopes: LinkScope {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                linked_at: now,
                last_seen_at: Some(now),
                transport_hint: Some("tailscale".to_string()),
                sync_base_url: Some("http://remote.tailnet.ts.net:4130".to_string()),
                tailscale_base_url: Some("http://remote.tailnet.ts.net:4130".to_string()),
                lan_base_url: None,
                localhost_base_url: None,
                public_base_url: None,
                endpoint_inventory: Vec::new(),
                reachability: TrustedNodeReachability::Unknown,
            })
            .await
            .unwrap();

        receive_remote_revoke(&state, "node_remote").await.unwrap();

        let nodes = list_linked_nodes(&state).await.unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].status, LinkStatus::Revoked);
    }

    #[tokio::test]
    async fn linking_service_redeem_preserves_existing_routes_during_renegotiation() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);
        let issued_by_node_id = local_node_id(&state).await;

        let first_token = issue_pairing_token(
            &state,
            IssuePairingTokenInput {
                issued_by_node_id: issued_by_node_id.clone(),
                ttl_seconds: None,
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: true,
                    execute_repo_tasks: false,
                },
                target_node_id: None,
                target_node_display_name: None,
                target_base_url: None,
            },
        )
        .await
        .unwrap();
        redeem_pairing_token(
            &state,
            RedeemPairingTokenInput {
                token_code: first_token.token_code,
                node_id: "node_beta".to_string(),
                node_display_name: "Beta".to_string(),
                transport_hint: Some("tailscale".to_string()),
                requested_scopes: None,
                sync_base_url: Some("http://node-beta.tailnet.ts.net:4130".to_string()),
                tailscale_base_url: Some("http://node-beta.tailnet.ts.net:4130".to_string()),
                lan_base_url: Some("http://192.168.1.55:4130".to_string()),
                localhost_base_url: None,
                public_base_url: Some("https://beta.example.com".to_string()),
            },
        )
        .await
        .unwrap();

        let second_token = issue_pairing_token(
            &state,
            IssuePairingTokenInput {
                issued_by_node_id,
                ttl_seconds: None,
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                target_node_id: None,
                target_node_display_name: None,
                target_base_url: None,
            },
        )
        .await
        .unwrap();
        let linked = redeem_pairing_token(
            &state,
            RedeemPairingTokenInput {
                token_code: second_token.token_code,
                node_id: "node_beta".to_string(),
                node_display_name: "Beta".to_string(),
                transport_hint: Some("lan".to_string()),
                requested_scopes: None,
                sync_base_url: Some("http://192.168.1.55:4130".to_string()),
                tailscale_base_url: None,
                lan_base_url: None,
                localhost_base_url: None,
                public_base_url: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(linked.transport_hint.as_deref(), Some("lan"));
        assert_eq!(
            linked.sync_base_url.as_deref(),
            Some("http://192.168.1.55:4130")
        );
        assert_eq!(
            linked.tailscale_base_url.as_deref(),
            Some("http://node-beta.tailnet.ts.net:4130")
        );
        assert_eq!(
            linked.lan_base_url.as_deref(),
            Some("http://192.168.1.55:4130")
        );
        assert_eq!(
            linked.public_base_url.as_deref(),
            Some("https://beta.example.com")
        );
        assert!(!linked.scopes.write_safe_actions);
    }

    #[tokio::test]
    async fn linking_service_rejects_self_targeted_issue() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);
        let issued_by_node_id = local_node_id(&state).await;

        let error = issue_pairing_token(
            &state,
            IssuePairingTokenInput {
                issued_by_node_id: issued_by_node_id.clone(),
                ttl_seconds: None,
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                target_node_id: Some(issued_by_node_id),
                target_node_display_name: Some("Self".to_string()),
                target_base_url: Some("http://127.0.0.1:4130".to_string()),
            },
        )
        .await
        .expect_err("self-targeted issue must fail");

        assert_eq!(
            error.to_string(),
            "target_node_id must not match the issuing node"
        );
    }

    #[tokio::test]
    async fn linking_service_rejects_self_redeem_and_malformed_prompt() {
        let storage = vel_storage::Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_state(storage);
        let issued_by_node_id = local_node_id(&state).await;

        let token = issue_pairing_token(
            &state,
            IssuePairingTokenInput {
                issued_by_node_id: issued_by_node_id.clone(),
                ttl_seconds: None,
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                target_node_id: None,
                target_node_display_name: None,
                target_base_url: None,
            },
        )
        .await
        .unwrap();
        let redeem_error = redeem_pairing_token(
            &state,
            RedeemPairingTokenInput {
                token_code: token.token_code,
                node_id: issued_by_node_id.clone(),
                node_display_name: "Self".to_string(),
                transport_hint: None,
                requested_scopes: None,
                sync_base_url: None,
                tailscale_base_url: None,
                lan_base_url: None,
                localhost_base_url: None,
                public_base_url: None,
            },
        )
        .await
        .expect_err("self redeem must fail");
        assert_eq!(
            redeem_error.to_string(),
            "node_id must not match the issuing node"
        );

        let prompt_error = receive_linking_prompt(
            &state,
            LinkingPromptData {
                target_node_id: issued_by_node_id.clone(),
                target_node_display_name: Some("Self".to_string()),
                issued_by_node_id: "node_remote".to_string(),
                issued_by_node_display_name: Some("Remote".to_string()),
                issued_at: OffsetDateTime::now_utc(),
                expires_at: OffsetDateTime::now_utc() + Duration::seconds(300),
                scopes: LinkScopeData {
                    read_context: true,
                    write_safe_actions: false,
                    execute_repo_tasks: false,
                },
                issuer_sync_base_url: String::new(),
                issuer_sync_transport: "tailscale".to_string(),
                issuer_tailscale_base_url: None,
                issuer_lan_base_url: None,
                issuer_localhost_base_url: None,
                issuer_public_base_url: None,
            },
        )
        .await
        .expect_err("prompt without redeemable address must fail");
        assert_eq!(
            prompt_error.to_string(),
            "linking prompt does not advertise a redeemable address"
        );
    }
}
