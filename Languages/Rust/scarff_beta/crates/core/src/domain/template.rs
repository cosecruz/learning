//! Template domain model and system.
//!
//! This module defines the complete template system for Scarff:
//! - Template definitions (what to generate)
//! - Template matching (when to use a template)
//! - Template metadata (for discovery and documentation)
//!
//! ## Architecture
//!
//! ```text
//! TemplateRecord (storage wrapper)
//!   └─ Template (declarative recipe)
//!       ├─ TargetMatcher (when to apply)
//!       ├─ TemplateMetadata (human info)
//!       └─ TemplateTree (what to generate)
//!           └─ TemplateNode[] (files & dirs)
//! ```
//!
//! ## Design Principles
//!
//! - Templates are **declarative** (describe WHAT, not HOW)
//! - Templates never perform I/O
//! - Templates are validated before use
//! - Matching is based on specificity scoring

use std::{fmt, path::PathBuf};
use uuid::Uuid;

use crate::{
    CoreError,
    domain::{
        Architecture, DomainError, Framework, Language, ProjectKind, ProjectStructure, Target,
        common::{Permissions, RelativePath},
        validator,
    },
    template::TemplateError,
};

// ============================================================================
// Template Engine Trait
// ============================================================================

/// Template engine interface for loading and rendering templates.
///
/// This trait abstracts the template system, allowing different implementations:
/// - In-memory stores (for built-in templates)
/// - Filesystem stores (for user templates)
/// - Remote registries (for community templates)
///
/// ## Responsibilities
///
/// 1. **Resolution**: Find the right template for a target
/// 2. **Loading**: Retrieve template definitions
/// 3. **Validation**: Ensure templates are well-formed
///
/// ## Example
///
/// ```rust,ignore
/// struct MyEngine;
///
/// impl TemplateEngine for MyEngine {
///     fn resolve(&self, target: &Target) -> Result<TemplateRecord, DomainError> {
///         // Find best matching template
///         todo!()
///     }
/// }
/// ```
pub trait TemplateEngine {
    /// Resolve a target into a matching template.
    /// create a template record
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - validating target fails
    /// - No matching template is found
    /// - Multiple templates match with same specificity (ambiguous)
    fn resolve(&self, target: &Target) -> Result<TemplateRecord, TemplateError>;

    /// Renders a template record into a project structure.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - validating `template_record` fails
    /// - Multiple templates match with same specificity (ambiguous)
    fn render(&self, template_record: &TemplateRecord) -> Result<ProjectStructure, TemplateError>;
}

// ============================================================================
// TemplateId - Semantic Identifier
// ============================================================================

/// Stable, semantic identifier for a template.
///
/// This is what users see, reference, and reason about.
/// Format: `name@version` (e.g., `rust-web-api-axum@1.2.0`)
///
/// ## Design Notes
///
/// - **Stable**: IDs don't change across versions
/// - **Semantic**: Includes version for compatibility
/// - **Human-readable**: Easy to reference in CLI/docs
///
/// ## Example
///
/// ```rust
/// # use std::fmt;
/// # #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// # pub struct TemplateId { pub name: String, pub version: String }
/// # impl TemplateId {
/// #     pub fn new(name: impl Into<String>, version: String) -> Self {
/// #         Self { name: name.into(), version }
/// #     }
/// # }
/// # impl fmt::Display for TemplateId {
/// #     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/// #         write!(f, "{}@{}", self.name, self.version)
/// #     }
/// # }
/// let id = TemplateId::new("rust-cli", "1.0.0".to_string());
/// assert_eq!(id.to_string(), "rust-cli@1.0.0");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateId {
    pub name: String,
    /// Semantic version (TODO: use semver::Version post-MVP)
    pub version: String,
}

impl TemplateId {
    /// Create a new template ID.
    ///
    /// # Arguments
    ///
    /// * `name` - Template name (e.g., "rust-cli-layered")
    /// * `version` - Semantic version string (e.g., "1.0.0")
    pub fn new(name: impl Into<String>, version: String) -> Self {
        Self {
            name: name.into(),
            version,
        }
    }

    /// Get the name component.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the version component.
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for TemplateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

// ============================================================================
// TemplateRecord - Storage Wrapper
// ============================================================================

/// Internal record wrapper for templates.
///
/// This wraps a template with storage metadata (UUID for internal indexing).
/// Users never see the UUID - it's for internal bookkeeping only.
///
/// ## Why Both UUID and TemplateId?
///
/// - **UUID**: Internal, globally unique, immutable
/// - **TemplateId**: External, human-readable, versioned
///
/// This allows templates to be renamed/repackaged while maintaining
/// internal consistency.
#[derive(Debug, Clone)]
pub struct TemplateRecord {
    /// Internal UUID (never shown to users)
    pub uuid: Uuid,
    /// The actual template definition
    pub template: Template,
}

impl TemplateRecord {
    /// Create a new template record with generated UUID.
    pub fn new(template: Template) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            template,
        }
    }

    /// Create a template record with explicit UUID (for deserialization).
    pub fn with_uuid(uuid: Uuid, template: Template) -> Self {
        Self { uuid, template }
    }

    /// Validate this template record.
    ///
    /// Checks:
    /// - Template tree is not empty
    /// - No absolute paths in file specs
    /// - No duplicate paths
    /// - All paths are valid
    ///
    /// # Errors
    ///
    /// Returns a domain error if validation fails.
    pub fn validate(&self) -> Result<(), DomainError> {
        // Check tree is not empty
        validator::validate_template_record(self)?;

        Ok(())
    }
}

// ============================================================================
// Template - The Core Model
// ============================================================================

/// Declarative recipe describing a project.
///
/// A template defines:
/// - **When** it applies (via [`TargetMatcher`])
/// - **What** files and directories to create
/// - **Metadata** for discovery and documentation
///
/// ## Invariants
///
/// - Templates are **immutable** after creation
/// - Templates **never perform I/O** directly
/// - Templates are **validated** before use
///
/// ## Example
///
/// ```rust,ignore
/// let template = Template {
///     matcher: TargetMatcher {
///         language: Some(Language::Rust),
///         framework: None,
///         kind: Some(ProjectKind::Cli),
///         architecture: Some(Architecture::Layered),
///     },
///     metadata: TemplateMetadata::new("Rust CLI")
///         .description("A simple CLI application")
///         .tags(vec!["rust", "cli"]),
///     tree: TemplateTree::default()
///         .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
///         .with_node(TemplateNode::File(
///             FileSpec::new("src/main.rs", TemplateContent::Literal(...))
///         )),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Template {
    /// External identifier (shown to users)
    pub id: TemplateId,
    /// Matcher defining when this template applies
    pub matcher: TargetMatcher,
    /// Human-readable metadata
    pub metadata: TemplateMetadata,
    /// Filesystem structure to generate
    pub tree: TemplateTree,
}

impl Template {
    /// Create a new template with builder pattern.
    pub fn builder() -> TemplateBuilder {
        TemplateBuilder::default()
    }
}

// ============================================================================
// TemplateBuilder
// ============================================================================

/// Builder for constructing templates.
#[derive(Default)]
pub struct TemplateBuilder {
    id: Option<TemplateId>,
    matcher: Option<TargetMatcher>,
    metadata: Option<TemplateMetadata>,
    tree: TemplateTree,
}

// TODO: id and metadata are almost the same think how to align

impl TemplateBuilder {
    pub fn id(mut self, id: TemplateId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn matcher(mut self, matcher: TargetMatcher) -> Self {
        self.matcher = Some(matcher);
        self
    }

    pub fn metadata(mut self, metadata: TemplateMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn tree(mut self, tree: TemplateTree) -> Self {
        self.tree = tree;
        self
    }

    pub fn add_node(mut self, node: TemplateNode) -> Self {
        self.tree.push(node);
        self
    }

    pub fn build(self) -> Result<Template, DomainError> {
        Ok(Template {
            id: self.id.ok_or_else(|| {
                DomainError::InvalidTemplate("template id is required".to_string())
            })?,
            matcher: self
                .matcher
                .ok_or_else(|| DomainError::InvalidTemplate("Matcher is required".to_string()))?,
            metadata: self
                .metadata
                .ok_or_else(|| DomainError::InvalidTemplate("Metadata is required".to_string()))?,
            tree: self.tree,
        })
    }
}

// ============================================================================
// TargetMatcher - When to Apply a Template
// ============================================================================

/// Describes when a template applies to a [`Target`].
///
/// ## Matching Semantics
///
/// - `None` → **wildcard** (matches any value)
/// - `Some(x)` → **constraint** (must equal target field)
///
/// ## Specificity
///
/// When multiple templates match, the most specific wins:
/// - 4 fields set > 3 fields set > 2 fields set > 1 field set
/// - All else equal, first match wins (deterministic)
///
/// ## Example
///
/// ```rust,ignore
/// // Matches ANY Rust project
/// let broad = TargetMatcher {
///     language: Some(Language::Rust),
///     framework: None,
///     kind: None,
///     architecture: None,
/// };
///
/// // Matches ONLY Rust CLI with Layered architecture
/// let specific = TargetMatcher {
///     language: Some(Language::Rust),
///     framework: None,
///     kind: Some(ProjectKind::Cli),
///     architecture: Some(Architecture::Layered),
/// };
///
/// assert_eq!(broad.specificity(), 1);
/// assert_eq!(specific.specificity(), 3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetMatcher {
    pub language: Option<Language>,
    pub framework: Option<Framework>,
    pub kind: Option<ProjectKind>,
    pub architecture: Option<Architecture>,
}

impl TargetMatcher {
    /// Create a new matcher builder.
    pub fn builder() -> TargetMatcherBuilder {
        TargetMatcherBuilder::default()
    }

