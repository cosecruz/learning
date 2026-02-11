//! Template-specific errors.

use thiserror::Error;

use crate::domain::{Target, TemplateId};

/// Errors that can occur during template operations.
#[derive(Debug, Error, Clone)]
pub enum TemplateError {
    /// Target validation failed
    #[error("Invalid target: {0}")]
    InvalidTarget(String),

    /// Template validation failed
    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    /// Template not found in store
    #[error("Template not found: {0}")]
    NotFound(TemplateId),

    /// Template already exists (duplicate ID)
    #[error("Template already exists: {0}")]
    AlreadyExists(TemplateId),

    /// Store lock error (for thread-safe stores)
    #[error("Store lock error")]
    LockError,

    /// No matching template for target
    #[error("No matching template for target: {target}")]
    NoMatch { target: String },

    /// Multiple templates match with same specificity (ambiguous)
    #[error("Ambiguous match: {count} templates match target {target} with equal specificity")]
    AmbiguousMatch { target: String, count: usize },

    /// UUID parsing failed
    #[error("Invalid UUID")]
    UuidParseError,

    /// Rendering error
    #[error("Rendering failed: {0}")]
    RenderingFailed(String),
}

impl TemplateError {
    /// Create a NoMatch error from a target.
    pub fn no_match(target: &Target) -> Self {
        Self::NoMatch {
            target: target.to_string(),
        }
    }

    /// Create an AmbiguousMatch error.
    pub fn ambiguous_match(target: &Target, count: usize) -> Self {
        Self::AmbiguousMatch {
            target: target.to_string(),
            count,
        }
    }
}
