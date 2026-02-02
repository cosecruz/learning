//! Data Transfer Objects for HTTP API
//!
//! DTOs define the contract between HTTP clients and the application layer.

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use validator::Validate;

use crate::domain::model::{Verb, VerbId, VerbState};

// ==================================================
// Request DTOs
// ==================================================

/// Request to create a new verb
#[derive(Debug, Deserialize, Validate)]
pub struct CreateVerbRequest {
    #[validate(length(min = 1, max = 200, message = "Title must be 1-200 characters"))]
    pub title: String,

    #[validate(length(max = 2000, message = "Description cannot exceed 2000 characters"))]
    pub description: Option<String>,
}

/// Request to update verb state
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStateRequest {
    pub state: VerbStateDTO,

    #[validate(length(max = 500, message = "Reason cannot exceed 500 characters"))]
    pub reason: Option<String>,
}

/// Query parameters for listing verbs
#[derive(Debug, Deserialize, Validate)]
pub struct ListVerbsQuery {
    pub state: Option<VerbStateDTO>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,

    pub offset: Option<u32>,
}

impl Default for ListVerbsQuery {
    fn default() -> Self {
        Self {
            state: None,
            limit: Some(50),
            offset: Some(0),
        }
    }
}

// ==================================================
// Response DTOs
// ==================================================

/// Verb state DTO (serializable enum)
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum VerbStateDTO {
    Captured,
    Active,
    Paused,
    Done,
    Dropped,
}

impl From<VerbState> for VerbStateDTO {
    fn from(state: VerbState) -> Self {
        match state {
            VerbState::Captured => VerbStateDTO::Captured,
            VerbState::Active => VerbStateDTO::Active,
            VerbState::Paused => VerbStateDTO::Paused,
            VerbState::Done => VerbStateDTO::Done,
            VerbState::Dropped => VerbStateDTO::Dropped,
        }
    }
}

impl From<VerbStateDTO> for VerbState {
    fn from(dto: VerbStateDTO) -> Self {
        match dto {
            VerbStateDTO::Captured => VerbState::Captured,
            VerbStateDTO::Active => VerbState::Active,
            VerbStateDTO::Paused => VerbState::Paused,
            VerbStateDTO::Done => VerbState::Done,
            VerbStateDTO::Dropped => VerbState::Dropped,
        }
    }
}

/// Verb response DTO
#[derive(Debug, Serialize)]
pub struct VerbResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub state: VerbStateDTO,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Verb> for VerbResponse {
    fn from(verb: Verb) -> Self {
        Self {
            id: verb.id().to_string(),
            title: verb.title().to_string(),
            description: verb.description().to_string(),
            state: verb.state().into(),
            created_at: verb.created_at().to_string(),
            updated_at: verb.updated_at().to_string(),
        }
    }
}

/// List of verbs response
#[derive(Debug, Serialize)]
pub struct ListVerbsResponse {
    pub verbs: Vec<VerbResponse>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

// ==================================================
// Standard API Response Envelope
// ==================================================

/// Standard API response wrapper
#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ApiResponse<T> {
    Ok { data: T, meta: Option<Value> },
    Error { error: ApiError },
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self::Ok { data, meta: None }
    }

    pub fn ok_with_meta(data: T, meta: Value) -> Self {
        Self::Ok {
            data,
            meta: Some(meta),
        }
    }

    pub fn error(code: ErrorCode, message: impl Into<String>) -> Self {
        Self::Error {
            error: ApiError {
                code,
                message: message.into(),
                details: None,
            },
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::Ok { data, meta } => {
                let body = json!({
                    "status": "ok",
                    "data": data,
                    "meta": meta
                });
                (StatusCode::OK, Json(body)).into_response()
            }
            ApiResponse::Error { error } => {
                let status = match error.code {
                    ErrorCode::ValidationError => StatusCode::BAD_REQUEST,
                    ErrorCode::NotFound => StatusCode::NOT_FOUND,
                    ErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
                    ErrorCode::Conflict => StatusCode::CONFLICT,
                    ErrorCode::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let body = json!({
                    "status": "error",
                    "error": error
                });

                (status, Json(body)).into_response()
            }
        }
    }
}

/// API error structure
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<Value>,
}

/// Error codes for API responses
#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    ValidationError,
    NotFound,
    Unauthorized,
    Conflict,
    InternalError,
}
