use std::path::PathBuf;

/// Fully resolved, ready-to-write project layout.
/// This is the output of template resolution + rendering.
#[derive(Debug, Clone)]
pub struct ProjectStructure {
    pub root: PathBuf,
    pub entries: Vec<FsEntry>,
}

impl ProjectStructure {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FsEntry {
    File(FileToWrite),
    Directory(DirectoryToCreate),
}

#[derive(Debug, Clone)]
pub struct FileToWrite {
    pub path: PathBuf,
    pub content: String,
    pub permissions: FilePermissions,
}

#[derive(Debug, Clone)]
pub struct DirectoryToCreate {
    pub path: PathBuf,
    pub permissions: FilePermissions,
}

#[derive(Debug, Clone, Copy)]
pub struct FilePermissions {
    pub executable: bool,
}

impl FilePermissions {
    pub const DEFAULT: Self = Self { executable: false };
}
