use axum::extract::{Path, State};
use serde::Serialize;
use tracing::instrument;

use crate::{
    api::{
        AppState,
        dto::{ApiResponse, ErrorCode},
    },
    domain::model::{ActionLog, VerbId},
    infra::db::Database,
};

#[derive(Debug, Serialize)]
pub struct ActionLogResponse {
    pub id: String,
    pub verb_id: String,
    pub action_type: String,
    pub from_state: Option<String>,
    pub to_state: String,
    pub reason: Option<String>,
    pub timestamp: String,
}

impl From<ActionLog> for ActionLogResponse {
    fn from(log: ActionLog) -> Self {
        Self {
            id: log.id().to_string(),
            verb_id: log.verb_id().to_string(),
            action_type: log.action_type().as_str().to_string(),
            from_state: log.from_state().map(|s| s.as_str().to_string()),
            to_state: log.to_state().as_str().to_string(),
            reason: log.reason().map(|s| s.to_string()),
            timestamp: log.timestamp().to_string(),
        }
    }
}

/// Handler: Get action logs for a verb
#[instrument(skip(state), fields(verb_id = %id))]
pub async fn get_verb_logs<D: Database>(
    Path(id): Path<String>,
    State(state): State<AppState<D>>,
) -> ApiResponse<Vec<ActionLogResponse>> {
    // Parse verb ID
    let verb_id = match id.parse::<VerbId>() {
        Ok(id) => id,
        Err(_) => {
            return ApiResponse::error(ErrorCode::ValidationError, "Invalid verb ID format");
        }
    };

    match state.verb_facade.get_verb_action_logs(verb_id, None).await {
        Ok(logs) => {
            let log_res = logs
                .action_logs
                .into_iter()
                .map(ActionLogResponse::from)
                .collect();

            ApiResponse::ok(log_res)
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to get action logs");
            ApiResponse::error(ErrorCode::InternalError, "Failed to retrieve logs")
            //             }
        }
    }

    // Get transaction and repository
    // match state.verb_facade.db.begin_tx().await {
    //     Ok(tx) => {
    //         let log_repo = tx.action_log_repository();

    //         match log_repo.find_by_verb(verb_id, 100).await {
    //             Ok(logs) => {
    //                 let responses: Vec<ActionLogResponse> =
    //                     logs.into_iter().map(ActionLogResponse::from).collect();
    //                 ApiResponse::ok(responses)
    //             }
    //             Err(e) => {
    //                 tracing::error!(error = %e, "Failed to get action logs");
    //                 ApiResponse::error(ErrorCode::InternalError, "Failed to retrieve logs")
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         tracing::error!(error = %e, "Failed to begin transaction");
    //         ApiResponse::error(ErrorCode::InternalError, "Database error")
    //     }
    // }
}
