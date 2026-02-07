use std::path::Path;

use anyhow::Context;
use tracing::{debug, info, instrument, warn};

use crate::{
    CoreResult,
    domain::{FsEntry, ProjectStructure},
    scaffold::{errors::ScaffoldError, filesystem::Filesystem},
};

pub trait Writer {
    // this might be async because its I/O bound operation
    fn write(&self, structure: &ProjectStructure) -> CoreResult<()>;
}

// there can be different kind of writers but we focus on File Writers
// they write ProjectStructure into filesystem
// filesystems are OS dependent
// so this is gonna use abstract FileSystem that allows implementation FileWriter that os specific filesystems can use
//============================================================================
// FileWriter
// ============================================================================

/// Writes a `ProjectStructure` to the filesystem.
///
/// This struct coordinates the process of taking an in-memory project
/// structure and materializing it on disk, handling errors gracefully.
///
/// # Examples
///
/// ```rust,no_run
/// use scarff_core::{
///     scaffold::{FileWriter, filesystem::RealFilesystem},
///     domain::ProjectStructure,
/// };
///
/// let writer = FileWriter::new(Box::new(RealFilesystem));
/// let structure = ProjectStructure::new("/tmp/my-project");
/// // ... populate structure ...
///
/// writer.write(&structure)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct FileWriter {
    filesystem: Box<dyn Filesystem>,
}

impl FileWriter {
    /// Create a new file writer with the given filesystem implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::scaffold::{FileWriter, filesystem::RealFilesystem};
    ///
    /// let writer = FileWriter::new(Box::new(RealFilesystem));
    /// ```
    pub fn new(filesystem: Box<dyn Filesystem>) -> Self {
        Self { filesystem }
    }
}

impl Writer for FileWriter {
    /// Write a project structure to the filesystem.
    ///
    /// This method:
    /// 1. Validates the structure
    /// 2. Checks if the project directory already exists
    /// 3. Creates directories
    /// 4. Writes files
    /// 5. Sets permissions
    /// 6. Rolls back on error (best effort)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The project directory already exists
    /// - Filesystem operations fail
    /// - The structure is invalid
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use scarff_core::{
    /// #     scaffold::{FileWriter, filesystem::RealFilesystem},
    /// #     domain::ProjectStructure,
    /// # };
    /// let writer = FileWriter::new(Box::new(RealFilesystem));
    /// let structure = ProjectStructure::new("/tmp/my-project");
    /// writer.write(&structure)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[instrument(skip(self, structure), fields(root = %structure.root.display()))]
    fn write(&self, structure: &ProjectStructure) -> CoreResult<()> {
        info!("Starting file write operation");

        // 1. Validate structure
        structure.validate()?;

        // 2. Check if project directory exists
        if self.filesystem.exists(&structure.root) {
            return Err(ScaffoldError::ProjectExists {
                path: structure.root.clone(),
            })?;
        }

        // 3. Try to write everything, rolling back on error
        match self.write_all(structure) {
            Ok(()) => {
                info!(
                    files = structure.file_count(),
                    directories = structure.directory_count(),
                    "Successfully wrote all files and directories"
                );
                Ok(())
            }
            Err(e) => {
                warn!("Write operation failed, attempting rollback");
                self.rollback(&structure.root);
                Err(e).context("failed to write project structure to file system, rolled back")?
            }
        }
    }
}

impl FileWriter {
    /// Write all entries to the filesystem.
    ///
    /// This is the internal implementation that actually performs the writes.
    fn write_all(&self, structure: &ProjectStructure) -> CoreResult<()> {
        // Create root directory first
        self.filesystem
            .create_dir_all(&structure.root)
            .map_err(|e| ScaffoldError::FilesystemWrite {
                path: structure.root.clone(),
                reason: "Failed to create project root directory".to_string(),
                io_error: e.to_string(),
            })?;

        debug!("Created project root directory");

        // Process all entries
        for (idx, entry) in structure.entries.iter().enumerate() {
            debug!(
                progress = format!("{}/{}", idx + 1, structure.entries.len()),
                "Processing entry"
            );

            match entry {
                FsEntry::Directory(dir) => {
                    let full_path = structure.root.join(&dir.path);
                    self.write_directory(&full_path, dir.permissions)?;
                }
                FsEntry::File(file) => {
                    let full_path = structure.root.join(&file.path);
                    self.write_file(&full_path, &file.content, file.permissions)?;
                }
            }
        }

        Ok(())
    }

