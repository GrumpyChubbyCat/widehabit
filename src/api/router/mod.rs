pub mod auth;
use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    body::Body,
    extract::{FromRef, Request},
    response::Response,
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::{router::auth::{AUTH_TAG, auth_router}},
    config::AuthConfig,
    db::{DbPool, repo::UserRepository},
    service::user::UserService,
};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = AUTH_TAG, description = "Authorization API endpoints"),
    )
)]
struct WideApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub auth_config: AuthConfig,
    pub user_service: Arc<UserService>,
}

impl FromRef<AppState> for AuthConfig {
    fn from_ref(state: &AppState) -> Self {
        state.auth_config.clone()
    }
}

// Allows use State(user_service): State<Arc<UserService>>
impl FromRef<AppState> for Arc<UserService> {
    fn from_ref(state: &AppState) -> Self {
        state.user_service.clone()
    }
}

pub fn api_router(db_pool: DbPool, auth_config: AuthConfig) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|_request: &Request<Body>, _span: &Span| tracing::info!("request_started",))
        .on_response(|response: &Response, latency: Duration, _span: &Span| {
            let res_status = response.status();
            let latency_ms = latency.as_millis();
            let status = res_status.as_u16();

            if res_status.is_server_error() {
                tracing::error!(status, latency_ms, "server_error");
            } else if res_status.is_client_error() {
                tracing::warn!(status, latency_ms, "client_error");
            } else {
                tracing::info!(status, latency_ms, "request_success")
            }
        })
        .on_failure(
            |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                tracing::error!("app_error {}", error)
            },
        );

    let user_repo = UserRepository::new(db_pool);
    let user_service = Arc::new(UserService::new(user_repo, auth_config.clone()));

    let app_state = AppState {
        auth_config,
        user_service,
    };

    let user_router = auth_router();

    let (router, _api) = OpenApiRouter::with_openapi(WideApiDoc::openapi())
        .routes(routes!(root_get))
        .nest("/api/v1/auth", user_router)
        .layer(trace_layer)
        .with_state(app_state)
        .split_for_parts();

    #[cfg(debug_assertions)]
    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", _api));

    router
}

#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn root_get() -> &'static str {
    "Hi from widehabit"
}
