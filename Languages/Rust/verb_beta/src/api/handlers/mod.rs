use axum::{
    Json,
    extract::{Path, State},
};

use super::AppState;

mod create_verb;
pub(super) use create_verb::create_verb;

pub async fn list_verbs(State(_state): State<AppState>) -> Json<&'static str> {
    Json("list all verbs")
}

pub async fn get_verb(Path(id): Path<u64>, State(_state): State<AppState>) -> Json<&'static str> {
    Json("get single verb")
}

pub async fn update_verb(
    Path(id): Path<u64>,
    State(_state): State<AppState>,
) -> Json<&'static str> {
    Json("update verb")
}

pub async fn delete_verb(
    Path(id): Path<u64>,
    State(_state): State<AppState>,
) -> Json<&'static str> {
    Json("delete verb")
}

pub async fn verb_logs(Path(id): Path<u64>, State(_state): State<AppState>) -> Json<&'static str> {
    Json("verb logs")
}

// Built-in extractors:
// Extractor            What It Extracts
// Path<T>              Path parameters
// Query<T>             Query string
// Json<T>              JSON body
// Form<T>              Form data
// State<T>             Shared application state
// Extension<T>         Request extensions
// headers::HeaderMap   HTTP headers
// String               Raw body as string
// Bytes                Raw body as bytes
