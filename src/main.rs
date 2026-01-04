use std::str::FromStr;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use widehobby::{config::WideConfig, errors::WideAppError, run};

#[tokio::main]
async fn main() -> Result<(), WideAppError> {
    // This returns an error if the `.env` file doesn't exist, but that's not what we want
    // since we're not going to use a `.env` file if we deploy this application.
    dotenv::dotenv().ok();

    // Trying to deserialize our config
    let config = WideConfig::new()?;
    
    // Setup log level for tracing
    let log_level = Level::from_str(&config.log_level)?;

    //Setting up our tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .compact()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    // Running the widehobby server application
    run(config).await
}
