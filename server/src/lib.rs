use tokio::{net::TcpListener, signal};

use diesel_async::pooled_connection::{AsyncDieselConnectionManager, bb8};

use crate::{api::router::api_router, config::WideConfig, errors::StartError};

pub mod config;
pub mod db;
pub mod service;
pub mod model;
pub mod errors;
pub mod api;

pub async fn run(config: WideConfig) -> Result<(), StartError> {
    let db_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(config.database_url);
    let db_pool = bb8::Pool::builder()
        .max_size(config.database_pool)
        .build(db_config)
        .await?;

    let router = api_router(db_pool, config.auth_config);

    let host = config.listen_address;
    let port = config.listen_port;

    let address = format!("{host}:{port}");
    let listener = TcpListener::bind(address).await?;

    tracing::info!("Started widehabit server on {host}:{port}");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown())
        .await?;

    Ok(())
}

async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to register Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => (),
        _ = terminate => (),
    }

    tracing::info!("Termination signal received, starting shutdown...");
}
