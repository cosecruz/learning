use tracing::{debug, error};

use crate::{config::telemetry, error::AppResult};

mod api;
mod config;
mod domain;
mod error;
mod infra;
mod server;

#[tokio::main]
async fn main() -> AppResult<()> {
    // -- Load Config
    // region: Config
    let config = config::Config::load()?;
    // endregion: Config

    // -- Initialize telemetry
    // region: telemetry
    telemetry::init_tracing(&config);

    // Log startup info with structured fields
    debug!(
        host = %config.host,
        port = config.port,
        environment = %config.environment,
        "Starting Verb server"
    );

    // endregion: telemetry

    // start server
    if let Err(e) = server::start_server(&config).await {
        error!(
            error = %e,
            "Server failed"
        );
        std::process::exit(1);
    }

    Ok(())
}
