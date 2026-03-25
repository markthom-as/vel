use std::path::Path;

use time::OffsetDateTime;
use vel_core::CapabilityDescriptor;

use crate::{
    errors::AppError,
    services::{
        connect_runtime::{self, LaunchConnectRuntimeRequest},
        execution_routing::{self, HandoffReviewState},
    },
    state::AppState,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchApprovedHandoffRequest {
    pub runtime_kind: String,
    pub actor_id: Option<String>,
    pub display_name: Option<String>,
    pub command: Vec<String>,
    pub working_dir: Option<String>,
    pub writable_roots: Vec<String>,
    pub capability_allowlist: Vec<CapabilityDescriptor>,
    pub lease_seconds: Option<i64>,
}

pub async fn launch_approved_handoff(
    state: &AppState,
    handoff_id: &str,
    request: LaunchApprovedHandoffRequest,
) -> Result<vel_core::ConnectInstance, AppError> {
    let handoff_id = handoff_id.trim();
    let record = execution_routing::get_execution_handoff(state, handoff_id)
        .await?
        .ok_or_else(|| AppError::not_found("execution handoff not found"))?;

    if record.review_state != HandoffReviewState::Approved {
        return Err(AppError::bad_request(
            "execution handoff must be approved before launch",
        ));
    }
    if record.launched_at.is_some() {
        return Err(AppError::bad_request(
            "execution handoff is already launched",
        ));
    }

    let actor_id = request
        .actor_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(record.handoff.handoff.to_agent.as_str())
        .to_string();
    if actor_id.trim().is_empty() {
        return Err(AppError::bad_request(
            "actor_id must not be empty after handoff resolution",
        ));
    }

    let working_dir = request
        .working_dir
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| record.handoff.repo.path.clone());
    if !Path::new(&working_dir).is_absolute() {
        return Err(AppError::bad_request(
            "working_dir must be an absolute path",
        ));
    }
    if !path_within_scope(&working_dir, &record.handoff.repo.path) {
        return Err(AppError::forbidden(
            "working_dir is outside the handoff repo root",
        ));
    }

    let writable_roots = if request.writable_roots.is_empty() {
        record.routing.write_scopes.clone()
    } else {
        request.writable_roots
    };
    if writable_roots.is_empty() {
        return Err(AppError::bad_request(
            "writable_roots must not be empty; declare write scopes in the handoff or launch payload",
        ));
    }
    for root in &writable_roots {
        if !Path::new(root).is_absolute() {
            return Err(AppError::bad_request(
                "writable roots must be absolute paths",
            ));
        }
        if !path_within_scope(root, &working_dir) {
            return Err(AppError::forbidden(format!(
                "writable root {root} escapes working_dir {working_dir}"
            )));
        }
        if !record.routing.write_scopes.is_empty()
            && !record
                .routing
                .write_scopes
                .iter()
                .any(|scope| path_within_scope(root, scope))
        {
            return Err(AppError::forbidden(format!(
                "writable root {root} is outside approved handoff write scopes"
            )));
        }
    }

    let launched = connect_runtime::launch_connect_runtime(
        state,
        LaunchConnectRuntimeRequest {
            runtime_kind: request.runtime_kind,
            actor_id,
            display_name: request.display_name,
            command: request.command,
            working_dir: Some(working_dir),
            writable_roots,
            capability_allowlist: request.capability_allowlist,
            lease_seconds: request.lease_seconds,
        },
    )
    .await?;

    let now = OffsetDateTime::now_utc();
    state
        .storage
        .update_execution_handoff_review(
            handoff_id,
            "approved",
            record.reviewed_by.as_deref(),
            record.decision_reason.as_deref(),
            record.reviewed_at,
            Some(now),
            now,
        )
        .await?
        .ok_or_else(|| AppError::not_found("execution handoff not found during launch"))?;

    Ok(launched)
}

fn path_within_scope(value: &str, scope: &str) -> bool {
    Path::new(value).starts_with(Path::new(scope))
}