    /// Write a single directory.
    fn write_directory(
        &self,
        path: &Path,
        _permissions: crate::domain::FilePermissions,
    ) -> CoreResult<()> {
        debug!(path = %path.display(), "Creating directory");

        self.filesystem
            .create_dir_all(path)
            .map_err(|e| ScaffoldError::FilesystemWrite {
                path: path.to_path_buf(),
                reason: "Failed to create directory".to_string(),
                io_error: e.to_string(),
            })?;

        // Note: We don't set directory permissions in MVP
        // This could be added in the future

        Ok(())
    }

    /// Write a single file.
    fn write_file(
        &self,
        path: &Path,
        content: &str,
        permissions: crate::domain::FilePermissions,
    ) -> CoreResult<()> {
        debug!(
            path = %path.display(),
            size = content.len(),
            executable = permissions.executable,
            "Writing file"
        );

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            self.filesystem
                .create_dir_all(parent)
                .map_err(|e| ScaffoldError::FilesystemWrite {
                    path: parent.to_path_buf(),
                    reason: "Failed to create parent directory".to_string(),
                    io_error: e.to_string(),
                })?;
        }

        // Write file content
        self.filesystem
            .write_file(path, content)
            .map_err(|e| ScaffoldError::FilesystemWrite {
                path: path.to_path_buf(),
                reason: "Failed to write file content".to_string(),
                io_error: e.to_string(),
            })?;

        // Set permissions if needed
        if permissions.executable {
            self.filesystem
                .set_permissions(path, permissions)
                .map_err(|e| ScaffoldError::FilesystemWrite {
                    path: path.to_path_buf(),
                    reason: "Failed to set file permissions".to_string(),
                    io_error: e.to_string(),
                })?;
        }

        Ok(())
    }

    /// Attempt to rollback a failed write operation.
    ///
    /// This is a best-effort operation - it tries to clean up what was
    /// written, but doesn't guarantee complete cleanup.
    fn rollback(&self, root: &Path) {
        warn!(root = %root.display(), "Rolling back filesystem changes");

        if let Err(e) = self.filesystem.remove_dir_all(root) {
            warn!(
                error = %e,
                root = %root.display(),
                "Failed to rollback filesystem changes"
            );
        } else {
            info!("Successfully rolled back filesystem changes");
        }
    }
}

// ============================================================================
// Builder Pattern for Configuration (Future Extension)
// ============================================================================

/// Configuration for file writing operations.
///
/// Currently minimal, but can be extended with options like:
/// - Skip existing files vs error
/// - Dry-run mode
/// - Progress callbacks
/// - Custom rollback behavior
#[derive(Default, Debug, Clone)]
pub struct WriteConfig {
    /// Whether to overwrite existing files (default: false)
    pub overwrite: bool,

    /// Whether to perform a dry-run (don't actually write) (default: false)
    pub dry_run: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{domain::FilePermissions, scaffold::filesystem::MockFilesystem};

    fn create_simple_structure() -> ProjectStructure {
        ProjectStructure::new("/test-project")
            .with_directory("src", FilePermissions::DEFAULT)
            .with_file(
                "src/main.rs",
                "fn main() {}\n".to_string(),
                FilePermissions::DEFAULT,
            )
            .with_file(
                "Cargo.toml",
                "[package]\nname = \"test\"\n".to_string(),
                FilePermissions::DEFAULT,
            )
    }

    #[test]
    fn writer_writes_simple_structure() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone: Box<MockFilesystem> = fs.clone();
        let writer = FileWriter::new(fs);

        let structure = create_simple_structure();

        writer.write(&structure).expect("Write should succeed");

        // Verify files were created
        assert!(fs_clone.exists(Path::new("/test-project")));
        assert!(fs_clone.exists(Path::new("/test-project/src")));
        assert!(fs_clone.exists(Path::new("/test-project/src/main.rs")));
        assert!(fs_clone.exists(Path::new("/test-project/Cargo.toml")));

