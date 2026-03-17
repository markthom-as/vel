use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use vel_config::AppConfig;
use vel_storage::Storage;

use crate::{policy_config::PolicyConfig, routes, state::AppState};

const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";
const WORKER_AUTH_HEADER: &str = "x-vel-worker-token";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RouteExposureClass {
    LocalPublic,
    OperatorAuthenticated,
    WorkerAuthenticated,
    FutureExternal,
}

#[derive(Debug, Clone, Default)]
struct HttpExposurePolicy {
    operator_api_token: Option<String>,
    worker_api_token: Option<String>,
    strict_auth: bool,
}

impl HttpExposurePolicy {
    fn from_env() -> Self {
        Self {
            operator_api_token: std::env::var("VEL_OPERATOR_API_TOKEN").ok(),
            worker_api_token: std::env::var("VEL_WORKER_API_TOKEN").ok(),
            strict_auth: env_flag_enabled("VEL_STRICT_HTTP_AUTH"),
        }
    }
}

fn env_flag_enabled(name: &str) -> bool {
    matches!(
        std::env::var(name).ok().as_deref(),
        Some("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON")
    )
}

#[derive(Debug, Clone)]
struct ExposureGate {
    class: RouteExposureClass,
    policy: HttpExposurePolicy,
}

impl ExposureGate {
    fn new(class: RouteExposureClass, policy: HttpExposurePolicy) -> Self {
        Self { class, policy }
    }
}

fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/v1/health", get(routes::health::health))
        .route(
            "/api/integrations/google-calendar/oauth/callback",
            get(routes::integrations::google_calendar_oauth_callback),
        )
}

fn operator_authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/v1/command/plan", post(routes::command_lang::plan_command))
        .route(
            "/v1/command/execute",
            post(routes::command_lang::execute_command),
        )
        .route("/v1/cluster/bootstrap", get(routes::cluster::bootstrap))
        .route("/v1/cluster/workers", get(routes::cluster::workers))
        .route("/v1/doctor", get(routes::doctor::doctor))
        .route(
            "/v1/captures",
            get(routes::captures::list_captures).post(routes::captures::create_capture),
        )
        .route("/v1/captures/:id", get(routes::captures::get_capture))
        .route(
            "/v1/journal/mood",
            post(routes::journal::create_mood_journal),
        )
        .route(
            "/v1/journal/pain",
            post(routes::journal::create_pain_journal),
        )
        .route(
            "/v1/commitments",
            get(routes::commitments::list_commitments).post(routes::commitments::create_commitment),
        )
        .route(
            "/v1/commitments/:id",
            get(routes::commitments::get_commitment).patch(routes::commitments::update_commitment),
        )
        .route(
            "/v1/commitments/:id/dependencies",
            get(routes::commitments::list_commitment_dependencies)
                .post(routes::commitments::add_commitment_dependency),
        )
        .route("/v1/risk", get(routes::risk::list_risk))
        .route("/v1/risk/:id", get(routes::risk::get_commitment_risk))
        .route("/v1/suggestions", get(routes::suggestions::list))
        .route(
            "/v1/suggestions/:id/evidence",
            get(routes::suggestions::evidence),
        )
        .route(
            "/v1/suggestions/:id",
            get(routes::suggestions::get).patch(routes::suggestions::update),
        )
        .route(
            "/v1/suggestions/:id/accept",
            post(routes::suggestions::accept),
        )
        .route(
            "/v1/suggestions/:id/reject",
            post(routes::suggestions::reject),
        )
        .route(
            "/v1/artifacts",
            get(routes::artifacts::list_artifacts).post(routes::artifacts::create_artifact),
        )
        .route(
            "/v1/artifacts/latest",
            get(routes::artifacts::get_artifact_latest),
        )
        .route("/v1/artifacts/:id", get(routes::artifacts::get_artifact))
        .route("/v1/runs", get(routes::runs::list_runs))
        .route(
            "/v1/runs/:id",
            get(routes::runs::get_run).patch(routes::runs::update_run),
        )
        .route("/v1/context/today", get(routes::context::today))
        .route("/v1/context/morning", get(routes::context::morning))
        .route("/v1/context/end-of-day", get(routes::context::end_of_day))
        .route("/v1/context/current", get(routes::context::current))
        .route("/v1/context/timeline", get(routes::context::timeline))
        .route("/v1/now", get(routes::now::get_now))
        .route("/v1/explain/nudge/:id", get(routes::explain::explain_nudge))
        .route("/v1/explain/context", get(routes::explain::explain_context))
        .route(
            "/v1/explain/commitment/:id",
            get(routes::explain::explain_commitment),
        )
        .route("/v1/explain/drift", get(routes::explain::explain_drift))
        .route(
            "/v1/threads",
            get(routes::threads::list_threads).post(routes::threads::create_thread),
        )
        .route(
            "/v1/threads/:id",
            get(routes::threads::get_thread).patch(routes::threads::update_thread),
        )
        .route(
            "/v1/threads/:id/links",
            post(routes::threads::add_thread_link),
        )
        .route("/v1/search", get(routes::search::search))
        .route(
            "/v1/signals",
            get(routes::signals::list_signals).post(routes::signals::create_signal),
        )
        .route("/v1/nudges", get(routes::nudges::list_nudges))
        .route("/v1/nudges/:id", get(routes::nudges::get_nudge))
        .route("/v1/nudges/:id/done", post(routes::nudges::nudge_done))
        .route("/v1/nudges/:id/snooze", post(routes::nudges::nudge_snooze))
        .route(
            "/v1/nudges/:id/dismiss",
            post(routes::nudges::nudge_dismiss),
        )
        .route(
            "/v1/uncertainty",
            get(routes::uncertainty::list_uncertainty),
        )
        .route(
            "/v1/uncertainty/:id",
            get(routes::uncertainty::get_uncertainty),
        )
        .route(
            "/v1/uncertainty/:id/resolve",
            post(routes::uncertainty::resolve_uncertainty),
        )
        .route("/v1/loops", get(routes::loops::list_loops))
        .route(
            "/v1/loops/:kind",
            get(routes::loops::get_loop).patch(routes::loops::update_loop),
        )
        .route("/v1/sync/calendar", post(routes::sync::sync_calendar))
        .route("/v1/sync/todoist", post(routes::sync::sync_todoist))
        .route("/v1/sync/activity", post(routes::sync::sync_activity))
        .route("/v1/sync/health", post(routes::sync::sync_health))
        .route("/v1/sync/git", post(routes::sync::sync_git))
        .route("/v1/sync/messaging", post(routes::sync::sync_messaging))
        .route("/v1/sync/reminders", post(routes::sync::sync_reminders))
        .route("/v1/sync/notes", post(routes::sync::sync_notes))
        .route("/v1/sync/transcripts", post(routes::sync::sync_transcripts))
        .route("/v1/sync/bootstrap", get(routes::sync::sync_bootstrap))
        .route("/v1/sync/cluster", get(routes::sync::sync_cluster))
        .route("/v1/evaluate", post(routes::evaluate::run_evaluate))
        .route("/api/components", get(routes::components::list_components))
        .route(
            "/api/components/:id/logs",
            get(routes::components::list_component_logs),
        )
        .route(
            "/api/components/:id/restart",
            post(routes::components::restart_component),
        )
        .route(
            "/api/integrations",
            get(routes::integrations::get_integrations),
        )
        .route(
            "/api/integrations/connections",
            get(routes::integrations::list_integration_connections),
        )
        .route(
            "/api/integrations/connections/:id",
            get(routes::integrations::get_integration_connection),
        )
        .route(
            "/api/integrations/connections/:id/events",
            get(routes::integrations::list_integration_connection_events),
        )
        .route(
            "/api/integrations/:id/logs",
            get(routes::integrations::list_integration_logs),
        )
        .route(
            "/api/integrations/:id/source",
            axum::routing::patch(routes::integrations::patch_local_integration_source),
        )
        .route(
            "/api/integrations/google-calendar",
            axum::routing::patch(routes::integrations::patch_google_calendar),
        )
        .route(
            "/api/integrations/google-calendar/disconnect",
            post(routes::integrations::disconnect_google_calendar),
        )
        .route(
            "/api/integrations/google-calendar/auth/start",
            post(routes::integrations::start_google_calendar_auth),
        )
        .route(
            "/api/integrations/todoist",
            axum::routing::patch(routes::integrations::patch_todoist),
        )
        .route(
            "/api/integrations/todoist/disconnect",
            post(routes::integrations::disconnect_todoist),
        )
        .route(
            "/v1/synthesis/week",
            post(routes::synthesis::synthesis_week),
        )
        .route(
            "/v1/synthesis/project/:slug",
            post(routes::synthesis::synthesis_project),
        )
        .route("/ws", get(routes::ws::ws_handler))
        .merge(routes::chat::chat_routes())
}

fn worker_authenticated_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/v1/cluster/branch-sync",
            post(routes::cluster::branch_sync_request),
        )
        .route(
            "/v1/cluster/validation",
            post(routes::cluster::validation_request),
        )
        .route("/v1/sync/heartbeat", post(routes::sync::sync_heartbeat))
        .route(
            "/v1/sync/work-assignments",
            get(routes::sync::list_work_assignments)
                .post(routes::sync::claim_work_assignment)
                .patch(routes::sync::update_work_assignment),
        )
        .route("/v1/sync/work-queue", get(routes::sync::list_worker_queue))
        .route(
            "/v1/sync/work-queue/claim-next",
            post(routes::sync::claim_next_worker_queue_item),
        )
        .route("/v1/sync/actions", post(routes::sync::sync_actions))
        .route(
            "/v1/sync/branch-sync",
            post(routes::sync::sync_branch_sync_request),
        )
        .route(
            "/v1/sync/validation",
            post(routes::sync::sync_validation_request),
        )
}

fn future_external_routes() -> Router<AppState> {
    Router::new()
        .route("/v1/connect", any(deny_undefined_route))
        .route("/v1/connect/*path", any(deny_undefined_route))
        .route("/v1/cluster/clients", any(deny_undefined_route))
        .route("/v1/cluster/clients/*path", any(deny_undefined_route))
}

fn extract_bearer_token(request: &Request<Body>) -> Option<&str> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn extract_header_token<'a>(request: &'a Request<Body>, header_name: &str) -> Option<&'a str> {
    request
        .headers()
        .get(header_name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn expected_token_for_class<'a>(
    class: RouteExposureClass,
    policy: &'a HttpExposurePolicy,
) -> Option<(&'static str, &'a str)> {
    match class {
        RouteExposureClass::LocalPublic => None,
        RouteExposureClass::OperatorAuthenticated => policy
            .operator_api_token
            .as_deref()
            .map(|token| (OPERATOR_AUTH_HEADER, token)),
        RouteExposureClass::WorkerAuthenticated => policy
            .worker_api_token
            .as_deref()
            .map(|token| (WORKER_AUTH_HEADER, token)),
        RouteExposureClass::FutureExternal => Some((OPERATOR_AUTH_HEADER, "")),
    }
}

fn unauthorized_response(class: RouteExposureClass) -> Response {
    match class {
        RouteExposureClass::OperatorAuthenticated => (
            StatusCode::UNAUTHORIZED,
            format!("missing or invalid auth token in {OPERATOR_AUTH_HEADER} or Authorization"),
        )
            .into_response(),
        RouteExposureClass::WorkerAuthenticated => (
            StatusCode::UNAUTHORIZED,
            format!("missing or invalid auth token in {WORKER_AUTH_HEADER} or Authorization"),
        )
            .into_response(),
        RouteExposureClass::FutureExternal => (
            StatusCode::FORBIDDEN,
            "future_external route class is disabled by default",
        )
            .into_response(),
        RouteExposureClass::LocalPublic => {
            (StatusCode::INTERNAL_SERVER_ERROR, "invalid exposure class").into_response()
        }
    }
}

async fn enforce_exposure_gate(
    axum::extract::State(gate): axum::extract::State<ExposureGate>,
    request: Request<Body>,
    next: Next,
) -> Response {
    if gate.class == RouteExposureClass::LocalPublic {
        return next.run(request).await;
    }

    if gate.class == RouteExposureClass::FutureExternal {
        return unauthorized_response(gate.class);
    }

    let Some((header_name, expected_token)) = expected_token_for_class(gate.class, &gate.policy)
    else {
        return if gate.policy.strict_auth {
            unauthorized_response(gate.class)
        } else {
            next.run(request).await
        };
    };

    let provided =
        extract_header_token(&request, header_name).or_else(|| extract_bearer_token(&request));
    match provided {
        Some(token) if token == expected_token => next.run(request).await,
        _ => unauthorized_response(gate.class),
    }
}

async fn deny_undefined_route() -> Response {
    (
        StatusCode::NOT_FOUND,
        "route is not defined for this authority runtime",
    )
        .into_response()
}

/// Builds the app from storage/config; used by tests. Production uses build_app_with_state.
#[allow(dead_code)]
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
    build_app_with_policy(state, HttpExposurePolicy::from_env())
}

