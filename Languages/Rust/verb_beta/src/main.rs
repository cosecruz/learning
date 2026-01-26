pub(crate) use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;

mod config;

#[allow(unused)]
#[tokio::main]
async fn main() {
    //note: program for both dev, uat and prod in mind
    // TODO: setup tracing properly

    // region: Config
    let config = config::Config::load().expect("Failed to load configurations");
    println!("Config: {config:?}");
    // endregion: Config

    println!("[VERB SERVER] starting");

    // region: LISTENER
    let addr = config.socket_addr();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind listener");

    println!("[VERB SERVER] listening on {addr}");
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
    println!("[VERB SERVER] received shutdown signal");
}