        // Verify content
        let main_content = fs_clone
            .read_file(Path::new("/test-project/src/main.rs"))
            .unwrap();
        assert_eq!(main_content, "fn main() {}\n");
    }

    #[test]
    fn writer_sets_executable_permissions() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
        let writer = FileWriter::new(fs);

        let structure = ProjectStructure::new("/test-project").with_file(
            "script.sh",
            "#!/bin/bash\necho 'test'\n".to_string(),
            FilePermissions::EXECUTABLE,
        );

        writer.write(&structure).expect("Write should succeed");

        // Verify executable flag is set
        assert!(
            fs_clone
                .is_executable(Path::new("/test-project/script.sh"))
                .unwrap()
        );
    }

    #[test]
    fn writer_creates_parent_directories() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
        let writer = FileWriter::new(fs);

        let structure = ProjectStructure::new("/test-project").with_file(
            "deeply/nested/path/file.txt",
            "content".to_string(),
            FilePermissions::DEFAULT,
        );

        writer.write(&structure).expect("Write should succeed");

        // Verify parent directories were created
        assert!(fs_clone.exists(Path::new("/test-project/deeply")));
        assert!(fs_clone.exists(Path::new("/test-project/deeply/nested")));
        assert!(fs_clone.exists(Path::new("/test-project/deeply/nested/path")));
        assert!(fs_clone.exists(Path::new("/test-project/deeply/nested/path/file.txt")));
    }

    // #[test]
    // #[should_panic]
    // fn writer_fails_if_project_exists() {
    //     let fs = Box::new(MockFilesystem::new());
    //     let writer = FileWriter::new(fs.clone());

    //     // Create the directory first
    //     fs.create_dir_all(Path::new("/test-project")).unwrap();

    //     let structure = create_simple_structure();

    //     let result = writer.write(&structure);

    //     assert!(result.is_err());
    //     let err = result;
    //     assert!(matches!(err, crate::CoreError::Scaffold(_)));
    // }

    #[test]
    fn writer_rolls_back_on_error() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
        let writer = FileWriter::new(fs);

        // Create a structure where writing will fail partway through
        // (In MockFilesystem, writing to a non-existent parent fails)
        let mut structure = ProjectStructure::new("/test-project");
        structure.add_file(
            "src/main.rs",
            "content".to_string(),
            FilePermissions::DEFAULT,
        );

        // This should trigger rollback because parent dir doesn't exist
        let result = writer.write(&structure);

        // Write should fail
        assert!(result.is_err());

        // Project directory should be cleaned up (rolled back)
        assert!(!fs_clone.exists(Path::new("/test-project")));
    }

    #[test]
    fn writer_handles_empty_files() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
        let writer = FileWriter::new(fs);

        let structure = ProjectStructure::new("/test-project").with_file(
            ".gitkeep",
            String::new(),
            FilePermissions::DEFAULT,
        );

        writer.write(&structure).expect("Write should succeed");

        let content = fs_clone
            .read_file(Path::new("/test-project/.gitkeep"))
            .unwrap();
        assert_eq!(content, "");
    }

    #[test]
    fn writer_handles_large_files() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
        let writer = FileWriter::new(fs);

        // Create a large file (1MB of 'a' characters)
        let large_content = "a".repeat(1_000_000);

        let structure = ProjectStructure::new("/test-project").with_file(
            "large.txt",
            large_content.clone(),
            FilePermissions::DEFAULT,
        );

        writer.write(&structure).expect("Write should succeed");

        let content = fs_clone
            .read_file(Path::new("/test-project/large.txt"))
            .unwrap();
        assert_eq!(content.len(), 1_000_000);
    }

    #[test]
    fn writer_handles_unicode_content() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
        let writer = FileWriter::new(fs);

        let unicode_content = "Hello 世界! 🦀 Rust is awesome! مرحبا";

        let structure = ProjectStructure::new("/test-project").with_file(
            "unicode.txt",
            unicode_content.to_string(),
            FilePermissions::DEFAULT,
        );

        writer.write(&structure).expect("Write should succeed");

        let content = fs_clone
            .read_file(Path::new("/test-project/unicode.txt"))
            .unwrap();
        assert_eq!(content, unicode_content);
    }

    #[test]
    fn writer_handles_complex_directory_structure() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
        let writer = FileWriter::new(fs);

        let structure = ProjectStructure::new("/test-project")
            .with_directory("src", FilePermissions::DEFAULT)
            .with_directory("src/domain", FilePermissions::DEFAULT)
            .with_directory("src/application", FilePermissions::DEFAULT)
            .with_directory("tests", FilePermissions::DEFAULT)
            .with_directory("docs", FilePermissions::DEFAULT)
            .with_file("src/main.rs", "".to_string(), FilePermissions::DEFAULT)
            .with_file(
                "src/domain/mod.rs",
                "".to_string(),
                FilePermissions::DEFAULT,
            )
            .with_file(
                "src/application/mod.rs",
                "".to_string(),
                FilePermissions::DEFAULT,
            )
            .with_file("Cargo.toml", "".to_string(), FilePermissions::DEFAULT)
            .with_file("README.md", "".to_string(), FilePermissions::DEFAULT);

        writer.write(&structure).expect("Write should succeed");

        // Verify all directories exist
        assert_eq!(fs_clone.directory_count(), 6); // project root + 5 subdirs

        // Verify all files exist
        assert_eq!(fs_clone.file_count(), 5);
    }
}