fn build_app_with_policy(state: AppState, exposure_policy: HttpExposurePolicy) -> Router {
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
        .merge(public_routes())
        .merge(
            operator_authenticated_routes().layer(middleware::from_fn_with_state(
                operator_auth_gate,
                enforce_exposure_gate,
            )),
        )
        .merge(
            worker_authenticated_routes().layer(middleware::from_fn_with_state(
                worker_auth_gate,
                enforce_exposure_gate,
            )),
        )
        .merge(
            future_external_routes().layer(middleware::from_fn_with_state(
                future_external_gate,
                enforce_exposure_gate,
            )),
        )
        .fallback(deny_undefined_route)
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy_config::PolicyConfig;
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
    };
    use time::OffsetDateTime;
    use tower::util::ServiceExt;

    fn test_policy_config() -> PolicyConfig {
        PolicyConfig::default()
    }

    fn repo_root_for_tests() -> String {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .to_string_lossy()
            .to_string()
    }

    fn test_app_state(storage: Storage) -> AppState {
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        AppState::new(
            storage,
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        )
    }

    fn test_app_with_policy(storage: Storage, exposure_policy: HttpExposurePolicy) -> Router {
        build_app_with_policy(test_app_state(storage), exposure_policy)
    }

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn operator_route_requires_token_when_operator_policy_is_set() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: Some("operator-secret".to_string()),
                worker_api_token: None,
                strict_auth: false,
            },
        );

        let denied = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/doctor")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);

        let allowed = app
            .oneshot(
                Request::builder()
                    .uri("/v1/doctor")
                    .header(OPERATOR_AUTH_HEADER, "operator-secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(allowed.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn worker_route_requires_token_when_worker_policy_is_set() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: None,
                worker_api_token: Some("worker-secret".to_string()),
                strict_auth: false,
            },
        );

        let denied = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/heartbeat")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);

        let allowed = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/heartbeat")
                    .header(WORKER_AUTH_HEADER, "worker-secret")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(allowed.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn worker_claim_next_route_requires_worker_token_when_worker_policy_is_set() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: None,
                worker_api_token: Some("worker-secret".to_string()),
                strict_auth: false,
            },
        );

        let denied = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/work-queue/claim-next")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);

        let allowed = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/work-queue/claim-next")
                    .header(WORKER_AUTH_HEADER, "worker-secret")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(allowed.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn public_route_stays_accessible_when_auth_policies_are_set() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: Some("operator-secret".to_string()),
                worker_api_token: Some("worker-secret".to_string()),
                strict_auth: false,
            },
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn undefined_route_is_denied_by_default() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(storage, HttpExposurePolicy::default());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/not-a-real-route")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn strict_auth_denies_operator_route_when_token_is_unset() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: None,
                worker_api_token: None,
                strict_auth: true,
            },
        );

        let denied = app
            .oneshot(
                Request::builder()
                    .uri("/v1/doctor")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn strict_auth_denies_worker_route_when_token_is_unset() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: None,
                worker_api_token: None,
                strict_auth: true,
            },
        );

        let denied = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/heartbeat")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn future_external_routes_are_forbidden_by_default() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(storage, HttpExposurePolicy::default());

        let connect_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/connect")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(connect_response.status(), StatusCode::FORBIDDEN);

        let clients_response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/cluster/clients")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(clients_response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn future_external_routes_remain_forbidden_with_auth_tokens() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: Some("operator-secret".to_string()),
                worker_api_token: Some("worker-secret".to_string()),
                strict_auth: true,
            },
        );

        let connect_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/connect/worker")
                    .header(OPERATOR_AUTH_HEADER, "operator-secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(connect_response.status(), StatusCode::FORBIDDEN);

        let clients_response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/cluster/clients/node-1")
                    .header(header::AUTHORIZATION, "Bearer worker-secret")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(clients_response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn mounted_api_routes_require_operator_token_when_operator_policy_is_set() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: Some("operator-secret".to_string()),
                worker_api_token: None,
                strict_auth: false,
            },
        );

        for (method, uri) in [
            ("GET", "/api/components"),
            ("GET", "/api/integrations"),
            ("GET", "/api/conversations"),
            ("GET", "/api/settings"),
        ] {
            let denied = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(method)
                        .uri(uri)
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(
                denied.status(),
                StatusCode::UNAUTHORIZED,
                "{method} {uri} should require operator auth",
            );

            let allowed = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(method)
                        .uri(uri)
                        .header(OPERATOR_AUTH_HEADER, "operator-secret")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_ne!(
                allowed.status(),
                StatusCode::UNAUTHORIZED,
                "{method} {uri} should not be denied after valid operator auth",
            );
        }
    }

    #[tokio::test]
    async fn ws_route_requires_operator_token_when_operator_policy_is_set() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: Some("operator-secret".to_string()),
                worker_api_token: None,
                strict_auth: false,
            },
        );

        let denied = app
            .clone()
            .oneshot(Request::builder().uri("/ws").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(denied.status(), StatusCode::UNAUTHORIZED);

        let allowed = app
            .oneshot(
                Request::builder()
                    .uri("/ws")
                    .header(OPERATOR_AUTH_HEADER, "operator-secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(allowed.status(), StatusCode::UNAUTHORIZED);
        assert_ne!(allowed.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn oauth_callback_route_remains_local_public_with_strict_auth() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: Some("operator-secret".to_string()),
                worker_api_token: Some("worker-secret".to_string()),
                strict_auth: true,
            },
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/integrations/google-calendar/oauth/callback")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn undefined_api_and_ws_paths_remain_not_found_with_or_without_auth() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: Some("operator-secret".to_string()),
                worker_api_token: Some("worker-secret".to_string()),
                strict_auth: true,
            },
        );

        for (uri, with_auth) in [
            ("/api/not-a-real-route", false),
            ("/api/not-a-real-route", true),
            ("/ws/not-a-real-route", false),
            ("/ws/not-a-real-route", true),
        ] {
            let mut request = Request::builder().uri(uri);
            if with_auth {
                request = request.header(OPERATOR_AUTH_HEADER, "operator-secret");
            }
            let response = app
                .clone()
                .oneshot(request.body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(
                response.status(),
                StatusCode::NOT_FOUND,
                "{uri} should fail closed with 404",
            );
        }
    }

    #[tokio::test]
    async fn unsupported_methods_on_mounted_operator_routes_are_rejected() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = test_app_with_policy(
            storage,
            HttpExposurePolicy {
                operator_api_token: Some("operator-secret".to_string()),
                worker_api_token: None,
                strict_auth: true,
            },
        );

        let api_method_denied = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri("/api/components")
                    .header(OPERATOR_AUTH_HEADER, "operator-secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(api_method_denied.status(), StatusCode::METHOD_NOT_ALLOWED);

        let ws_method_denied = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/ws")
                    .header(OPERATOR_AUTH_HEADER, "operator-secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ws_method_denied.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn journal_mood_endpoint_creates_capture_and_signal() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/journal/mood")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "score": 7,
                            "label": "steady",
                            "note": "good enough",
                            "source_device": "test-cli"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let captures = storage.list_captures_recent(10, false).await.unwrap();
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].capture_type, "mood_log");
        assert_eq!(captures[0].content_text, "mood 7/10 steady - good enough");

        let mood_signals = storage
            .list_signals(Some("mood_log"), None, 10)
            .await
            .unwrap();
        assert_eq!(mood_signals.len(), 1);
        assert_eq!(mood_signals[0].payload_json["score"], 7);
        assert_eq!(mood_signals[0].payload_json["label"], "steady");
    }

    #[tokio::test]
    async fn journal_pain_endpoint_creates_capture_and_signal() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/journal/pain")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "severity": 4,
                            "location": "lower back",
                            "note": "after driving",
                            "source_device": "test-watch"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let captures = storage.list_captures_recent(10, false).await.unwrap();
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].capture_type, "pain_log");
        assert_eq!(
            captures[0].content_text,
            "pain 4/10 in lower back - after driving"
        );

        let pain_signals = storage
            .list_signals(Some("pain_log"), None, 10)
            .await
            .unwrap();
        assert_eq!(pain_signals.len(), 1);
        assert_eq!(pain_signals[0].payload_json["severity"], 4);
        assert_eq!(pain_signals[0].payload_json["location"], "lower back");
    }

    #[tokio::test]
    async fn journal_entries_appear_in_now_and_explain_context_after_evaluate() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        for (path, body) in [
            (
                "/v1/journal/mood",
                serde_json::json!({
                    "score": 6,
                    "label": "flat",
                    "note": "poor sleep"
                })
                .to_string(),
            ),
            (
                "/v1/journal/pain",
                serde_json::json!({
                    "severity": 3,
                    "location": "neck",
                    "note": "desk posture"
                })
                .to_string(),
            ),
        ] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri(path)
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
        }

        let evaluate_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(evaluate_response.status(), StatusCode::OK);

        let now_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/now")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(now_response.status(), StatusCode::OK);
        let now_body = axum::body::to_bytes(now_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let now_payload: vel_api_types::ApiResponse<vel_api_types::NowData> =
            serde_json::from_slice(&now_body).unwrap();
        let now_data = now_payload.data.unwrap();
        assert_eq!(
            now_data.sources.mood.as_ref().unwrap().summary["label"],
            "flat"
        );
        assert_eq!(
            now_data.sources.pain.as_ref().unwrap().summary["location"],
            "neck"
        );

        let explain_response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/context")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(explain_response.status(), StatusCode::OK);
        let explain_body = axum::body::to_bytes(explain_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let explain_payload: vel_api_types::ApiResponse<vel_api_types::ContextExplainData> =
            serde_json::from_slice(&explain_body).unwrap();
        let explain_data = explain_payload.data.unwrap();
        assert_eq!(
            explain_data.source_summaries.mood.as_ref().unwrap().summary["score"],
            6
        );
        assert_eq!(
            explain_data.source_summaries.pain.as_ref().unwrap().summary["severity"],
            3
        );
    }

    #[tokio::test]
    async fn doctor_endpoint_returns_ok_with_schema_version() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/doctor")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn cluster_bootstrap_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let mut config = AppConfig::default();
        config.node_id = Some("vel-desktop".to_string());
        config.node_display_name = Some("Vel Desktop".to_string());
        config.tailscale_base_url = Some("http://vel-desktop.tailnet.ts.net:4130".to_string());
        let app = build_app(storage, config, test_policy_config(), None, None);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/cluster/bootstrap")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn cluster_workers_endpoint_returns_current_node_capacity() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let mut config = AppConfig::default();
        config.node_id = Some("vel-desktop".to_string());
        config.node_display_name = Some("Vel Desktop".to_string());
        config.tailscale_base_url = Some("http://vel-desktop.tailnet.ts.net:4130".to_string());
        let app = build_app(storage, config, test_policy_config(), None, None);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/cluster/workers")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<serde_json::Value> =
            serde_json::from_slice(&body).unwrap();
        let data = payload.data.unwrap();
        let workers = data["workers"].as_array().unwrap();
        assert_eq!(workers.len(), 1);
        assert_eq!(workers[0]["node_id"], "vel-desktop");
        assert_eq!(workers[0]["sync_transport"], "tailscale");
        assert!(workers[0]["capacity"]["max_concurrency"].as_u64().unwrap() >= 1);
    }

    #[tokio::test]
    async fn sync_bootstrap_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let mut config = AppConfig::default();
        config.node_id = Some("vel-desktop".to_string());
        config.tailscale_base_url = Some("http://vel-desktop.tailnet.ts.net:4130".to_string());
        let app = build_app(storage, config, test_policy_config(), None, None);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/sync/bootstrap")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn sync_cluster_endpoint_returns_nodes_and_workers() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let mut config = AppConfig::default();
        config.node_id = Some("vel-desktop".to_string());
        config.node_display_name = Some("Vel Desktop".to_string());
        config.tailscale_base_url = Some("http://vel-desktop.tailnet.ts.net:4130".to_string());
        let app = build_app(storage, config, test_policy_config(), None, None);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/sync/cluster")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<vel_api_types::SyncClusterStateData> =
            serde_json::from_slice(&body).unwrap();
        let data = payload.data.unwrap();
        assert_eq!(data.nodes.len(), 1);
        assert_eq!(data.workers.len(), 1);
        assert_eq!(data.nodes[0].node_id, "vel-desktop");
        assert_eq!(data.workers[0].worker_id, "vel-desktop");
        assert_eq!(data.sync_transport.as_deref(), Some("tailscale"));
    }

    #[tokio::test]
    async fn sync_heartbeat_endpoint_persists_remote_worker() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let body = serde_json::json!({
            "node_id": "vel-remote",
            "node_display_name": "Vel Remote",
            "worker_id": "vel-remote",
            "worker_classes": ["validation"],
            "capabilities": ["build_test_profiles"],
            "status": "ready",
            "max_concurrency": 8,
            "current_load": 1,
            "queue_depth": 0,
            "reachability": "reachable",
            "latency_class": "low",
            "compute_class": "high",
            "power_class": "ac_or_unknown",
            "tailscale_preferred": true,
            "sync_base_url": "http://vel-remote.tailnet.ts.net:4130",
            "sync_transport": "tailscale",
            "tailscale_base_url": "http://vel-remote.tailnet.ts.net:4130",
            "preferred_tailnet_endpoint": "http://vel-remote.tailnet.ts.net:4130",
            "tailscale_reachable": true
        })
        .to_string();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/heartbeat")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let workers = storage.list_cluster_workers().await.unwrap();
        assert!(workers.iter().any(|worker| worker.node_id == "vel-remote"));
    }

    #[tokio::test]
    async fn sync_actions_endpoint_applies_nudge_snooze() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let nudge_id = storage
            .insert_nudge(vel_storage::NudgeInsert {
                nudge_type: "response_debt".to_string(),
                level: "warning".to_string(),
                state: "active".to_string(),
                related_commitment_id: None,
                message: "Follow up".to_string(),
                snoozed_until: None,
                resolved_at: None,
                signals_snapshot_json: None,
                inference_snapshot_json: None,
                metadata_json: Some(serde_json::json!({})),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let body = serde_json::json!({
            "actions": [
                {
                    "action_id": "a1",
                    "action_type": "nudge_snooze",
                    "target_id": nudge_id,
                    "minutes": 5
                }
            ]
        })
        .to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/actions")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<vel_api_types::ClientActionBatchResultData> =
            serde_json::from_slice(&body).unwrap();
        let data = payload.data.unwrap();
        assert_eq!(data.applied, 1);
        assert_eq!(data.results.len(), 1);
        assert_eq!(data.results[0].status, "applied");
    }

    #[tokio::test]
    async fn sync_branch_sync_endpoint_queues_structured_work_request() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let body = serde_json::json!({
            "repo_root": repo_root_for_tests(),
            "branch": "main",
            "remote": "origin",
            "mode": "pull",
            "requested_by": "cli"
        })
        .to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/branch-sync")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<vel_api_types::QueuedWorkRoutingData> =
            serde_json::from_slice(&body).unwrap();
        let data = payload.data.unwrap();
        assert_eq!(
            data.request_type,
            vel_api_types::QueuedWorkRoutingKindData::BranchSync
        );
        assert_eq!(data.status, "queued");
        assert_eq!(data.queued_signal_type, "client_branch_sync_requested");
        assert_eq!(data.target_worker_class, "repo_sync");
        assert_eq!(data.requested_capability, "branch_sync");
        assert_eq!(data.queued_via, "sync_route");

        let signals = storage
            .list_signals(Some("client_branch_sync_requested"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
    }

    #[tokio::test]
    async fn cluster_validation_endpoint_queues_structured_work_request() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let body = serde_json::json!({
            "repo_root": repo_root_for_tests(),
            "profile_id": "repo-verify",
            "environment": "repo",
            "requested_by": "cli"
        })
        .to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/cluster/validation")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<vel_api_types::QueuedWorkRoutingData> =
            serde_json::from_slice(&body).unwrap();
        let data = payload.data.unwrap();
        assert_eq!(
            data.request_type,
            vel_api_types::QueuedWorkRoutingKindData::Validation
        );
        assert_eq!(data.status, "queued");
        assert_eq!(data.queued_signal_type, "client_validation_requested");
        assert_eq!(data.target_worker_class, "validation");
        assert_eq!(data.requested_capability, "build_test_profiles");
        assert_eq!(data.queued_via, "cluster_route");

        let signals = storage
            .list_signals(Some("client_validation_requested"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
    }

    #[tokio::test]
    async fn validation_work_request_prefers_tailscale_remote_worker_when_available() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let heartbeat = serde_json::json!({
            "node_id": "vel-remote",
            "node_display_name": "Vel Remote",
            "worker_id": "vel-remote",
            "worker_classes": ["validation"],
            "capabilities": ["build_test_profiles"],
            "status": "ready",
            "max_concurrency": 12,
            "current_load": 0,
            "queue_depth": 0,
            "reachability": "reachable",
            "latency_class": "low",
            "compute_class": "high",
            "power_class": "ac_or_unknown",
            "tailscale_preferred": true,
            "sync_base_url": "http://vel-remote.tailnet.ts.net:4130",
            "sync_transport": "tailscale",
            "tailscale_base_url": "http://vel-remote.tailnet.ts.net:4130",
            "preferred_tailnet_endpoint": "http://vel-remote.tailnet.ts.net:4130",
            "tailscale_reachable": true
        })
        .to_string();
        let heartbeat_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/heartbeat")
                    .header("content-type", "application/json")
                    .body(Body::from(heartbeat))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(heartbeat_response.status(), StatusCode::OK);

        let body = serde_json::json!({
            "repo_root": repo_root_for_tests(),
            "profile_id": "repo-verify",
            "environment": "repo",
            "requested_by": "cli"
        })
        .to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/validation")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<vel_api_types::QueuedWorkRoutingData> =
            serde_json::from_slice(&body).unwrap();
        let data = payload.data.unwrap();
        assert_eq!(data.target_node_id, "vel-remote");
        assert_eq!(data.target_worker_class, "validation");
    }

    #[tokio::test]
    async fn work_assignment_lifecycle_claims_updates_and_lists_receipts() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let claim_body = serde_json::json!({
            "work_request_id": "wrkreq-123",
            "worker_id": "worker-1",
            "worker_class": "validation",
            "capability": "build_test_profiles"
        })
        .to_string();
        let claim_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/work-assignments")
                    .header("content-type", "application/json")
                    .body(Body::from(claim_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(claim_response.status(), StatusCode::OK);
        let claim_bytes = axum::body::to_bytes(claim_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let claim_payload: vel_api_types::ApiResponse<vel_api_types::WorkAssignmentReceiptData> =
            serde_json::from_slice(&claim_bytes).unwrap();
        let claimed = claim_payload.data.unwrap();
        assert_eq!(claimed.work_request_id, "wrkreq-123");
        assert_eq!(
            claimed.status,
            vel_api_types::WorkAssignmentStatusData::Assigned
        );

        let update_body = serde_json::json!({
            "receipt_id": claimed.receipt_id,
            "status": "completed",
            "completed_at": 200,
            "result": "ok"
        })
        .to_string();
        let update_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/v1/sync/work-assignments")
                    .header("content-type", "application/json")
                    .body(Body::from(update_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(update_response.status(), StatusCode::OK);

        let list_response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/v1/sync/work-assignments?work_request_id=wrkreq-123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_response.status(), StatusCode::OK);
        let list_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let list_payload: vel_api_types::ApiResponse<
            Vec<vel_api_types::WorkAssignmentReceiptData>,
        > = serde_json::from_slice(&list_bytes).unwrap();
        let receipts = list_payload.data.unwrap();
        assert_eq!(receipts.len(), 1);
        assert_eq!(
            receipts[0].status,
            vel_api_types::WorkAssignmentStatusData::Completed
        );
        assert_eq!(receipts[0].result.as_deref(), Some("ok"));
    }

    #[tokio::test]
    async fn duplicate_queue_request_returns_in_progress_when_receipt_is_active() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_app_state(storage.clone());

        let first = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-dup".to_string()),
        )
        .await
        .unwrap();
        let _claimed = crate::services::client_sync::claim_work_assignment(
            &state,
            crate::services::client_sync::WorkAssignmentClaimRequestData {
                work_request_id: "wrkreq-dup".to_string(),
                worker_id: "vel-node".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();

        let second = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-dup".to_string()),
        )
        .await
        .unwrap();

        assert_eq!(first.work_request_id, second.work_request_id);
        assert_eq!(second.status, "in_progress");
    }

    #[tokio::test]
    async fn duplicate_queue_request_returns_stale_reclaim_without_new_signal() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_app_state(storage.clone());

        let first = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-stale-dup".to_string()),
        )
        .await
        .unwrap();
        let claimed = crate::services::client_sync::claim_work_assignment(
            &state,
            crate::services::client_sync::WorkAssignmentClaimRequestData {
                work_request_id: "wrkreq-stale-dup".to_string(),
                worker_id: "vel-node".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        storage
            .set_work_assignment_last_updated(
                &claimed.receipt_id,
                time::OffsetDateTime::now_utc().unix_timestamp() - 600,
            )
            .await
            .unwrap();

        let second = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-stale-dup".to_string()),
        )
        .await
        .unwrap();

        assert_eq!(first.work_request_id, second.work_request_id);
        assert_eq!(second.status, "stale_reclaim");

        let signals = storage
            .list_signals(Some("client_validation_requested"), None, 10)
            .await
            .unwrap();
        let matching: Vec<_> = signals
            .into_iter()
            .filter(|signal| signal.source_ref.as_deref() == Some("wrkreq-stale-dup"))
            .collect();
        assert_eq!(matching.len(), 1);
    }

    #[tokio::test]
    async fn worker_queue_lists_pending_item_and_hides_completed_receipt() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let body = serde_json::json!({
            "repo_root": repo_root_for_tests(),
            "profile_id": "repo-verify",
            "environment": "repo",
            "requested_by": "cli"
        })
        .to_string();
        let queue_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/validation")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let queue_bytes = axum::body::to_bytes(queue_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let queue_payload: vel_api_types::ApiResponse<vel_api_types::QueuedWorkRoutingData> =
            serde_json::from_slice(&queue_bytes).unwrap();
        let routed = queue_payload.data.unwrap();

        let list_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/sync/work-queue?node_id=vel-node&worker_class=validation")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_response.status(), StatusCode::OK);
        let list_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let list_payload: vel_api_types::ApiResponse<Vec<vel_api_types::QueuedWorkItemData>> =
            serde_json::from_slice(&list_bytes).unwrap();
        let items = list_payload.data.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].work_request_id, routed.work_request_id);

        let claim_body = serde_json::json!({
            "work_request_id": routed.work_request_id,
            "worker_id": "vel-node",
            "worker_class": "validation",
            "capability": "build_test_profiles"
        })
        .to_string();
        let claim_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/work-assignments")
                    .header("content-type", "application/json")
                    .body(Body::from(claim_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let claim_bytes = axum::body::to_bytes(claim_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let claim_payload: vel_api_types::ApiResponse<vel_api_types::WorkAssignmentReceiptData> =
            serde_json::from_slice(&claim_bytes).unwrap();
        let claimed = claim_payload.data.unwrap();

        let complete_body = serde_json::json!({
            "receipt_id": claimed.receipt_id,
            "status": "completed",
            "completed_at": 300,
            "result": "ok"
        })
        .to_string();
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/v1/sync/work-assignments")
                    .header("content-type", "application/json")
                    .body(Body::from(complete_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        let empty_response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/sync/work-queue?node_id=vel-node&worker_class=validation")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let empty_bytes = axum::body::to_bytes(empty_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let empty_payload: vel_api_types::ApiResponse<Vec<vel_api_types::QueuedWorkItemData>> =
            serde_json::from_slice(&empty_bytes).unwrap();
        assert!(empty_payload.data.unwrap().is_empty());
    }

    #[tokio::test]
    async fn worker_queue_marks_stale_assigned_receipt_reclaimable() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_app_state(storage.clone());

        let routed = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-stale-queue".to_string()),
        )
        .await
        .unwrap();
        let claimed = crate::services::client_sync::claim_work_assignment(
            &state,
            crate::services::client_sync::WorkAssignmentClaimRequestData {
                work_request_id: routed.work_request_id.clone(),
                worker_id: "worker-1".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        storage
            .set_work_assignment_last_updated(
                &claimed.receipt_id,
                time::OffsetDateTime::now_utc().unix_timestamp() - 600,
            )
            .await
            .unwrap();

        let queue = crate::services::client_sync::list_worker_queue(
            &state,
            "vel-node",
            Some("validation"),
            Some("build_test_profiles"),
        )
        .await
        .unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].claim_reason.as_deref(), Some("stale_reclaim"));
        assert!(queue[0].claimable_now);

        let next = crate::services::client_sync::claim_next_work_for_worker(
            &state,
            crate::services::client_sync::WorkAssignmentClaimNextRequestData {
                node_id: "vel-node".to_string(),
                worker_id: "worker-2".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap()
        .claim
        .unwrap();
        assert_eq!(next.queue_item.work_request_id, routed.work_request_id);
        assert_eq!(
            next.queue_item.claim_reason.as_deref(),
            Some("stale_reclaim")
        );
        assert_eq!(next.receipt.worker_id, "worker-2");
    }

    #[tokio::test]
    async fn claim_next_work_picks_oldest_unclaimed_item() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_app_state(storage.clone());

        let first = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-a".to_string()),
        )
        .await
        .unwrap();
        let _second = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-b".to_string()),
        )
        .await
        .unwrap();

        let claimed = crate::services::client_sync::claim_next_work_for_worker(
            &state,
            crate::services::client_sync::WorkAssignmentClaimNextRequestData {
                node_id: "vel-node".to_string(),
                worker_id: "worker-1".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap()
        .claim
        .unwrap();
        assert_eq!(claimed.queue_item.work_request_id, first.work_request_id);
        assert_eq!(
            claimed.queue_item.claim_reason.as_deref(),
            Some("unclaimed")
        );
        assert_eq!(claimed.receipt.worker_id, "worker-1");
    }

    #[tokio::test]
    async fn claim_next_work_skips_fresh_failed_retry_until_backoff_expires() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_app_state(storage.clone());

        let routed = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-retry-fresh".to_string()),
        )
        .await
        .unwrap();
        let claimed = crate::services::client_sync::claim_work_assignment(
            &state,
            crate::services::client_sync::WorkAssignmentClaimRequestData {
                work_request_id: routed.work_request_id.clone(),
                worker_id: "worker-1".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        crate::services::client_sync::update_work_assignment_receipt(
            &state,
            crate::services::client_sync::WorkAssignmentUpdateRequest {
                receipt_id: claimed.receipt_id.clone(),
                status: crate::services::client_sync::WorkAssignmentStatusData::Failed,
                started_at: None,
                completed_at: Some(now),
                result: None,
                error_message: Some("boom".to_string()),
            },
        )
        .await
        .unwrap();

        let queue = crate::services::client_sync::list_worker_queue(
            &state,
            "vel-node",
            Some("validation"),
            Some("build_test_profiles"),
        )
        .await
        .unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].claim_reason.as_deref(), Some("retry_backoff"));
        assert!(!queue[0].claimable_now);
        assert!(queue[0].next_retry_at.unwrap() > now);

        let next = crate::services::client_sync::claim_next_work_for_worker(
            &state,
            crate::services::client_sync::WorkAssignmentClaimNextRequestData {
                node_id: "vel-node".to_string(),
                worker_id: "worker-2".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        assert!(next.claim.is_none());
    }

    #[tokio::test]
    async fn failed_work_becomes_retry_ready_after_backoff_window() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_app_state(storage.clone());

        let routed = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-retry-ready".to_string()),
        )
        .await
        .unwrap();
        let claimed = crate::services::client_sync::claim_work_assignment(
            &state,
            crate::services::client_sync::WorkAssignmentClaimRequestData {
                work_request_id: routed.work_request_id.clone(),
                worker_id: "worker-1".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        let past = time::OffsetDateTime::now_utc().unix_timestamp() - 120;
        crate::services::client_sync::update_work_assignment_receipt(
            &state,
            crate::services::client_sync::WorkAssignmentUpdateRequest {
                receipt_id: claimed.receipt_id.clone(),
                status: crate::services::client_sync::WorkAssignmentStatusData::Failed,
                started_at: None,
                completed_at: Some(past),
                result: None,
                error_message: Some("boom".to_string()),
            },
        )
        .await
        .unwrap();

        let queue = crate::services::client_sync::list_worker_queue(
            &state,
            "vel-node",
            Some("validation"),
            Some("build_test_profiles"),
        )
        .await
        .unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].claim_reason.as_deref(), Some("retry_ready"));
        assert!(queue[0].claimable_now);
        assert_eq!(queue[0].attempt_count, 1);

        let next = crate::services::client_sync::claim_next_work_for_worker(
            &state,
            crate::services::client_sync::WorkAssignmentClaimNextRequestData {
                node_id: "vel-node".to_string(),
                worker_id: "worker-2".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        let claimed_retry = next.claim.unwrap();
        assert_eq!(
            claimed_retry.queue_item.work_request_id,
            routed.work_request_id
        );
        assert_eq!(
            claimed_retry.queue_item.claim_reason.as_deref(),
            Some("retry_ready")
        );
        assert_eq!(claimed_retry.receipt.worker_id, "worker-2");
    }

    #[tokio::test]
    async fn duplicate_queue_request_after_failed_receipt_returns_retry_backoff_status() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let state = test_app_state(storage.clone());

        let routed = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-retry-status".to_string()),
        )
        .await
        .unwrap();
        let claimed = crate::services::client_sync::claim_work_assignment(
            &state,
            crate::services::client_sync::WorkAssignmentClaimRequestData {
                work_request_id: routed.work_request_id.clone(),
                worker_id: "worker-1".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        crate::services::client_sync::update_work_assignment_receipt(
            &state,
            crate::services::client_sync::WorkAssignmentUpdateRequest {
                receipt_id: claimed.receipt_id,
                status: crate::services::client_sync::WorkAssignmentStatusData::Failed,
                started_at: None,
                completed_at: Some(now),
                result: None,
                error_message: Some("boom".to_string()),
            },
        )
        .await
        .unwrap();

        let duplicate = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-retry-status".to_string()),
        )
        .await
        .unwrap();

        assert_eq!(duplicate.work_request_id, routed.work_request_id);
        assert_eq!(duplicate.status, "retry_backoff");
    }

    #[tokio::test]
    async fn queued_work_respects_policy_retry_exhaustion() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let mut policy_config = test_policy_config();
        policy_config.queued_work.validation.max_failure_attempts = 1;
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(8);
        let state = AppState::new(
            storage.clone(),
            AppConfig::default(),
            policy_config,
            broadcast_tx,
            None,
            None,
        );

        let routed = crate::services::client_sync::queue_validation_request(
            &state,
            crate::services::client_sync::ValidationRequestData {
                repo_root: repo_root_for_tests(),
                profile_id: "repo-verify".to_string(),
                branch: None,
                environment: Some("repo".to_string()),
                requested_by: Some("cli".to_string()),
            },
            "test",
            Some("wrkreq-retry-exhausted".to_string()),
        )
        .await
        .unwrap();
        let claimed = crate::services::client_sync::claim_work_assignment(
            &state,
            crate::services::client_sync::WorkAssignmentClaimRequestData {
                work_request_id: routed.work_request_id.clone(),
                worker_id: "worker-1".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        crate::services::client_sync::update_work_assignment_receipt(
            &state,
            crate::services::client_sync::WorkAssignmentUpdateRequest {
                receipt_id: claimed.receipt_id,
                status: crate::services::client_sync::WorkAssignmentStatusData::Failed,
                started_at: None,
                completed_at: Some(time::OffsetDateTime::now_utc().unix_timestamp() - 120),
                result: None,
                error_message: Some("boom".to_string()),
            },
        )
        .await
        .unwrap();

        let queue = crate::services::client_sync::list_worker_queue(
            &state,
            "vel-node",
            Some("validation"),
            Some("build_test_profiles"),
        )
        .await
        .unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].claim_reason.as_deref(), Some("retry_exhausted"));
        assert!(!queue[0].claimable_now);

        let next = crate::services::client_sync::claim_next_work_for_worker(
            &state,
            crate::services::client_sync::WorkAssignmentClaimNextRequestData {
                node_id: "vel-node".to_string(),
                worker_id: "worker-2".to_string(),
                worker_class: Some("validation".to_string()),
                capability: Some("build_test_profiles".to_string()),
            },
        )
        .await
        .unwrap();
        assert!(next.claim.is_none());
    }

    #[tokio::test]
    async fn loops_endpoint_lists_known_runtime_loops() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/loops")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<Vec<vel_api_types::LoopData>> =
            serde_json::from_slice(&body).unwrap();
        let loops = payload.data.unwrap();
        assert!(loops
            .iter()
            .any(|loop_data| loop_data.kind == "evaluate_current_state"));
        assert!(loops
            .iter()
            .any(|loop_data| loop_data.kind == "sync_calendar"));
    }

    #[tokio::test]
    async fn loop_patch_updates_enabled_state() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/v1/loops/evaluate_current_state")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"enabled":false}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<vel_api_types::LoopData> =
            serde_json::from_slice(&body).unwrap();
        let loop_data = payload.data.unwrap();
        assert_eq!(loop_data.kind, "evaluate_current_state");
        assert!(!loop_data.enabled);
    }

    #[tokio::test]
    async fn loop_get_returns_single_runtime_loop() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/loops/sync_calendar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<vel_api_types::LoopData> =
            serde_json::from_slice(&body).unwrap();
        let loop_data = payload.data.unwrap();
        assert_eq!(loop_data.kind, "sync_calendar");
        assert_eq!(loop_data.interval_seconds, 900);
    }

    #[tokio::test]
    async fn loop_patch_rejects_non_positive_interval() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/v1/loops/evaluate_current_state")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"interval_seconds":0}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn loop_patch_updates_interval_seconds() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/v1/loops/sync_calendar")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"interval_seconds":1200}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let payload: vel_api_types::ApiResponse<vel_api_types::LoopData> =
            serde_json::from_slice(&body).unwrap();
        let loop_data = payload.data.unwrap();
        assert_eq!(loop_data.kind, "sync_calendar");
        assert_eq!(loop_data.interval_seconds, 1200);
    }

    #[tokio::test]
    async fn loop_endpoint_rejects_unknown_loop_kind() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/loops/not_a_loop")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn uncertainty_endpoints_list_inspect_and_resolve_records() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let uncertainty_id = storage
            .insert_uncertainty_record(vel_storage::UncertaintyRecordInsert {
                subject_type: "suggestion_candidate".to_string(),
                subject_id: Some("increase_commute_buffer".to_string()),
                decision_kind: "suggestion_generation".to_string(),
                confidence_band: "low".to_string(),
                confidence_score: Some(0.42),
                reasons_json: serde_json::json!({
                    "summary": "Borderline commute evidence."
                }),
                missing_evidence_json: Some(serde_json::json!({
                    "more_events_needed": 1
                })),
                resolution_mode: "defer".to_string(),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let list_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/uncertainty")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list_body = axum::body::to_bytes(list_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let list_json: serde_json::Value = serde_json::from_slice(&list_body).unwrap();
        assert_eq!(
            list_json["data"].as_array().map(|items| items.len()),
            Some(1)
        );
        assert_eq!(
            list_json["data"][0]["id"].as_str(),
            Some(uncertainty_id.as_str())
        );
        assert_eq!(list_json["data"][0]["status"].as_str(), Some("open"));

        let inspect_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/uncertainty/{}", uncertainty_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(inspect_resp.status(), StatusCode::OK);
        let inspect_body = axum::body::to_bytes(inspect_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let inspect_json: serde_json::Value = serde_json::from_slice(&inspect_body).unwrap();
        assert_eq!(
            inspect_json["data"]["subject_id"].as_str(),
            Some("increase_commute_buffer")
        );
        assert_eq!(
            inspect_json["data"]["resolution_mode"].as_str(),
            Some("defer")
        );

        let resolve_resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/uncertainty/{}/resolve", uncertainty_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resolve_resp.status(), StatusCode::OK);
        let resolve_body = axum::body::to_bytes(resolve_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let resolve_json: serde_json::Value = serde_json::from_slice(&resolve_body).unwrap();
        assert_eq!(resolve_json["data"]["status"].as_str(), Some("resolved"));

        let stored = storage
            .get_uncertainty_record(&uncertainty_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(stored.status, "resolved");
        assert!(stored.resolved_at.is_some());
    }

    #[tokio::test]
    async fn command_plan_endpoint_returns_service_plan() {
        let db_path = format!("/tmp/vel_command_plan_{}.db", uuid::Uuid::new_v4().simple());
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/plan")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "execute",
                                "targets": [
                                    {
                                        "kind": "context",
                                        "attributes": {}
                                    }
                                ],
                                "arguments": [],
                                "original_text": "execute context"
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["data"]["operation"].as_str(), Some("execute"));
        assert_eq!(json["data"]["mode"].as_str(), Some("dry_run_only"));
        assert_eq!(
            json["data"]["intent_hints"]["target_kind"].as_str(),
            Some("context")
        );
        assert_eq!(
            json["data"]["intent_hints"]["mode"].as_str(),
            Some("execute")
        );
        assert!(json["data"]["planned_records"]
            .as_array()
            .is_some_and(|records| records.is_empty()));
        assert_eq!(json["data"]["validation"]["is_valid"].as_bool(), Some(true));
        assert_eq!(
            json["data"]["steps"][2]["title"].as_str(),
            Some("Dry-run summary only")
        );
    }

    #[tokio::test]
    async fn command_plan_endpoint_includes_delegation_hints() {
        let db_path = format!(
            "/tmp/vel_command_delegate_plan_{}.db",
            uuid::Uuid::new_v4().simple()
        );
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/plan")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "create",
                                "targets": [
                                    {
                                        "kind": "delegation_plan",
                                        "attributes": {
                                            "goal": "queue cleanup"
                                        }
                                    }
                                ],
                                "inferred": {},
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["delegation_hints"]["coordination"].as_str(),
            Some("review_gated")
        );
        assert_eq!(
            json["data"]["delegation_hints"]["linked_record_strategy"].as_str(),
            Some("artifact_plus_thread")
        );
        assert_eq!(
            json["data"]["planned_records"][0]["record_type"].as_str(),
            Some("artifact")
        );
        assert_eq!(
            json["data"]["planned_records"][1]["record_type"].as_str(),
            Some("thread")
        );
    }

    #[tokio::test]
    async fn command_execute_endpoint_creates_capture() {
        let db_path = format!(
            "/tmp/vel_command_execute_{}.db",
            uuid::Uuid::new_v4().simple()
        );
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/execute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "create",
                                "targets": [
                                    {
                                        "kind": "capture",
                                        "attributes": {
                                            "text": "remember this",
                                            "capture_type": "quick_note"
                                        }
                                    }
                                ],
                                "inferred": {
                                    "capture_type": "quick_note",
                                    "source_device": "vel-command"
                                },
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["result"]["result_kind"].as_str(),
            Some("capture_created")
        );
        let capture_id = json["data"]["result"]["data"]["capture_id"]
            .as_str()
            .expect("capture_id in payload");
        let stored_capture = storage
            .get_capture_by_id(capture_id)
            .await
            .unwrap()
            .expect("stored capture");
        assert_eq!(stored_capture.content_text, "remember this");
        assert_eq!(stored_capture.capture_type, "quick_note");
        assert_eq!(stored_capture.source_device.as_deref(), Some("vel-command"));
    }

    #[tokio::test]
    async fn command_execute_endpoint_creates_commitment() {
        let db_path = format!(
            "/tmp/vel_command_commitment_{}.db",
            uuid::Uuid::new_v4().simple()
        );
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/execute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "create",
                                "targets": [
                                    {
                                        "kind": "commitment",
                                        "attributes": {
                                            "text": "follow up with Dimitri",
                                            "project": "vel"
                                        }
                                    }
                                ],
                                "inferred": {},
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["result"]["result_kind"].as_str(),
            Some("commitment_created")
        );
        let commitment_id = json["data"]["result"]["data"]["id"]
            .as_str()
            .expect("commitment id in payload");
        let stored = storage
            .get_commitment_by_id(commitment_id)
            .await
            .unwrap()
            .expect("stored commitment");
        assert_eq!(stored.text, "follow up with Dimitri");
        assert_eq!(stored.project.as_deref(), Some("vel"));
    }

    #[tokio::test]
    async fn command_execute_endpoint_warns_when_planning_title_is_defaulted() {
        let db_path = format!(
            "/tmp/vel_command_spec_warning_{}.db",
            uuid::Uuid::new_v4().simple()
        );
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/execute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "create",
                                "targets": [
                                    {
                                        "kind": "spec_draft",
                                        "attributes": {}
                                    }
                                ],
                                "inferred": {
                                    "planning_status": "planned"
                                },
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["warnings"][0].as_str(),
            Some("no topic, goal, or text was provided; defaulted spec_draft title")
        );
        assert_eq!(
            json["data"]["result"]["data"]["artifact"]["title"].as_str(),
            Some("spec draft")
        );
    }

    #[tokio::test]
    async fn command_execute_endpoint_explains_drift() {
        let db_path = format!(
            "/tmp/vel_command_explain_drift_{}.db",
            uuid::Uuid::new_v4().simple()
        );
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let context_json = serde_json::json!({
            "attention_state": "scattered",
            "drift_type": "context_switching",
            "drift_severity": "medium",
            "attention_reasons": ["many competing threads"],
            "signals_used": [],
            "commitments_used": [],
        })
        .to_string();
        storage
            .set_current_context(OffsetDateTime::now_utc().unix_timestamp(), &context_json)
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/execute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "explain",
                                "targets": [
                                    {
                                        "kind": "context",
                                        "selector": {
                                            "type": "custom",
                                            "value": "drift"
                                        },
                                        "attributes": {
                                            "scope": "drift"
                                        }
                                    }
                                ],
                                "inferred": {
                                    "explain_target": "drift"
                                },
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["result"]["result_kind"].as_str(),
            Some("drift_explained")
        );
        assert_eq!(
            json["data"]["result"]["data"]["drift_type"].as_str(),
            Some("context_switching")
        );
    }

    #[tokio::test]
    async fn command_execute_endpoint_runs_weekly_synthesis() {
        let db_path = format!(
            "/tmp/vel_command_synthesize_week_{}.db",
            uuid::Uuid::new_v4().simple()
        );
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/execute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "execute",
                                "targets": [
                                    {
                                        "kind": "artifact",
                                        "selector": {
                                            "type": "custom",
                                            "value": "week"
                                        },
                                        "attributes": {
                                            "scope": "week"
                                        }
                                    }
                                ],
                                "inferred": {
                                    "synthesis_scope": "week"
                                },
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["result"]["result_kind"].as_str(),
            Some("synthesis_created")
        );
        assert!(json["data"]["result"]["data"]["run_id"]
            .as_str()
            .is_some_and(|value| value.starts_with("run_")));
        assert!(json["data"]["result"]["data"]["artifact_id"]
            .as_str()
            .is_some_and(|value| value.starts_with("art_")));
    }

    #[tokio::test]
    async fn command_execute_endpoint_creates_spec_draft_artifact() {
        let db_path = format!("/tmp/vel_command_spec_{}.db", uuid::Uuid::new_v4().simple());
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/execute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "create",
                                "targets": [
                                    {
                                        "kind": "spec_draft",
                                        "attributes": {
                                            "topic": "cluster sync"
                                        }
                                    }
                                ],
                                "inferred": {
                                    "planning_status": "planned"
                                },
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["result"]["result_kind"].as_str(),
            Some("spec_draft_created")
        );
        assert_eq!(
            json["data"]["result"]["data"]["artifact"]["artifact_type"].as_str(),
            Some("spec_draft")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["thread_type"].as_str(),
            Some("planning_spec")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["planning_kind"].as_str(),
            Some("spec")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["lifecycle_stage"].as_str(),
            Some("planned")
        );
        let thread_id = json["data"]["result"]["data"]["thread"]["id"]
            .as_str()
            .expect("thread id in payload");
        let artifact_id = json["data"]["result"]["data"]["artifact"]["artifact_id"]
            .as_str()
            .expect("artifact id in payload");
        let stored = storage.get_artifact_by_id(artifact_id).await.unwrap();
        assert!(stored.is_some());
        let stored_thread = storage.get_thread_by_id(thread_id).await.unwrap();
        assert!(stored_thread.is_some());
    }

    #[tokio::test]
    async fn command_execute_endpoint_creates_execution_plan_artifact() {
        let db_path = format!("/tmp/vel_command_plan_{}.db", uuid::Uuid::new_v4().simple());
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/execute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "create",
                                "targets": [
                                    {
                                        "kind": "execution_plan",
                                        "attributes": {
                                            "goal": "message backlog"
                                        }
                                    }
                                ],
                                "inferred": {
                                    "planning_status": "planned"
                                },
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["result"]["result_kind"].as_str(),
            Some("execution_plan_created")
        );
        assert_eq!(
            json["data"]["result"]["data"]["artifact"]["artifact_type"].as_str(),
            Some("execution_plan")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["thread_type"].as_str(),
            Some("planning_execution")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["planning_kind"].as_str(),
            Some("execution_plan")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["lifecycle_stage"].as_str(),
            Some("planned")
        );
        let thread_id = json["data"]["result"]["data"]["thread"]["id"]
            .as_str()
            .expect("thread id in payload");
        let artifact_id = json["data"]["result"]["data"]["artifact"]["artifact_id"]
            .as_str()
            .expect("artifact id in payload");
        let stored = storage.get_artifact_by_id(artifact_id).await.unwrap();
        assert!(stored.is_some());
        let stored_thread = storage.get_thread_by_id(thread_id).await.unwrap();
        assert!(stored_thread.is_some());
    }

    #[tokio::test]
    async fn command_execute_endpoint_creates_delegation_plan_artifact() {
        let db_path = format!(
            "/tmp/vel_command_delegate_{}.db",
            uuid::Uuid::new_v4().simple()
        );
        let storage = Storage::connect(&db_path).await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/command/execute")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "command": {
                                "operation": "create",
                                "targets": [
                                    {
                                        "kind": "delegation_plan",
                                        "attributes": {
                                            "goal": "queue cleanup"
                                        }
                                    }
                                ],
                                "inferred": {
                                    "planning_status": "planned"
                                },
                                "assumptions": [],
                                "resolution": {
                                    "parser": "deterministic",
                                    "model_assisted": false,
                                    "confirmation_required": false
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["result"]["result_kind"].as_str(),
            Some("delegation_plan_created")
        );
        assert_eq!(
            json["data"]["result"]["data"]["artifact"]["artifact_type"].as_str(),
            Some("delegation_plan")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["thread_type"].as_str(),
            Some("planning_delegation")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["planning_kind"].as_str(),
            Some("delegation_plan")
        );
        assert_eq!(
            json["data"]["result"]["data"]["thread"]["lifecycle_stage"].as_str(),
            Some("planned")
        );
        let thread_id = json["data"]["result"]["data"]["thread"]["id"]
            .as_str()
            .expect("thread id in payload");
        let artifact_id = json["data"]["result"]["data"]["artifact"]["artifact_id"]
            .as_str()
            .expect("artifact id in payload");
        let stored = storage.get_artifact_by_id(artifact_id).await.unwrap();
        assert!(stored.is_some());
        let stored_thread = storage.get_thread_by_id(thread_id).await.unwrap();
        assert!(stored_thread.is_some());
    }

    #[tokio::test]
    async fn resolved_suggestion_uncertainty_suppresses_immediate_recreation() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let recent = now_ts - 3600;

        for _ in 0..2 {
            storage
                .insert_nudge(vel_storage::NudgeInsert {
                    nudge_type: "commute_leave_time".to_string(),
                    level: "danger".to_string(),
                    state: "resolved".to_string(),
                    related_commitment_id: None,
                    message: "Leave now".to_string(),
                    snoozed_until: None,
                    resolved_at: Some(recent),
                    signals_snapshot_json: None,
                    inference_snapshot_json: None,
                    metadata_json: None,
                })
                .await
                .unwrap();
        }

        let created =
            crate::services::suggestions::evaluate_after_nudges(&storage, &test_policy_config())
                .await
                .unwrap();
        assert_eq!(
            created, 0,
            "borderline candidate should defer into uncertainty"
        );

        let uncertainty = storage
            .list_uncertainty_records(Some("open"), 10)
            .await
            .unwrap()
            .into_iter()
            .next()
            .expect("expected deferred uncertainty");
        assert_eq!(
            uncertainty.subject_id.as_deref(),
            Some("increase_commute_buffer")
        );

        storage
            .resolve_uncertainty_record(&uncertainty.id)
            .await
            .unwrap()
            .expect("uncertainty should resolve");

        let created_again =
            crate::services::suggestions::evaluate_after_nudges(&storage, &test_policy_config())
                .await
                .unwrap();
        assert_eq!(
            created_again, 0,
            "resolved uncertainty should suppress immediate recreation of the same deferred candidate"
        );
        assert!(storage
            .list_uncertainty_records(Some("open"), 10)
            .await
            .unwrap()
            .is_empty());
        assert!(storage
            .list_suggestions(Some("pending"), 10)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn search_endpoint_returns_ok_for_matching_capture() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_capture(vel_storage::CaptureInsert {
                content_text: "remember lidar budget".to_string(),
                capture_type: "quick_note".to_string(),
                source_device: Some("test".to_string()),
                privacy_class: vel_core::PrivacyClass::Private,
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/search?q=lidar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn today_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/today")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn context_briefs_include_recent_signal_summaries() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "external_task".to_string(),
                source: "todoist".to_string(),
                source_ref: Some("todoist:followup".to_string()),
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(serde_json::json!({
                    "text": "follow up with Dimitri about the forecast"
                })),
            })
            .await
            .unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "message_thread".to_string(),
                source: "messaging".to_string(),
                source_ref: Some("thread:budget".to_string()),
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(serde_json::json!({
                    "waiting_state": "waiting_on_me",
                    "summary": "forecast review reply"
                })),
            })
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let today_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/today")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(today_resp.status(), StatusCode::OK);
        let today_body = axum::body::to_bytes(today_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let today_json: serde_json::Value = serde_json::from_slice(&today_body).unwrap();
        let reminders = today_json["data"]["reminders"]
            .as_array()
            .expect("today reminders");
        let focus = today_json["data"]["focus_candidates"]
            .as_array()
            .expect("today focus candidates");
        assert!(reminders
            .iter()
            .any(|item| item == "todo follow up with Dimitri about the forecast"));
        assert!(focus.iter().any(|item| item == "forecast"));

        let morning_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/morning")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(morning_resp.status(), StatusCode::OK);
        let morning_body = axum::body::to_bytes(morning_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let morning_json: serde_json::Value = serde_json::from_slice(&morning_body).unwrap();
        assert_eq!(
            morning_json["data"]["suggested_focus"].as_str(),
            Some("forecast")
        );
    }

    /// Canonical runtime integration test: context generation flows through run → artifact → refs.
    /// Verifies run creation, status transitions, event sequence, artifact creation, and provenance refs.
    #[tokio::test]
    async fn context_today_creates_run_artifact_and_ref() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let today_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/today")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(today_resp.status(), StatusCode::OK);

        let runs = storage.list_runs(10, None, None).await.unwrap();
        assert_eq!(runs.len(), 1, "one run should exist");
        let run = &runs[0];
        assert_eq!(run.status, vel_core::RunStatus::Succeeded);
        assert_eq!(run.kind, vel_core::RunKind::ContextGeneration);

        let events = storage.list_run_events(run.id.as_ref()).await.unwrap();
        let event_types: Vec<String> = events.iter().map(|e| e.event_type.to_string()).collect();
        assert_eq!(
            event_types,
            [
                "run_created",
                "run_started",
                "context_generated",
                "artifact_written",
                "refs_created",
                "run_succeeded",
            ],
            "event sequence should match"
        );

        let refs_from_run = storage
            .list_refs_from("run", run.id.as_ref())
            .await
            .unwrap();
        assert_eq!(
            refs_from_run.len(),
            1,
            "run should have one ref (run → artifact)"
        );
        assert_eq!(refs_from_run[0].to_type, "artifact");

        let artifact_id = &refs_from_run[0].to_id;
        let artifact = storage.get_artifact_by_id(artifact_id).await.unwrap();
        assert!(artifact.is_some(), "artifact should exist");
        let art = artifact.unwrap();
        assert_eq!(art.storage_kind, vel_core::ArtifactStorageKind::Managed);
        assert_eq!(art.artifact_type, "context_brief");
        assert!(art.storage_uri.contains("context/today"));
        assert!(art
            .content_hash
            .as_deref()
            .map(|h| h.starts_with("sha256:"))
            .unwrap_or(false));
    }

    #[tokio::test]
    async fn context_today_failure_sets_run_failed_and_no_artifact_ref() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!("vel_test_root_{}", uuid::Uuid::new_v4().simple()));
        std::fs::File::create(&file_path).unwrap();
        let config = vel_config::AppConfig {
            artifact_root: file_path.to_string_lossy().to_string(),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let today_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/today")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(today_resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let runs = storage.list_runs(10, None, None).await.unwrap();
        assert_eq!(runs.len(), 1);
        let run = &runs[0];
        assert_eq!(run.status, vel_core::RunStatus::Failed);
        assert!(run.error_json.is_some());

        let refs_from_run = storage
            .list_refs_from("run", run.id.as_ref())
            .await
            .unwrap();
        assert!(refs_from_run.is_empty(), "no artifact ref on failure");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn update_run_retry_scheduled_persists_metadata_and_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::ContextGeneration,
                &serde_json::json!({ "context_kind": "today" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "retry_scheduled",
                            "retry_after_seconds": 30,
                            "reason": "transient_failure"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"];
        assert_eq!(data["status"], "retry_scheduled");
        assert_eq!(data["retry_reason"], "transient_failure");
        assert!(data.get("retry_scheduled_at").is_some());

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::RetryScheduled);
        assert_eq!(
            run.output_json.as_ref().and_then(|v| v.get("retry_reason")),
            Some(&serde_json::json!("transient_failure"))
        );

        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        let retry_event = events
            .iter()
            .find(|event| event.event_type == vel_core::RunEventType::RunRetryScheduled)
            .expect("retry scheduling event should be appended");
        assert_eq!(retry_event.payload_json["reason"], "transient_failure");
        assert!(retry_event.payload_json.get("retry_at").is_some());
    }

    #[tokio::test]
    async fn update_run_emits_runs_updated_websocket_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        let mut rx = broadcast_tx.subscribe();
        let state = crate::state::AppState::new(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        );
        let app = build_app_with_state(state);

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "blocked",
                            "blocked_reason": "waiting_on_dependency"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let envelope = rx
            .recv()
            .await
            .expect("websocket event should be broadcast");
        assert_eq!(envelope.event_type.to_string(), "runs:updated");
        assert_eq!(envelope.payload["id"], run_id.as_ref());
        assert_eq!(envelope.payload["kind"], "synthesis");
        assert_eq!(envelope.payload["status"], "blocked");
    }

    #[tokio::test]
    async fn update_run_blocked_persists_blocked_reason() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "blocked",
                            "blocked_reason": "waiting_on_dependency"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"];
        assert_eq!(data["status"], "blocked");
        assert_eq!(data["blocked_reason"], "waiting_on_dependency");

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::Blocked);
        assert_eq!(
            run.output_json
                .as_ref()
                .and_then(|v| v.get("blocked_reason")),
            Some(&serde_json::json!("waiting_on_dependency"))
        );
    }

    #[tokio::test]
    async fn update_run_rejects_retry_fields_for_non_retry_status() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::ContextGeneration,
                &serde_json::json!({ "context_kind": "today" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "blocked",
                            "retry_after_seconds": 30
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::Queued);
    }

    #[tokio::test]
    async fn update_run_rejects_conflicting_retry_fields() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let body = serde_json::to_string(&vel_api_types::RunUpdateRequest {
            status: "retry_scheduled".to_string(),
            retry_at: Some(time::OffsetDateTime::now_utc()),
            retry_after_seconds: Some(30),
            reason: None,
            allow_unsupported_retry: false,
            blocked_reason: None,
        })
        .unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert_eq!(
            events.len(),
            1,
            "no retry event should be appended on invalid input"
        );
    }

    #[tokio::test]
    async fn update_run_rejects_unsupported_retry_without_override() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Search,
                &serde_json::json!({ "query": "lidar" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "retry_scheduled",
                            "reason": "transient_failure"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["error"]["message"]
            .as_str()
            .expect("error message should be present")
            .contains("allow_unsupported_retry=true"));

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::Queued);
        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        assert_eq!(
            events.len(),
            1,
            "no retry event should be appended on rejection"
        );
    }

    #[tokio::test]
    async fn update_run_allows_unsupported_retry_with_override() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Search,
                &serde_json::json!({ "query": "lidar" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/v1/runs/{}", run_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "status": "retry_scheduled",
                            "reason": "operator_override",
                            "allow_unsupported_retry": true
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"];
        assert_eq!(data["status"], "retry_scheduled");
        assert_eq!(data["retry_reason"], "operator_override");
        assert_eq!(data["automatic_retry_supported"], false);
        assert_eq!(data["unsupported_retry_override"], true);
        assert_eq!(
            data["unsupported_retry_override_reason"],
            "manual operator override"
        );

        let run = storage
            .get_run_by_id(run_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(run.status, vel_core::RunStatus::RetryScheduled);
        assert_eq!(
            run.output_json
                .as_ref()
                .and_then(|v| v.get("unsupported_retry_override"))
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_eq!(
            run.output_json
                .as_ref()
                .and_then(|v| v.get("unsupported_retry_override_reason"))
                .and_then(serde_json::Value::as_str),
            Some("manual operator override")
        );
        let events = storage.list_run_events(run_id.as_ref()).await.unwrap();
        let retry_event = events
            .iter()
            .find(|event| event.event_type == vel_core::RunEventType::RunRetryScheduled)
            .expect("retry event should be appended when override is used");
        assert_eq!(retry_event.payload_json["unsupported_retry_override"], true);
        assert_eq!(
            retry_event.payload_json["unsupported_retry_override_reason"],
            "manual operator override"
        );
    }

    #[tokio::test]
    async fn get_run_includes_automatic_retry_policy() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Synthesis,
                &serde_json::json!({ "synthesis_kind": "week", "window_days": 7 }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/runs/{}", run_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"];
        assert_eq!(data["automatic_retry_supported"], true);
        assert_eq!(
            data["automatic_retry_reason"],
            "worker can re-execute the original run input"
        );
    }

    #[tokio::test]
    async fn list_runs_includes_unsupported_automatic_retry_policy() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let run_id = vel_core::RunId::new();
        storage
            .create_run(
                &run_id,
                vel_core::RunKind::Search,
                &serde_json::json!({ "query": "lidar" }),
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/runs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let run = json["data"]
            .as_array()
            .and_then(|runs| runs.iter().find(|run| run["id"] == run_id.as_ref()))
            .expect("run should be present in list response");
        assert_eq!(run["automatic_retry_supported"], false);
        assert_eq!(
            run["automatic_retry_reason"],
            "search runs do not have an automatic retry executor"
        );
    }

    #[tokio::test]
    async fn end_of_day_endpoint_returns_ok() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/end-of-day")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_artifact_returns_ok_and_get_returns_it() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let create_body = serde_json::json!({
            "artifact_type": "transcript",
            "title": "Test transcript",
            "storage_uri": "file:///var/artifacts/t.txt",
        });
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/artifacts")
                    .header("content-type", "application/json")
                    .body(Body::from(create_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_resp.status(), StatusCode::OK);

        let create_bytes = axum::body::to_bytes(create_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let create_json: serde_json::Value = serde_json::from_slice(&create_bytes).unwrap();
        let artifact_id = create_json["data"]["artifact_id"].as_str().unwrap();

        let get_resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/artifacts/{}", artifact_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get_resp.status(), StatusCode::OK);
    }

    /// Commute policy: no commute nudge when calendar event has no travel_minutes.
    #[tokio::test]
    async fn commute_nudge_does_not_fire_without_travel_minutes() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: None,
                timestamp: now_ts + 3600,
                payload_json: Some(serde_json::json!({
                    "start_time": now_ts + 3600,
                    "title": "Meeting"
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(nudges_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let commute_nudges: Vec<_> = nudges
            .iter()
            .filter(|n| n["nudge_type"].as_str() == Some("commute_leave_time"))
            .collect();
        assert!(
            commute_nudges.is_empty(),
            "commute nudge must not trigger when travel_minutes missing"
        );
    }

    /// Canonical day: commute nudge fires when calendar event has travel_minutes and we are in leave-by window.
    #[tokio::test]
    async fn commute_nudge_fires_with_travel_minutes_in_leave_window() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let event_start = now_ts + 30 * 60;
        let travel_minutes: i64 = 40;
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: None,
                timestamp: event_start,
                payload_json: Some(serde_json::json!({
                    "start_time": event_start,
                    "title": "Meeting with Dimitri",
                    "travel_minutes": travel_minutes
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(nudges_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let commute_nudges: Vec<_> = nudges
            .iter()
            .filter(|n| n["nudge_type"].as_str() == Some("commute_leave_time"))
            .collect();
        assert!(
            !commute_nudges.is_empty(),
            "commute nudge must fire when travel_minutes set and in leave-by window"
        );
    }

    #[tokio::test]
    async fn commute_nudge_escalates_in_place_when_leave_window_worsens() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "vel_nudge_escalate_{}.db",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        let path_str = path.to_string_lossy().to_string();

        let storage = Storage::connect(&path_str).await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: Some("gentle-window".to_string()),
                timestamp: now_ts + 24 * 60,
                payload_json: Some(serde_json::json!({
                    "start_time": now_ts + 24 * 60,
                    "title": "Meeting with Dimitri",
                    "travel_minutes": 10
                })),
            })
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let storage2 = Storage::connect(&path_str).await.unwrap();
        let commute = storage2
            .list_nudges(None, 10)
            .await
            .unwrap()
            .into_iter()
            .find(|nudge| nudge.nudge_type == "commute_leave_time")
            .expect("commute nudge should exist after first evaluate");
        assert_eq!(commute.level, "gentle");

        storage2
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: Some("danger-window".to_string()),
                timestamp: now_ts + 8 * 60,
                payload_json: Some(serde_json::json!({
                    "start_time": now_ts + 8 * 60,
                    "title": "Meeting moved earlier",
                    "travel_minutes": 10
                })),
            })
            .await
            .unwrap();

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let escalated = storage2
            .get_nudge(&commute.nudge_id)
            .await
            .unwrap()
            .expect("commute nudge should still exist");
        assert_eq!(escalated.level, "danger");
        assert_eq!(escalated.state, "active");
        assert_eq!(escalated.message, "You may be late unless you leave now.");
        let commute_nudges: Vec<_> = storage2
            .list_nudges(None, 20)
            .await
            .unwrap()
            .into_iter()
            .filter(|nudge| nudge.nudge_type == "commute_leave_time")
            .collect();
        assert_eq!(commute_nudges.len(), 1);
        assert_eq!(commute_nudges[0].nudge_id, commute.nudge_id);

        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn snoozed_commute_nudge_reactivates_when_leave_window_worsens() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "vel_nudge_unsnooze_escalate_{}.db",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        let path_str = path.to_string_lossy().to_string();

        let storage = Storage::connect(&path_str).await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: Some("gentle-window".to_string()),
                timestamp: now_ts + 24 * 60,
                payload_json: Some(serde_json::json!({
                    "start_time": now_ts + 24 * 60,
                    "title": "Meeting with Dimitri",
                    "travel_minutes": 10
                })),
            })
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let storage2 = Storage::connect(&path_str).await.unwrap();
        let commute = storage2
            .list_nudges(None, 10)
            .await
            .unwrap()
            .into_iter()
            .find(|nudge| nudge.nudge_type == "commute_leave_time")
            .expect("commute nudge should exist after first evaluate");
        assert_eq!(commute.level, "gentle");

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/nudges/{}/snooze", commute.nudge_id))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::json!({ "minutes": 15 }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        storage2
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: Some("danger-window".to_string()),
                timestamp: now_ts + 8 * 60,
                payload_json: Some(serde_json::json!({
                    "start_time": now_ts + 8 * 60,
                    "title": "Meeting moved earlier",
                    "travel_minutes": 10
                })),
            })
            .await
            .unwrap();

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let reactivated = storage2
            .get_nudge(&commute.nudge_id)
            .await
            .unwrap()
            .expect("commute nudge should still exist");
        assert_eq!(reactivated.state, "active");
        assert_eq!(reactivated.level, "danger");
        assert_eq!(reactivated.snoozed_until, None);
        let commute_nudges: Vec<_> = storage2
            .list_nudges(None, 20)
            .await
            .unwrap()
            .into_iter()
            .filter(|nudge| nudge.nudge_type == "commute_leave_time")
            .collect();
        assert_eq!(commute_nudges.len(), 1);
        let event_types: Vec<_> = storage2
            .list_nudge_events(&commute.nudge_id, 10)
            .await
            .unwrap()
            .into_iter()
            .map(|event| event.event_type)
            .collect();
        assert_eq!(
            event_types,
            vec![
                "nudge_created".to_string(),
                "nudge_snoozed".to_string(),
                "nudge_reactivated".to_string(),
                "nudge_escalated".to_string(),
            ]
        );

        let _ = std::fs::remove_file(&path);
    }

    /// Context explain returns signals_used and commitments_used.
    #[tokio::test]
    async fn context_explain_includes_signals_and_commitments_used() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "git_activity".to_string(),
                source: "git".to_string(),
                source_ref: Some("git:/home/jove/code/vel|main|commit|abc123".to_string()),
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(serde_json::json!({
                    "repo": "/home/jove/code/vel",
                    "branch": "main",
                    "operation": "commit",
                    "message": "hydrate explain",
                })),
            })
            .await
            .unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "note_document".to_string(),
                source: "notes".to_string(),
                source_ref: Some("notes:cap_note_context".to_string()),
                timestamp: now_ts - 30,
                payload_json: Some(serde_json::json!({
                    "path": "daily/today.md",
                    "title": "Today",
                })),
            })
            .await
            .unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "assistant_message".to_string(),
                source: "chatgpt".to_string(),
                source_ref: Some("tr_context".to_string()),
                timestamp: now_ts - 10,
                payload_json: Some(serde_json::json!({
                    "conversation_id": "conv_context",
                    "role": "assistant",
                    "source": "chatgpt",
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/context")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(
            json["data"]["signals_used"].is_array(),
            "signals_used must be present"
        );
        assert!(
            json["data"]["commitments_used"].is_array(),
            "commitments_used must be present"
        );
        assert!(
            json["data"]["reasons"].is_array(),
            "reasons must be present"
        );
        assert_eq!(
            json["data"]["source_summaries"]["git_activity"]["summary"]["branch"],
            "main"
        );
        assert_eq!(
            json["data"]["source_summaries"]["note_document"]["summary"]["path"],
            "daily/today.md"
        );
        assert_eq!(
            json["data"]["source_summaries"]["assistant_message"]["summary"]["conversation_id"],
            "conv_context"
        );
        let summaries = json["data"]["signal_summaries"]
            .as_array()
            .map(|value| value.as_slice())
            .unwrap_or_default();
        let git_summary = summaries
            .iter()
            .find(|summary| summary["signal_type"].as_str() == Some("git_activity"))
            .expect("git_activity summary must be present");
        assert_eq!(git_summary["summary"]["branch"], "main");
        assert_eq!(git_summary["summary"]["operation"], "commit");
        assert_eq!(
            json["data"]["adaptive_policy_overrides"]
                .as_array()
                .map(|items| items.len()),
            Some(0)
        );
    }

    #[tokio::test]
    async fn context_explain_includes_active_adaptive_policy_overrides() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(
                "adaptive_policy_overrides",
                &serde_json::json!({
                    "commute_buffer_minutes": 30,
                    "commute_buffer_source_suggestion_id": "sug_commute",
                    "commute_buffer_source_title": "Increase commute buffer",
                    "commute_buffer_source_accepted_at": 1710000100,
                    "default_prep_minutes": 45,
                    "default_prep_source_suggestion_id": "sug_prep",
                    "default_prep_source_title": "Increase prep window",
                    "default_prep_source_accepted_at": 1710000200
                }),
            )
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/context")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let overrides = json["data"]["adaptive_policy_overrides"]
            .as_array()
            .expect("adaptive policy overrides should be an array");
        assert_eq!(overrides.len(), 2);
        assert!(overrides.iter().any(|item| {
            item["policy_key"].as_str() == Some("commute_buffer")
                && item["value_minutes"].as_u64() == Some(30)
                && item["source_suggestion_id"].as_str() == Some("sug_commute")
        }));
        assert!(overrides.iter().any(|item| {
            item["policy_key"].as_str() == Some("default_prep")
                && item["value_minutes"].as_u64() == Some(45)
                && item["source_title"].as_str() == Some("Increase prep window")
        }));
    }

    /// Read boundary: explain endpoints must not create commitment_risk or nudge_events rows (repo-feedback 001).
    #[tokio::test]
    async fn explain_endpoints_do_not_mutate_persisted_state() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "vel_read_boundary_{}.db",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        let path_str = path.to_string_lossy().to_string();

        let storage = Storage::connect(&path_str).await.unwrap();
        storage.migrate().await.unwrap();
        let commitment_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Take meds".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("medication".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap()
            .as_ref()
            .to_string();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let storage2 = Storage::connect(&path_str).await.unwrap();
        let suggestion_id = storage2
            .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                suggestion_type: "increase_prep_window".to_string(),
                state: "pending".to_string(),
                title: Some("Increase prep window".to_string()),
                summary: Some("Prep nudges keep repeating.".to_string()),
                priority: 60,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("increase_prep_window".to_string()),
                payload_json: serde_json::json!({
                    "type": "increase_prep_window",
                    "current_minutes": 30,
                    "suggested_minutes": 45
                }),
                decision_context_json: Some(serde_json::json!({
                    "summary": "Resolved prep-window nudges repeated in the recent window."
                })),
            })
            .await
            .unwrap();
        let current_context_before = storage2.get_current_context().await.unwrap();
        let inferred_state_before = storage2.count_inferred_state().await.unwrap();
        let context_timeline_before = storage2.count_context_timeline().await.unwrap();
        let risk_before = storage2.count_commitment_risk().await.unwrap();
        let nudge_events_before = storage2.count_nudge_events().await.unwrap();
        let runs_before = storage2.list_runs(20, None, None).await.unwrap().len();
        let suggestions_before = storage2.list_suggestions(None, 20).await.unwrap().len();
        let nudge_id = storage2
            .list_nudges(None, 10)
            .await
            .unwrap()
            .into_iter()
            .next()
            .expect("evaluate should create at least one nudge")
            .nudge_id;

        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/context")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/drift")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/explain/commitment/{}", commitment_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/explain/nudge/{}", nudge_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/now")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/suggestions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/suggestions/{}", suggestion_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/timeline?limit=10")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let current_context_after = storage2.get_current_context().await.unwrap();
        let inferred_state_after = storage2.count_inferred_state().await.unwrap();
        let context_timeline_after = storage2.count_context_timeline().await.unwrap();
        let risk_after = storage2.count_commitment_risk().await.unwrap();
        let nudge_events_after = storage2.count_nudge_events().await.unwrap();
        let runs_after = storage2.list_runs(20, None, None).await.unwrap().len();
        let suggestions_after = storage2.list_suggestions(None, 20).await.unwrap().len();

        assert_eq!(
            current_context_before, current_context_after,
            "read-only explain/context routes must not mutate current_context"
        );
        assert_eq!(
            inferred_state_before, inferred_state_after,
            "read-only explain/context routes must not create inferred_state rows"
        );
        assert_eq!(
            context_timeline_before, context_timeline_after,
            "read-only explain/context routes must not create context_timeline rows"
        );
        assert_eq!(
            risk_before, risk_after,
            "read-only explain/context routes must not create commitment_risk rows"
        );
        assert_eq!(
            nudge_events_before, nudge_events_after,
            "read-only explain/context routes must not create nudge_events rows"
        );
        assert_eq!(
            runs_before, runs_after,
            "read-only operator routes must not create run rows"
        );
        assert_eq!(
            suggestions_before, suggestions_after,
            "read-only operator routes must not mutate suggestions"
        );

        let _ = std::fs::remove_file(&path);
    }

    /// Resolution order: resolved nudge never escalates; second evaluate does not re-trigger.
    #[tokio::test]
    async fn resolved_nudge_stays_resolved_after_second_evaluate() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Take meds".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("medication".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let meds_nudge = nudges
            .iter()
            .find(|n| n["nudge_type"].as_str() == Some("meds_not_logged"));
        let nudge_id = meds_nudge
            .and_then(|n| n["nudge_id"].as_str())
            .expect("meds nudge should exist");
        let done_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/nudges/{}/done", nudge_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(done_resp.status(), StatusCode::OK);
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_resp2 = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body2 = axum::body::to_bytes(nudges_resp2.into_body(), usize::MAX)
            .await
            .unwrap();
        let json2: serde_json::Value = serde_json::from_slice(&body2).unwrap();
        let resolved: Vec<_> = json2["data"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|n| n["nudge_id"].as_str() == Some(nudge_id))
            .collect();
        assert_eq!(resolved.len(), 1, "nudge should appear exactly once");
        assert_eq!(
            resolved[0]["state"].as_str(),
            Some("resolved"),
            "resolved nudge must stay resolved after second evaluate"
        );
    }

    #[tokio::test]
    async fn dismissed_nudge_is_listed_with_dismiss_history() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Take meds".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("medication".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);

        let nudges_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudge_id = json["data"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|n| n["nudge_type"].as_str() == Some("meds_not_logged"))
            .and_then(|n| n["nudge_id"].as_str())
            .expect("meds nudge should exist")
            .to_string();

        let dismiss_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/nudges/{}/dismiss", nudge_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(dismiss_resp.status(), StatusCode::OK);
        let dismiss_body = axum::body::to_bytes(dismiss_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let dismiss_json: serde_json::Value = serde_json::from_slice(&dismiss_body).unwrap();
        assert_eq!(dismiss_json["data"]["state"], "dismissed");

        let list_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list_body = axum::body::to_bytes(list_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let list_json: serde_json::Value = serde_json::from_slice(&list_body).unwrap();
        let dismissed = list_json["data"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|n| n["nudge_id"].as_str() == Some(nudge_id.as_str()))
            .expect("dismissed nudge should remain visible in operator list");
        assert_eq!(dismissed["state"], "dismissed");

        let events = storage.list_nudge_events(&nudge_id, 10).await.unwrap();
        let event_types: Vec<_> = events.into_iter().map(|event| event.event_type).collect();
        assert_eq!(event_types, vec!["nudge_created", "nudge_dismissed"]);
    }

    #[tokio::test]
    async fn expired_snoozed_meds_nudge_reactivates_on_evaluate() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "vel_nudge_reactivate_{}.db",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        let path_str = path.to_string_lossy().to_string();

        let storage = Storage::connect(&path_str).await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Take meds".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("medication".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let storage2 = Storage::connect(&path_str).await.unwrap();
        let nudge = storage2
            .list_nudges(None, 10)
            .await
            .unwrap()
            .into_iter()
            .find(|nudge| nudge.nudge_type == "meds_not_logged")
            .expect("meds nudge should exist");
        let nudge_events_before = storage2.count_nudge_events().await.unwrap();
        storage2
            .update_nudge_state(
                &nudge.nudge_id,
                "snoozed",
                Some(time::OffsetDateTime::now_utc().unix_timestamp() - 60),
                None,
            )
            .await
            .unwrap();

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let reloaded = storage2.get_nudge(&nudge.nudge_id).await.unwrap().unwrap();
        assert_eq!(reloaded.state, "active");
        assert_eq!(reloaded.snoozed_until, None);
        let nudge_events_after = storage2.count_nudge_events().await.unwrap();
        assert_eq!(nudge_events_after, nudge_events_before + 1);

        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn explain_nudge_includes_lifecycle_events() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "vel_nudge_history_{}.db",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));
        let path_str = path.to_string_lossy().to_string();

        let storage = Storage::connect(&path_str).await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Take meds".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("medication".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let storage2 = Storage::connect(&path_str).await.unwrap();
        let nudge = storage2
            .list_nudges(None, 10)
            .await
            .unwrap()
            .into_iter()
            .find(|nudge| nudge.nudge_type == "meds_not_logged")
            .expect("meds nudge should exist");
        storage2
            .update_nudge_state(
                &nudge.nudge_id,
                "snoozed",
                Some(time::OffsetDateTime::now_utc().unix_timestamp() - 60),
                None,
            )
            .await
            .unwrap();
        let _ = storage2
            .insert_nudge_event(
                &nudge.nudge_id,
                "nudge_snoozed",
                r#"{"reason":"test"}"#,
                time::OffsetDateTime::now_utc().unix_timestamp() - 60,
            )
            .await;

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let explain_resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/explain/nudge/{}", nudge.nudge_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(explain_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(explain_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let event_types: Vec<_> = json["data"]["events"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|event| event["event_type"].as_str())
            .collect();
        assert_eq!(
            event_types,
            vec!["nudge_created", "nudge_snoozed", "nudge_reactivated"]
        );

        let _ = std::fs::remove_file(&path);
    }

    // --- Canonical day fixture: Meeting with Dimitri at 11:00, prep 30 min, travel 40 min, meds/prep/commute open ---
    async fn canonical_day_fixture(
        storage: &vel_storage::Storage,
        now_ts: i64,
        event_offset_minutes: i64,
    ) -> (i64, i64, String, String, String) {
        let event_start = now_ts + event_offset_minutes * 60;
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: None,
                timestamp: event_start,
                payload_json: Some(serde_json::json!({
                    "start_time": event_start,
                    "title": "Meeting with Dimitri",
                    "location": "Salt Lake City",
                    "prep_minutes": 30,
                    "travel_minutes": 40
                })),
            })
            .await
            .unwrap();
        let meds_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Take meds".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("medication".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        let prep_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Prepare for Meeting with Dimitri".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: Some("vel".to_string()),
                commitment_kind: Some("prep".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        let commute_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Commute to Meeting with Dimitri".to_string(),
                source_type: "test".to_string(),
                source_id: None,
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("commute".to_string()),
                metadata_json: None,
            })
            .await
            .unwrap();
        (
            event_start,
            now_ts,
            meds_id.as_ref().to_string(),
            prep_id.as_ref().to_string(),
            commute_id.as_ref().to_string(),
        )
    }

    /// §6.1 Context assertions: prep/commute window, meds status, next commitment present.
    #[tokio::test]
    async fn canonical_day_context_assertions() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = &json["data"]["context"];
        assert!(
            data.get("prep_window_active").is_some(),
            "prep_window_active must be present"
        );
        assert!(
            data.get("commute_window_active").is_some(),
            "commute_window_active must be present"
        );
        assert!(
            data.get("meds_status").is_some(),
            "meds_status must be present"
        );
        assert!(data.get("mode").is_some(), "mode must be present");
    }

    /// §6.2 Risk assertions: risk list non-empty, factors present.
    #[tokio::test]
    async fn canonical_day_risk_assertions() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/risk")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let list = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        assert!(
            !list.is_empty(),
            "risk list should be non-empty when commitments exist"
        );
    }

    /// §6.3 Nudge: snooze suppresses repeated firing until snoozed_until.
    #[tokio::test]
    async fn canonical_day_nudge_snooze_suppresses() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let nudges = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let commute = nudges
            .iter()
            .find(|n| n["nudge_type"].as_str() == Some("commute_leave_time"));
        let nudge_id = commute
            .and_then(|n| n["nudge_id"].as_str())
            .expect("commute nudge should exist");
        let _snooze_until = now_ts + 15 * 60;
        let snooze_body = serde_json::json!({ "minutes": 15 }).to_string();
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/nudges/{}/snooze", nudge_id))
                    .header("content-type", "application/json")
                    .body(Body::from(snooze_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_resp2 = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body2 = axum::body::to_bytes(nudges_resp2.into_body(), usize::MAX)
            .await
            .unwrap();
        let json2: serde_json::Value = serde_json::from_slice(&body2).unwrap();
        let same_nudge: Vec<_> = json2["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default()
            .iter()
            .filter(|n| n["nudge_id"].as_str() == Some(nudge_id))
            .collect();
        assert_eq!(
            same_nudge.len(),
            1,
            "snoozed nudge should still appear once"
        );
        assert_eq!(
            same_nudge[0]["state"].as_str(),
            Some("snoozed"),
            "nudge should be snoozed"
        );
    }

    /// §6.3 Nudge: event start suppresses or resolves stale commute nudge.
    #[tokio::test]
    async fn canonical_day_event_start_suppresses_commute_nudge() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let event_start = now_ts - 60;
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: None,
                timestamp: event_start,
                payload_json: Some(serde_json::json!({
                    "start_time": event_start,
                    "title": "Meeting with Dimitri",
                    "travel_minutes": 40
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let active_commute: Vec<_> = json["data"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|n| {
                n["nudge_type"].as_str() == Some("commute_leave_time")
                    && n["state"].as_str() == Some("active")
            })
            .collect();
        assert!(
            active_commute.is_empty(),
            "commute nudge should be resolved or absent after event start passed"
        );
    }

    /// §6.5 Explain: context explain references commitment ids and signal ids.
    #[tokio::test]
    async fn canonical_day_explain_references_commitments_and_signals() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let (_, _, meds_id, prep_id, _) = canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/explain/context")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let commitments_used = json["data"]["commitments_used"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let signals_used = json["data"]["signals_used"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let signal_summaries = json["data"]["signal_summaries"]
            .as_array()
            .map(|value| value.as_slice())
            .unwrap_or_default();
        assert!(
            !signals_used.is_empty(),
            "signals_used must reference calendar signal"
        );
        let calendar_summary = signal_summaries
            .iter()
            .find(|summary| summary["signal_type"].as_str() == Some("calendar_event"))
            .expect("calendar_event summary must be present");
        assert!(
            calendar_summary["summary"]["title"].is_string()
                || calendar_summary["summary"]["travel_minutes"].is_number()
        );
        let commitment_ids: Vec<&str> =
            commitments_used.iter().filter_map(|c| c.as_str()).collect();
        assert!(
            commitment_ids.contains(&meds_id.as_str())
                || commitment_ids.contains(&prep_id.as_str()),
            "commitments_used should include fixture commitments"
        );
    }

    /// §6.6 Synthesis: project synthesis artifact created with open commitments.
    #[tokio::test]
    async fn canonical_day_project_synthesis_artifact() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/synthesis/project/vel")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(
            json["data"].get("artifact_id").is_some() || json["data"].get("run_id").is_some(),
            "project synthesis should return artifact or run"
        );
    }

    /// Variant A (success path): meds done reduces active nudges.
    #[tokio::test]
    async fn canonical_day_variant_a_success_meds_done() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let (_, _, meds_id, _, _) = canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("/v1/commitments/{}", meds_id))
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"status":"done"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let meds_nudges: Vec<_> = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default()
            .iter()
            .filter(|n| {
                n["nudge_type"].as_str() == Some("meds_not_logged")
                    && (n["state"].as_str() == Some("active")
                        || n["state"].as_str() == Some("snoozed"))
            })
            .collect();
        assert!(
            meds_nudges.is_empty(),
            "meds nudge should be gone after commitment done"
        );
    }

    /// Variant B (drift path): in danger window, drift and commute nudge present.
    #[tokio::test]
    async fn canonical_day_variant_b_drift_commute_danger() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        canonical_day_fixture(&storage, now_ts, 35).await;
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let context_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(context_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let ctx: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let current_context = &ctx["data"]["context"];
        let drift_type = current_context.get("drift_type");
        let nudges_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nbody = axum::body::to_bytes(nudges_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let njson: serde_json::Value = serde_json::from_slice(&nbody).unwrap();
        let commute: Vec<_> = njson["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default()
            .iter()
            .filter(|n| n["nudge_type"].as_str() == Some("commute_leave_time"))
            .collect();
        assert!(
            !commute.is_empty(),
            "commute nudge should exist in danger window (variant B)"
        );
        assert!(
            drift_type.is_some() || current_context.get("attention_state").is_some(),
            "drift or attention state should be present"
        );
    }

    /// Variant C (suggestion path): repeated commute danger triggers increase_commute_buffer suggestion.
    #[tokio::test]
    async fn canonical_day_variant_c_suggestion_from_repeated_evidence() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let window_start = now_ts - 7 * 86400;
        for _ in 0..3 {
            let _ = storage
                .insert_nudge(vel_storage::NudgeInsert {
                    nudge_type: "commute_leave_time".to_string(),
                    level: "danger".to_string(),
                    state: "resolved".to_string(),
                    related_commitment_id: None,
                    message: "You may be late.".to_string(),
                    snoozed_until: None,
                    resolved_at: Some(window_start + 86400),
                    signals_snapshot_json: None,
                    inference_snapshot_json: None,
                    metadata_json: None,
                })
                .await
                .unwrap();
        }
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/suggestions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let suggestions = json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default();
        let commute_buf: Vec<_> = suggestions
            .iter()
            .filter(|s| s["suggestion_type"].as_str() == Some("increase_commute_buffer"))
            .collect();
        assert!(
            !commute_buf.is_empty(),
            "increase_commute_buffer suggestion should appear after repeated commute danger (variant C)"
        );
        assert_eq!(
            commute_buf[0]["title"].as_str(),
            Some("Increase commute buffer")
        );
        assert!(commute_buf[0]["summary"].as_str().is_some());
        assert!(
            commute_buf[0]["priority"]
                .as_i64()
                .is_some_and(|priority| priority >= 55),
            "ranked suggestion priority should reflect a nontrivial base priority"
        );
        assert_eq!(commute_buf[0]["confidence"].as_str(), Some("medium"));
        assert_eq!(commute_buf[0]["evidence_count"].as_u64(), Some(3));
        assert!(commute_buf[0]["decision_context_summary"]
            .as_str()
            .is_some());

        let suggestion_id = commute_buf[0]["id"]
            .as_str()
            .expect("suggestion id should be present");
        let inspect_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/suggestions/{}", suggestion_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(inspect_resp.status(), StatusCode::OK);
        let inspect_body = axum::body::to_bytes(inspect_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let inspect_json: serde_json::Value = serde_json::from_slice(&inspect_body).unwrap();
        assert_eq!(
            inspect_json["data"]["decision_context_summary"].as_str(),
            commute_buf[0]["decision_context_summary"].as_str()
        );
        assert_eq!(inspect_json["data"]["evidence_count"].as_u64(), Some(3));
        assert_eq!(
            inspect_json["data"]["decision_context"]["trigger"].as_str(),
            Some("resolved_commute_danger")
        );
        let evidence = inspect_json["data"]["evidence"]
            .as_array()
            .expect("inspect response should include suggestion evidence");
        assert_eq!(evidence.len(), 3);
        assert!(
            evidence
                .iter()
                .all(|item| item["evidence_type"].as_str() == Some("nudge")),
            "suggestion evidence should point back to the nudges that triggered it"
        );

        let evidence_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/suggestions/{}/evidence", suggestion_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(evidence_resp.status(), StatusCode::OK);
        let evidence_body = axum::body::to_bytes(evidence_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let evidence_json: serde_json::Value = serde_json::from_slice(&evidence_body).unwrap();
        assert_eq!(
            evidence_json["data"].as_array().map(|items| items.len()),
            Some(3)
        );

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let dedupe_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/suggestions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(dedupe_resp.status(), StatusCode::OK);
        let dedupe_body = axum::body::to_bytes(dedupe_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let dedupe_json: serde_json::Value = serde_json::from_slice(&dedupe_body).unwrap();
        let deduped_commute: Vec<_> = dedupe_json["data"]
            .as_array()
            .map(|v| v.as_slice())
            .unwrap_or_default()
            .iter()
            .filter(|s| s["suggestion_type"].as_str() == Some("increase_commute_buffer"))
            .collect();
        assert_eq!(
            deduped_commute.len(),
            1,
            "pending commute-buffer suggestion should not duplicate on a second evaluate"
        );
    }

    #[tokio::test]
    async fn accepting_commute_buffer_suggestion_updates_overrides_and_recomputes_context() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let event_start = now_ts + 40 * 60;
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "test".to_string(),
                source_ref: Some("adaptive-commute".to_string()),
                timestamp: event_start,
                payload_json: Some(serde_json::json!({
                    "start_time": event_start,
                    "title": "Meeting with Dimitri",
                    "travel_minutes": 20
                })),
            })
            .await
            .unwrap();
        let suggestion_id = storage
            .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                suggestion_type: "increase_commute_buffer".to_string(),
                state: "pending".to_string(),
                title: Some("Increase commute buffer".to_string()),
                summary: Some("Leave earlier for similar meetings.".to_string()),
                priority: 55,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("increase_commute_buffer".to_string()),
                payload_json: serde_json::json!({
                    "type": "increase_commute_buffer",
                    "current_minutes": 20,
                    "suggested_minutes": 30
                }),
                decision_context_json: Some(serde_json::json!({
                    "summary": "Repeated commute danger nudges."
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_before = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_before_body = axum::body::to_bytes(nudges_before.into_body(), usize::MAX)
            .await
            .unwrap();
        let nudges_before_json: serde_json::Value =
            serde_json::from_slice(&nudges_before_body).unwrap();
        assert!(
            nudges_before_json["data"]
                .as_array()
                .map(|items| items
                    .iter()
                    .all(|item| { item["nudge_type"].as_str() != Some("commute_leave_time") }))
                .unwrap_or(true),
            "baseline evaluation should not create a commute nudge before the adaptive override"
        );

        let accept_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/suggestions/{}/accept", suggestion_id))
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(accept_resp.status(), StatusCode::OK);
        let accept_body = axum::body::to_bytes(accept_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let accept_json: serde_json::Value = serde_json::from_slice(&accept_body).unwrap();
        assert_eq!(
            accept_json["data"]["latest_feedback_outcome"].as_str(),
            Some("accepted_and_policy_changed")
        );
        assert_eq!(
            accept_json["data"]["adaptive_policy"]["policy_key"].as_str(),
            Some("commute_buffer")
        );
        assert_eq!(
            accept_json["data"]["adaptive_policy"]["suggested_minutes"].as_u64(),
            Some(30)
        );
        assert_eq!(
            accept_json["data"]["adaptive_policy"]["is_active_source"].as_bool(),
            Some(true)
        );
        assert_eq!(
            accept_json["data"]["adaptive_policy"]["active_override"]["source_suggestion_id"]
                .as_str(),
            Some(suggestion_id.as_ref())
        );

        let overrides = storage
            .get_all_settings()
            .await
            .unwrap()
            .remove("adaptive_policy_overrides")
            .expect("adaptive overrides should be stored after accepting suggestion");
        assert_eq!(overrides["commute_buffer_minutes"].as_u64(), Some(30));
        assert_eq!(
            overrides["commute_buffer_source_suggestion_id"].as_str(),
            Some(suggestion_id.as_ref())
        );
        assert_eq!(
            overrides["commute_buffer_source_title"].as_str(),
            Some("Increase commute buffer")
        );
        let feedback = storage
            .list_suggestion_feedback(suggestion_id.as_ref())
            .await
            .unwrap();
        assert_eq!(feedback.len(), 1);
        assert_eq!(feedback[0].outcome_type, "accepted_and_policy_changed");

        let inspect_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/suggestions/{}", suggestion_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(inspect_resp.status(), StatusCode::OK);
        let inspect_body = axum::body::to_bytes(inspect_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let inspect_json: serde_json::Value = serde_json::from_slice(&inspect_body).unwrap();
        assert_eq!(
            inspect_json["data"]["adaptive_policy"]["active_override"]["value_minutes"].as_u64(),
            Some(30)
        );
        assert_eq!(
            inspect_json["data"]["adaptive_policy"]["active_override"]["source_title"].as_str(),
            Some("Increase commute buffer")
        );

        let nudges_after = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/nudges")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let nudges_after_body = axum::body::to_bytes(nudges_after.into_body(), usize::MAX)
            .await
            .unwrap();
        let nudges_after_json: serde_json::Value =
            serde_json::from_slice(&nudges_after_body).unwrap();
        assert!(
            nudges_after_json["data"]
                .as_array()
                .map(|items| items
                    .iter()
                    .any(|item| { item["nudge_type"].as_str() == Some("commute_leave_time") }))
                .unwrap_or(false),
            "accepting the commute-buffer suggestion should trigger a reevaluation with an earlier leave-by window"
        );

        let current_context = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("current context should exist after reevaluation");
        let context_json: serde_json::Value = serde_json::from_str(&current_context.1).unwrap();
        assert_eq!(
            context_json["leave_by_ts"].as_i64(),
            Some(event_start - 30 * 60)
        );
    }

    #[tokio::test]
    async fn accepting_non_policy_suggestion_reports_no_effect() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let suggestion_id = storage
            .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                suggestion_type: "add_start_routine".to_string(),
                state: "pending".to_string(),
                title: Some("Add start routine".to_string()),
                summary: Some("Morning drift suggests a stronger startup block.".to_string()),
                priority: 35,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("add_start_routine".to_string()),
                payload_json: serde_json::json!({
                    "type": "add_start_routine",
                    "suggested_block_minutes": 20
                }),
                decision_context_json: Some(serde_json::json!({
                    "summary": "Observed repeated morning drift."
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let accept_resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/suggestions/{}/accept", suggestion_id))
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(accept_resp.status(), StatusCode::OK);
        let accept_body = axum::body::to_bytes(accept_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let accept_json: serde_json::Value = serde_json::from_slice(&accept_body).unwrap();
        assert_eq!(
            accept_json["data"]["latest_feedback_outcome"].as_str(),
            Some("accepted_no_effect")
        );

        let feedback = storage
            .list_suggestion_feedback(suggestion_id.as_ref())
            .await
            .unwrap();
        assert_eq!(feedback.len(), 1);
        assert_eq!(feedback[0].outcome_type, "accepted_no_effect");
        assert!(storage
            .get_all_settings()
            .await
            .unwrap()
            .get("adaptive_policy_overrides")
            .is_none());
    }

    #[tokio::test]
    async fn rejecting_suggestion_records_reason_in_payload() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let suggestion_id = storage
            .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                suggestion_type: "add_start_routine".to_string(),
                state: "pending".to_string(),
                title: Some("Add start routine".to_string()),
                summary: Some("Morning drift suggests a stronger startup block.".to_string()),
                priority: 35,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("add_start_routine".to_string()),
                payload_json: serde_json::json!({
                    "type": "add_start_routine",
                    "suggested_block_minutes": 20
                }),
                decision_context_json: Some(serde_json::json!({
                    "summary": "Observed repeated morning drift."
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let reject_resp = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/suggestions/{}/reject", suggestion_id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"reason":"not useful right now"}"#.to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(reject_resp.status(), StatusCode::OK);

        let stored = storage
            .get_suggestion_by_id(suggestion_id.as_ref())
            .await
            .unwrap()
            .expect("suggestion should still exist after rejection");
        assert_eq!(stored.state, "rejected");
        assert_eq!(
            stored.payload_json["rejection_reason"].as_str(),
            Some("not useful right now")
        );
        assert!(stored.resolved_at.is_some());
        let feedback = storage
            .list_suggestion_feedback(suggestion_id.as_ref())
            .await
            .unwrap();
        assert_eq!(feedback.len(), 1);
        assert_eq!(feedback[0].outcome_type, "rejected_not_useful");
        assert_eq!(feedback[0].notes.as_deref(), Some("not useful right now"));
    }

    #[tokio::test]
    async fn feedback_history_adjusts_future_suggestion_scoring() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let recent = now_ts - 3600;

        for _ in 0..2 {
            let prior_commute = storage
                .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                    suggestion_type: "increase_commute_buffer".to_string(),
                    state: "rejected".to_string(),
                    title: Some("Increase commute buffer".to_string()),
                    summary: None,
                    priority: 70,
                    confidence: Some("medium".to_string()),
                    dedupe_key: None,
                    payload_json: serde_json::json!({
                        "type": "increase_commute_buffer",
                        "suggested_minutes": 30
                    }),
                    decision_context_json: None,
                })
                .await
                .unwrap();
            storage
                .insert_suggestion_feedback(vel_storage::SuggestionFeedbackInsert {
                    suggestion_id: prior_commute,
                    outcome_type: "rejected_incorrect".to_string(),
                    notes: Some("travel estimate was wrong".to_string()),
                    observed_at: recent,
                    payload_json: None,
                })
                .await
                .unwrap();
        }

        for _ in 0..2 {
            let prior_followup = storage
                .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                    suggestion_type: "add_followup_block".to_string(),
                    state: "accepted".to_string(),
                    title: Some("Add follow-up block".to_string()),
                    summary: None,
                    priority: 50,
                    confidence: Some("medium".to_string()),
                    dedupe_key: None,
                    payload_json: serde_json::json!({
                        "type": "add_followup_block",
                        "suggested_block_minutes": 20
                    }),
                    decision_context_json: None,
                })
                .await
                .unwrap();
            storage
                .insert_suggestion_feedback(vel_storage::SuggestionFeedbackInsert {
                    suggestion_id: prior_followup,
                    outcome_type: "accepted_and_policy_changed".to_string(),
                    notes: Some("helpful".to_string()),
                    observed_at: recent,
                    payload_json: None,
                })
                .await
                .unwrap();
        }

        for (nudge_type, level) in [
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
        ] {
            storage
                .insert_nudge(vel_storage::NudgeInsert {
                    nudge_type: nudge_type.to_string(),
                    level: level.to_string(),
                    state: "resolved".to_string(),
                    related_commitment_id: None,
                    message: format!("{} happened", nudge_type),
                    snoozed_until: None,
                    resolved_at: Some(recent),
                    signals_snapshot_json: None,
                    inference_snapshot_json: None,
                    metadata_json: None,
                })
                .await
                .unwrap();
        }

        let created =
            crate::services::suggestions::evaluate_after_nudges(&storage, &test_policy_config())
                .await
                .unwrap();
        assert_eq!(created, 1);

        let suggestions = storage.list_suggestions(Some("pending"), 10).await.unwrap();
        let followup = suggestions
            .iter()
            .find(|suggestion| suggestion.suggestion_type == "add_followup_block")
            .expect("follow-up suggestion should exist");
        assert!(suggestions
            .iter()
            .all(|suggestion| suggestion.suggestion_type != "increase_commute_buffer"));
        assert!(followup.priority > 50);
        let uncertainties = storage
            .list_uncertainty_records(Some("open"), 10)
            .await
            .unwrap();
        assert_eq!(uncertainties.len(), 1);
        assert_eq!(
            uncertainties[0].subject_id.as_deref(),
            Some("increase_commute_buffer")
        );
        assert_eq!(uncertainties[0].resolution_mode, "defer");
    }

    #[tokio::test]
    async fn evaluate_creates_multiple_suggestion_families_in_priority_order() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let recent = now_ts - 3600;

        storage
            .set_current_context(
                now_ts,
                &serde_json::json!({
                    "global_risk_level": "high",
                    "global_risk_score": 0.9,
                    "attention_state": "drifting",
                    "drift_type": "morning_drift",
                    "message_waiting_on_me_count": 3
                })
                .to_string(),
            )
            .await
            .unwrap();

        for (nudge_type, level) in [
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
            ("morning_drift", "warning"),
            ("morning_drift", "warning"),
            ("morning_drift", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
        ] {
            storage
                .insert_nudge(vel_storage::NudgeInsert {
                    nudge_type: nudge_type.to_string(),
                    level: level.to_string(),
                    state: "resolved".to_string(),
                    related_commitment_id: None,
                    message: format!("{} happened", nudge_type),
                    snoozed_until: None,
                    resolved_at: Some(recent),
                    signals_snapshot_json: None,
                    inference_snapshot_json: None,
                    metadata_json: None,
                })
                .await
                .unwrap();
        }

        let mut policy_config = test_policy_config();
        policy_config.suggestions.max_new_per_evaluate = 4;
        policy_config.suggestions.response_debt.threshold = 2;
        policy_config.suggestions.morning_drift.threshold = 2;

        let created = crate::services::suggestions::evaluate_after_nudges(&storage, &policy_config)
            .await
            .unwrap();
        assert_eq!(created, 4);

        let suggestions = storage.list_suggestions(Some("pending"), 10).await.unwrap();
        let ordered_types: Vec<_> = suggestions
            .iter()
            .map(|suggestion| suggestion.suggestion_type.as_str())
            .collect();
        assert_eq!(
            ordered_types,
            vec![
                "increase_commute_buffer",
                "increase_prep_window",
                "add_followup_block",
                "add_start_routine"
            ]
        );
    }

    #[tokio::test]
    async fn suggestion_policy_limits_new_items_and_suppresses_recent_rejections() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now_ts = time::OffsetDateTime::now_utc().unix_timestamp();
        let recent = now_ts - 3600;

        storage
            .set_current_context(
                now_ts,
                &serde_json::json!({
                    "global_risk_level": "high",
                    "global_risk_score": 0.8,
                    "attention_state": "drifting",
                    "drift_type": "morning_drift",
                    "message_waiting_on_me_count": 3
                })
                .to_string(),
            )
            .await
            .unwrap();

        storage
            .insert_suggestion_v2(vel_storage::SuggestionInsertV2 {
                suggestion_type: "increase_prep_window".to_string(),
                state: "rejected".to_string(),
                title: Some("Increase prep window".to_string()),
                summary: None,
                priority: 60,
                confidence: Some("medium".to_string()),
                dedupe_key: Some("increase_prep_window".to_string()),
                payload_json: serde_json::json!({
                    "type": "increase_prep_window",
                    "suggested_minutes": 45
                }),
                decision_context_json: None,
            })
            .await
            .unwrap();
        let rejected = storage
            .find_recent_suggestion_by_dedupe_key("increase_prep_window")
            .await
            .unwrap()
            .unwrap();
        storage
            .update_suggestion_state(&rejected.id, "rejected", Some(recent), None)
            .await
            .unwrap();

        for (nudge_type, level) in [
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("commute_leave_time", "danger"),
            ("meeting_prep_window", "warning"),
            ("meeting_prep_window", "warning"),
            ("morning_drift", "warning"),
            ("morning_drift", "warning"),
            ("morning_drift", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
            ("response_debt", "warning"),
        ] {
            storage
                .insert_nudge(vel_storage::NudgeInsert {
                    nudge_type: nudge_type.to_string(),
                    level: level.to_string(),
                    state: "resolved".to_string(),
                    related_commitment_id: None,
                    message: format!("{} happened", nudge_type),
                    snoozed_until: None,
                    resolved_at: Some(recent),
                    signals_snapshot_json: None,
                    inference_snapshot_json: None,
                    metadata_json: None,
                })
                .await
                .unwrap();
        }

        let mut policy_config = test_policy_config();
        policy_config.suggestions.max_new_per_evaluate = 2;

        let created = crate::services::suggestions::evaluate_after_nudges(&storage, &policy_config)
            .await
            .unwrap();
        assert_eq!(created, 2);

        let suggestions = storage.list_suggestions(Some("pending"), 10).await.unwrap();
        let suggestion_types: Vec<_> = suggestions
            .iter()
            .map(|suggestion| suggestion.suggestion_type.as_str())
            .collect();
        assert_eq!(
            suggestion_types,
            vec!["increase_commute_buffer", "add_followup_block"]
        );
    }

    // --- Chat API (ticket 034) ---

    #[tokio::test]
    async fn chat_list_conversations_empty() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/conversations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["ok"].as_bool().unwrap());
        assert!(json["data"]
            .as_array()
            .map(|a| a.is_empty())
            .unwrap_or(false));
    }

    #[tokio::test]
    async fn chat_create_conversation_then_list() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let create_body = r#"{"title":"Test conv","kind":"general"}"#;
        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(create_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(create_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(create_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["ok"].as_bool().unwrap());
        let id = json["data"]["id"].as_str().unwrap();
        assert!(id.starts_with("conv_"));

        let list_resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/conversations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list_body = axum::body::to_bytes(list_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let list_json: serde_json::Value = serde_json::from_slice(&list_body).unwrap();
        let convs = list_json["data"].as_array().unwrap();
        assert_eq!(convs.len(), 1);
        assert_eq!(convs[0]["id"].as_str().unwrap(), id);
    }

    #[tokio::test]
    async fn chat_get_conversation_404() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/conversations/conv_nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn chat_create_message_then_list() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"user","kind":"text","content":{"text":"hello"}}"#;
        let msg_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);
        let msg_resp_body = axum::body::to_bytes(msg_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let msg_json: serde_json::Value = serde_json::from_slice(&msg_resp_body).unwrap();
        let user_msg = &msg_json["data"]["user_message"];
        assert!(user_msg["id"].as_str().unwrap().starts_with("msg_"));
        assert_eq!(user_msg["content"]["text"].as_str().unwrap(), "hello");

        let list_resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(list_resp.status(), StatusCode::OK);
        let list_body = axum::body::to_bytes(list_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let list_json: serde_json::Value = serde_json::from_slice(&list_body).unwrap();
        assert_eq!(list_json["data"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn chat_inbox_empty() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/inbox")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["ok"].as_bool().unwrap());
        assert!(json["data"]
            .as_array()
            .map(|a| a.is_empty())
            .unwrap_or(false));
    }

    #[tokio::test]
    async fn chat_list_conversation_interventions() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let conversation_id = storage
            .create_conversation(vel_storage::ConversationInsert {
                id: "conv_test".to_string(),
                title: Some("T".to_string()),
                kind: "general".to_string(),
                pinned: false,
                archived: false,
            })
            .await
            .unwrap();
        let message_id = storage
            .create_message(vel_storage::MessageInsert {
                id: "msg_test".to_string(),
                conversation_id: conversation_id.as_ref().to_string(),
                role: "assistant".to_string(),
                kind: "reminder_card".to_string(),
                content_json: r#"{"title":"Reminder"}"#.to_string(),
                status: None,
                importance: None,
            })
            .await
            .unwrap();
        storage
            .create_intervention(vel_storage::InterventionInsert {
                id: "intv_test".to_string(),
                message_id: message_id.as_ref().to_string(),
                kind: "reminder".to_string(),
                state: "active".to_string(),
                surfaced_at: 100,
                resolved_at: None,
                snoozed_until: None,
                confidence: None,
                source_json: None,
                provenance_json: None,
            })
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/conversations/conv_test/interventions")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = json["data"].as_array().unwrap();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0]["id"].as_str().unwrap(), "intv_test");
        assert_eq!(data[0]["message_id"].as_str().unwrap(), "msg_test");
    }

    #[tokio::test]
    async fn chat_assistant_card_message_creates_intervention() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"assistant","kind":"reminder_card","content":{"title":"Take meds","reason":"morning routine"}}"#;
        let msg_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);
        let msg_resp_body = axum::body::to_bytes(msg_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let msg_json: serde_json::Value = serde_json::from_slice(&msg_resp_body).unwrap();
        let message_id = msg_json["data"]["user_message"]["id"].as_str().unwrap();

        let interventions = storage
            .get_interventions_by_conversation(conv_id)
            .await
            .unwrap();
        assert_eq!(interventions.len(), 1);
        assert_eq!(interventions[0].message_id.as_ref(), message_id);
        assert_eq!(interventions[0].kind, "reminder");
        assert_eq!(interventions[0].state, "active");

        let events = storage
            .list_events_by_aggregate("intervention", interventions[0].id.as_ref(), 10)
            .await
            .unwrap();
        assert!(events
            .iter()
            .any(|event| event.event_name == "intervention.created"));
    }

    #[tokio::test]
    async fn chat_assistant_card_message_emits_typed_websocket_events() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        let mut rx = broadcast_tx.subscribe();
        let state = crate::state::AppState::new(
            storage,
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        );
        let app = build_app_with_state(state);

        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"assistant","kind":"reminder_card","content":{"title":"Take meds","reason":"morning routine"}}"#;
        let msg_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);

        let intervention_event = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
            .await
            .unwrap()
            .unwrap();
        let message_event = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            intervention_event.event_type,
            vel_api_types::WsEventType::InterventionsNew
        );
        assert_eq!(
            intervention_event.payload["kind"].as_str().unwrap(),
            "reminder"
        );
        assert_eq!(
            intervention_event.payload["state"].as_str().unwrap(),
            "active"
        );

        assert_eq!(
            message_event.event_type,
            vel_api_types::WsEventType::MessagesNew
        );
        assert_eq!(message_event.payload["role"].as_str().unwrap(), "assistant");
        assert_eq!(
            message_event.payload["kind"].as_str().unwrap(),
            "reminder_card"
        );
        assert_eq!(
            message_event.payload["conversation_id"].as_str().unwrap(),
            conv_id
        );

        let message_id = message_event.payload["id"].as_str().unwrap();
        assert_eq!(
            intervention_event.payload["message_id"].as_str().unwrap(),
            message_id
        );
    }

    #[tokio::test]
    async fn chat_message_provenance_hydrates_message_and_intervention_data() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"assistant","kind":"reminder_card","content":{"title":"Take meds","reason":"morning routine","confidence":0.82}}"#;
        let msg_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);
        let msg_resp_body = axum::body::to_bytes(msg_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let msg_json: serde_json::Value = serde_json::from_slice(&msg_resp_body).unwrap();
        let message_id = msg_json["data"]["user_message"]["id"].as_str().unwrap();

        let provenance_resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/messages/{}/provenance", message_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(provenance_resp.status(), StatusCode::OK);
        let provenance_body = axum::body::to_bytes(provenance_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let provenance_json: serde_json::Value = serde_json::from_slice(&provenance_body).unwrap();
        let data = &provenance_json["data"];

        assert_eq!(data["message_id"].as_str().unwrap(), message_id);

        let events = data["events"].as_array().unwrap();
        assert!(!events.is_empty());
        assert!(events.iter().any(|event| {
            event["event_name"].as_str() == Some("message.created")
                || event["event_name"].as_str() == Some("message.updated")
        }));

        let linked_objects = data["linked_objects"].as_array().unwrap();
        assert!(linked_objects.iter().any(|object| {
            object["kind"].as_str() == Some("message")
                && object["id"].as_str() == Some(message_id)
                && object["conversation_id"].as_str() == Some(conv_id)
                && object["message_kind"].as_str() == Some("reminder_card")
        }));
        assert!(linked_objects.iter().any(|object| {
            object["kind"].as_str() == Some("intervention")
                && object["message_id"].as_str() == Some(message_id)
                && object["intervention_kind"].as_str() == Some("reminder")
                && object["state"].as_str() == Some("active")
                && object["source"]["title"].as_str() == Some("Take meds")
                && object["source"]["reason"].as_str() == Some("morning routine")
                && object["provenance"]["message_id"].as_str() == Some(message_id)
                && object["provenance"]["conversation_id"].as_str() == Some(conv_id)
        }));

        let signals = data["signals"].as_array().unwrap();
        assert!(signals.iter().any(|signal| {
            signal["kind"].as_str() == Some("message_content")
                && signal["message_kind"].as_str() == Some("reminder_card")
                && signal["title"].as_str() == Some("Take meds")
                && signal["reason"].as_str() == Some("morning routine")
        }));
        assert!(signals.iter().any(|signal| {
            signal["kind"].as_str() == Some("intervention_source")
                && signal["intervention_kind"].as_str() == Some("reminder")
                && signal["payload"]["title"].as_str() == Some("Take meds")
        }));
        assert!(signals.iter().any(|signal| {
            signal["kind"].as_str() == Some("intervention_provenance")
                && signal["intervention_id"].is_string()
                && signal["payload"]["message_kind"].as_str() == Some("reminder_card")
        }));

        let policy_decisions = data["policy_decisions"].as_array().unwrap();
        assert!(policy_decisions.iter().any(|decision| {
            decision["kind"].as_str() == Some("message_policy")
                && decision["message_kind"].as_str() == Some("reminder_card")
                && decision["reason"].as_str() == Some("morning routine")
                && decision["confidence"].as_f64() == Some(0.82)
        }));
        assert!(policy_decisions.iter().any(|decision| {
            decision["kind"].as_str() == Some("intervention_state")
                && decision["intervention_kind"].as_str() == Some("reminder")
                && decision["state"].as_str() == Some("active")
        }));
    }

    #[tokio::test]
    async fn chat_assistant_text_message_does_not_create_intervention() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let create_conv = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/conversations")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"T","kind":"general"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let conv_body = axum::body::to_bytes(create_conv.into_body(), usize::MAX)
            .await
            .unwrap();
        let conv_json: serde_json::Value = serde_json::from_slice(&conv_body).unwrap();
        let conv_id = conv_json["data"]["id"].as_str().unwrap();

        let msg_body = r#"{"role":"assistant","kind":"text","content":{"text":"plain reply"}}"#;
        let msg_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/conversations/{}/messages", conv_id))
                    .header("content-type", "application/json")
                    .body(Body::from(msg_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(msg_resp.status(), StatusCode::OK);

        let interventions = storage
            .get_interventions_by_conversation(conv_id)
            .await
            .unwrap();
        assert!(interventions.is_empty());
    }

    #[tokio::test]
    async fn sync_calendar_ingests_tzid_events() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_calendar_tzid_{}.ics",
            uuid::Uuid::new_v4().simple()
        ));
        let ics = r#"BEGIN:VCALENDAR
VERSION:2.0
BEGIN:VEVENT
UID:evt_tzid_1
DTSTART;TZID=America/Denver:20260116T110000
DTEND;TZID=America/Denver:20260116T120000
SUMMARY:Planning meeting
LOCATION:Studio
END:VEVENT
END:VCALENDAR
"#;
        std::fs::write(&file_path, ics).unwrap();

        let config = vel_config::AppConfig {
            calendar_ics_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/calendar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let signals = storage
            .list_signals(Some("calendar_event"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1, "TZID calendar event should be ingested");

        let expected_start = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2026, time::Month::January, 16).unwrap(),
            time::Time::from_hms(11, 0, 0).unwrap(),
        )
        .assume_offset(time::UtcOffset::from_hms(-7, 0, 0).unwrap())
        .unix_timestamp();
        let expected_end = time::PrimitiveDateTime::new(
            time::Date::from_calendar_date(2026, time::Month::January, 16).unwrap(),
            time::Time::from_hms(12, 0, 0).unwrap(),
        )
        .assume_offset(time::UtcOffset::from_hms(-7, 0, 0).unwrap())
        .unix_timestamp();

        assert_eq!(signals[0].timestamp, expected_start);
        assert_eq!(signals[0].payload_json["event_id"], "evt_tzid_1");
        assert_eq!(signals[0].payload_json["title"], "Planning meeting");
        assert_eq!(signals[0].payload_json["location"], "Studio");
        assert_eq!(signals[0].payload_json["start"], expected_start);
        assert_eq!(signals[0].payload_json["end"], expected_end);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_calendar_preserves_explicit_prep_and_travel_minutes() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_calendar_fields_{}.ics",
            uuid::Uuid::new_v4().simple()
        ));
        let ics = r#"BEGIN:VCALENDAR
