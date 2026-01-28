//! Core Service Errors
//!

use thiserror::Error;

use super::VerbState;

/// Errors raised by domain rule violations.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("InvalidStateTransition '{from} -> {to}'")]
    InvalidStateTransition { from: VerbState, to: VerbState },

    #[error("InvalidReason")]
    InvalidReason,
}
