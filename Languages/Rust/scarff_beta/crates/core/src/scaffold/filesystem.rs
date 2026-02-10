//! Filesystem abstraction for scaffolding operations.
//!
//! This module provides a trait-based abstraction over filesystem operations,
//! enabling both real filesystem access and in-memory testing.
//!
//! # Architecture
//!
//! The `Filesystem` trait defines the interface for file operations. Two
//! implementations are provided:
//!
//! - `RealFilesystem`: Uses `std::fs` for actual filesystem operations
//! - `MockFilesystem`: In-memory implementation for testing
//!
//! # Examples
//!
//! ```rust,no_run
//! use scarff_core::scaffold::filesystem::{Filesystem, RealFilesystem};
//! use std::path::Path;
//!
//! let fs = RealFilesystem;
//! fs.create_dir_all(Path::new("./my-project"))?;
//! fs.write_file(Path::new("./my-project/README.md"), "# My Project")?;
//! # Ok::<(), std::io::Error>(())
//! `
use std::{
    collections::{HashMap, HashSet},
    io,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use crate::domain::Permissions;

// ============================================================================
// Filesystem Trait
// ============================================================================

/// Abstract interface for filesystem operations.
///
/// This trait enables dependency injection and testing by abstracting
/// filesystem access behind a common interface.
pub trait Filesystem: Send + Sync {
    /// Create a directory and all of its parent directories if they don't exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created.
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;

    /// Write content to a file, creating it if it doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    fn write_file(&self, path: &Path, content: &str) -> io::Result<()>;

    /// Set file permissions.
    ///
    /// # Errors
    ///
    /// Returns an error if permissions cannot be set.
    fn set_permissions(&self, path: &Path, permissions: Permissions) -> io::Result<()>;

    /// Check if a path exists.
    fn exists(&self, path: &Path) -> bool;

    /// Check if a path is a directory.
    fn is_dir(&self, path: &Path) -> bool;

    /// Check if a path is a file.
    fn is_file(&self, path: &Path) -> bool;

    /// Remove a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be removed.
    fn remove_file(&self, path: &Path) -> io::Result<()>;

    /// Remove a directory and all its contents.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be removed.
    fn remove_dir_all(&self, path: &Path) -> io::Result<()>;
}

// ============================================================================
// RealFilesystem - Production Implementation
// ============================================================================

/// Real filesystem implementation using `std::fs`.
///
/// This implementation performs actual filesystem operations.
///
/// # Examples
///
/// ```rust,no_run
/// use scarff_core::scaffold::filesystem::{Filesystem, RealFilesystem};
/// use std::path::Path;
///
/// let fs = RealFilesystem;
/// fs.create_dir_all(Path::new("./test-dir"))?;
/// fs.write_file(Path::new("./test-dir/file.txt"), "content")?;
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RealFilesystem;

impl Filesystem for RealFilesystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
        std::fs::write(path, content)
    }

    #[cfg(unix)]
    fn set_permissions(&self, path: &Path, permissions: Permissions) -> io::Result<()> {
        use std::os::unix::fs::PermissionsExt;

        if permissions.executable_flag() {
            let metadata = std::fs::metadata(path)?;
            let mut perms = metadata.permissions();

            // Add executable bit for owner, group, and others
            let mode = perms.mode();
            perms.set_mode(mode | 0o111);

            std::fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    #[cfg(windows)]
    fn set_permissions(&self, _path: &Path, _permissions: Permissions) -> io::Result<()> {
        // Windows doesn't have executable bit
        // Could use file extensions or other mechanisms if needed
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_file(path)
    }

    fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_dir_all(path)
    }
}

// ============================================================================
// MockFilesystem - Testing Implementation
// ============================================================================

