use axum::{Json, extract::State};
use tracing::instrument;
use validator::Validate;

use crate::{
    api::{
        AppState,
        dto::{ApiResponse, CreateVerbRequest, ErrorCode, VerbResponse},
    },
    infra::db::Database,
};

/// Handler: Create a new verb
///
/// ## Flow
/// 1. Validate request DTO
/// 2. Call application facade
/// 3. Convert domain entity to response DTO
/// 4. Return HTTP response
#[instrument(skip(state), fields(title = %payload.title))]
pub async fn create_verb<D: Database>(
    State(state): State<AppState<D>>,
    Json(payload): Json<CreateVerbRequest>,
) -> ApiResponse<VerbResponse> {
    // Step 1: Validate input
    if let Err(validation_errors) = payload.validate() {
        return ApiResponse::error(
            ErrorCode::ValidationError,
            format!("Validation failed: {}", validation_errors),
        );
    }

    // Step 2: Call application layer
    let description = payload.description.unwrap_or_default();

    match state
        .verb_facade
        .create_verb(payload.title, description)
        .await
    {
        Ok(verb) => {
            tracing::info!(verb_id = %verb.id(), "Verb created successfully");
            ApiResponse::ok(VerbResponse::from(verb))
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to create verb");
            ApiResponse::error(ErrorCode::InternalError, e.to_string())
        }
    }
}
