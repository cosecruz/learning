use axum::{
    Json, Router,
    response::Html,
    routing::{delete, get, post, put},
};

use crate::{api::handlers, infra::db::Database};

use super::AppState;

/// Build the root application router
///
/// ## Type Parameter
/// The router is generic over `D: Database` to maintain type safety
/// throughout the application stack.
pub fn app<D: Database>(state: AppState<D>) -> Router {
    Router::new()
        .merge(root_routes())
        .nest("/api/v1", api_routes_v1())
        .with_state(state)
}

/// Root (non-versioned) routes
fn root_routes<D: Database>() -> Router<AppState<D>> {
    Router::new()
        .route("/", get(|| async { Html("<h1>Welcome, Let's Verb</h1>") }))
        .route("/health", get(|| async { Json("ok") }))
}

/// Version 1 API routes
fn api_routes_v1<D: Database>() -> Router<AppState<D>> {
    Router::new().nest("/verbs", verb_routes())
}

/// Verb resource routes
///
/// REST endpoints:
/// - POST   /verbs          → Create verb
/// - GET    /verbs          → List verbs
/// - GET    /verbs/{id}      → Get single verb
/// - PUT    /verbs/{id}      → Update verb state
/// - DELETE /verbs/{id}      → Drop verb
/// - GET    /verbs/{id}/logs → Get action logs
fn verb_routes<D: Database>() -> Router<AppState<D>> {
    Router::new()
        .route("/", post(handlers::create_verb::<D>))
        .route("/", get(handlers::list_verbs::<D>))
        .route("/{id}", get(handlers::get_verb::<D>))
        .route("/{id}/state", put(handlers::update_verb_state::<D>))
        .route("/{id}", delete(handlers::drop_verb::<D>))
        .route("/{id}/logs", get(handlers::get_verb_logs::<D>))
}

// ## Key Points

// 1. **Set state once** at the top-most router level with `.with_state()`
// 2. **Type annotations** on sub-routers: `Router<AppState>` tells Axum these routers expect state
// 3. **Extract state** in handlers with `State<Arc<AppState>>`
// 4. **AppState must derive Clone** for this pattern to work
// 5. **Don't call `.with_state()` multiple times** on nested routers

// The error you were getting was likely:
