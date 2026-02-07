//! Errors for scaffolding operations.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during scaffolding operations.
#[derive(Debug, Error, Clone)]
pub enum ScaffoldError {
    /// Target validation failed
    #[error("Invalid target configuration: {reason}")]
    InvalidTarget {
        reason: String,
        // Removed Box<dyn Error> - store as String instead for Clone
        source_error: Option<String>,
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
        // Changed from std::io::Error to String for Clone
        io_error: String,
    },

    /// Project directory already exists
    #[error("Project directory already exists: {path}")]
    ProjectExists { path: PathBuf },
}

impl ScaffoldError {
    /// Create a helpful error with suggestions.
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        if let ScaffoldError::TemplateResolution { suggestions: s, .. } = &mut self {
            *s = suggestions;
        }
        self
    }

    /// Create an InvalidTarget error with a source error.
    pub fn invalid_target(
        reason: impl Into<String>,
        source: Option<impl std::error::Error>,
    ) -> Self {
        ScaffoldError::InvalidTarget {
            reason: reason.into(),
            source_error: source.map(|e| e.to_string()),
        }
    }

    /// Create a FilesystemWrite error from an io::Error.
    pub fn filesystem_write(
        path: impl Into<PathBuf>,
        reason: impl Into<String>,
        source: std::io::Error,
    ) -> Self {
        ScaffoldError::FilesystemWrite {
            path: path.into(),
            reason: reason.into(),
            io_error: source.to_string(),
        }
    }
}

// Implement From for common error types
impl From<std::io::Error> for ScaffoldError {
    fn from(err: std::io::Error) -> Self {
        ScaffoldError::FilesystemWrite {
            path: PathBuf::from("unknown"),
            reason: "I/O operation failed".to_string(),
            io_error: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_error_is_cloneable() {
        let err = ScaffoldError::ProjectExists {
            path: PathBuf::from("/tmp/test"),
        };
        let _cloned = err.clone();
    }

    #[test]
    fn with_suggestions_updates_suggestions() {
        let err = ScaffoldError::TemplateResolution {
            target: "rust-cli".to_string(),
            suggestions: vec![],
        };

        let updated = err.with_suggestions(vec!["template1".to_string(), "template2".to_string()]);

        if let ScaffoldError::TemplateResolution { suggestions, .. } = updated {
            assert_eq!(suggestions.len(), 2);
        } else {
            panic!("Expected TemplateResolution variant");
        }
    }

    #[test]
    fn invalid_target_with_source() {
        let err = ScaffoldError::invalid_target(
            "Test reason",
            Some(std::io::Error::new(std::io::ErrorKind::NotFound, "test")),
        );

        if let ScaffoldError::InvalidTarget {
            reason,
            source_error,
        } = err
        {
            assert_eq!(reason, "Test reason");
            assert!(source_error.is_some());
        } else {
            panic!("Expected InvalidTarget variant");
        }
    }

    #[test]
    fn filesystem_write_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
        let err = ScaffoldError::filesystem_write("/tmp/file", "Cannot write", io_err);

        if let ScaffoldError::FilesystemWrite {
            path,
            reason,
            io_error,
        } = err
        {
            assert_eq!(path, PathBuf::from("/tmp/file"));
            assert_eq!(reason, "Cannot write");
            assert!(io_error.contains("permission denied"));
        } else {
            panic!("Expected FilesystemWrite variant");
        }
    }

    #[test]
    fn from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let scaffold_err: ScaffoldError = io_err.into();

        assert!(matches!(
            scaffold_err,
            ScaffoldError::FilesystemWrite { .. }
        ));
    }
}
