use axum::response::Html;
use axum::routing::get;
use tracing::{info, warn};

use crate::config::telemetry;

mod config;
mod web;

#[allow(unused)]
#[tokio::main]
async fn main() {
    // -- Load Config
    // region: Config
    let config = config::Config::load().expect("Failed to load configurations");
    println!("Config: {config:?}");
    // endregion: Config

    // -- Initialize telemetry
    // region: telemetry
    telemetry::init_tracing(&config);

    // endregion: telemetry

    // region: LISTENER
    let addr = config.socket_addr();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind listener");

    info!("[VERB SERVER] listening on {addr}");
    // endregion: LISTENER

    // region: SERVICE ROUTERS
    let app = axum::Router::new()
        .route("/", get(|| async { Html("<h1>Hello Verb</h1>") }))
        .into_make_service();
    // endregion: SERVICE ROUTERS

    // spin up app routers with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await;
}

// shutdown_signal
async fn shutdown_signal() {
    // SIGINT (Ctrl+C) or similar
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    warn!("[VERB SERVER] received shutdown signal");
}
