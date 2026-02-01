//! Core Service Errors
//!

use thiserror::Error;

use super::model::verb::VerbState;

/// Errors raised by domain rule violations.
#[derive(Debug, Error)]
pub enum CoreError {
    // #[error("InvalidStateTransition '{from} -> {to}'")]
    // InvalidStateTransition { from: VerbState, to: VerbState },

    // #[error("InvalidReason")]
    // EmptyReason,

    // #[error("InvalidTimeStamp")]
    // InvalidTimeStamp,

    // #[error("InvalidDescription")]
    // EmptyDescription,

    // Validation errors
    #[error("Title cannot be empty")]
    VerbEmptyTitle,

    #[error("Title cannot exceed 200 characters")]
    VerbTitleTooLong,

    #[error("Description cannot exceed 2000 characters")]
    VerbDescriptionTooLong,

    #[error("Reason cannot exceed 500 characters")]
    ReasonTooLong,

    // State machine errors
    #[error("Invalid state transition: {from:?} -> {to:?}")]
    InvalidTransition { from: VerbState, to: VerbState },

    // Not found
    #[error("Verb not found")]
    NotFound,

    // Infrastructure errors
    // #[error("Database error: {0}")]
    // Database(#[from] sqlx::Error),
    #[error("Invalid state value: {0}")]
    VerbInvalidState(String),
}
