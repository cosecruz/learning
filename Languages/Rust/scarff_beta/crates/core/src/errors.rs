// crates/core/src/errors.rs
use std::path::PathBuf;
use thiserror::Error;

/// Top-level error type for the core crate
#[derive(Debug, Error)]
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

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for core operations
pub type CoreResult<T> = anyhow::Result<T>;
