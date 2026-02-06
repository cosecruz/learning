// crates/core/src/domain/project_structure.rs
//! ProjectStructure - the output of template rendering.

use std::path::PathBuf;

use super::DomainError;
use super::common::FilePermissions;

// ============================================================================
// ProjectStructure
// ============================================================================

/// Fully resolved, ready-to-write project layout.
///
/// This is the output of template resolution + rendering.
/// Contains everything needed to write a project to disk.
#[derive(Debug, Clone)]
pub(crate) struct ProjectStructure {
    pub(crate) root: PathBuf,
    pub(crate) entries: Vec<FsEntry>,
}

impl ProjectStructure {
    /// Create a new project structure with a root path.
    pub(crate) fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            entries: Vec::new(),
        }
    }

    /// Add a file to the structure (mutable).
    pub(crate) fn add_file(
        &mut self,
        path: impl Into<PathBuf>,
        content: String,
        permissions: FilePermissions,
    ) {
        self.entries.push(FsEntry::File(FileToWrite {
            path: path.into(),
            content,
            permissions,
        }));
    }

    /// Add a directory to the structure (mutable).
    pub(crate) fn add_directory(&mut self, path: impl Into<PathBuf>, permissions: FilePermissions) {
        self.entries.push(FsEntry::Directory(DirectoryToCreate {
            path: path.into(),
            permissions,
        }));
    }

    /// Add a file to the structure (builder style).
    pub(crate) fn with_file(
        mut self,
        path: impl Into<PathBuf>,
        content: String,
        permissions: FilePermissions,
    ) -> Self {
        self.add_file(path, content, permissions);
        self
    }

    /// Add a directory to the structure (builder style).
    pub(crate) fn with_directory(
        mut self,
        path: impl Into<PathBuf>,
        permissions: FilePermissions,
    ) -> Self {
        self.add_directory(path, permissions);
        self
    }

    /// Validate the structure before writing.
    ///
    /// Checks:
    /// - No duplicate paths
    /// - No absolute paths (all paths should be relative to root)
    /// - No empty file content (warn, not error)
    pub(crate) fn validate(&self) -> Result<(), DomainError> {
        let mut seen_paths = std::collections::HashSet::new();

        for entry in &self.entries {
            let path = match entry {
                FsEntry::File(f) => &f.path,
                FsEntry::Directory(d) => &d.path,
            };

            // Check for duplicates
            if !seen_paths.insert(path) {
                return Err(DomainError::ProjectStructureError(format!(
                    "Duplicate path: {:?}",
                    path
                )));
            }

            // Check path is relative (not absolute)
            if path.is_absolute() {
                return Err(DomainError::ProjectStructureError(format!(
                    "Absolute path not allowed in project structure: {:?}",
                    path
                )));
            }
        }

        Ok(())
    }

    /// Get all files in this structure.
    pub(crate) fn files(&self) -> impl Iterator<Item = &FileToWrite> {
        self.entries.iter().filter_map(|e| match e {
            FsEntry::File(f) => Some(f),
            _ => None,
        })
    }

    /// Get all directories in this structure.
    pub(crate) fn directories(&self) -> impl Iterator<Item = &DirectoryToCreate> {
        self.entries.iter().filter_map(|e| match e {
            FsEntry::Directory(d) => Some(d),
            _ => None,
        })
    }

    /// Count total entries.
    pub(crate) fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Count files only.
    pub(crate) fn file_count(&self) -> usize {
        self.files().count()
    }

    /// Count directories only.
    pub(crate) fn directory_count(&self) -> usize {
        self.directories().count()
    }
}

// ============================================================================
// FsEntry
// ============================================================================

/// A filesystem entry (file or directory).
#[derive(Debug, Clone)]
pub(crate) enum FsEntry {
    File(FileToWrite),
    Directory(DirectoryToCreate),
}

// ============================================================================
// FileToWrite
// ============================================================================

/// A file to be written to disk.
#[derive(Debug, Clone)]
pub(crate) struct FileToWrite {
    pub path: PathBuf,
    pub content: String,
    pub permissions: FilePermissions,
}

impl FileToWrite {
    pub(crate) fn new(
        path: impl Into<PathBuf>,
        content: String,
        permissions: FilePermissions,
    ) -> Self {
        Self {
            path: path.into(),
            content,
            permissions,
        }
    }

    /// Check if this file is empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get file size in bytes.
    pub(crate) fn size(&self) -> usize {
        self.content.len()
    }
}

// ============================================================================
// DirectoryToCreate
// ============================================================================

