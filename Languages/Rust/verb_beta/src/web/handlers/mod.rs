use axum::{Json, extract::State};

use super::AppState;

///Handlers for routes
///
pub async fn create_verb(
    State(_state): State<AppState>, // ✅ Use AppState, not Arc<AppState>
) -> Json<&'static str> {
    Json("create verb")
}

pub async fn list_verbs(State(_state): State<AppState>) -> Json<&'static str> {
    Json("list all verbs")
}

pub async fn get_verb(State(_state): State<AppState>) -> Json<&'static str> {
    Json("get single verb")
}

pub async fn update_verb(State(_state): State<AppState>) -> Json<&'static str> {
    Json("update verb")
}

pub async fn delete_verb(State(_state): State<AppState>) -> Json<&'static str> {
    Json("delete verb")
}

pub async fn verb_logs(State(_state): State<AppState>) -> Json<&'static str> {
    Json("verb logs")
}
