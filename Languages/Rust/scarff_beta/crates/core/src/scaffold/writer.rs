//! File writing operations for scaffolding.

use std::path::Path;
use tracing::{debug, info, instrument, warn};

use crate::{
    domain::{FsEntry, Permissions, ProjectStructure},
    errors::CoreResult,
    scaffold::{errors::ScaffoldError, filesystem::Filesystem},
};

/// Trait for writing project structures to storage.
pub trait Writer {
    /// Write a project structure to its destination.
    fn write(&self, structure: &ProjectStructure) -> CoreResult<()>;
}

// ============================================================================
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
    /// #     scaffold::{FileWriter, Writer, filesystem::RealFilesystem},
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
            }
            .into());
        }

        // 3. FIXME: Try to write everything, rolling back on error
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
                Err(e)
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
                io_error: std::sync::Arc::new(e),
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
    fn write_directory(&self, path: &Path, _permissions: Permissions) -> CoreResult<()> {
        debug!(path = %path.display(), "Creating directory");

        self.filesystem
            .create_dir_all(path)
            .map_err(|e| ScaffoldError::FilesystemWrite {
                path: path.to_path_buf(),
                reason: "Failed to create directory".to_string(),
                io_error: std::sync::Arc::new(e),
            })?;

        // Note: We don't set directory permissions in MVP
        // This could be added in the future

        Ok(())
    }

    /// Write a single file.
    fn write_file(&self, path: &Path, content: &str, permissions: Permissions) -> CoreResult<()> {
        debug!(
            path = %path.display(),
            size = content.len(),
            executable = permissions.executable_flag(),
            "Writing file"
        );

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            self.filesystem
                .create_dir_all(parent)
                .map_err(|e| ScaffoldError::FilesystemWrite {
                    path: parent.to_path_buf(),
                    reason: "Failed to create parent directory".to_string(),
                    io_error: std::sync::Arc::new(e),
                })?;
        }

        // Write file content
        self.filesystem
            .write_file(path, content)
            .map_err(|e| ScaffoldError::FilesystemWrite {
                path: path.to_path_buf(),
                reason: "Failed to write file content".to_string(),
                io_error: std::sync::Arc::new(e),
            })?;

        // Set permissions if needed
        if permissions.executable_flag() {
            self.filesystem
                .set_permissions(path, permissions)
                .map_err(|e| ScaffoldError::FilesystemWrite {
                    path: path.to_path_buf(),
                    reason: "Failed to set file permissions".to_string(),
                    io_error: std::sync::Arc::new(e),
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
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{domain::Permissions, scaffold::filesystem::MockFilesystem};
    use std::path::PathBuf;

    fn create_simple_structure() -> ProjectStructure {
        ProjectStructure::new("/test-project")
            .with_directory("src", Permissions::read_write())
            .with_file(
                "src/main.rs",
                "fn main() {}\n".to_string(),
                Permissions::read_write(),
            )
            .with_file(
                "Cargo.toml",
                "[package]\nname = \"test\"\n".to_string(),
                Permissions::read_write(),
            )
    }

    #[test]
    fn writer_writes_simple_structure() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
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
            Permissions::executable(),
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
            Permissions::read_write(),
        );

        writer.write(&structure).expect("Write should succeed");

        // Verify parent directories were created
        assert!(fs_clone.exists(Path::new("/test-project/deeply")));
        assert!(fs_clone.exists(Path::new("/test-project/deeply/nested")));
        assert!(fs_clone.exists(Path::new("/test-project/deeply/nested/path")));
        assert!(fs_clone.exists(Path::new("/test-project/deeply/nested/path/file.txt")));
    }

    #[test]
    fn writer_fails_if_project_exists() {
        let fs = Box::new(MockFilesystem::new());
        let writer = FileWriter::new(fs.clone());

        // Create the directory first
        fs.create_dir_all(Path::new("/test-project")).unwrap();

        let structure = create_simple_structure();

        let result = writer.write(&structure);

        assert!(result.is_err());
        // assert!(matches!(
        //     result.unwrap_err().scaffold_error(),
        //     Some(ScaffoldError::ProjectExists { .. })
        // ));
    }

    #[test]
    fn writer_rolls_back_on_error() {
        let fs = Box::new(MockFilesystem::new());
        let fs_clone = fs.clone();
        let writer = FileWriter::new(fs);

        // Create a structure where writing will fail partway through
        let mut structure = ProjectStructure::new("/");
        structure.add_file(
            "src/main.rs",
            "content".to_string(),
            Permissions::read_write(),
        );

        println!("{structure:?}");

        // This should trigger rollback because parent dir doesn't exist? i am not sure this fails
        let result = writer.write(&structure);

        println!("{result:?}");

        // Write should fail
        assert!(result.is_err());

        // Project directory should be cleaned up (rolled back)
        assert!(!fs_clone.exists(Path::new("/test-project")));
    }
}
