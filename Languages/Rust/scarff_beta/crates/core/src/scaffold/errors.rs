// crates/core/src/scaffold/errors.rs
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScaffoldError {
    /// Target validation failed
    #[error("Invalid target configuration: {reason}")]
    InvalidTarget {
        reason: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Template resolution failed
    #[error("Could not find suitable template for {target}")]
    TemplateResolution {
        target: String,
        suggestions: Vec<String>,
    },

    /// Rendering failed
    #[error("Template rendering failed: {reason}")]
    RenderingFailed { reason: String, template_id: String },

    /// Filesystem operation failed
    #[error("Failed to write to {path}: {reason}")]
    FilesystemWrite {
        path: PathBuf,
        reason: String,
        #[source]
        source: std::io::Error,
    },

    /// Project directory already exists
    #[error("Project directory already exists: {path}")]
    ProjectExists { path: PathBuf },
}

impl ScaffoldError {
    /// Create a helpful error with suggestions
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        if let ScaffoldError::TemplateResolution { suggestions: s, .. } = &mut self {
            *s = suggestions;
        }
        self
    }
}
