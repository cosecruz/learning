use axum::extract::{Path, State};
use tracing::instrument;

use crate::{
    api::{
        AppState,
        dto::{ApiResponse, ErrorCode, VerbResponse},
    },
    domain::model::{VerbId, VerbState},
    infra::db::Database,
};

/// Handler: Drop a verb (transition to Dropped state)
#[instrument(skip(state), fields(verb_id = %id))]
pub async fn drop_verb<D: Database>(
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

    // Transition to Dropped state
    match state
        .verb_facade
        .transition_verb(
            verb_id,
            VerbState::Dropped,
            Some("Dropped via API".to_string()),
        )
        .await
    {
        Ok(verb) => {
            tracing::info!(verb_id = %verb.id(), "Verb dropped");
            ApiResponse::ok(VerbResponse::from(verb))
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to drop verb");
            ApiResponse::error(ErrorCode::InternalError, e.to_string())
        }
    }
}
