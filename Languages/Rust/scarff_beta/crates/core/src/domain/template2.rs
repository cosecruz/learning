//! Template domain model.
//!
//! This module defines **declarative templates** used by Scarff.
//! Templates describe *what* a project looks like, not *how* it is generated.
//!
//! Rendering and filesystem operations are handled by the scaffold engine.

use std::fmt;
use uuid::Uuid;

use crate::{
    Architecture, Framework, Language, ProjectKind, Target,
    domain::{
        ProjectStructure,
        common2::{Permissions, RelativePath},
    },
    template::TemplateError,
};

//
// ============================================================================
// Template Engine Trait
// ============================================================================
//

/// Template engine interface.
///
/// Responsible for:
/// - Resolving a [`Target`] into a [`Template`]
/// - Rendering a [`Template`] into a [`ProjectStructure`]
///
/// Implementations may:
/// - Load templates from memory
/// - Load templates from disk
/// - Select templates using scoring or heuristics
pub trait TemplateEngine {
    /// Resolve a target into a matching template.
    fn resolve(&self, target: &Target) -> Result<TemplateRecord, TemplateError>;

    /// Render a resolved template into a project structure.
    fn render(&self, template: &TemplateRecord) -> Result<ProjectStructure, TemplateError>;
}

//
// ============================================================================
// TemplateId
// ============================================================================
//

/// Stable, semantic identifier for a template.
///
/// This is what users see, reference, and reason about.
/// Example: `rust-web-api-axum@1.2.0`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateId {
    pub name: String,
    // TODO: use semver::Version POSTMVP
    pub version: String,
}

impl TemplateId {
    pub fn new(name: impl Into<String>, version: String) -> Self {
        Self {
            name: name.into(),
            version,
        }
    }
}

impl fmt::Display for TemplateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

/// Internal record wrapper for templates.
///
/// This is never shown to users.
/// Used for storage, caching, and registry indexing.
#[derive(Debug, Clone)]
pub struct TemplateRecord {
    pub uuid: Uuid,
    pub id: TemplateId,
    pub template: Template,
}

impl TemplateRecord {
    pub fn new(id: TemplateId, template: Template) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            id,
            template,
        }
    }
}

//
// ============================================================================
// Template
// ============================================================================
//

/// Declarative recipe describing a project.
///
/// A template defines:
/// - When it applies (via [`TargetMatcher`])
/// - What files and directories exist
/// - Metadata for discovery and documentation
///
/// A template **never performs rendering or I/O**.
#[derive(Debug, Clone)]
pub struct Template {
    pub matcher: TargetMatcher,
    pub metadata: TemplateMetadata,
    pub tree: TemplateTree,
}

//
// ============================================================================
// TargetMatcher
// ============================================================================
//

/// Describes when a template applies to a [`Target`].
///
/// Matching semantics:
/// - `None` → wildcard (matches any value)
/// - `Some(x)` → constraint (must equal target field)
///
/// Multiple matching templates are resolved using specificity scoring.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetMatcher {
    pub language: Option<Language>,
    pub framework: Option<Framework>,
    pub kind: Option<ProjectKind>,
    pub architecture: Option<Architecture>,
}

impl TargetMatcher {
    /// Check whether this matcher applies to a target.
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
        .expect("should be a valid u8")
    }
}

//
// ============================================================================
// TemplateMetadata
// ============================================================================
//

/// Human-readable metadata describing a template.
/// All fields are 'static, so metadata is compile-time defined — good for built-ins.
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
            author: "Scarff",
            tags: Vec::new(),
        }
    }

    pub fn description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn tags(mut self, tags: Vec<&'static str>) -> Self {
        self.tags = tags;
        self
    }
}

//
// ============================================================================
// TemplateTree
// ============================================================================
//

/// Declarative description of a project’s filesystem structure.
#[derive(Debug, Clone, Default)]
pub struct TemplateTree {
    pub nodes: Vec<TemplateNode>,
}

