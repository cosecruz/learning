//! # **Error - Domain Layer**
//! Contains only domain specific errors: errors that can occur in the domain layer
//!
use thiserror::Error;

use crate::domain::model::VerbState;

/// Domain-level errors represent business rule violations.
/// These have no knowledge of infrastructure (DB, HTTP, etc).
#[derive(Debug, Error)]
pub enum DomainError {
    //==================Model Errors======================
    //
    // Verb Specific Errors
    // Validation errors
    //
    ///Verb: invalid id conversion from str
    #[error("String not a valid Id")]
    VerbInvalidIdFromStr(#[from] uuid::Error),

    ///Verb: empty title
    #[error("Title cannot be empty")]
    VerbEmptyTitle,

    ///Verb: Title should be <=200
    #[error("Title cannot exceed 200 characters")]
    VerbTitleTooLong,

    ///Verb: description should be <=2000
    #[error("Description cannot exceed 2000 characters")]
    VerbDescriptionTooLong,

    ///Verb: Invalid state
    #[error("Invalid state value: {0}")]
    VerbInvalidState(String),

    // Verb: State machine errors
    ///Verb: State Transition Invalid
    #[error("Invalid state transition: {from:?} -> {to:?}")]
    InvalidTransition { from: VerbState, to: VerbState },

    // ActionLog State Specific Errors
    //
    ///Action_Log: reason given for paused and dropped states should be <=500
    #[error("Reason cannot exceed 500 characters")]
    ReasonTooLong,
}
