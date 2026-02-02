use tracing::{debug, error};

use crate::{bootstrap::bootstrap, config::telemetry, error::AppResult};

mod api;
mod application;
mod bootstrap;
mod config;
mod domain;
mod error;
mod infra;
mod server;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Load configuration
    let config = config::Config::load()?;

    // Initialize telemetry
    telemetry::init_tracing(&config);

    debug!(
        host = %config.host,
        port = config.port,
        environment = %config.environment,
        "Starting Verb server"
    );

    // Start server
    if let Err(e) = server::start_server(&config).await {
        error!(error = %e, "Server failed");
        std::process::exit(1);
    }

    Ok(())

    //start bootstrap - for testing and prototyping
    // if let Err(e) = bootstrap().await {
    //     error!(error = %e,
    //     "Bootstrap failed");
    //     std::process::exit(1)
    // }
}
