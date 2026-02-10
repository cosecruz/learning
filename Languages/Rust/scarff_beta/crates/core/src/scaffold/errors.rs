//! Errors for scaffolding operations.

use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

/// Errors that can occur during scaffolding operations.
///
/// All variants are cloneable to support concurrent operations.
#[derive(Debug, Error, Clone)]
pub enum ScaffoldError {
    /// Target validation failed
    #[error("Invalid target configuration: {reason}")]
    InvalidTarget {
        reason: String,
        /// Source error message (stored as String for Clone)
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
        /// Wrapped in Arc to preserve error while supporting Clone
        io_error: Arc<std::io::Error>,
    },

    /// Project directory already exists
    #[error("Project directory already exists: {path}")]
    ProjectExists { path: PathBuf },

    /// Permission denied
    #[error("Permission denied: {path}")]
    PermissionDenied { path: PathBuf },

    /// Validation failed
    #[error("Validation failed: {reason}")]
    ValidationFailed { reason: String },
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
            io_error: Arc::new(source),
        }
    }

    /// Create a PermissionDenied error.
    pub fn permission_denied(path: impl Into<PathBuf>) -> Self {
        ScaffoldError::PermissionDenied { path: path.into() }
    }

    /// Create a ValidationFailed error.
    pub fn validation_failed(reason: impl Into<String>) -> Self {
        ScaffoldError::ValidationFailed {
            reason: reason.into(),
        }
    }

    /// Check if this is a filesystem write error.
    pub fn is_filesystem_error(&self) -> bool {
        matches!(self, ScaffoldError::FilesystemWrite { .. })
    }

    /// Check if this is a permission error.
    pub fn is_permission_error(&self) -> bool {
        matches!(self, ScaffoldError::PermissionDenied { .. })
    }

    /// Get the underlying IO error, if this is a filesystem error.
    pub fn io_error(&self) -> Option<&std::io::Error> {
        match self {
            ScaffoldError::FilesystemWrite { io_error, .. } => Some(io_error.as_ref()),
            _ => None,
        }
    }
}

// Implement From for common error types
impl From<std::io::Error> for ScaffoldError {
    fn from(err: std::io::Error) -> Self {
        ScaffoldError::FilesystemWrite {
            path: PathBuf::from("unknown"),
            reason: "I/O operation failed".to_string(),
            io_error: Arc::new(err),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

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
    fn scaffold_error_io_wrapped_is_cloneable() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let err = ScaffoldError::FilesystemWrite {
            path: PathBuf::from("/tmp/test"),
            reason: "failed".to_string(),
            io_error: Arc::new(io_err),
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
            assert_eq!(io_error.kind(), std::io::ErrorKind::PermissionDenied);
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
        assert!(scaffold_err.is_filesystem_error());
    }

    #[test]
    fn error_predicates() {
        let fs_err = ScaffoldError::filesystem_write(
            "/tmp/test",
            "failed",
            std::io::Error::new(std::io::ErrorKind::NotFound, "test"),
        );
        let perm_err = ScaffoldError::permission_denied("/tmp/test");

        assert!(fs_err.is_filesystem_error());
        assert!(!fs_err.is_permission_error());

        assert!(!perm_err.is_filesystem_error());
        assert!(perm_err.is_permission_error());
    }

    #[test]
    fn can_extract_io_error() {
        let original = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err = ScaffoldError::filesystem_write("/tmp/test", "failed", original);

        let extracted = err.io_error().unwrap();
        assert_eq!(extracted.kind(), std::io::ErrorKind::PermissionDenied);
    }

    #[test]
    fn permission_denied_error() {
        let err = ScaffoldError::permission_denied("/tmp/test");
        assert!(err.is_permission_error());

        if let ScaffoldError::PermissionDenied { path } = err {
            assert_eq!(path, PathBuf::from("/tmp/test"));
        } else {
            panic!("Expected PermissionDenied variant");
        }
    }

    #[test]
    fn validation_failed_error() {
        let err = ScaffoldError::validation_failed("invalid structure");

        if let ScaffoldError::ValidationFailed { reason } = err {
            assert_eq!(reason, "invalid structure");
        } else {
            panic!("Expected ValidationFailed variant");
        }
    }
}
