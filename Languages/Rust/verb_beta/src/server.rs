use std::sync::Arc;

use anyhow::Context;
use tracing::{info, instrument, warn};

use crate::{
    api::{self, AppState},
    application::VerbFacade,
    config::Config,
    error::AppResult,
    infra::db::DatabaseBuilder,
};

/// Start the HTTP server
#[instrument(skip(cfg), fields(environment = %cfg.environment))]
pub async fn start_server(cfg: &Config) -> AppResult<()> {
    // Step 1: Build database
    info!("Building database...");
    let db = DatabaseBuilder::new()
        .in_memory() // Change to .sqlite("app.db") for persistence
        .build()
        .await
        .context("Failed to build database")?;

    // Step 2: Create application facade
    info!("Creating application facade...");
    let facade = VerbFacade::new(Arc::new(db));

    // Step 3: Create application state
    let state = AppState::new(facade);

    // Step 4: Bind listener
    let addr = cfg.bind_addr().context("Failed to resolve bind address")?;
    tracing::Span::current().record("addr", tracing::field::display(&addr));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind TCP listener")?;

    info!("Listening and ready to accept connections");

    // Step 5: Build router with state
    let app = api::app(state);

    // Step 6: Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Axum server exited unexpectedly")?;

    Ok(())
}

/// Gracefully shutdown server on signal
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    warn!("[VERB SERVER] Received shutdown signal");
}
