use time::{Duration, OffsetDateTime};
use vel_api_types::LinkScopeData;
use vel_core::{LinkScope, LinkStatus, LinkedNodeRecord, PairingTokenRecord};

use crate::{errors::AppError, state::AppState};

const DEFAULT_TOKEN_TTL_SECONDS: i64 = 900;
const MAX_TOKEN_TTL_SECONDS: i64 = 3600;

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

    Ok(state.storage.issue_pairing_token(&record).await?)
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
}
