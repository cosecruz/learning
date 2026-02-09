//! Core error types.
//!
//! This module defines the error hierarchy for the core library.

use thiserror::Error;

/// Top-level error type for the core crate.
///
/// This is the main error type returned by core library operations.
/// It wraps more specific error types from different modules.
#[derive(Debug, Error, Clone)]
pub enum CoreError {
    /// Domain validation errors
    #[error("Domain error: {0}")]
    Domain(#[from] crate::domain::DomainError),

    /// Template errors (resolution, rendering, storage)
    #[error("Template error: {0}")]
    Template(#[from] crate::template::TemplateError),

    /// Scaffold orchestration errors
    #[error("Scaffold error: {0}")]
    Scaffold(#[from] crate::scaffold::errors::ScaffoldError),

    // I/O errors
    // #[error("I/O error: {0}")]
    // Io(#[from] std::io::Error),
    /// I/O errors (wrapped in a string for Clone support)
    #[error("I/O error: {0}")]
    Io(String),
}

// Manual From implementation for io::Error to convert to String
impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> Self {
        CoreError::Io(err.to_string())
    }
}

/// Result type for core operations.
///
/// Uses anyhow::Result for flexible error handling with context.
pub type CoreResult<T> = Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_error_is_cloneable() {
        let err = CoreError::Io("test error".to_string());
        let _cloned = err.clone();
    }

    #[test]
    fn io_error_converts_to_core_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let core_err: CoreError = io_err.into();
        assert!(matches!(core_err, CoreError::Io(_)));
    }
}