impl TemplateTree {
    pub fn push(&mut self, node: TemplateNode) {
        self.nodes.push(node);
    }
}

//
// ============================================================================
// TemplateNode
// ============================================================================
//

/// A node in a template filesystem tree.
#[derive(Debug, Clone)]
pub enum TemplateNode {
    File(FileSpec),
    Directory(DirectorySpec),
}

//
// ============================================================================
// FileSpec
// ============================================================================
//

/// Declarative specification for a generated file.
#[derive(Debug, Clone)]
pub struct FileSpec {
    pub path: RelativePath,
    pub content: TemplateContent,
    pub permissions: Permissions,
}

impl FileSpec {
    pub fn new(path: impl Into<RelativePath>, content: TemplateContent) -> Self {
        Self {
            path: path.into(),
            content,
            permissions: Permissions::read_write(),
        }
    }

    pub fn executable(mut self) -> Self {
        self.permissions = Permissions::executable();
        self
    }
}

//
// ============================================================================
// DirectorySpec
// ============================================================================
//

/// Declarative specification for a generated directory.
#[derive(Debug, Clone)]
pub struct DirectorySpec {
    pub path: RelativePath,
    pub permissions: Permissions,
}

impl DirectorySpec {
    pub fn new(path: impl Into<RelativePath>) -> Self {
        Self {
            path: path.into(),
            permissions: Permissions::read_write(),
        }
    }
}

//
// ============================================================================
// TemplateContent
// ============================================================================
//

/// Declarative description of file content.
#[derive(Debug, Clone)]
pub enum TemplateContent {
    /// Static literal content.
    Literal(TemplateSource),

    /// Content requiring variable substitution.
    Parameterized(TemplateSource),

    /// External template resolved by a rendering engine.
    External(ContentTemplateId),
}

#[derive(Debug, Clone)]
pub enum TemplateSource {
    Static(&'static str),
    Owned(String),
}

/// Identifier for external template engines.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContentTemplateId(pub &'static str);

// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Architecture, Framework, Language, ProjectKind, RustFramework, Target};

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    fn target() -> Target {
        Target::builder()
            .language(Language::Rust)
            .framework(Framework::Rust(RustFramework::Axum))
            .unwrap()
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap()
    }

    fn matcher_all() -> TargetMatcher {
        TargetMatcher {
            language: Some(Language::Rust),
            framework: Some(Framework::Rust(RustFramework::Axum)),
            kind: Some(ProjectKind::WebBackend),
            architecture: Some(Architecture::Layered),
        }
    }

    fn matcher_wildcard() -> TargetMatcher {
        TargetMatcher {
            language: None,
            framework: None,
            kind: None,
            architecture: None,
        }
    }

    // -------------------------------------------------------------------------
    // TemplateId
    // -------------------------------------------------------------------------

    #[test]
    fn template_id_display() {
        let id = TemplateId::new("rust-web", "1.2.3".into());
        assert_eq!(id.to_string(), "rust-web@1.2.3");
    }

