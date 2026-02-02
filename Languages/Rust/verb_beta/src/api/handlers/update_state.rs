use axum::extract::{Json, Path, State};
use tracing::instrument;
use validator::Validate;

use crate::{
    api::{
        AppState,
        dto::{ApiResponse, ErrorCode, UpdateStateRequest, VerbResponse},
    },
    domain::model::VerbId,
    infra::db::Database,
};

/// Handler: Update verb state
#[instrument(skip(state), fields(verb_id = %id))]
pub async fn update_verb_state<D: Database>(
    Path(id): Path<String>,
    State(state): State<AppState<D>>,
    Json(payload): Json<UpdateStateRequest>,
) -> ApiResponse<VerbResponse> {
    // Validate
    if let Err(validation_errors) = payload.validate() {
        return ApiResponse::error(
            ErrorCode::ValidationError,
            format!("Validation failed: {}", validation_errors),
        );
    }

    // Parse verb ID
    let verb_id = match id.parse::<VerbId>() {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::error(ErrorCode::ValidationError, "Invalid verb ID format");
        }
    };

    // Convert DTO state to domain state
    let next_state = payload.state.into();

    match state
        .verb_facade
        .transition_verb(verb_id, next_state, payload.reason)
        .await
    {
        Ok(verb) => {
            tracing::info!(
                verb_id = %verb.id(),
                new_state = ?verb.state(),
                "Verb state updated"
            );
            ApiResponse::ok(VerbResponse::from(verb))
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to update verb state");
            ApiResponse::error(ErrorCode::InternalError, e.to_string())
        }
    }
}