    /// Check whether this matcher applies to a target.
    ///
    /// A matcher matches if all non-None fields equal the target's corresponding field.
    pub fn matches(&self, target: &Target) -> bool {
        self.language
            .as_ref()
            .is_none_or(|l| *l == target.language())
            && self
                .framework
                .as_ref()
                .is_none_or(|f| Some(*f) == target.framework())
            && self.kind.as_ref().is_none_or(|k| *k == target.kind())
            && self
                .architecture
                .as_ref()
                .is_none_or(|a| *a == target.architecture())
    }

    /// Calculate specificity score (higher = more specific).
    ///
    /// Specificity is the count of non-None fields.
    /// Used to resolve conflicts when multiple templates match.
    pub fn specificity(&self) -> u8 {
        u8::try_from(
            [
                self.language.is_some(),
                self.framework.is_some(),
                self.kind.is_some(),
                self.architecture.is_some(),
            ]
            .into_iter()
            .filter(|b| *b)
            .count(),
        )
        .expect("specificity count should fit in u8")
    }
}

// ============================================================================
// TargetMatcherBuilder
// ============================================================================

#[derive(Default)]
pub struct TargetMatcherBuilder {
    language: Option<Language>,
    framework: Option<Framework>,
    kind: Option<ProjectKind>,
    architecture: Option<Architecture>,
}

impl TargetMatcherBuilder {
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn framework(mut self, framework: Framework) -> Self {
        self.framework = Some(framework);
        self
    }

    pub fn kind(mut self, kind: ProjectKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn architecture(mut self, architecture: Architecture) -> Self {
        self.architecture = Some(architecture);
        self
    }

    pub fn build(self) -> TargetMatcher {
        TargetMatcher {
            language: self.language,
            framework: self.framework,
            kind: self.kind,
            architecture: self.architecture,
        }
    }
}

// ============================================================================
// TemplateMetadata - Human-Readable Info
// ============================================================================

/// Human-readable metadata describing a template.
///
/// All fields are `&'static str` for simplicity in MVP.
/// Post-MVP: Can be changed to `String` for dynamic templates.
#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub version: &'static str,
    pub author: &'static str,
    pub tags: Vec<&'static str>,
}

impl TemplateMetadata {
    /// Create new metadata with just a name.
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            description: "",
            version: "0.1.0",
            author: "Scarff",
            tags: Vec::new(),
        }
    }

    /// Set description (builder style).
    pub fn description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    /// Set version (builder style).
    pub fn version(mut self, version: &'static str) -> Self {
        self.version = version;
        self
    }

    /// Set author (builder style).
    pub fn author(mut self, author: &'static str) -> Self {
        self.author = author;
        self
    }

    /// Set tags (builder style).
    pub fn tags(mut self, tags: Vec<&'static str>) -> Self {
        self.tags = tags;
        self
    }
}

// ============================================================================
// TemplateTree - Filesystem Structure
// ============================================================================

/// Declarative description of a project's filesystem structure.
///
/// Contains an ordered list of files and directories to create.
/// Order matters for dependencies (e.g., create directories before files).
#[derive(Debug, Clone, Default)]
pub struct TemplateTree {
    pub nodes: Vec<TemplateNode>,
}

impl TemplateTree {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a node (mutable).
    pub fn push(&mut self, node: TemplateNode) {
        self.nodes.push(node);
    }