/// In-memory filesystem implementation for testing.
///
/// This implementation stores files and directories in memory, allowing
/// tests to verify filesystem operations without touching the actual disk.
///
/// # Examples
///
/// ```rust
/// use scarff_core::scaffold::filesystem::{Filesystem, MockFilesystem};
/// use std::path::Path;
///
/// let fs = MockFilesystem::new();
/// fs.create_dir_all(Path::new("/test-dir"))?;
/// fs.write_file(Path::new("/test-dir/file.txt"), "content")?;
///
/// assert!(fs.exists(Path::new("/test-dir/file.txt")));
/// assert_eq!(fs.read_file(Path::new("/test-dir/file.txt")).unwrap(), "content");
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct MockFilesystem {
    inner: Arc<RwLock<MockFilesystemInner>>,
}

#[derive(Debug, Clone)]
struct MockFilesystemInner {
    files: HashMap<PathBuf, FileEntry>,
    directories: HashSet<PathBuf>,
}

#[derive(Debug, Clone)]
struct FileEntry {
    content: String,
    permissions: Permissions,
}

impl MockFilesystem {
    /// Create a new empty mock filesystem.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(MockFilesystemInner {
                files: HashMap::new(),
                directories: HashSet::new(),
            })),
        }
    }

    /// Read a file's content (only available in MockFilesystem).
    ///
    /// This is a testing helper not available in the trait.
    ///
    /// # Errors
    ///
    /// Returns an error if the file doesn't exist.
    pub fn read_file(&self, path: &Path) -> io::Result<String> {
        let inner = self
            .inner
            .read()
            .map_err(|_| io::Error::other("Lock poisoned"))?;

        inner
            .files
            .get(path)
            .map(|entry| entry.content.clone())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File not found: {}", path.display()),
                )
            })
    }

    /// Get the permissions of a file (only available in MockFilesystem).
    ///
    /// # Errors
    ///
    /// Returns an error if the file doesn't exist.
    pub fn get_permissions(&self, path: &Path) -> io::Result<Permissions> {
        let inner = self
            .inner
            .read()
            .map_err(|_| io::Error::other("Lock poisoned"))?;

        inner
            .files
            .get(path)
            .map(|entry| entry.permissions)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File not found: {}", path.display()),
                )
            })
    }

    /// Check if a file is marked as executable.
    ///
    /// # Errors
    ///
    /// Returns an error if the file doesn't exist.
    pub fn is_executable(&self, path: &Path) -> io::Result<bool> {
        Ok(self.get_permissions(path)?.executable_flag())
    }

    /// List all files in the mock filesystem.
    pub fn list_files(&self) -> Vec<PathBuf> {
        let inner = self.inner.read().unwrap();
        inner.files.keys().cloned().collect()
    }

    /// List all directories in the mock filesystem.
    pub fn list_directories(&self) -> Vec<PathBuf> {
        let inner = self.inner.read().unwrap();
        inner.directories.iter().cloned().collect()
    }

    /// Clear all files and directories.
    pub fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.files.clear();
        inner.directories.clear();
    }

    /// Get the total number of files.
    pub fn file_count(&self) -> usize {
        let inner = self.inner.read().unwrap();
        inner.files.len()
    }

    /// Get the total number of directories.
    pub fn directory_count(&self) -> usize {
        let inner = self.inner.read().unwrap();
        inner.directories.len()
    }
}

