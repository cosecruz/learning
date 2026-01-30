//! # **Domain Error - VerbError**
//! COntains only domain specific errors: errors that can occur in the domain layer
//! TODO: List and explain all the errors;
use thiserror::Error;

use crate::domain::model::VerbState;

#[derive(Debug, Error)]
pub enum DomainError {
    // Validation errors
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Title cannot exceed 200 characters")]
    TitleTooLong,

    #[error("Description cannot exceed 2000 characters")]
    DescriptionTooLong,

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
    InvalidState(String),
}
