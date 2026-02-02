use axum::extract::{Path, State};
use tracing::instrument;

use crate::{
    api::{
        AppState,
        dto::{ApiResponse, ErrorCode, VerbResponse},
    },
    domain::model::VerbId,
    infra::db::Database,
};

/// Handler: Get a single verb by ID
#[instrument(skip(state), fields(verb_id = %id))]
pub async fn get_verb<D: Database>(
    Path(id): Path<String>,
    State(state): State<AppState<D>>,
) -> ApiResponse<VerbResponse> {
    // Parse verb ID
    let verb_id = match id.parse::<VerbId>() {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::error(ErrorCode::ValidationError, "Invalid verb ID format");
        }
    };

    match state.verb_facade.get_verb(verb_id).await {
        Ok(verb) => ApiResponse::ok(VerbResponse::from(verb)),
        Err(e) => {
            tracing::warn!(error = %e, "Verb not found");
            ApiResponse::error(ErrorCode::NotFound, "Verb not found")
        }
    }
}
