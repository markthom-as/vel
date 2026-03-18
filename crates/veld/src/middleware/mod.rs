use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub(crate) const OPERATOR_AUTH_HEADER: &str = "x-vel-operator-token";
pub(crate) const WORKER_AUTH_HEADER: &str = "x-vel-worker-token";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RouteExposureClass {
    LocalPublic,
    OperatorAuthenticated,
    WorkerAuthenticated,
    FutureExternal,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct HttpExposurePolicy {
    pub(crate) operator_api_token: Option<String>,
    pub(crate) worker_api_token: Option<String>,
    pub(crate) strict_auth: bool,
}

impl HttpExposurePolicy {
    pub(crate) fn from_env() -> Self {
        Self {
            operator_api_token: std::env::var("VEL_OPERATOR_API_TOKEN").ok(),
            worker_api_token: std::env::var("VEL_WORKER_API_TOKEN").ok(),
            strict_auth: env_flag_enabled("VEL_STRICT_HTTP_AUTH"),
        }
    }
}

pub(crate) fn env_flag_enabled(name: &str) -> bool {
    matches!(
        std::env::var(name).ok().as_deref(),
        Some("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON")
    )
}

#[derive(Debug, Clone)]
pub(crate) struct ExposureGate {
    pub(crate) class: RouteExposureClass,
    pub(crate) policy: HttpExposurePolicy,
}

impl ExposureGate {
    pub(crate) fn new(class: RouteExposureClass, policy: HttpExposurePolicy) -> Self {
        Self { class, policy }
    }
}

pub(crate) fn extract_bearer_token(request: &Request<Body>) -> Option<&str> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

pub(crate) fn extract_header_token<'a>(
    request: &'a Request<Body>,
    header_name: &str,
) -> Option<&'a str> {
    request
        .headers()
        .get(header_name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

pub(crate) fn expected_token_for_class(
    class: RouteExposureClass,
    policy: &HttpExposurePolicy,
) -> Option<(&'static str, &str)> {
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

pub(crate) fn unauthorized_response(class: RouteExposureClass) -> Response {
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

pub(crate) async fn enforce_exposure_gate(
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
