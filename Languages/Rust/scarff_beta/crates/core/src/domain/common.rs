// crates/core/src/domain/common.rs
//! Shared types used across domain models.

use std::path::{Path, PathBuf};

// ============================================================================
// FilePermissions
// ============================================================================

/// File permissions for generated files and directories.
///
/// Currently only tracks executable bit. Can be extended for full Unix permissions later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct FilePermissions {
    pub(crate) executable: bool,
}

impl FilePermissions {
    /// Default permissions: readable/writable, not executable
    pub(crate) const DEFAULT: Self = Self { executable: false };

    /// Executable permissions: for scripts and binaries
    pub(crate) const EXECUTABLE: Self = Self { executable: true };
}

impl Default for FilePermissions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// ============================================================================
// RelativePath
// ============================================================================

/// A path guaranteed to be relative (not absolute).
///
/// This wrapper prevents accidentally using absolute paths in templates
/// or project structures, which would break portability.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct RelativePath(PathBuf);

impl RelativePath {
    /// Create a relative path.
    ///
    /// # Panics
    ///
    /// Panics in debug builds if the path is absolute.
    pub(crate) fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        debug_assert!(
            !path.is_absolute(),
            "RelativePath cannot be absolute: {:?}",
            path
        );
        Self(path)
    }

    /// Try to create a relative path, returning error if absolute.
    ///
    /// This is the safe variant that doesn't panic.
    pub(crate) fn try_new(path: impl Into<PathBuf>) -> Result<Self, PathBuf> {
        let path = path.into();
        if path.is_absolute() {
            Err(path)
        } else {
            Ok(Self(path))
        }
    }

    /// Get the inner PathBuf as a reference.
    pub(crate) fn as_path(&self) -> &Path {
        &self.0
    }

    /// Convert into the inner PathBuf.
    pub(crate) fn into_path_buf(self) -> PathBuf {
        self.0
    }

    /// Join this relative path with another path component.
    pub(crate) fn join(&self, path: impl AsRef<Path>) -> Self {
        Self(self.0.join(path))
    }
}

impl AsRef<Path> for RelativePath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl From<&str> for RelativePath {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for RelativePath {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_permissions_defaults() {
        const { assert!(!FilePermissions::DEFAULT.executable) };
        const { assert!(FilePermissions::EXECUTABLE.executable) };
        assert_eq!(FilePermissions::default(), FilePermissions::DEFAULT);
    }

    #[test]
    fn relative_path_from_str() {
        let path = RelativePath::new("src/main.rs");
        assert_eq!(path.as_path(), Path::new("src/main.rs"));
    }

    #[test]
    fn relative_path_try_new_valid() {
        let result = RelativePath::try_new("src/lib.rs");
        assert!(result.is_ok());
    }

    #[test]
    fn relative_path_try_new_absolute_fails() {
        let result = RelativePath::try_new("/absolute/path");
        assert!(result.is_err());
    }

    #[test]
    fn relative_path_join() {
        let base = RelativePath::new("src");
        let joined = base.join("main.rs");
        assert_eq!(joined.as_path(), Path::new("src/main.rs"));
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn relative_path_panics_on_absolute_debug() {
        RelativePath::new("/absolute/path");
    }
}
