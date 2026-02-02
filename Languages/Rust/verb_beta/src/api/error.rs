//! Error conversion for API layer
//!
//! Converts application errors to HTTP responses

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::application::ApplicationError;

use super::dto::{ApiError, ApiResponse, ErrorCode};

/// Convert ApplicationError to HTTP response
impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        let (code, message) = match self {
            ApplicationError::Domain(e) => (ErrorCode::ValidationError, e.to_string()),
            ApplicationError::NotFound => (ErrorCode::NotFound, "Resource not found".to_string()),
            ApplicationError::Transaction(e) => {
                tracing::error!(error = %e, "Transaction error");
                (ErrorCode::InternalError, "Transaction failed".to_string())
            }
            // ApplicationError::Infrastructure(e) => {
            //     tracing::error!(error = %e, "Infrastructure error");
            //     (
            //         ErrorCode::InternalError,
            //         "Internal server error".to_string(),
            //     )
            // }
            ApplicationError::Database(_) => todo!(),
        };

        ApiResponse::<()>::error(code, message).into_response()
    }
}
