use axum::Router;
use axum::response::Html;
use axum::routing::{get, get_service};

/// Builds the API router for version 1 (`/v1`).
///
/// ## Responsibility
///
/// This router defines all HTTP routes related to the `Todo` resource.
/// It does **not**:
/// - Contain business logic
/// - Perform database access directly
///
/// Instead, it delegates to handler functions that call domain services.
///
/// ## Mounted At
///
/// ```text
/// /v1
/// ```
///
/// ## Example
///
/// ```text
/// GET https://api.verb.com/v1/todos
/// ```
pub(super) fn v1_router() -> Router {
    const V1_PATH: &str = "/v1/";
    let mut todo_path = V1_PATH.to_string();

    todo_path.push_str("todos");
    println!("path: {}", todo_path);
    Router::new()
        // path: '/todos'
        .route(
            &todo_path,
            get(|| async { Html("Hello <strong>Verb</strong>") }),
        )
}