VERSION:2.0
BEGIN:VEVENT
UID:evt_fields_1
DTSTART:20260116T180000Z
DTEND:20260116T190000Z
SUMMARY:Client review
LOCATION:HQ
X-VEL-PREP-MINUTES:30
X-VEL-TRAVEL-MINUTES:40
END:VEVENT
END:VCALENDAR
"#;
        std::fs::write(&file_path, ics).unwrap();

        let config = vel_config::AppConfig {
            calendar_ics_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/calendar")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let signals = storage
            .list_signals(Some("calendar_event"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].payload_json["event_id"], "evt_fields_1");
        assert_eq!(signals[0].payload_json["prep_minutes"], 30);
        assert_eq!(signals[0].payload_json["travel_minutes"], 40);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_transcripts_ingests_rows_and_signals() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_transcripts_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "source": "chatgpt",
            "conversation_id": "conv_external",
            "messages": [
                {
                    "timestamp": now - 60,
                    "role": "user",
                    "content": "What did we decide about Vel?",
                    "metadata": { "project_hint": "vel" }
                },
                {
                    "timestamp": now,
                    "role": "assistant",
                    "content": "You said to prioritize repeated personal use.",
                    "metadata": {}
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            transcript_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/transcripts")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let transcripts = storage
            .list_assistant_transcripts_by_conversation("conv_external")
            .await
            .unwrap();
        assert_eq!(transcripts.len(), 2);
        assert_eq!(transcripts[0].source, "chatgpt");
        assert_eq!(transcripts[0].role, "user");

        let signals = storage
            .list_signals(Some("assistant_message"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 2);
        assert_eq!(signals[0].source, "chatgpt");

        let (_, context_json) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("transcript sync should trigger evaluate and store current context");
        let context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
        assert_eq!(context["inferred_activity"], "assistant_reflection");
        assert_eq!(
            context["assistant_message_summary"]["conversation_id"],
            "conv_external"
        );
        assert_eq!(context["assistant_message_summary"]["role"], "assistant");
        assert_eq!(context["assistant_message_summary"]["source"], "chatgpt");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_transcripts_replay_is_deduplicated() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_transcripts_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!([
            {
                "id": "tr_fixed_1",
                "source": "chatgpt",
                "conversation_id": "conv_dedupe",
                "timestamp": 1700000100,
                "role": "user",
                "content": "hello",
                "metadata": {}
            }
        ]);
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            transcript_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        for _ in 0..2 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/v1/sync/transcripts")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        let transcripts = storage
            .list_assistant_transcripts_by_conversation("conv_dedupe")
            .await
            .unwrap();
        assert_eq!(transcripts.len(), 1);

        let signals = storage
            .list_signals(Some("assistant_message"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_activity_ingests_snapshot_events() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_activity_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "source": "workstation",
            "events": [
                {
                    "signal_type": "shell_login",
                    "timestamp": 1700001000,
                    "host": "ws-1",
                    "details": { "tty": "pts/1" }
                },
                {
                    "signal_type": "computer_activity",
                    "timestamp": 1700001060,
                    "host": "ws-1",
                    "details": { "app": "zed" }
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            activity_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/activity")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let shell_signals = storage
            .list_signals(Some("shell_login"), None, 10)
            .await
            .unwrap();
        let activity_signals = storage
            .list_signals(Some("computer_activity"), None, 10)
            .await
            .unwrap();
        assert_eq!(shell_signals.len(), 1);
        assert_eq!(activity_signals.len(), 1);
        assert_eq!(shell_signals[0].source, "workstation");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_health_ingests_snapshot_and_triggers_evaluation() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!("vel_health_{}.json", uuid::Uuid::new_v4().simple()));
        let snapshot = serde_json::json!({
            "source": "healthkit",
            "samples": [
                {
                    "metric_type": "step_count",
                    "timestamp": now,
                    "value": 6400,
                    "unit": "count",
                    "source_app": "Health",
                    "device": "Apple Watch"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            health_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let signals = storage
            .list_signals(Some("health_metric"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].source, "healthkit");

        let (_, context_json) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("health sync should trigger evaluate");
        let context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
        assert_eq!(context["health_summary"]["metric_type"], "step_count");
        assert_eq!(context["health_summary"]["value"], 6400);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_notes_ingests_markdown_files_as_captures() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir().join(format!("vel_notes_{}", uuid::Uuid::new_v4().simple()));
        let nested_dir = dir.join("daily");
        std::fs::create_dir_all(&nested_dir).unwrap();
        std::fs::write(nested_dir.join("today.md"), "# Today\nShip notes sync\n").unwrap();
        std::fs::write(dir.join("ignore.json"), "{\"skip\":true}").unwrap();

        let config = vel_config::AppConfig {
            notes_path: Some(dir.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/notes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let captures = storage.list_captures_recent(10, false).await.unwrap();
        assert_eq!(captures.len(), 1);
        assert_eq!(captures[0].capture_type, "note_document");
        assert!(captures[0].content_text.contains("Ship notes sync"));

        let signals = storage
            .list_signals(Some("note_document"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].source, "notes");
        assert_eq!(signals[0].payload_json["path"], "daily/today.md");
        assert_eq!(signals[0].payload_json["title"], "Today");

        let (_, context_json) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("notes sync should trigger evaluate and store current context");
        let context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
        assert_eq!(context["inferred_activity"], "note_review");
        assert_eq!(context["note_document_summary"]["path"], "daily/today.md");
        assert_eq!(context["note_document_summary"]["title"], "Today");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn sync_notes_replay_is_deduplicated() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir().join(format!("vel_notes_{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("ideas.md"), "# Ideas\nMore context\n").unwrap();

        let config = vel_config::AppConfig {
            notes_path: Some(dir.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        for _ in 0..2 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/v1/sync/notes")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        assert_eq!(storage.capture_count().await.unwrap(), 1);
        let signals = storage
            .list_signals(Some("note_document"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn sync_git_replay_is_deduplicated_by_source_ref() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!("vel_git_{}.json", uuid::Uuid::new_v4().simple()));
        let snapshot = serde_json::json!({
            "source": "git",
            "events": [
                {
                    "timestamp": 1700002000,
                    "repo": "/home/jove/code/vel",
                    "repo_name": "vel",
                    "branch": "main",
                    "operation": "commit",
                    "commit_oid": "abc123",
                    "message": "feat: add git sync"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            git_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        for _ in 0..2 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/v1/sync/git")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        let signals = storage
            .list_signals(Some("git_activity"), None, 10)
            .await
            .unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(
            signals[0].source_ref.as_deref(),
            Some("git:/home/jove/code/vel|main|commit|abc123|1700002000")
        );

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_messaging_ingests_snapshot_and_triggers_evaluation() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_messages_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "source": "messaging",
            "account_id": "local-default",
            "threads": [
                {
                    "thread_id": "thr_ops",
                    "platform": "sms",
                    "title": "Review reschedule",
                    "participants": [
                        { "id": "me", "name": "Me", "is_me": true },
                        { "id": "+15551234567", "name": "Sam", "is_me": false }
                    ],
                    "latest_timestamp": now,
                    "waiting_state": "me",
                    "scheduling_related": true,
                    "urgent": true,
                    "summary": "Need to answer the review reschedule request.",
                    "snippet": "Can we move the review to 3?"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            messaging_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        for _ in 0..2 {
            let resp = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method(Method::POST)
                        .uri("/v1/sync/messaging")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        let signals = storage
            .list_signals(Some("message_thread"), None, 10)
            .await
            .unwrap();
        let expected_source_ref = format!("messaging:sms:local-default:thr_ops:{now}");
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].source, "messaging");
        assert_eq!(
            signals[0].source_ref.as_deref(),
            Some(expected_source_ref.as_str())
        );
        assert_eq!(signals[0].payload_json["platform"], "sms");
        assert_eq!(signals[0].payload_json["thread_id"], "thr_ops");
        assert_eq!(signals[0].payload_json["waiting_state"], "me");
        assert_eq!(signals[0].payload_json["scheduling_related"], true);
        assert_eq!(signals[0].payload_json["urgent"], true);
        assert_eq!(
            signals[0].payload_json["snippet"],
            "Can we move the review to 3?"
        );

        let (_, context_json) = storage
            .get_current_context()
            .await
            .unwrap()
            .expect("sync should trigger evaluate and store current context");
        let context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
        assert_eq!(context["message_waiting_on_me_count"], 1);
        assert_eq!(context["message_scheduling_thread_count"], 1);
        assert_eq!(context["message_urgent_thread_count"], 1);

        let nudges = storage.list_nudges(None, 20).await.unwrap();
        let response_debt = nudges
            .iter()
            .find(|n| n.nudge_type == "response_debt")
            .expect("sync-triggered evaluate should create response_debt nudge");
        assert_eq!(response_debt.state, "active");
        assert_eq!(response_debt.level, "warning");

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn evaluate_includes_messaging_summary_in_current_context() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "message_thread".to_string(),
                source: "messaging".to_string(),
                source_ref: Some(format!("messaging:gmail:default:thr_1:{}", now)),
                timestamp: now,
                payload_json: Some(serde_json::json!({
                    "thread_id": "thr_1",
                    "platform": "gmail",
                    "title": "Dimitri follow-up",
                    "participants": [
                        { "id": "me@example.com", "name": "Me", "is_me": true },
                        { "id": "dimitri@example.com", "name": "Dimitri", "is_me": false }
                    ],
                    "latest_timestamp": now,
                    "waiting_state": "me",
                    "scheduling_related": true,
                    "urgent": true,
                    "snippet": "Can you send the updated draft?"
                })),
            })
            .await
            .unwrap();

        let evaluate_result = crate::services::evaluate::run(&storage, &test_policy_config()).await;
        assert!(evaluate_result.is_ok());

        let (_, context_json) = storage.get_current_context().await.unwrap().unwrap();
        let context: serde_json::Value = serde_json::from_str(&context_json).unwrap();
        assert_eq!(context["message_waiting_on_me_count"], 1);
        assert_eq!(context["message_scheduling_thread_count"], 1);
        assert_eq!(context["message_urgent_thread_count"], 1);
        assert_eq!(
            context["message_summary"]["top_threads"][0]["title"],
            "Dimitri follow-up"
        );
    }

    #[tokio::test]
    async fn evaluate_creates_response_debt_nudge_from_messaging_context() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "message_thread".to_string(),
                source: "messaging".to_string(),
                source_ref: Some(format!("messaging:sms:default:thr_sched:{}", now)),
                timestamp: now,
                payload_json: Some(serde_json::json!({
                    "thread_id": "thr_sched",
                    "platform": "sms",
                    "title": "Team reschedule",
                    "latest_timestamp": now,
                    "waiting_state": "me",
                    "scheduling_related": true,
                    "urgent": true,
                    "snippet": "Can we move the standup to 3?"
                })),
            })
            .await
            .unwrap();

        let evaluate_result = crate::services::evaluate::run(&storage, &test_policy_config()).await;
        assert!(evaluate_result.is_ok());

        let nudges = storage.list_nudges(None, 20).await.unwrap();
        let nudge = nudges
            .iter()
            .find(|n| n.nudge_type == "response_debt")
            .expect("response_debt nudge should exist");
        assert_eq!(nudge.state, "active");
        assert_eq!(nudge.level, "warning");
        assert_eq!(
            nudge.message,
            "You have messages waiting on you, including scheduling follow-up."
        );
        let inference = nudge
            .inference_snapshot_json
            .as_ref()
            .map(|s| serde_json::from_str::<serde_json::Value>(s).unwrap())
            .unwrap();
        assert_eq!(inference["message_waiting_on_me_count"], 1);
        assert_eq!(inference["message_scheduling_thread_count"], 1);
        assert_eq!(inference["message_urgent_thread_count"], 1);
        let metadata = &nudge.metadata_json;
        assert_eq!(metadata["policy"], "response_debt");
    }

    #[tokio::test]
    async fn response_debt_nudge_resolves_when_waiting_count_clears() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .set_current_context(
                now,
                &serde_json::json!({
                    "message_waiting_on_me_count": 0,
                    "message_scheduling_thread_count": 0,
                    "message_urgent_thread_count": 0
                })
                .to_string(),
            )
            .await
            .unwrap();

        let nudge_id = storage
            .insert_nudge(vel_storage::NudgeInsert {
                nudge_type: "response_debt".to_string(),
                level: "warning".to_string(),
                state: "active".to_string(),
                related_commitment_id: None,
                message: "You have messages waiting on you.".to_string(),
                snoozed_until: None,
                resolved_at: None,
                signals_snapshot_json: None,
                inference_snapshot_json: None,
                metadata_json: Some(serde_json::json!({ "policy": "response_debt" })),
            })
            .await
            .unwrap();

        let updated_result =
            crate::services::nudge_engine::evaluate(&storage, &test_policy_config(), 0).await;
        assert!(updated_result.is_ok());
        let updated = updated_result.unwrap_or_default();
        assert_eq!(updated, 1);

        let nudges = storage.list_nudges(None, 20).await.unwrap();
        let nudge = nudges
            .iter()
            .find(|n| n.nudge_id == nudge_id)
            .expect("response_debt nudge should still exist");
        assert_eq!(nudge.state, "resolved");
        assert!(nudge.resolved_at.is_some());
    }

    #[tokio::test]
    async fn sync_todoist_reopens_and_updates_existing_commitment() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let commitment_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Old title".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todoist_123".to_string()),
                status: vel_core::CommitmentStatus::Done,
                due_at: None,
                project: Some("old".to_string()),
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({ "todoist_id": "123" })),
            })
            .await
            .unwrap();
        storage
            .update_commitment(
                commitment_id.as_ref(),
                None,
                Some(vel_core::CommitmentStatus::Done),
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_todoist_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "items": [
                {
                    "id": "123",
                    "content": "Updated title",
                    "checked": false,
                    "due": { "date": "2026-03-17T09:30:00" },
                    "labels": ["health"],
                    "project_id": "proj-1"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            todoist_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/todoist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let updated = storage
            .get_commitment_by_id(commitment_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.status, vel_core::CommitmentStatus::Open);
        assert_eq!(updated.text, "Updated title");
        assert_eq!(updated.project.as_deref(), Some("proj-1"));
        assert_eq!(updated.commitment_kind.as_deref(), Some("medication"));
        assert!(updated.resolved_at.is_none());
        assert_eq!(updated.metadata_json["priority"], 1);
        assert_eq!(updated.metadata_json["has_due_time"], true);

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn sync_todoist_marks_commitment_done_when_task_checked() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let commitment_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Ship feature".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todoist_456".to_string()),
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({ "todoist_id": "456" })),
            })
            .await
            .unwrap();

        let dir = std::env::temp_dir();
        let file_path = dir.join(format!(
            "vel_todoist_{}.json",
            uuid::Uuid::new_v4().simple()
        ));
        let snapshot = serde_json::json!({
            "items": [
                {
                    "id": "456",
                    "content": "Ship feature",
                    "checked": true,
                    "labels": [],
                    "project_id": "proj-2"
                }
            ]
        });
        std::fs::write(&file_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();

        let config = vel_config::AppConfig {
            todoist_snapshot_path: Some(file_path.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/todoist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let updated = storage
            .get_commitment_by_id(commitment_id.as_ref())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(updated.status, vel_core::CommitmentStatus::Done);
        assert!(updated.resolved_at.is_some());

        let _ = std::fs::remove_file(&file_path);
    }

    #[tokio::test]
    async fn inference_uses_shell_login_as_workstation_activity() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "shell_login".to_string(),
                source: "workstation".to_string(),
                source_ref: None,
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(
                    serde_json::json!({ "host": "ws-1", "activity": "shell_login" }),
                ),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);

        let ctx_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ctx_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(ctx_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["context"]["inferred_activity"],
            "computer_active"
        );
        assert_eq!(json["data"]["context"]["morning_state"], "engaged");
        assert!(json["data"]["context"]["git_activity_summary"].is_null());
    }

    #[tokio::test]
    async fn inference_uses_git_activity_as_workstation_activity() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "git_activity".to_string(),
                source: "git".to_string(),
                source_ref: Some(
                    "git:/home/jove/code/vel|main|commit|abc123|1700002000".to_string(),
                ),
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(serde_json::json!({
                    "repo": "/home/jove/code/vel",
                    "branch": "main",
                    "operation": "commit",
                    "commit_oid": "abc123"
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);

        let ctx_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ctx_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(ctx_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["data"]["context"]["inferred_activity"], "coding");
        assert_eq!(json["data"]["context"]["morning_state"], "engaged");
        assert_eq!(
            json["data"]["context"]["git_activity_summary"]["repo"],
            "vel"
        );
        assert_eq!(
            json["data"]["context"]["git_activity_summary"]["branch"],
            "main"
        );
        assert_eq!(
            json["data"]["context"]["git_activity_summary"]["operation"],
            "commit"
        );
    }

    #[tokio::test]
    async fn evaluate_exposes_explicit_missing_risk_state_in_current_context() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "computer_activity".to_string(),
                source: "activity".to_string(),
                source_ref: Some("activity:test".to_string()),
                timestamp: time::OffsetDateTime::now_utc().unix_timestamp(),
                payload_json: Some(serde_json::json!({ "host": "ws-1" })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);

        let ctx_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ctx_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(ctx_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["data"]["context"]["global_risk_level"], "unknown");
        assert!(json["data"]["context"]["global_risk_score"].is_null());
        assert_eq!(json["data"]["context"]["global_risk_missing"], true);
    }

    #[tokio::test]
    async fn evaluate_current_context_prefers_nearest_future_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "calendar".to_string(),
                source_ref: Some("calendar:past".to_string()),
                timestamp: now - 20 * 60,
                payload_json: Some(serde_json::json!({
                    "title": "Past event",
                    "start_time": now - 20 * 60,
                })),
            })
            .await
            .unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "calendar".to_string(),
                source_ref: Some("calendar:later".to_string()),
                timestamp: now + 60 * 60,
                payload_json: Some(serde_json::json!({
                    "title": "Later event",
                    "start_time": now + 60 * 60,
                })),
            })
            .await
            .unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "calendar".to_string(),
                source_ref: Some("calendar:next".to_string()),
                timestamp: now + 15 * 60,
                payload_json: Some(serde_json::json!({
                    "title": "Next event",
                    "start_time": now + 15 * 60,
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);

        let ctx_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ctx_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(ctx_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["context"]["next_event_start_ts"],
            now + 15 * 60
        );
    }

    #[tokio::test]
    async fn evaluate_excludes_unchecked_google_calendar_events_from_current_context() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        storage
            .set_setting(
                "integration_google_calendar",
                &serde_json::json!({
                    "client_id": "client",
                    "calendars": [
                        {
                            "id": "cal_1",
                            "summary": "Personal",
                            "primary": true,
                            "selected": false
                        }
                    ],
                    "all_calendars_selected": false,
                    "last_sync_at": now,
                    "last_sync_status": "ok",
                    "last_error": null,
                    "last_item_count": 1
                }),
            )
            .await
            .unwrap();
        storage
            .set_setting(
                "integration_google_calendar_secrets",
                &serde_json::json!({
                    "client_secret": "secret",
                    "access_token": "token",
                    "refresh_token": "refresh",
                    "token_expires_at": now + 3600
                }),
            )
            .await
            .unwrap();
        let signal_id = storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: Some("google_calendar:cal_1:evt_1".to_string()),
                timestamp: now + 15 * 60,
                payload_json: Some(serde_json::json!({
                    "calendar_id": "cal_1",
                    "calendar_name": "Personal",
                    "title": "Should be ignored",
                    "start": now + 15 * 60,
                    "end": now + 45 * 60,
                    "prep_minutes": 15,
                    "travel_minutes": 0
                })),
            })
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let eval_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(eval_resp.status(), StatusCode::OK);

        let ctx_resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/context/current")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(ctx_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(ctx_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["data"]["context"]["next_event_start_ts"].is_null());
        assert_eq!(json["data"]["context"]["prep_window_active"], false);
        assert!(!json["data"]["context"]["signals_used"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value.as_str() == Some(signal_id.as_str())));
    }

    #[tokio::test]
    async fn chat_settings_get_and_patch() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let get_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(get_resp.status(), StatusCode::OK);

        let patch_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/settings")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"disable_proactive":true,"timezone":"America/Denver"}"#.to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["data"]["disable_proactive"].as_bool().unwrap());
        assert_eq!(json["data"]["timezone"], "America/Denver");
    }

    #[tokio::test]
    async fn chat_settings_rejects_invalid_timezone() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let patch_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/settings")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"timezone":"Mars/Olympus"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn chat_settings_include_adaptive_policy_overrides() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(
                "adaptive_policy_overrides",
                &serde_json::json!({
                    "commute_buffer_minutes": 30,
                    "default_prep_minutes": 45,
                    "commute_buffer_source_suggestion_id": "sug_commute",
                    "commute_buffer_source_title": "Increase commute buffer",
                    "commute_buffer_source_accepted_at": 1710000100,
                    "default_prep_source_suggestion_id": "sug_prep",
                    "default_prep_source_title": "Increase prep window",
                    "default_prep_source_accepted_at": 1710000200
                }),
            )
            .await
            .unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/settings")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["adaptive_policy_overrides"]["commute_buffer_minutes"].as_u64(),
            Some(30)
        );
        assert_eq!(
            json["data"]["adaptive_policy_overrides"]["default_prep_minutes"].as_u64(),
            Some(45)
        );
        assert_eq!(
            json["data"]["adaptive_policy_overrides"]["commute_buffer_source_title"].as_str(),
            Some("Increase commute buffer")
        );
        assert_eq!(
            json["data"]["adaptive_policy_overrides"]["default_prep_source_title"].as_str(),
            Some("Increase prep window")
        );
    }

    #[tokio::test]
    async fn integrations_google_calendar_settings_and_auth_start() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let config = AppConfig {
            base_url: "http://127.0.0.1:4130".to_string(),
            ..Default::default()
        };
        let app = build_app(storage, config, test_policy_config(), None, None);

        let patch_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/google-calendar")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"client_id":"gcal-client","client_secret":"gcal-secret"}"#.to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let patch_body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let patch_json: serde_json::Value = serde_json::from_slice(&patch_body).unwrap();
        assert_eq!(patch_json["data"]["google_calendar"]["configured"], true);
        assert_eq!(patch_json["data"]["google_calendar"]["connected"], false);
        assert_eq!(patch_json["data"]["google_calendar"]["has_client_id"], true);
        assert_eq!(
            patch_json["data"]["google_calendar"]["has_client_secret"],
            true
        );

        let auth_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/integrations/google-calendar/auth/start")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(auth_resp.status(), StatusCode::OK);
        let auth_body = axum::body::to_bytes(auth_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let auth_json: serde_json::Value = serde_json::from_slice(&auth_body).unwrap();
        let auth_url = auth_json["data"]["auth_url"]
            .as_str()
            .expect("auth_url should be returned");
        assert!(auth_url.starts_with("https://accounts.google.com/o/oauth2/v2/auth?"));
        assert!(auth_url.contains("client_id=gcal-client"));
        assert!(auth_url.contains("redirect_uri=http%3A%2F%2F127.0.0.1%3A4130%2Fapi%2Fintegrations%2Fgoogle-calendar%2Foauth%2Fcallback"));
        assert!(
            auth_url.contains("scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fcalendar.readonly")
        );
    }

    #[tokio::test]
    async fn integrations_store_sensitive_tokens_separately_from_public_settings() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/google-calendar")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"client_id":"gcal-client","client_secret":"gcal-secret"}"#.to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        let _ = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/todoist")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"api_token":"todoist-token"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let settings = storage.get_all_settings().await.unwrap();
        let google_public = settings
            .get("integration_google_calendar")
            .expect("google public settings should exist");
        let google_secrets = settings
            .get("integration_google_calendar_secrets")
            .expect("google secrets should exist");
        let todoist_public = settings
            .get("integration_todoist")
            .expect("todoist public settings should exist");
        let todoist_secrets = settings
            .get("integration_todoist_secrets")
            .expect("todoist secrets should exist");

        assert_eq!(google_public["client_id"], "gcal-client");
        assert!(
            google_public.get("client_secret").is_none()
                || google_public["client_secret"].is_null()
        );
        assert!(
            google_public.get("refresh_token").is_none()
                || google_public["refresh_token"].is_null()
        );
        assert_eq!(google_secrets["client_secret"], "gcal-secret");

        assert!(todoist_public.get("api_token").is_none() || todoist_public["api_token"].is_null());
        assert_eq!(todoist_secrets["api_token"], "todoist-token");
    }

    #[tokio::test]
    async fn integrations_todoist_patch_and_disconnect() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let patch_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/todoist")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"api_token":"todoist-token"}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let patch_body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let patch_json: serde_json::Value = serde_json::from_slice(&patch_body).unwrap();
        assert_eq!(patch_json["data"]["todoist"]["configured"], true);
        assert_eq!(patch_json["data"]["todoist"]["connected"], true);
        assert_eq!(patch_json["data"]["todoist"]["has_api_token"], true);

        let disconnect_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/integrations/todoist/disconnect")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(disconnect_resp.status(), StatusCode::OK);
        let disconnect_body = axum::body::to_bytes(disconnect_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let disconnect_json: serde_json::Value = serde_json::from_slice(&disconnect_body).unwrap();
        assert_eq!(disconnect_json["data"]["todoist"]["configured"], false);
        assert_eq!(disconnect_json["data"]["todoist"]["connected"], false);
        assert_eq!(disconnect_json["data"]["todoist"]["has_api_token"], false);
        assert_eq!(
            disconnect_json["data"]["todoist"]["last_sync_status"],
            "disconnected"
        );
    }

    #[tokio::test]
    async fn integrations_google_calendar_selection_persists_and_disconnect_clears_connection() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        storage
            .set_setting(
                "integration_google_calendar",
                &serde_json::json!({
                    "client_id": "gcal-client",
                    "calendars": [
                        { "id": "cal_a", "summary": "Primary", "primary": true, "selected": true },
                        { "id": "cal_b", "summary": "Work", "primary": false, "selected": false }
                    ],
                    "all_calendars_selected": false
                }),
            )
            .await
            .unwrap();
        storage
            .set_setting(
                "integration_google_calendar_secrets",
                &serde_json::json!({
                    "client_secret": "gcal-secret",
                    "refresh_token": "refresh-token",
                    "access_token": "access-token"
                }),
            )
            .await
            .unwrap();
        let app = build_app(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let patch_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/google-calendar")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"selected_calendar_ids":["cal_b"],"all_calendars_selected":false}"#
                            .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let patch_body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let patch_json: serde_json::Value = serde_json::from_slice(&patch_body).unwrap();
        let calendars = patch_json["data"]["google_calendar"]["calendars"]
            .as_array()
            .expect("calendars should be an array");
        assert_eq!(calendars[0]["selected"], false);
        assert_eq!(calendars[1]["selected"], true);
        assert_eq!(
            patch_json["data"]["google_calendar"]["all_calendars_selected"],
            false
        );
        assert_eq!(patch_json["data"]["google_calendar"]["connected"], true);

        let disconnect_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/integrations/google-calendar/disconnect")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(disconnect_resp.status(), StatusCode::OK);
        let disconnect_body = axum::body::to_bytes(disconnect_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let disconnect_json: serde_json::Value = serde_json::from_slice(&disconnect_body).unwrap();
        assert_eq!(
            disconnect_json["data"]["google_calendar"]["connected"],
            false
        );
        assert_eq!(
            disconnect_json["data"]["google_calendar"]["has_client_id"],
            true
        );
        assert_eq!(
            disconnect_json["data"]["google_calendar"]["has_client_secret"],
            true
        );
        let calendars = disconnect_json["data"]["google_calendar"]["calendars"]
            .as_array()
            .expect("calendars should be an array");
        assert_eq!(calendars[1]["selected"], true);
    }

    #[tokio::test]
    async fn integrations_get_includes_local_adapter_statuses() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let record_result =
            crate::services::integrations::record_sync_success(&storage, "notes", 2).await;
        assert!(record_result.is_ok());
        let config = AppConfig {
            activity_snapshot_path: Some("/tmp/activity.json".to_string()),
            health_snapshot_path: Some("/tmp/health.json".to_string()),
            git_snapshot_path: Some("/tmp/git.json".to_string()),
            messaging_snapshot_path: Some("/tmp/messaging.json".to_string()),
            notes_path: Some("/tmp/notes".to_string()),
            transcript_snapshot_path: Some("/tmp/transcripts.json".to_string()),
            ..Default::default()
        };
        let app = build_app(storage, config, test_policy_config(), None, None);

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/integrations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["data"]["activity"]["configured"], true);
        assert_eq!(
            json["data"]["activity"]["source_path"],
            "/tmp/activity.json"
        );
        assert_eq!(json["data"]["health"]["configured"], true);
        assert_eq!(json["data"]["health"]["source_path"], "/tmp/health.json");
        assert_eq!(json["data"]["notes"]["configured"], true);
        assert_eq!(json["data"]["notes"]["source_path"], "/tmp/notes");
        assert_eq!(json["data"]["notes"]["last_sync_status"], "ok");
        assert_eq!(json["data"]["notes"]["last_item_count"], 2);
        assert_eq!(
            json["data"]["google_calendar"]["guidance"]["action"],
            "Save credentials"
        );
        assert_eq!(json["data"]["activity"]["guidance"]["action"], "Sync now");
    }

    #[tokio::test]
    async fn integrations_local_source_path_patch_is_used_for_sync() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let dir = std::env::temp_dir().join(format!(
            "vel_notes_override_{}",
            uuid::Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("plan.md"), "# Plan\nShip the smooth path.\n").unwrap();
        let notes_path = dir.to_string_lossy().to_string();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let patch_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/integrations/notes/source")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({ "source_path": notes_path.clone() }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(patch_resp.status(), StatusCode::OK);
        let patch_body = axum::body::to_bytes(patch_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let patch_json: serde_json::Value = serde_json::from_slice(&patch_body).unwrap();
        assert_eq!(patch_json["data"]["notes"]["configured"], true);
        assert_eq!(patch_json["data"]["notes"]["source_path"], notes_path);

        let sync_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/notes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(sync_resp.status(), StatusCode::OK);
        let sync_body = axum::body::to_bytes(sync_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let sync_json: serde_json::Value = serde_json::from_slice(&sync_body).unwrap();
        assert_eq!(sync_json["data"]["signals_ingested"], 1);

        let integrations_resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/integrations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(integrations_resp.status(), StatusCode::OK);
        let integrations_body = axum::body::to_bytes(integrations_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let integrations_json: serde_json::Value =
            serde_json::from_slice(&integrations_body).unwrap();
        assert_eq!(
            integrations_json["data"]["notes"]["source_path"],
            notes_path
        );
        assert_eq!(integrations_json["data"]["notes"]["last_sync_status"], "ok");
        assert_eq!(integrations_json["data"]["notes"]["last_item_count"], 1);
    }

    #[tokio::test]
    async fn sync_notes_updates_integrations_status_and_sync_messaging_records_error() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let dir = std::env::temp_dir().join(format!("vel_notes_{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("plan.md"), "# plan\n").unwrap();
        let config = AppConfig {
            notes_path: Some(dir.to_string_lossy().to_string()),
            ..Default::default()
        };
        let app = build_app(storage.clone(), config, test_policy_config(), None, None);

        let notes_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/notes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(notes_resp.status(), StatusCode::OK);

        let integrations_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/integrations")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let integrations_body = axum::body::to_bytes(integrations_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let integrations_json: serde_json::Value =
            serde_json::from_slice(&integrations_body).unwrap();
        assert_eq!(integrations_json["data"]["notes"]["last_sync_status"], "ok");
        assert_eq!(integrations_json["data"]["notes"]["last_item_count"], 1);

        let failing_app = build_app(
            storage.clone(),
            AppConfig {
                notes_path: Some(dir.to_string_lossy().to_string()),
                messaging_snapshot_path: Some(
                    dir.join("missing.json").to_string_lossy().to_string(),
                ),
                ..Default::default()
            },
            test_policy_config(),
            None,
            None,
        );
        let messaging_resp = failing_app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/sync/messaging")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(messaging_resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let integrations_resp = build_app(
            storage,
            AppConfig {
                notes_path: Some(dir.to_string_lossy().to_string()),
                messaging_snapshot_path: Some(
                    dir.join("missing.json").to_string_lossy().to_string(),
                ),
                ..Default::default()
            },
            test_policy_config(),
            None,
            None,
        )
        .oneshot(
            Request::builder()
                .uri("/api/integrations")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
        let integrations_body = axum::body::to_bytes(integrations_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let integrations_json: serde_json::Value =
            serde_json::from_slice(&integrations_body).unwrap();
        assert_eq!(
            integrations_json["data"]["messaging"]["last_sync_status"],
            "error"
        );
        assert!(integrations_json["data"]["messaging"]["last_error"]
            .as_str()
            .is_some());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn integrations_logs_endpoint_lists_recent_sync_history() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        crate::services::integrations::record_sync_success(&storage, "notes", 2)
            .await
            .unwrap();
        crate::services::integrations::record_sync_error(
            &storage,
            "notes",
            "notes snapshot missing",
        )
        .await
        .unwrap();
        let app = build_app(
            storage,
            AppConfig {
                notes_path: Some("/tmp/notes".to_string()),
                ..Default::default()
            },
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/integrations/notes/logs?limit=5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let entries = json["data"].as_array().expect("integration logs array");
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().any(|entry| {
            entry["integration_id"] == "notes"
                && entry["status"] == "error"
                && entry["message"]
                    .as_str()
                    .unwrap_or_default()
                    .contains("notes snapshot missing")
        }));
        assert!(entries
            .iter()
            .any(|entry| entry["status"] == "ok" && entry["payload"]["item_count"] == 2));
    }

    #[tokio::test]
    async fn integration_connections_endpoint_lists_canonical_connection_records() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let provider =
            vel_core::IntegrationProvider::new(vel_core::IntegrationFamily::Messaging, "signal")
                .unwrap();
        let connection_id = storage
            .insert_integration_connection(vel_storage::IntegrationConnectionInsert {
                family: vel_core::IntegrationFamily::Messaging,
                provider,
                status: vel_core::IntegrationConnectionStatus::Connected,
                display_name: "Signal personal".to_string(),
                account_ref: Some("+15555550123".to_string()),
                metadata_json: serde_json::json!({ "scope": "personal" }),
            })
            .await
            .unwrap();
        storage
            .upsert_integration_connection_setting_ref(
                connection_id.as_ref(),
                "messaging_snapshot_path",
                "/tmp/signal.json",
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/integrations/connections?family=messaging")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let connections = json["data"].as_array().expect("connection array");
        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0]["id"], connection_id.as_ref());
        assert_eq!(connections[0]["family"], "messaging");
        assert_eq!(connections[0]["provider_key"], "signal");
        assert_eq!(connections[0]["status"], "connected");
        assert_eq!(
            connections[0]["setting_refs"][0]["setting_key"],
            "messaging_snapshot_path"
        );
    }

    #[tokio::test]
    async fn integration_connection_detail_and_events_endpoints_return_foundation_data() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();

        let connection_id = storage
            .insert_integration_connection(vel_storage::IntegrationConnectionInsert {
                family: vel_core::IntegrationFamily::Calendar,
                provider: vel_core::IntegrationProvider::new(
                    vel_core::IntegrationFamily::Calendar,
                    "google",
                )
                .unwrap(),
                status: vel_core::IntegrationConnectionStatus::Connected,
                display_name: "Google workspace".to_string(),
                account_ref: Some("me@example.com".to_string()),
                metadata_json: serde_json::json!({ "workspace": true }),
            })
            .await
            .unwrap();
        storage
            .insert_integration_connection_event(
                connection_id.as_ref(),
                vel_core::IntegrationConnectionEventType::SyncStarted,
                &serde_json::json!({ "job": "manual" }),
                1_700_000_100,
            )
            .await
            .unwrap();
        storage
            .insert_integration_connection_event(
                connection_id.as_ref(),
                vel_core::IntegrationConnectionEventType::SyncSucceeded,
                &serde_json::json!({ "items": 42 }),
                1_700_000_200,
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let detail_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/api/integrations/connections/{}", connection_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(detail_response.status(), StatusCode::OK);
        let detail_body = axum::body::to_bytes(detail_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let detail_json: serde_json::Value = serde_json::from_slice(&detail_body).unwrap();
        assert_eq!(detail_json["data"]["provider_key"], "google");
        assert_eq!(detail_json["data"]["metadata"]["workspace"], true);

        let events_response = app
            .oneshot(
                Request::builder()
                    .uri(format!(
                        "/api/integrations/connections/{}/events?limit=5",
                        connection_id
                    ))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(events_response.status(), StatusCode::OK);
        let events_body = axum::body::to_bytes(events_response.into_body(), usize::MAX)
            .await
            .unwrap();
        let events_json: serde_json::Value = serde_json::from_slice(&events_body).unwrap();
        let events = events_json["data"].as_array().expect("event array");
        assert_eq!(events.len(), 2);
        assert_eq!(events[0]["event_type"], "sync_succeeded");
        assert_eq!(events[0]["payload"]["items"], 42);
        assert_eq!(events[1]["event_type"], "sync_started");
    }

    #[tokio::test]
    async fn list_components_returns_all_known_components() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/components")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let data = json["data"].as_array().expect("components array");
        assert_eq!(data.len(), 10);
        let ids: Vec<&str> = data
            .iter()
            .map(|entry| entry["id"].as_str().unwrap_or_default())
            .collect();
        assert!(ids.contains(&"google-calendar"));
        assert!(ids.contains(&"todoist"));
        assert!(ids.contains(&"activity"));
        assert!(ids.contains(&"health"));
        assert!(ids.contains(&"git"));
        assert!(ids.contains(&"messaging"));
        assert!(ids.contains(&"reminders"));
        assert!(ids.contains(&"notes"));
        assert!(ids.contains(&"transcripts"));
        assert!(ids.contains(&"evaluate"));
        assert_eq!(json["data"][0]["status"], "idle");
    }

    #[tokio::test]
    async fn restart_unknown_component_returns_404() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/components/does-not-exist/restart")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["ok"], false);
        assert_eq!(json["error"]["code"], "not_found");
    }

    #[tokio::test]
    async fn restart_evaluate_emits_status_and_logs() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let restart_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/components/evaluate/restart")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(restart_resp.status(), StatusCode::OK);
        let restart_body = axum::body::to_bytes(restart_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let restart_json: serde_json::Value = serde_json::from_slice(&restart_body).unwrap();
        assert_eq!(restart_json["ok"], true);
        assert_eq!(restart_json["data"]["id"], "evaluate");
        assert_eq!(restart_json["data"]["status"], "ok");
        assert_eq!(restart_json["data"]["restart_count"], 1);
        assert!(
            restart_json["data"]["last_restarted_at"]
                .as_i64()
                .unwrap_or(0)
                > 0
        );

        let logs_resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/components/evaluate/logs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(logs_resp.status(), StatusCode::OK);
        let logs_body = axum::body::to_bytes(logs_resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let logs_json: serde_json::Value = serde_json::from_slice(&logs_body).unwrap();
        let logs = logs_json["data"].as_array().expect("logs array");
        assert!(!logs.is_empty());
        assert!(logs
            .iter()
            .any(|entry| entry["event_name"] == "component.restart.requested"));
        assert!(logs
            .iter()
            .any(|entry| entry["event_name"] == "component.restart.completed"));
    }

    #[tokio::test]
    async fn restart_evaluate_emits_components_updated_websocket_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        let mut rx = broadcast_tx.subscribe();
        let state = crate::state::AppState::new(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        );
        let app = build_app_with_state(state);

        let restart_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/components/evaluate/restart")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(restart_resp.status(), StatusCode::OK);
        let envelope = rx
            .recv()
            .await
            .expect("websocket event should be broadcast");
        assert_eq!(envelope.event_type.to_string(), "components:updated");
        assert_eq!(envelope.payload["id"], "evaluate");
        assert_eq!(envelope.payload["status"], "ok");
        assert_eq!(envelope.payload["restart_count"], 1);

        let context_envelope = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
            .await
            .expect("context websocket event should be broadcast after evaluate restart")
            .expect("context websocket event should be readable");
        assert_eq!(context_envelope.event_type.to_string(), "context:updated");
        assert!(
            context_envelope.payload["computed_at"]
                .as_i64()
                .unwrap_or_default()
                > 0
        );
        assert!(context_envelope.payload["context"].is_object());
    }

    #[tokio::test]
    async fn evaluate_emits_context_updated_websocket_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        let mut rx = broadcast_tx.subscribe();
        let state = crate::state::AppState::new(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        );
        let app = build_app_with_state(state);

        let evaluate_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/v1/evaluate")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(evaluate_resp.status(), StatusCode::OK);

        let envelope = rx
            .recv()
            .await
            .expect("websocket event should be broadcast");
        assert_eq!(envelope.event_type.to_string(), "context:updated");
        assert!(envelope.payload["computed_at"].as_i64().unwrap_or_default() > 0);
        assert!(envelope.payload["context"].is_object());
    }

    #[tokio::test]
    async fn now_endpoint_returns_consolidated_snapshot() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: None,
                timestamp: now + 3600,
                payload_json: Some(serde_json::json!({
                    "title": "Design review",
                    "start": now + 3600,
                    "end": now + 5400,
                    "location": "Room 4B",
                    "prep_minutes": 15,
                    "travel_minutes": 0
                })),
            })
            .await
            .unwrap();
        storage
            .set_setting("timezone", &serde_json::json!("America/Denver"))
            .await
            .unwrap();
        let commitment_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Reply to Dimitri".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todo_1".to_string()),
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({})),
            })
            .await
            .unwrap();
        storage
            .set_current_context(
                now,
                &serde_json::json!({
                    "mode": "day_mode",
                    "morning_state": "engaged",
                    "meds_status": "pending",
                    "global_risk_level": "medium",
                    "global_risk_score": 0.72,
                    "attention_state": "on_task",
                    "drift_type": "none",
                    "drift_severity": "none",
                    "attention_confidence": 0.8,
                    "attention_reasons": ["recent git activity indicates active work"],
                    "git_activity_summary": {
                        "timestamp": now - 300,
                        "repo": "vel",
                        "branch": "main",
                        "operation": "commit"
                    },
                    "note_document_summary": {
                        "timestamp": now - 180,
                        "title": "Today",
                        "path": "daily/today.md"
                    },
                    "assistant_message_summary": {
                        "timestamp": now - 60,
                        "conversation_id": "conv_external",
                        "role": "assistant",
                        "source": "chatgpt"
                    },
                    "signals_used": ["sig_manual"],
                    "next_commitment_id": commitment_id.as_ref()
                })
                .to_string(),
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/now")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["data"]["summary"]["mode"]["label"], "Day");
        assert_eq!(json["data"]["timezone"], "America/Denver");
        assert_eq!(json["data"]["summary"]["risk"]["label"], "medium · 72%");
        assert_eq!(
            json["data"]["tasks"]["todoist"][0]["text"],
            "Reply to Dimitri"
        );
        assert_eq!(
            json["data"]["schedule"]["upcoming_events"][0]["title"],
            "Design review"
        );
        assert_eq!(
            json["data"]["sources"]["git_activity"]["label"],
            "Git activity"
        );
        assert_eq!(
            json["data"]["sources"]["git_activity"]["summary"]["repo"],
            "vel"
        );
        assert_eq!(
            json["data"]["sources"]["note_document"]["summary"]["path"],
            "daily/today.md"
        );
        assert_eq!(
            json["data"]["sources"]["assistant_message"]["summary"]["conversation_id"],
            "conv_external"
        );
        assert_eq!(json["data"]["freshness"]["sources"][0]["key"], "context");
    }

    #[tokio::test]
    async fn now_endpoint_prioritizes_urgent_todoist_tasks() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc();
        let due_today = now + time::Duration::hours(2);
        let overdue = now - time::Duration::hours(1);
        storage
            .set_setting("timezone", &serde_json::json!("America/Denver"))
            .await
            .unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Backlog cleanup".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todo_backlog".to_string()),
                status: vel_core::CommitmentStatus::Open,
                due_at: None,
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({ "priority": 1 })),
            })
            .await
            .unwrap();
        let overdue_id = storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Follow up with finance".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todo_overdue".to_string()),
                status: vel_core::CommitmentStatus::Open,
                due_at: Some(overdue),
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({
                    "priority": 2,
                    "has_due_time": true
                })),
            })
            .await
            .unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Take meds".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todo_meds".to_string()),
                status: vel_core::CommitmentStatus::Open,
                due_at: Some(due_today),
                project: None,
                commitment_kind: Some("medication".to_string()),
                metadata_json: Some(serde_json::json!({
                    "priority": 4,
                    "has_due_time": true
                })),
            })
            .await
            .unwrap();
        storage
            .insert_commitment(vel_storage::CommitmentInsert {
                text: "Prep interview packet".to_string(),
                source_type: "todoist".to_string(),
                source_id: Some("todo_due_today".to_string()),
                status: vel_core::CommitmentStatus::Open,
                due_at: Some(due_today),
                project: None,
                commitment_kind: Some("todo".to_string()),
                metadata_json: Some(serde_json::json!({
                    "priority": 1,
                    "has_due_time": true
                })),
            })
            .await
            .unwrap();
        storage
            .set_current_context(
                now.unix_timestamp(),
                &serde_json::json!({
                    "mode": "day_mode",
                    "morning_state": "engaged",
                    "meds_status": "pending",
                    "global_risk_level": "low",
                    "global_risk_score": 0.21,
                    "attention_state": "on_task",
                    "drift_type": "none",
                    "drift_severity": "none",
                    "attention_confidence": 0.9,
                    "next_commitment_id": overdue_id.as_ref()
                })
                .to_string(),
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/now")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let tasks = json["data"]["tasks"]["todoist"]
            .as_array()
            .expect("todoist tasks should be an array");
        let ordered_texts = tasks
            .iter()
            .map(|task| {
                task["text"]
                    .as_str()
                    .expect("task text should be present")
                    .to_string()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            ordered_texts,
            vec![
                "Take meds".to_string(),
                "Follow up with finance".to_string(),
                "Prep interview packet".to_string(),
                "Backlog cleanup".to_string(),
            ]
        );
        assert_eq!(
            json["data"]["tasks"]["next_commitment"]["text"],
            "Follow up with finance"
        );
        assert_eq!(json["data"]["timezone"], "America/Denver");
    }

    #[tokio::test]
    async fn now_endpoint_filters_out_old_calendar_events_and_reports_selection_empty_state() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        storage
            .set_setting(
                "integration_google_calendar",
                &serde_json::json!({
                    "client_id": "client",
                    "calendars": [
                        {
                            "id": "cal_1",
                            "summary": "Personal",
                            "primary": true,
                            "selected": false
                        }
                    ],
                    "all_calendars_selected": false,
                    "last_sync_at": now,
                    "last_sync_status": "ok",
                    "last_error": null,
                    "last_item_count": 1
                }),
            )
            .await
            .unwrap();
        storage
            .set_setting(
                "integration_google_calendar_secrets",
                &serde_json::json!({
                    "client_secret": "secret",
                    "access_token": "token",
                    "refresh_token": "refresh",
                    "token_expires_at": now + 3600
                }),
            )
            .await
            .unwrap();
        storage
            .insert_signal(vel_storage::SignalInsert {
                signal_type: "calendar_event".to_string(),
                source: "google_calendar".to_string(),
                source_ref: None,
                timestamp: now - 86_400,
                payload_json: Some(serde_json::json!({
                    "title": "Old retro",
                    "start": now - 86_400,
                    "end": now - 85_800
                })),
            })
            .await
            .unwrap();
        storage
            .set_current_context(
                now,
                &serde_json::json!({
                    "mode": "day_mode",
                    "morning_state": "engaged",
                    "meds_status": "done",
                    "global_risk_level": "low",
                    "global_risk_score": 0.1,
                    "attention_state": "on_task",
                    "drift_type": "none",
                    "drift_severity": "none",
                    "attention_confidence": 0.9,
                    "signals_used": []
                })
                .to_string(),
            )
            .await
            .unwrap();

        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/v1/now")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["data"]["schedule"]["upcoming_events"],
            serde_json::json!([])
        );
        assert_eq!(
            json["data"]["schedule"]["empty_message"],
            "No calendars are selected in Settings."
        );
        assert_eq!(
            json["data"]["freshness"]["sources"][1]["status"],
            "unchecked"
        );
    }

    #[tokio::test]
    async fn restart_evaluate_emits_context_updated_websocket_event() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let (broadcast_tx, _) = tokio::sync::broadcast::channel(64);
        let mut rx = broadcast_tx.subscribe();
        let state = crate::state::AppState::new(
            storage.clone(),
            AppConfig::default(),
            test_policy_config(),
            broadcast_tx,
            None,
            None,
        );
        let app = build_app_with_state(state);

        let restart_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/components/evaluate/restart")
                    .body(Body::from("{}".to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(restart_resp.status(), StatusCode::OK);

        let first = rx
            .recv()
            .await
            .expect("components websocket event should be broadcast");
        assert_eq!(first.event_type.to_string(), "components:updated");

        let second = rx
            .recv()
            .await
            .expect("context websocket event should be broadcast");
        assert_eq!(second.event_type.to_string(), "context:updated");
        assert!(second.payload["computed_at"].as_i64().unwrap_or_default() > 0);
        assert!(second.payload["context"].is_object());
    }

    #[tokio::test]
    async fn chat_intervention_snooze_404_for_nonexistent() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/interventions/intv_nonexistent/snooze")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"minutes":15}"#.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn ws_endpoint_responds_to_get() {
        let storage = Storage::connect(":memory:").await.unwrap();
        storage.migrate().await.unwrap();
        let app = build_app(
            storage,
            AppConfig::default(),
            test_policy_config(),
            None,
            None,
        );
        let resp = app
            .oneshot(Request::builder().uri("/ws").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert!(!resp.status().is_success());
        assert_ne!(resp.status(), StatusCode::NOT_FOUND);
    }
}
