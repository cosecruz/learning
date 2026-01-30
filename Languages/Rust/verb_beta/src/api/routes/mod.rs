use axum::{
    Json, Router,
    response::Html,
    routing::{delete, get, post, put},
};

use super::AppState;

/// Build the root application router.
///
/// This is the top-level router responsible for composing
/// all HTTP routes exposed by the application.
///
/// Routing structure:
///
/// ```text
/// /
/// └── /api
///     └── /v1
///         └── /verbs
/// ```
pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(root_routes())
        .nest("/api/v1", api_routes_v1())
        .with_state(state)
}

/// Root (non-versioned) routes.
///
/// These routes are typically used for:
/// - health checks
/// - landing pages
/// - temporary testing endpoints
fn root_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { Html("<h1>Welcome, Let’s Verb</h1>") }))
        .route("/health", get(|| async { Json("ok") }))
}

/// Version 1 API routes.
///
/// This function groups all `/api/v1/*` routes.
/// Versioning is handled at the routing level to allow
/// parallel evolution of APIs.
fn api_routes_v1() -> Router<AppState> {
    Router::new().nest("/verbs", verb_routes())
}

/// Routes related to `Verb` resources.
///
/// REST-style endpoints:
///
/// ```text
/// POST   /verbs          - Create a verb
/// GET    /verbs          - List all verbs
/// GET    /verbs/{id}      - View a single verb
/// PUT    /verbs/{id}      - Update verb state
/// DELETE /verbs/{id}      - Drop a verb
/// GET    /verbs/{id}/logs - View action log
/// ```
fn verb_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(super::handlers::create_verb))
        .route("/", get(super::handlers::list_verbs))
        .route("/{id}", get(super::handlers::get_verb))
        .route("/{id}", put(super::handlers::update_verb))
        .route("/{id}", delete(super::handlers::delete_verb))
        .route("/{id}/logs", get(super::handlers::verb_logs))
}

// ## Key Points

// 1. **Set state once** at the top-most router level with `.with_state()`
// 2. **Type annotations** on sub-routers: `Router<AppState>` tells Axum these routers expect state
// 3. **Extract state** in handlers with `State<Arc<AppState>>`
// 4. **AppState must derive Clone** for this pattern to work
// 5. **Don't call `.with_state()` multiple times** on nested routers

// The error you were getting was likely:
