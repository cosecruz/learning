//! Template types for project scaffolding.

use crate::domain::DomainError;

use super::{Architecture, Framework, Language, ProjectType, Target};
use crate::domain::{FilePermissions, RelativePath};

// ============================================================================
// Template
// ============================================================================

/// A reusable project recipe.
///
/// Templates are declarative and contain no I/O logic. They describe
/// WHAT to generate, not HOW to generate it.
#[derive(Debug, Clone)]
pub struct Template {
    pub id: TemplateId,
    pub metadata: TemplateMetadata,
    pub matcher: TargetMatcher,
    pub tree: TemplateTree,
}

impl Template {
    /// Validate this template for correctness.
    ///
    /// Checks:
    /// - Tree is not empty
    /// - No absolute paths
    /// - No duplicate paths
    pub fn validate(&self) -> Result<(), DomainError> {
        // Check tree is not empty
        if self.tree.nodes.is_empty() {
            return Err(DomainError::TemplateEmptyTree {
                template_id: self.id.0.to_string(),
            });
        }

        let mut seen_paths = std::collections::HashSet::new();

        // Check each node
        for node in &self.tree.nodes {
            let path = match node {
                TemplateNode::File(spec) => spec.path.as_path(),
                TemplateNode::Directory(spec) => spec.path.as_path(),
            };

            // Check for duplicates
            if !seen_paths.insert(path) {
                return Err(DomainError::TemplateDuplicatePath {
                    template_id: self.id.0.to_string(),
                    path: path.to_path_buf(),
                });
            }

            // Check not absolute (should be enforced by RelativePath, but double-check)
            if path.is_absolute() {
                return Err(DomainError::TemplateAbsolutePath {
                    template_id: self.id.0.to_string(),
                    path: path.to_path_buf(),
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// TemplateId
// ============================================================================

/// Unique identifier for a template.
///
/// Use kebab-case for consistency (e.g., "rust-cli-layered").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateId(pub &'static str);

impl TemplateId {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for TemplateId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

// ============================================================================
// TemplateMetadata
// ============================================================================

/// Metadata describing a template.
///
/// This information helps users understand what a template does.
#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub author: &'static str,
    pub tags: Vec<&'static str>,
}

impl TemplateMetadata {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            description: "",
            version: "0.1.0",
            author: "Scarff Team",
            tags: Vec::new(),
        }
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }

    pub fn author(mut self, author: &'static str) -> Self {
        self.author = author;
        self
    }

    pub fn tags(mut self, tags: Vec<&'static str>) -> Self {
        self.tags = tags;
        self
    }
}

// ============================================================================
// TargetMatcher
// ============================================================================

/// Describes when a template applies to a target.
///
/// For MVP, we use strict matching: all fields must be explicitly set and must match exactly.
/// Post-MVP, we can add wildcard/None support with specificity scoring.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetMatcher {
    pub language: Language,
    pub framework: Option<Framework>,
    pub project_type: ProjectType,
    pub architecture: Architecture,
}

impl TargetMatcher {
    /// Check if this matcher matches the given target (strict mode for MVP).
    ///
    /// All fields must match exactly.
    pub fn matches(&self, target: &Target) -> bool {
        self.language == *target.language()
            && self.framework == target.framework().copied()
            && self.project_type == *target.project_type()
            && self.architecture == *target.architecture()
    }

    /// Calculate specificity score (higher = more specific).
    ///
    /// Used to resolve conflicts when multiple templates match.
    /// Currently all templates have same specificity (4), but this is
    /// here for future use.
    pub fn specificity(&self) -> u8 {
        let mut score = 0;
        score += 1; // language (always present)
        if self.framework.is_some() {
            score += 1;
        }
        score += 1; // project_type (always present)
        score += 1; // architecture (always present)
        score
    }
}

// ============================================================================
// TemplateTree
// ============================================================================

/// A hierarchical description of files and directories.
#[derive(Debug, Clone)]
pub struct TemplateTree {
    pub nodes: Vec<TemplateNode>,
}

impl TemplateTree {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn with_node(mut self, node: TemplateNode) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn add_node(&mut self, node: TemplateNode) {
        self.nodes.push(node);
    }
}

impl Default for TemplateTree {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TemplateNode
// ============================================================================

#[derive(Debug, Clone)]
pub enum TemplateNode {
    File(FileSpec),
    Directory(DirectorySpec),
}

// ============================================================================
// FileSpec
// ============================================================================

/// Specification for a file in a template.
#[derive(Debug, Clone)]
pub struct FileSpec {
    pub(crate) path: RelativePath,
    pub(crate) content: TemplateContent,
    pub(crate) permissions: FilePermissions,
}

impl FileSpec {
    pub(crate) fn new(path: impl Into<RelativePath>, content: TemplateContent) -> Self {
        Self {
            path: path.into(),
            content,
            permissions: FilePermissions::DEFAULT,
        }
    }

    pub(crate) fn executable(mut self) -> Self {
        self.permissions = FilePermissions::EXECUTABLE;
        self
    }
}

// ============================================================================
// DirectorySpec
// ============================================================================

/// Specification for a directory in a template.
#[derive(Debug, Clone)]
pub struct DirectorySpec {
    pub(crate) path: RelativePath,
    pub(crate) permissions: FilePermissions,
}

impl DirectorySpec {
    pub(crate) fn new(path: impl Into<RelativePath>) -> Self {
        Self {
            path: path.into(),
            permissions: FilePermissions::DEFAULT,
        }
    }
}

// ============================================================================
// TemplateContent
// ============================================================================

/// Content for a template file.
#[derive(Debug, Clone)]
pub enum TemplateContent {
    /// Static content (no variable substitution).
    Static(&'static str),

    /// Template content with {{VARIABLE}} placeholders.
    ///
    /// Will be rendered using RenderContext.
    Template(&'static str),

    /// Reference to an external template (for complex rendering).
    ///
    /// Post-MVP: Can be used with Handlebars, Tera, etc.
    Rendered { template_id: ContentTemplateId },
}

// ============================================================================
// ContentTemplateId
// ============================================================================

/// Identifier for an external content template.
///
/// Used for complex templates that need a real template engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentTemplateId(pub &'static str);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::domain::DomainError;

    use super::*;

    fn rust_cli_matcher() -> TargetMatcher {
        TargetMatcher {
            language: Language::Rust,
            framework: None,
            project_type: ProjectType::Cli,
            architecture: Architecture::Layered,
        }
    }

    #[test]
    fn template_id_display() {
        let id = TemplateId("rust-cli-layered");
        assert_eq!(id.to_string(), "rust-cli-layered");
    }

    #[test]
    fn template_metadata_builder() {
        let metadata = TemplateMetadata::new("Rust CLI")
            .description("A simple Rust CLI application")
            .version("1.0.0")
            .author("Test Author")
            .tags(vec!["rust", "cli"]);

        assert_eq!(metadata.name, "Rust CLI");
        assert_eq!(metadata.version, "1.0.0");
        assert_eq!(metadata.tags, vec!["rust", "cli"]);
    }

    #[test]
    fn target_matcher_matches_exact() {
        let matcher = rust_cli_matcher();

        let target = Target::builder()
            .language(Language::Rust)
            .project_type(ProjectType::Cli)
            .architecture(Architecture::Layered)
            .build()
            .unwrap();

        assert!(matcher.matches(&target));
    }

    #[test]
    fn target_matcher_rejects_different_language() {
        let matcher = rust_cli_matcher();

        let target = Target::builder()
            .language(Language::Python)
            .project_type(ProjectType::Cli)
            .build()
            .unwrap();

        assert!(!matcher.matches(&target));
    }

    #[test]
    fn target_matcher_rejects_different_project_type() {
        let matcher = rust_cli_matcher();

        let target = Target::builder()
            .language(Language::Rust)
            .project_type(ProjectType::Backend)
            .build()
            .unwrap();

        assert!(!matcher.matches(&target));
    }

    #[test]
    fn target_matcher_specificity() {
        let matcher_no_framework = TargetMatcher {
            language: Language::Rust,
            framework: None,
            project_type: ProjectType::Cli,
            architecture: Architecture::Layered,
        };

        let matcher_with_framework = TargetMatcher {
            language: Language::Rust,
            framework: Some(Framework::Rust(super::super::RustFramework::Axum)),
            project_type: ProjectType::Backend,
            architecture: Architecture::Layered,
        };

        assert_eq!(matcher_no_framework.specificity(), 3);
        assert_eq!(matcher_with_framework.specificity(), 4);
    }

    #[test]
    fn template_tree_builder() {
        let tree = TemplateTree::new()
            .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/main.rs",
                TemplateContent::Static("fn main() {}"),
            )));

        assert_eq!(tree.nodes.len(), 2);
    }

    #[test]
    fn file_spec_executable() {
        let spec = FileSpec::new("build.sh", TemplateContent::Static("#!/bin/bash")).executable();

        assert!(spec.permissions.executable);
    }

    #[test]
    fn template_validate_empty_tree_fails() {
        let template = Template {
            id: TemplateId("test"),
            metadata: TemplateMetadata::new("Test"),
            matcher: rust_cli_matcher(),
            tree: TemplateTree::new(),
        };

        let result = template.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::TemplateEmptyTree { .. }
        ));
    }

    #[test]
    fn template_validate_duplicate_path_fails() {
        let template = Template {
            id: TemplateId("test"),
            metadata: TemplateMetadata::new("Test"),
            matcher: rust_cli_matcher(),
            tree: TemplateTree::new()
                .with_node(TemplateNode::File(FileSpec::new(
                    "main.rs",
                    TemplateContent::Static(""),
                )))
                .with_node(TemplateNode::File(FileSpec::new(
                    "main.rs",
                    TemplateContent::Static(""),
                ))),
        };

        let result = template.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::TemplateDuplicatePath { .. }
        ));
    }
}
