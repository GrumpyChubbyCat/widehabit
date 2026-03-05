use std::str::FromStr;

use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use widehabit_server::{config::WideConfig, errors::StartError, run};

#[tokio::main]
async fn main() -> Result<(), StartError> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    // Trying to deserialize our config
    let config = WideConfig::new()?;

    // Setup log level for tracing
    let log_level = Level::from_str(&config.log_level)?;
    let filter_layer = EnvFilter::from(log_level.as_str());

    let registry = tracing_subscriber::registry().with(filter_layer);

    if config.json_log {
        registry
            .with(
                fmt::layer()
                    .json()
                    .with_timer(fmt::time::ChronoUtc::rfc_3339())
                    .with_current_span(true),
            )
            .init();
    } else {
        registry.with(fmt::layer().compact()).init();
        tracing::info!("Tracing initialized in COMPACT mode (debug)");
    }

    // Running the widehobby server application
    run(config).await
}
