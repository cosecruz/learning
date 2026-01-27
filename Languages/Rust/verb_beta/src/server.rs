use anyhow::Context;
use axum::response::Html;
use axum::routing::get;
use tracing::{info, instrument, warn};

use crate::{config::Config, errors::AppResult};

/// run server
#[instrument(skip(cfg), fields(
    environment = %cfg.environment
))]
pub async fn start_server(cfg: &Config) -> AppResult<()> {
    //Step 1: Resolve a bind
    // region: LISTENER
    let addr = cfg.bind_addr().context("failed to resolve bind address")?;

    tracing::Span::current().record("addr", tracing::field::display(&addr));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("failed to bind TCP listener")?;

    info!("listening and ready to accept connections");
    // endregion: LISTENER

    //Step 2: Router
    // region: SERVICE ROUTERS
    let app = axum::Router::new()
        .route("/", get(|| async { Html("<h1>Hello Verb</h1>") }))
        .into_make_service();
    // endregion: SERVICE ROUTERS

    //Step 3: Axum Server
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum server exited unexpectedly")?;
    Ok(())
}

/// gracefully shutdown server on signal
async fn shutdown_signal() {
    // SIGINT (Ctrl+C) or similar
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    warn!("[VERB SERVER] received shutdown signal");
}
