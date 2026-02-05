use std::path::PathBuf;

use crate::domain::{Architecture, Framework, Language, ProjectType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateId(pub &'static str);

/// Describes when a template applies.
/// `None` means "don't care".
#[derive(Debug, Clone, Default)]
pub struct TargetMatcher {
    pub language: Option<Language>,
    pub framework: Option<Framework>,
    pub project_type: Option<ProjectType>,
    pub architecture: Option<Architecture>,
}

impl TargetMatcher {
    pub fn matches(&self, target: &crate::domain::Target) -> bool {
        self.language.map_or(true, |l| l == target.language)
            && self.framework.as_ref().map_or(true, |f| {
                target.framework.as_ref().map_or(false, |tf| tf == f)
            })
            && self
                .project_type
                .as_ref()
                .map_or(true, |p| p == &target.project_type)
            && self
                .architecture
                .as_ref()
                .map_or(true, |a| a == &target.architecture)
    }
}

/// A reusable project recipe.
/// Templates are declarative and contain no I/O logic.
#[derive(Debug, Clone)]
pub struct Template {
    pub id: TemplateId,
    pub matcher: TargetMatcher,
    pub tree: TemplateTree,
}

/// A hierarchical description of files and directories.
#[derive(Debug, Clone)]
pub struct TemplateTree {
    pub nodes: Vec<TemplateNode>,
}

#[derive(Debug, Clone)]
pub enum TemplateNode {
    File(FileSpec),
    Directory(DirectorySpec),
}

/// Relative path only — never absolute.
#[derive(Debug, Clone)]
pub struct FileSpec {
    pub path: RelativePath,
    pub content: TemplateContent,
    pub permissions: FilePermissions,
}

#[derive(Debug, Clone)]
pub struct DirectorySpec {
    pub path: RelativePath,
    pub permissions: FilePermissions,
}

#[derive(Debug, Clone)]
pub enum TemplateContent {
    /// Literal text
    Static(&'static str),

    /// Rendered later using a rendering engine
    Rendered { template_id: ContentTemplateId },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentTemplateId(pub &'static str);

/// Explicit type to prevent absolute paths.
#[derive(Debug, Clone)]
pub struct RelativePath(PathBuf);

impl RelativePath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        debug_assert!(!path.is_absolute());
        Self(path)
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FilePermissions {
    pub executable: bool,
}

impl FilePermissions {
    pub const DEFAULT: Self = Self { executable: false };
}
