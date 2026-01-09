pub mod auth;
use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    body::Body,
    extract::{FromRef, Request},
    response::Response,
    routing::get,
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

use crate::{
    api::router::auth::auth_router,
    config::AuthConfig,
    db::{DbPool, repo::UserRepository},
    service::user::UserService,
};

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

    Router::new()
        .route("/", get(root_get))
        .nest("/api/v1/auth", user_router)
        .layer(trace_layer)
        .with_state(app_state)
}

async fn root_get() -> &'static str {
    "Hi from widehabit"
}
