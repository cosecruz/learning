//! Core error types.
//!
//! This module defines the error hierarchy for the core library.
//! Errors are designed to be:
//! - **Cloneable** (for use in concurrent contexts)
//! - **Informative** (preserve context and error chains)
//! - **Actionable** (users can understand what went wrong)

use std::sync::Arc;
use thiserror::Error;

/// Top-level error type for the core crate.
///
/// This is the main error type returned by core library operations.
/// It wraps more specific error types from different modules.
///
/// ## Design Notes
///
/// - Uses `Arc<std::io::Error>` to preserve IO errors while supporting Clone
/// - All variants are cloneable for use in async/concurrent contexts
/// - Error chains are preserved for debugging
///
/// ## Example
///
/// ```rust,ignore
/// use scarff_core::{CoreError, CoreResult};
///
/// fn do_something() -> CoreResult<()> {
///     // Domain validation
///     validate_target()?;
///
///     // IO operations
///     write_files()?;
///
///     Ok(())
/// }
/// ```
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

    /// I/O errors (wrapped in Arc for Clone support)
    ///
    /// We use Arc because std::io::Error is not Clone, but we need
    /// CoreError to be Clone for use in concurrent contexts.
    #[error("I/O error: {0}")]
    Io(#[from] Arc<std::io::Error>),
}

// Manual From implementation for io::Error to wrap in Arc
impl From<std::io::Error> for CoreError {
    fn from(err: std::io::Error) -> Self {
        CoreError::Io(Arc::new(err))
    }
}

impl CoreError {
    /// Check if this is an IO error.
    pub fn is_io_error(&self) -> bool {
        matches!(self, CoreError::Io(_))
    }

    /// Check if this is a domain error.
    pub fn is_domain_error(&self) -> bool {
        matches!(self, CoreError::Domain(_))
    }

    /// Check if this is a template error.
    pub fn is_template_error(&self) -> bool {
        matches!(self, CoreError::Template(_))
    }

    /// Check if this is a scaffold error.
    pub fn is_scaffold_error(&self) -> bool {
        matches!(self, CoreError::Scaffold(_))
    }

    /// Get the underlying IO error, if this is an IO error.
    pub fn io_error(&self) -> Option<&std::io::Error> {
        match self {
            CoreError::Io(arc) => Some(arc.as_ref()),
            _ => None,
        }
    }

    /// Get the underlying domain error, if this is a domain error.
    pub fn domain_error(&self) -> Option<&crate::domain::DomainError> {
        match self {
            CoreError::Domain(err) => Some(err),
            _ => None,
        }
    }
}

/// Result type for core operations.
///
/// This is a type alias for `Result<T, CoreError>`.
/// Use this throughout the core crate for consistency.
///
/// ## Example
///
/// ```rust,ignore
/// fn validate(target: &Target) -> CoreResult<()> {
///     if !target.is_valid() {
///         return Err(CoreError::Domain(DomainError::InvalidTarget { .. }));
///     }
///     Ok(())
/// }
/// ```
pub type CoreResult<T> = Result<T, CoreError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_error_is_cloneable() {
        let err = CoreError::Io(Arc::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "test error",
        )));
        let _cloned = err.clone();
    }

    #[test]
    fn io_error_converts_to_core_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let core_err: CoreError = io_err.into();
        assert!(matches!(core_err, CoreError::Io(_)));
        assert!(core_err.is_io_error());
    }

    #[test]
    fn core_error_predicates() {
        let io_err = CoreError::Io(Arc::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "test",
        )));

        assert!(io_err.is_io_error());
        assert!(!io_err.is_domain_error());
        assert!(!io_err.is_template_error());
        assert!(!io_err.is_scaffold_error());
    }

    #[test]
    fn can_extract_io_error() {
        let original = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let core_err = CoreError::Io(Arc::new(original));

        let extracted = core_err.io_error().unwrap();
        assert_eq!(extracted.kind(), std::io::ErrorKind::PermissionDenied);
    }

    #[test]
    fn error_display() {
        let err = CoreError::Io(Arc::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        )));

        let display = format!("{}", err);
        assert!(display.contains("I/O error"));
        assert!(display.contains("file not found"));
    }
}