    /// Add a node (builder style).
    pub fn with_node(mut self, node: TemplateNode) -> Self {
        self.push(node);
        self
    }

    /// Check if tree is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Get node count.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

// ============================================================================
// TemplateNode - File or Directory
// ============================================================================

/// A node in a template filesystem tree.
#[derive(Debug, Clone)]
pub enum TemplateNode {
    File(FileSpec),
    Directory(DirectorySpec),
}

// ============================================================================
// FileSpec - File Specification
// ============================================================================

/// Declarative specification for a generated file.
///
/// ## Example
///
/// ```rust,ignore
/// let spec = FileSpec::new(
///     "src/main.rs",
///     TemplateContent::Literal(TemplateSource::Static("fn main() {}"))
/// ).executable();
/// ```
#[derive(Debug, Clone)]
pub struct FileSpec {
    pub path: RelativePath,
    pub content: TemplateContent,
    pub permissions: Permissions,
}

impl FileSpec {
    /// Create a new file spec with default permissions (rw).
    pub fn new(path: impl Into<RelativePath>, content: TemplateContent) -> Self {
        Self {
            path: path.into(),
            content,
            permissions: Permissions::read_write(),
        }
    }

    /// Mark file as executable (builder style).
    pub fn executable(mut self) -> Self {
        self.permissions = Permissions::executable();
        self
    }

    /// Set custom permissions (builder style).
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }
}

// ============================================================================
// DirectorySpec - Directory Specification
// ============================================================================

/// Declarative specification for a generated directory.
#[derive(Debug, Clone)]
pub struct DirectorySpec {
    pub path: RelativePath,
    pub permissions: Permissions,
}

impl DirectorySpec {
    /// Create a new directory spec with default permissions (rw).
    pub fn new(path: impl Into<RelativePath>) -> Self {
        Self {
            path: path.into(),
            permissions: Permissions::read_write(),
        }
    }

    /// Set custom permissions (builder style).
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions = permissions;
        self
    }
}

// ============================================================================
// TemplateContent - File Content Types
// ============================================================================

/// Declarative description of file content.
///
/// ## Variants
///
/// - **Literal**: Static content, no variable substitution
/// - **Parameterized**: Content with `{{VARIABLE}}` placeholders
/// - **External**: Reference to external template engine (Tera, Handlebars)
#[derive(Debug, Clone)]
pub enum TemplateContent {
    /// Static literal content (no variables).
    Literal(TemplateSource),

    /// Content requiring variable substitution.
    Parameterized(TemplateSource),

    /// External template resolved by a rendering engine.
    External(ContentTemplateId),
}

/// Source of template content.
#[derive(Debug, Clone)]
pub enum TemplateSource {
    /// Static string embedded in binary (via include_str!)
    Static(&'static str),
    /// Owned string (for dynamic templates)
    Owned(String),
}

impl From<&'static str> for TemplateSource {
    fn from(s: &'static str) -> Self {
        Self::Static(s)
    }
}

impl From<String> for TemplateSource {
    fn from(s: String) -> Self {
        Self::Owned(s)
    }
}

impl TemplateSource {
    /// Get the content as a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s,
        }
    }
}

/// Identifier for external template engines.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentTemplateId(pub &'static str);

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use std::process::id;

    use super::*;

    // -------------------------------------------------------------------------
    // TemplateId Tests
    // -------------------------------------------------------------------------

    #[test]
    fn template_id_display() {
        let id = TemplateId::new("rust-web", "1.2.3".to_string());
        assert_eq!(id.to_string(), "rust-web@1.2.3");
    }

    #[test]
    fn template_id_equality() {
        let a = TemplateId::new("a", "1.0.0".to_string());
        let b = TemplateId::new("a", "1.0.0".to_string());
        let c = TemplateId::new("a", "2.0.0".to_string());

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // -------------------------------------------------------------------------
    // TemplateRecord Tests
    // -------------------------------------------------------------------------

    #[test]
    fn template_record_generates_uuid() {
        let record = TemplateRecord::new(Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree: TemplateTree::default(),
        });

        assert_ne!(record.uuid, Uuid::nil());
    }

    #[test]
    fn template_record_validation_empty_tree() {
        let record = TemplateRecord::new(Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree: TemplateTree::default(),
        });

        let result = record.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::TemplateEmptyTree { .. }
        ));
    }

    // -------------------------------------------------------------------------
    // TargetMatcher Tests
    // -------------------------------------------------------------------------

    fn rust_cli_target() -> Target {
        Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap()
    }

    #[test]
    fn matcher_exact_match() {
        let matcher = TargetMatcher::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .architecture(Architecture::Layered)
            .build();

        let target = rust_cli_target();
        assert!(matcher.matches(&target));
    }

    #[test]
    fn matcher_language_mismatch() {
        let matcher = TargetMatcher::builder()
            .language(Language::Python)
            .kind(ProjectKind::Cli)
            .build();

        let target = rust_cli_target();
        assert!(!matcher.matches(&target));
    }

    #[test]
    fn matcher_wildcard_matches_anything() {
        let matcher = TargetMatcher::builder().build(); // All None

        let target = rust_cli_target();
        assert!(matcher.matches(&target));
    }

    #[test]
    fn matcher_partial_wildcard() {
        let matcher = TargetMatcher::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .build(); // architecture is None (wildcard)

        let target = rust_cli_target();
        assert!(matcher.matches(&target));
    }

    #[test]
    fn specificity_calculation() {
        let none = TargetMatcher::builder().build();
        let one = TargetMatcher::builder().language(Language::Rust).build();
        let two = TargetMatcher::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .build();
        let four = TargetMatcher::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .architecture(Architecture::Layered)
            .framework(Framework::Rust(crate::domain::RustFramework::Axum))
            .build();

        assert_eq!(none.specificity(), 0);
        assert_eq!(one.specificity(), 1);
        assert_eq!(two.specificity(), 2);
        assert_eq!(four.specificity(), 4);
    }

    // -------------------------------------------------------------------------
    // TemplateTree Tests
    // -------------------------------------------------------------------------

    #[test]
    fn template_tree_push_preserves_order() {
        let mut tree = TemplateTree::new();

        tree.push(TemplateNode::Directory(DirectorySpec::new(
            RelativePath::new("src"),
        )));
        tree.push(TemplateNode::File(FileSpec::new(
            RelativePath::new("src/main.rs"),
            TemplateContent::Literal(TemplateSource::Static("fn main() {}")),
        )));

        assert_eq!(tree.len(), 2);
        assert!(!tree.is_empty());
    }

    #[test]
    fn template_tree_builder_style() {
        let tree = TemplateTree::new()
            .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/main.rs",
                TemplateContent::Literal(TemplateSource::Static("fn main() {}")),
            )));

        assert_eq!(tree.len(), 2);
    }

    // -------------------------------------------------------------------------
    // FileSpec Tests
    // -------------------------------------------------------------------------

    #[test]
    fn file_spec_default_permissions() {
        let file = FileSpec::new(
            "main.rs",
            TemplateContent::Literal(TemplateSource::Static("")),
        );

        assert!(file.permissions.readable());
        assert!(file.permissions.writable());
        assert!(!file.permissions.executable_flag());
    }

    #[test]
    fn file_spec_executable() {
        let file = FileSpec::new(
            "build.sh",
            TemplateContent::Literal(TemplateSource::Static("")),
        )
        .executable();

        assert!(file.permissions.executable_flag());
    }

    // -------------------------------------------------------------------------
    // Template Builder Tests
    // -------------------------------------------------------------------------

    #[test]
    fn template_builder_requires_matcher_and_metadata() {
        let result = Template::builder().build();
        assert!(result.is_err());

        let result = Template::builder()
            .metadata(TemplateMetadata::new("test"))
            .build();
        assert!(result.is_err());

        // requires id
        let result = Template::builder()
            .matcher(TargetMatcher::builder().build())
            .metadata(TemplateMetadata::new("test"))
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn template_builder_complete() {
        let template = Template::builder()
            .id(TemplateId::new("name", "version.".into()))
            .matcher(
                TargetMatcher::builder()
                    .language(Language::Rust)
                    .kind(ProjectKind::Cli)
                    .build(),
            )
            .metadata(
                TemplateMetadata::new("Rust CLI")
                    .description("A CLI app")
                    .tags(vec!["rust", "cli"]),
            )
            .add_node(TemplateNode::Directory(DirectorySpec::new("src")))
            .add_node(TemplateNode::File(FileSpec::new(
                "src/main.rs",
                TemplateContent::Literal(TemplateSource::Static("fn main() {}")),
            )))
            .build()
            .unwrap();

        assert!(!template.tree.is_empty());
        assert_eq!(template.metadata.name, "Rust CLI");
    }
}
