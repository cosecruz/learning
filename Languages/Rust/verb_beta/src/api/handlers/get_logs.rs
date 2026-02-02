use axum::extract::{Path, Query, State};
use serde::Serialize;
use tracing::instrument;

use crate::{
    api::{
        AppState,
        dto::{ActionLogResponse, ApiResponse, ErrorCode, GetActionLogsResponse, GetLogsQuery},
    },
    domain::{
        model::{ActionLog, VerbId},
        repository::action_log_repo::{ActionLogFilter, ActionLogListResult},
    },
    infra::db::Database,
};

/// Handler: Get action logs for a verb
#[instrument(skip(state), fields(verb_id = %id))]
pub async fn get_verb_logs<D: Database>(
    Path(id): Path<String>,
    Query(query): Query<GetLogsQuery>,
    State(state): State<AppState<D>>,
) -> ApiResponse<GetActionLogsResponse> {
    // Convert DTO query to domain filter
    let filter = ActionLogFilter {
        state: query.state.map(Into::into),
        // FIXME: limit not working from query
        limit: query.limit.unwrap_or(50),
        offset: query.offset.unwrap_or(0),
    };

    // Parse verb ID
    let verb_id = match id.parse::<VerbId>() {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::error(ErrorCode::ValidationError, "Invalid verb ID format");
        }
    };

    // find verb by id
    match state.verb_facade.get_verb(verb_id).await {
        // if verb exists then get logs
        Ok(verb) => {
            match state
                .verb_facade
                .get_verb_action_logs(verb.id(), &filter)
                .await
            {
                Ok(logs) => {
                    let log_res = GetActionLogsResponse {
                        action_logs: logs
                            .action_logs
                            .into_iter()
                            .map(ActionLogResponse::from)
                            .collect(),
                        total: logs.total,
                        limit: filter.limit,
                        offset: filter.offset,
                    };

                    ApiResponse::ok(log_res)
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to get action logs");
                    ApiResponse::error(ErrorCode::InternalError, "Failed to retrieve logs")
                    //             }
                }
            }
        }
        // else: if it does not exists
        Err(e) => {
            tracing::error!(error=%e, "Failed to find verb");
            ApiResponse::error(ErrorCode::NotFound, "Verb not found")
        }
    }
}