    #[test]
    fn template_id_equality() {
        let a = TemplateId::new("a", "1.0.0".into());
        let b = TemplateId::new("a", "1.0.0".into());
        let c = TemplateId::new("a", "2.0.0".into());

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    // -------------------------------------------------------------------------
    // TemplateRecord
    // -------------------------------------------------------------------------

    #[test]
    fn template_record_generates_uuid() {
        let record = TemplateRecord::new(
            TemplateId::new("test", "0.1.0".into()),
            Template {
                matcher: matcher_wildcard(),
                metadata: TemplateMetadata::new("test"),
                tree: TemplateTree::default(),
            },
        );

        // UUID is opaque but must exist
        assert_ne!(record.uuid, Uuid::nil());
    }

    // -------------------------------------------------------------------------
    // TargetMatcher::matches
    // -------------------------------------------------------------------------

    #[test]
    fn matcher_exact_match() {
        let matcher = matcher_all();
        assert!(matcher.matches(&target()));
    }

    #[test]
    fn matcher_language_mismatch() {
        let matcher = TargetMatcher {
            language: Some(Language::Python),
            ..matcher_all()
        };

        assert!(!matcher.matches(&target()));
    }

    #[test]
    fn matcher_wildcard_matches_anything() {
        let matcher = matcher_wildcard();
        assert!(matcher.matches(&target()));
    }

    #[test]
    fn matcher_partial_wildcard() {
        let matcher = TargetMatcher {
            language: Some(Language::Rust),
            framework: None,
            kind: Some(ProjectKind::WebBackend),
            architecture: None,
        };

        assert!(matcher.matches(&target()));
    }

    #[test]
    fn matcher_framework_wildcard_should_match() {
        let matcher = TargetMatcher {
            framework: None,
            ..matcher_all()
        };

        assert!(matcher.matches(&target()));
    }

    // -------------------------------------------------------------------------
    // TargetMatcher::specificity
    // -------------------------------------------------------------------------

    #[test]
    fn specificity_all_none() {
        assert_eq!(matcher_wildcard().specificity(), 0);
    }

    #[test]
    fn specificity_partial() {
        let matcher = TargetMatcher {
            language: Some(Language::Rust),
            framework: None,
            kind: None,
            architecture: None,
        };

        assert_eq!(matcher.specificity(), 1);
    }

    #[test]
    fn specificity_full() {
        assert_eq!(matcher_all().specificity(), 4);
    }

    #[test]
    fn specificity_monotonicity() {
        let less = TargetMatcher {
            language: Some(Language::Rust),
            framework: None,
            kind: None,
            architecture: None,
        };

        let more = TargetMatcher {
            language: Some(Language::Rust),
            framework: Some(Framework::Rust(RustFramework::Axum)),
            kind: None,
            architecture: None,
        };

        assert!(more.specificity() > less.specificity());
    }

    // -------------------------------------------------------------------------
    // TemplateTree
    // -------------------------------------------------------------------------

    #[test]
    fn template_tree_push_preserves_order() {
        let mut tree = TemplateTree::default();

        tree.push(TemplateNode::Directory(DirectorySpec::new(
            RelativePath::new("src"),
        )));
        tree.push(TemplateNode::File(FileSpec::new(
            RelativePath::new("src/main.rs"),
            TemplateContent::Literal(TemplateSource::Static("fn main() {}")),
        )));

        assert_eq!(tree.nodes.len(), 2);
    }

    // -------------------------------------------------------------------------
    // FileSpec
    // -------------------------------------------------------------------------

    #[test]
    fn file_spec_default_permissions() {
        let path = RelativePath::new("main.rs");
        let file = FileSpec::new(path, TemplateContent::Literal(TemplateSource::Static("")));

        assert!(file.permissions.readable());
        assert!(file.permissions.writable());
        assert!(!file.permissions.executable_flag());
    }

    #[test]
    fn file_spec_executable() {
        let path = RelativePath::new("build.sh");
        let file =
            FileSpec::new(path, TemplateContent::Literal(TemplateSource::Static(""))).executable();

        assert!(file.permissions.executable_flag());
    }

    // -------------------------------------------------------------------------
    // TemplateContent
    // -------------------------------------------------------------------------

    #[test]
    fn template_content_variants() {
        let literal = TemplateContent::Literal(TemplateSource::Static("hi"));
        let param = TemplateContent::Parameterized(TemplateSource::Owned("{{name}}".into()));
        let external = TemplateContent::External(ContentTemplateId("tera:main"));

        match literal {
            TemplateContent::Literal(_) => {}
            _ => panic!("expected literal"),
        }

        match param {
            TemplateContent::Parameterized(_) => {}
            _ => panic!("expected parameterized"),
        }

        match external {
            TemplateContent::External(_) => {}
            _ => panic!("expected external"),
        }
    }
}
