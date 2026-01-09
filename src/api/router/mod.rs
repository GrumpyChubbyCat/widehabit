pub mod auth;
use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    body::Body,
    extract::Request,
    response::Response,
    routing::get,
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

use crate::{
    api::router::auth::auth_router, config::AuthConfig, db::{DbPool, repo::UserRepository}, service::user::UserService
};

pub fn api_router(db_pool: DbPool, auth_config: AuthConfig) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        .on_request(|request: &Request<Body>, _span: &Span| {
            tracing::info!(
                "Started processing request method={} path={} version={:?}",
                request.method(),
                request.uri().path(),
                request.version()
            )
        })
        .on_response(|response: &Response, latency: Duration, _span: &Span| {
            tracing::info!(
                "Response status={} in {:?} secs",
                response.status(),
                latency.as_secs_f32()
            )
        })
        .on_failure(
            |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                tracing::error!("Error {}", error)
            },
        );

    let user_repo = UserRepository::new(db_pool);
    let user_service = Arc::new(UserService::new(
        user_repo, auth_config,
    ));
    let user_router = auth_router();

    Router::new()
        .route("/", get(root_get))
        .nest("/api/v1/auth", user_router)
        .layer(trace_layer)
        .with_state(user_service)
}

async fn root_get() -> &'static str {
    "Hi from widehabit"
}
