use thiserror::Error;

use crate::domain::{Target, TemplateId};

#[derive(Debug, Error)]
pub enum TemplateError {
    ///
    /// Shared errors
    ///
    /// InvalidTemplate
    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    // Store Error
    #[error("Template not found: {0}")]
    NotFound(TemplateId),

    #[error("Template already exists: {0}")]
    AlreadyExists(TemplateId),

    #[error("Store lock error")]
    LockError,

    // Renderer Error
    /// Resolver Error
    #[error("Resolve error: no match for target: {target:?}")]
    NoMatch { target: Target },
    // AmbiguousMatch { target: Target, count: usize },
}