/// A directory to be created on disk.
#[derive(Debug, Clone)]
pub(crate) struct DirectoryToCreate {
    pub path: PathBuf,
    pub permissions: FilePermissions,
}

impl DirectoryToCreate {
    pub(crate) fn new(path: impl Into<PathBuf>, permissions: FilePermissions) -> Self {
        Self {
            path: path.into(),
            permissions,
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
    fn project_structure_new() {
        let structure = ProjectStructure::new("/tmp/my-project");
        assert_eq!(structure.root, PathBuf::from("/tmp/my-project"));
        assert_eq!(structure.entries.len(), 0);
    }

    #[test]
    fn project_structure_add_file() {
        let mut structure = ProjectStructure::new("/tmp/test");
        structure.add_file(
            "main.rs",
            "fn main() {}".to_string(),
            FilePermissions::DEFAULT,
        );

        assert_eq!(structure.entries.len(), 1);
        assert!(matches!(structure.entries[0], FsEntry::File(_)));
    }

    #[test]
    fn project_structure_add_directory() {
        let mut structure = ProjectStructure::new("/tmp/test");
        structure.add_directory("src", FilePermissions::DEFAULT);

        assert_eq!(structure.entries.len(), 1);
        assert!(matches!(structure.entries[0], FsEntry::Directory(_)));
    }

    #[test]
    fn project_structure_builder_style() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", FilePermissions::DEFAULT)
            .with_file(
                "src/main.rs",
                "fn main() {}".to_string(),
                FilePermissions::DEFAULT,
            )
            .with_file(
                "Cargo.toml",
                "[package]".to_string(),
                FilePermissions::DEFAULT,
            );

        assert_eq!(structure.entries.len(), 3);
        assert_eq!(structure.file_count(), 2);
        assert_eq!(structure.directory_count(), 1);
    }

    #[test]
    fn project_structure_validate_success() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", FilePermissions::DEFAULT)
            .with_file("src/main.rs", "".to_string(), FilePermissions::DEFAULT);

        assert!(structure.validate().is_ok());
    }

    #[test]
    fn project_structure_validate_duplicate_fails() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_file("main.rs", "".to_string(), FilePermissions::DEFAULT)
            .with_file("main.rs", "".to_string(), FilePermissions::DEFAULT);

        let result = structure.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::ProjectStructureError(_)
        ));
    }

    #[test]
    fn project_structure_validate_absolute_path_fails() {
        let mut structure = ProjectStructure::new("/tmp/test");
        structure.add_file("/absolute/path", "".to_string(), FilePermissions::DEFAULT);

        let result = structure.validate();
        assert!(result.is_err());
    }

    #[test]
    fn project_structure_files_iterator() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", FilePermissions::DEFAULT)
            .with_file("main.rs", "".to_string(), FilePermissions::DEFAULT)
            .with_file("lib.rs", "".to_string(), FilePermissions::DEFAULT);

        let files: Vec<_> = structure.files().collect();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn project_structure_directories_iterator() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", FilePermissions::DEFAULT)
            .with_directory("tests", FilePermissions::DEFAULT)
            .with_file("main.rs", "".to_string(), FilePermissions::DEFAULT);

        let dirs: Vec<_> = structure.directories().collect();
        assert_eq!(dirs.len(), 2);
    }

    #[test]
    fn file_to_write_new() {
        let file = FileToWrite::new("test.txt", "content".to_string(), FilePermissions::DEFAULT);
        assert_eq!(file.path, PathBuf::from("test.txt"));
        assert_eq!(file.content, "content");
        assert!(!file.is_empty());
        assert_eq!(file.size(), 7);
    }

    #[test]
    fn file_to_write_empty() {
        let file = FileToWrite::new("empty.txt", String::new(), FilePermissions::DEFAULT);
        assert!(file.is_empty());
        assert_eq!(file.size(), 0);
    }

    #[test]
    fn directory_to_create_new() {
        let dir = DirectoryToCreate::new("src", FilePermissions::DEFAULT);
        assert_eq!(dir.path, PathBuf::from("src"));
    }

    #[test]
    fn project_structure_entry_count() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", FilePermissions::DEFAULT)
            .with_file("main.rs", "".to_string(), FilePermissions::DEFAULT)
            .with_file("lib.rs", "".to_string(), FilePermissions::DEFAULT);

        assert_eq!(structure.entry_count(), 3);
        assert_eq!(structure.file_count(), 2);
        assert_eq!(structure.directory_count(), 1);
    }
}
