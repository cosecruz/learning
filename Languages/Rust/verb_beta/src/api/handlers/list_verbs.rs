use axum::{
    Json,
    extract::{Query, State},
};
use tracing::instrument;

use crate::{
    api::{
        AppState,
        dto::{ApiResponse, ErrorCode, ListVerbsQuery, ListVerbsResponse, VerbResponse},
    },
    domain::repository::VerbFilter,
    infra::db::Database,
};

/// Handler: List verbs with filtering
#[instrument(skip(state))]
pub async fn list_verbs<D: Database>(
    State(state): State<AppState<D>>,
    Query(query): Query<ListVerbsQuery>,
) -> ApiResponse<ListVerbsResponse> {
    // Convert DTO query to domain filter
    let filter = VerbFilter {
        state: query.state.map(Into::into),
        limit: query.limit.unwrap_or(50),
        offset: query.offset.unwrap_or(0),
    };

    let filter1 = filter.clone();

    match state.verb_facade.list_verbs(filter).await {
        Ok(verbs) => {
            let total = verbs.len() as u32;
            //FIXME: fix filter sharing from domain layer
            let filter = filter1;
            let response = ListVerbsResponse {
                verbs: verbs.into_iter().map(VerbResponse::from).collect(),
                total,
                limit: filter.limit,
                offset: filter.offset,
            };
            ApiResponse::ok(response)
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to list verbs");
            ApiResponse::error(ErrorCode::InternalError, e.to_string())
        }
    }
}