impl Default for MockFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Filesystem for MockFilesystem {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| io::Error::other("Lock poisoned"))?;

        // Add all parent directories
        let mut current = PathBuf::new();
        for component in path.components() {
            current.push(component);
            inner.directories.insert(current.clone());
        }

        Ok(())
    }

    fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| io::Error::other("Lock poisoned"))?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
            && !inner.directories.contains(parent)
        {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Parent directory does not exist: {}", parent.display()),
            ));
        }

        inner.files.insert(
            path.to_path_buf(),
            FileEntry {
                content: content.to_string(),
                permissions: Permissions::read_write(),
            },
        );

        Ok(())
    }

    fn set_permissions(&self, path: &Path, permissions: Permissions) -> io::Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| io::Error::other("Lock poisoned"))?;

        inner
            .files
            .get_mut(path)
            .map(|entry| entry.permissions = permissions)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File not found: {}", path.display()),
                )
            })
    }

    fn exists(&self, path: &Path) -> bool {
        let inner = self.inner.read().unwrap();
        inner.files.contains_key(path) || inner.directories.contains(path)
    }

    fn is_dir(&self, path: &Path) -> bool {
        let inner = self.inner.read().unwrap();
        inner.directories.contains(path)
    }

    fn is_file(&self, path: &Path) -> bool {
        let inner = self.inner.read().unwrap();
        inner.files.contains_key(path)
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| io::Error::other("Lock poisoned"))?;

        inner.files.remove(path).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )
        })?;

        Ok(())
    }

    fn remove_dir_all(&self, path: &Path) -> io::Result<()> {
        let mut inner = self
            .inner
            .write()
            .map_err(|_| io::Error::other("Lock poisoned"))?;

        // Remove the directory
        inner.directories.remove(path);

        // Remove all files and subdirectories within this path
        inner
            .files
            .retain(|file_path, _| !file_path.starts_with(path));
        inner
            .directories
            .retain(|dir_path| !dir_path.starts_with(path));

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // MockFilesystem Tests
    // ------------------------------------------------------------------------

    #[test]
    fn mock_fs_create_dir_all() {
        let fs = MockFilesystem::new();

        fs.create_dir_all(Path::new("/a/b/c")).unwrap();

        assert!(fs.exists(Path::new("/a")));
        assert!(fs.exists(Path::new("/a/b")));
        assert!(fs.exists(Path::new("/a/b/c")));
        assert!(fs.is_dir(Path::new("/a/b/c")));
    }

    #[test]
    fn mock_fs_write_and_read_file() {
        let fs = MockFilesystem::new();

        fs.create_dir_all(Path::new("/test")).unwrap();
        fs.write_file(Path::new("/test/file.txt"), "Hello, World!")
            .unwrap();

        assert!(fs.exists(Path::new("/test/file.txt")));
        assert!(fs.is_file(Path::new("/test/file.txt")));
        assert_eq!(
            fs.read_file(Path::new("/test/file.txt")).unwrap(),
            "Hello, World!"
        );
    }

    #[test]
    fn mock_fs_write_file_requires_parent_dir() {
        let fs = MockFilesystem::new();

        // Should fail because parent directory doesn't exist
        let result = fs.write_file(Path::new("/nonexistent/file.txt"), "content");
        assert!(result.is_err());
    }

    #[test]
    fn mock_fs_set_permissions() {
        let fs = MockFilesystem::new();

        fs.create_dir_all(Path::new("/test")).unwrap();
        fs.write_file(Path::new("/test/script.sh"), "#!/bin/bash")
            .unwrap();

        // Initially not executable
        assert!(!fs.is_executable(Path::new("/test/script.sh")).unwrap());

        // Make executable
        fs.set_permissions(Path::new("/test/script.sh"), Permissions::executable())
            .unwrap();

        assert!(fs.is_executable(Path::new("/test/script.sh")).unwrap());
    }

    #[test]
    fn mock_fs_remove_file() {
        let fs = MockFilesystem::new();

        fs.create_dir_all(Path::new("/test")).unwrap();
        fs.write_file(Path::new("/test/file.txt"), "content")
            .unwrap();

        assert!(fs.exists(Path::new("/test/file.txt")));

        fs.remove_file(Path::new("/test/file.txt")).unwrap();

        assert!(!fs.exists(Path::new("/test/file.txt")));
    }

    #[test]
    fn mock_fs_remove_dir_all() {
        let fs = MockFilesystem::new();

        fs.create_dir_all(Path::new("/test/subdir")).unwrap();
        fs.write_file(Path::new("/test/file1.txt"), "content1")
            .unwrap();
        fs.write_file(Path::new("/test/subdir/file2.txt"), "content2")
            .unwrap();

        assert!(fs.exists(Path::new("/test")));
        assert!(fs.exists(Path::new("/test/file1.txt")));
        assert!(fs.exists(Path::new("/test/subdir/file2.txt")));

        fs.remove_dir_all(Path::new("/test")).unwrap();

        assert!(!fs.exists(Path::new("/test")));
        assert!(!fs.exists(Path::new("/test/file1.txt")));
        assert!(!fs.exists(Path::new("/test/subdir/file2.txt")));
    }

    #[test]
    fn mock_fs_list_files_and_directories() {
        let fs = MockFilesystem::new();

        fs.create_dir_all(Path::new("/test/subdir")).unwrap();
        fs.write_file(Path::new("/test/file1.txt"), "content1")
            .unwrap();
        fs.write_file(Path::new("/test/subdir/file2.txt"), "content2")
            .unwrap();

        let files = fs.list_files();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&PathBuf::from("/test/file1.txt")));
        assert!(files.contains(&PathBuf::from("/test/subdir/file2.txt")));

        let dirs = fs.list_directories();
        assert!(dirs.contains(&PathBuf::from("/test")));
        assert!(dirs.contains(&PathBuf::from("/test/subdir")));
    }

    #[test]
    fn mock_fs_clear() {
        let fs = MockFilesystem::new();

        fs.create_dir_all(Path::new("/test")).unwrap();
        fs.write_file(Path::new("/test/file.txt"), "content")
            .unwrap();

        assert_eq!(fs.file_count(), 1);
        assert!(fs.directory_count() > 0);

        fs.clear();

        assert_eq!(fs.file_count(), 0);
        assert_eq!(fs.directory_count(), 0);
    }

    #[test]
    fn mock_fs_is_thread_safe() {
        use std::sync::Arc;
        use std::thread;

        let fs = Arc::new(MockFilesystem::new());
        fs.create_dir_all(Path::new("/test")).unwrap();

        let mut handles = vec![];

        // Spawn multiple threads writing files
        for i in 0..10 {
            let fs_clone = Arc::clone(&fs);
            let handle = thread::spawn(move || {
                let path = format!("/test/file{}.txt", i);
                fs_clone
                    .write_file(Path::new(&path), &format!("content {}", i))
                    .unwrap();
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all files were created
        assert_eq!(fs.file_count(), 10);
    }

    // ------------------------------------------------------------------------
    // RealFilesystem Tests (require actual disk I/O)
    // ------------------------------------------------------------------------

    #[test]
    fn real_fs_basic_operations() {
        use tempfile::tempdir;

        let temp = tempdir().unwrap();
        let fs = RealFilesystem;

        let dir_path = temp.path().join("test_dir");
        let file_path = dir_path.join("test_file.txt");

        // Create directory
        fs.create_dir_all(&dir_path).unwrap();
        assert!(fs.exists(&dir_path));
        assert!(fs.is_dir(&dir_path));

        // Write file
        fs.write_file(&file_path, "test content").unwrap();
        assert!(fs.exists(&file_path));
        assert!(fs.is_file(&file_path));

        // Verify content
        let content = std::fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "test content");

        // Clean up
        fs.remove_file(&file_path).unwrap();
        assert!(!fs.exists(&file_path));

        fs.remove_dir_all(&dir_path).unwrap();
        assert!(!fs.exists(&dir_path));
    }

    #[cfg(unix)]
    #[test]
    fn real_fs_set_executable() {
        use std::os::unix::fs::PermissionsExt;
        use tempfile::tempdir;

        let temp = tempdir().unwrap();
        let fs = RealFilesystem;

        let file_path = temp.path().join("script.sh");

        // Create file
        fs.write_file(&file_path, "#!/bin/bash\necho 'test'")
            .unwrap();

        // Set executable
        fs.set_permissions(&file_path, Permissions::executable())
            .unwrap();

        // Verify
        let metadata = std::fs::metadata(&file_path).unwrap();
        let perms = metadata.permissions();
        assert!(perms.mode() & 0o111 != 0);
    }
}
