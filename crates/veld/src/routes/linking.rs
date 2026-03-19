use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use vel_api_types::{
    ApiResponse, LinkScopeData, LinkedNodeData, LinkingPromptData, PairingTokenData, WsEventType,
};

use crate::{broadcast::WsEnvelope, errors::AppError, routes::response, services, state::AppState};

#[derive(Debug, Deserialize)]
pub struct IssuePairingTokenRequest {
    pub issued_by_node_id: String,
    pub ttl_seconds: Option<i64>,
    // Supported scope fields: read_context, write_safe_actions, execute_repo_tasks.
    pub scopes: LinkScopeData,
    pub target_node_id: Option<String>,
    pub target_node_display_name: Option<String>,
    pub target_base_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RedeemPairingTokenRequest {
    pub token_code: String,
    pub node_id: String,
    pub node_display_name: String,
    pub transport_hint: Option<String>,
    // Supported scope fields: read_context, write_safe_actions, execute_repo_tasks.
    pub requested_scopes: Option<LinkScopeData>,
    pub sync_base_url: Option<String>,
    pub tailscale_base_url: Option<String>,
    pub lan_base_url: Option<String>,
    pub localhost_base_url: Option<String>,
    pub public_base_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PublicRevokeLinkRequest {
    pub node_id: String,
}

pub async fn issue_pairing_token(
    State(state): State<AppState>,
    Json(payload): Json<IssuePairingTokenRequest>,
) -> Result<Json<ApiResponse<PairingTokenData>>, AppError> {
    let targeted_issue = payload
        .target_node_id
        .as_deref()
        .map(str::trim)
        .is_some_and(|value| !value.is_empty());
    let token = services::linking::issue_pairing_token(
        &state,
        services::linking::IssuePairingTokenInput {
            issued_by_node_id: payload.issued_by_node_id,
            ttl_seconds: payload.ttl_seconds,
            scopes: payload.scopes,
            target_node_id: payload.target_node_id,
            target_node_display_name: payload.target_node_display_name,
            target_base_url: payload.target_base_url,
        },
    )
    .await?;
    let mut data = PairingTokenData::from(token.clone());
    if !targeted_issue {
        data.suggested_targets =
            services::linking::suggested_targets(&state, &token.token_code).await?;
    }
    let _ = state.broadcast_tx.send(WsEnvelope::new(
        WsEventType::LinkingUpdated,
        serde_json::json!({}),
    ));
    Ok(response::success(data))
}

pub async fn redeem_pairing_token(
    State(state): State<AppState>,
    Json(payload): Json<RedeemPairingTokenRequest>,
) -> Result<Json<ApiResponse<LinkedNodeData>>, AppError> {
    let linked_node = services::linking::redeem_pairing_token(
        &state,
        services::linking::RedeemPairingTokenInput {
            token_code: payload.token_code,
            node_id: payload.node_id,
            node_display_name: payload.node_display_name,
            transport_hint: payload.transport_hint,
            requested_scopes: payload.requested_scopes,
            sync_base_url: payload.sync_base_url,
            tailscale_base_url: payload.tailscale_base_url,
            lan_base_url: payload.lan_base_url,
            localhost_base_url: payload.localhost_base_url,
            public_base_url: payload.public_base_url,
        },
    )
    .await?;
    let _ = state.broadcast_tx.send(WsEnvelope::new(
        WsEventType::LinkingUpdated,
        serde_json::json!({}),
    ));
    Ok(response::success(LinkedNodeData::from(linked_node)))
}

pub async fn public_redeem_pairing_token(
    State(state): State<AppState>,
    Json(payload): Json<RedeemPairingTokenRequest>,
) -> Result<Json<ApiResponse<LinkedNodeData>>, AppError> {
    redeem_pairing_token(State(state), Json(payload)).await
}

pub async fn receive_linking_prompt(
    State(state): State<AppState>,
    Json(payload): Json<LinkingPromptData>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    services::linking::receive_linking_prompt(&state, payload).await?;
    let _ = state.broadcast_tx.send(WsEnvelope::new(
        WsEventType::LinkingUpdated,
        serde_json::json!({}),
    ));
    Ok(response::success(serde_json::json!({ "accepted": true })))
}

pub async fn receive_remote_revoke(
    State(state): State<AppState>,
    Json(payload): Json<PublicRevokeLinkRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    services::linking::receive_remote_revoke(&state, &payload.node_id).await?;
    let _ = state.broadcast_tx.send(WsEnvelope::new(
        WsEventType::LinkingUpdated,
        serde_json::json!({}),
    ));
    Ok(response::success(serde_json::json!({ "accepted": true })))
}

pub async fn linking_status(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<LinkedNodeData>>>, AppError> {
    let nodes = services::linking::list_linked_nodes(&state).await?;
    Ok(response::success(
        nodes.into_iter().map(LinkedNodeData::from).collect(),
    ))
}

pub async fn revoke_link(
    State(state): State<AppState>,
    Path(node_id): Path<String>,
) -> Result<Json<ApiResponse<LinkedNodeData>>, AppError> {
    let linked_node = services::linking::revoke_linked_node(&state, node_id.trim()).await?;
    let _ = state.broadcast_tx.send(WsEnvelope::new(
        WsEventType::LinkingUpdated,
        serde_json::json!({}),
    ));
    Ok(response::success(LinkedNodeData::from(linked_node)))
}
