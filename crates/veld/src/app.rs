use axum::{middleware as axum_middleware, Router};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::middleware::{
    enforce_exposure_gate, ExposureGate, HttpExposurePolicy, RouteExposureClass,
};

mod route_groups;
use crate::{policy_config::PolicyConfig, state::AppState};

/// Builds the app from storage/config; used by tests. Production uses build_app_with_state.
pub fn build_app(
    storage: Storage,
    config: AppConfig,
    policy_config: PolicyConfig,
    llm_router: Option<std::sync::Arc<vel_llm::Router>>,
    chat_profile_id: Option<String>,
) -> Router {
    let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
    let state = AppState::new(
        storage,
        config,
        policy_config,
        broadcast_tx,
        llm_router,
        chat_profile_id,
    );

    build_app_with_state(state)
}

pub fn build_app_with_state(state: AppState) -> Router {
    build_app_with_exposure_policy(state, HttpExposurePolicy::from_env())
}

fn build_app_with_exposure_policy(state: AppState, exposure_policy: HttpExposurePolicy) -> Router {
    let operator_auth_gate = ExposureGate::new(
        RouteExposureClass::OperatorAuthenticated,
        exposure_policy.clone(),
    );
    let worker_auth_gate = ExposureGate::new(
        RouteExposureClass::WorkerAuthenticated,
        exposure_policy.clone(),
    );
    let future_external_gate =
        ExposureGate::new(RouteExposureClass::FutureExternal, exposure_policy);

    Router::new()
        .merge(route_groups::public_routes(state.clone()))
        .merge(route_groups::operator_authenticated_routes().layer(
            axum_middleware::from_fn_with_state(operator_auth_gate, enforce_exposure_gate),
        ))
        .merge(route_groups::worker_authenticated_routes().layer(
            axum_middleware::from_fn_with_state(worker_auth_gate, enforce_exposure_gate),
        ))
        .merge(
            route_groups::future_external_routes().layer(axum_middleware::from_fn_with_state(
                future_external_gate,
                enforce_exposure_gate,
            )),
        )
        .fallback(route_groups::deny_undefined_route)
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

pub fn build_app_with_auth(
    state: AppState,
    operator_api_token: Option<String>,
    worker_api_token: Option<String>,
    strict_auth: bool,
) -> Router {
    build_app_with_exposure_policy(
        state,
        HttpExposurePolicy {
            operator_api_token,
            worker_api_token,
            strict_auth,
        },
    )
}

#[cfg(test)]
mod tests;
