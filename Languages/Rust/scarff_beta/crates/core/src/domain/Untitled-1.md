//domain/mod.rs
//! Domain models for Scarff.
//!
//! This module contains the core business types and logic:
//! - Target: A validated project configuration
//! - Template: A reusable project recipe
//! - [`ProjectStructure`]: The output ready for writing
//! - Common types: Shared utilities

pub mod common;
mod errors;
mod project_structure;
mod render_context;
mod target;
mod template;
pub(crate) mod validator;

// Re-export common types
pub(crate) use common::{Permissions, RelativePath};
pub use errors::DomainError;

// Re-export project structure
pub(crate) use project_structure::{DirectoryToCreate, FileToWrite, FsEntry, ProjectStructure};

// Re-export render context
pub(crate) use render_context::RenderContext;

// Re-export target types
pub use target::{
Architecture, Framework, HasLanguage, Language, NoLanguage, ProjectKind, PythonFramework,
RustFramework, Target, TargetBuilder, TypeScriptFramework,
};

// Re-export template types
pub(crate) use template::{
ContentTemplateId, DirectorySpec, FileSpec, TargetMatcher, TargetMatcherBuilder, Template,
TemplateBuilder, TemplateContent, TemplateEngine, TemplateId, TemplateMetadata, TemplateNode,
TemplateRecord, TemplateSource, TemplateTree,
};

//domain/errors.rs
use std::path::PathBuf;

use thiserror::Error;

/// Domain-specific errors for Scarff's core types. #[derive(Debug, Error, Clone)]
pub enum DomainError {
// ========================================================================
// Language errors
// ========================================================================
/// Unsupported programming language #[error("Unsupported language '{language}'. Supported: rust, python, typescript")]
UnsupportedLanguage { language: String },

    // ========================================================================
    // ProjectKind errors
    // ========================================================================
    /// Unsupported project type
    #[error(
        "Unsupported project type '{kind}'. Supported: cli, backend, frontend, fullstack, worker"
    )]
    UnsupportedProjectKind { kind: String },

    /// Project type is incompatible with the specified language
    #[error(
        "Language '{language}' is not best used for type '{kind}'. This project type is better implemented with a different language ecosystem"
    )]
    ProjectKindLanguageMismatch { kind: String, language: String },

    // ========================================================================
    // Framework errors
    // ========================================================================
    /// Framework is incompatible with the specified language
    #[error(
        "Framework '{framework}' is not available for language '{language}'. This framework belongs to a different language ecosystem"
    )]
    FrameworkLanguageMismatch { framework: String, language: String },

    /// Framework doesn't support the project type
    #[error(
        "Framework '{framework}' does not support project type '{kind}'. Choose a compatible framework or different project type"
    )]
    FrameworkProjectKindMismatch { framework: String, kind: String },

    /// Framework is required for this project type but was not provided
    #[error(
        "A framework is required for project type '{kind}'. Specify a framework using --framework or choose a different project type"
    )]
    FrameworkRequired { kind: String },

    // ========================================================================
    // Architecture errors
    // ========================================================================
    /// Unsupported architecture style
    #[error("Unsupported architecture '{architecture}'. Supported: layered, mvc, clean")]
    UnsupportedArchitecture { architecture: String },

    /// Architecture is incompatible with the project type
    #[error(
        "Architecture '{architecture}' is not compatible with project type '{kind}'. Choose a compatible architecture or different project type"
    )]
    ArchitectureProjectKindMismatch { architecture: String, kind: String },

    /// Architecture is incompatible with the framework
    #[error(
        "Architecture '{architecture}' is not compatible with framework '{framework}'. Choose a compatible combination"
    )]
    ArchitectureFrameworkMismatch {
        architecture: String,
        framework: String,
    },

    // ========================================================================
    // Inference errors
    // ========================================================================
    /// Cannot infer a required field from provided inputs
    #[error("Cannot infer '{field}': {reason}. Please specify this field explicitly")]
    CannotInfer { field: String, reason: String },

    /// Multiple valid defaults exist and user must choose explicitly
    #[error("Ambiguous intent: {reason}. Suggestions: {}", suggestions.join(", "))]
    AmbiguousIntent {
        reason: String,
        suggestions: Vec<String>,
    },

    /// Inference was refused because the result would be surprising
    #[error("{message}. Suggestions: {}", suggestions.join(", "))]
    InferenceRefused {
        message: String,
        suggestions: Vec<String>,
    },

    // ========================================================================
    // Template errors
    // ========================================================================
    /// Template has no files or directories
    #[error("Template '{template_id}' has no files or directories")]
    TemplateEmptyTree { template_id: String },

    /// Template has duplicate paths
    #[error("Template '{template_id}' has duplicate path: {}", path.display())]
    TemplateDuplicatePath { template_id: String, path: PathBuf },

    /// Template contains absolute path
    #[error("Template '{template_id}' contains absolute path: {}", path.display())]
    TemplateAbsolutePath { template_id: String, path: PathBuf },

    /// Template validation failed (simple)
    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    /// Template validation failed (with metadata)
    #[error("Invalid template '{name}': {reason}")]
    InvalidTemplateWithMetadata { name: String, reason: String },

    // ========================================================================
    // ProjectStructure errors
    // ========================================================================
    /// Project structure errors
    #[error("Project structure error: {0}")]
    ProjectStructureError(String),

    // ========================================================================
    // General/Shared errors
    // ========================================================================
    /// Generic "not supported" error (fallback)
    #[error("This combination is not supported")]
    NotSupported,

}

impl DomainError {
/// Get actionable suggestions for fixing this error.
///
/// Returns a list of helpful suggestions that the CLI can display
/// to guide users toward fixing the issue.
///
/// # Examples
///
/// `rust,ignore
    /// match result {
    ///     Err(e) => {
    ///         eprintln!("Error: {}", e);
    ///         for suggestion in e.suggestions() {
    ///             eprintln!("  💡 {}", suggestion);
    ///         }
    ///     }
    ///     Ok(v) => { /* ... */ }
    /// }
    /// `
pub fn suggestions(&self) -> Vec<String> {
match self {
// Language errors
Self::UnsupportedLanguage { language } => vec![
                "Supported languages: rust, python, typescript".to_string(),
                format!("You provided: {}", language),
                "Use: scarff new <name> --language rust|python|typescript".to_string(),
            ],

            // ProjectKind errors
            Self::UnsupportedProjectKind { kind } => vec![
                "Supported project types:".to_string(),
                "  • cli       - Command-line applications".to_string(),
                "  • backend   - Web backend/API services".to_string(),
                "  • frontend  - Web frontend applications".to_string(),
                "  • fullstack - Full-stack web applications".to_string(),
                "  • worker    - Background workers/processors".to_string(),
                format!("You provided: {}", kind),
            ],

            Self::ProjectKindLanguageMismatch { kind, language } => vec![
                format!("{} projects are not commonly built with {}", kind, language),
                "Recommended combinations:".to_string(),
                "  • Rust     → CLI, backend, worker".to_string(),
                "  • Python   → Backend, fullstack, worker".to_string(),
                "  • TypeScript → Frontend, fullstack, backend".to_string(),
                "Run 'scarff list-templates' to see all available combinations".to_string(),
            ],

            // Framework errors
            Self::FrameworkRequired { kind } => vec![
                format!("{} projects require a framework to be specified", kind),
                "Add --framework <name> to your command".to_string(),
                "".to_string(),
                "Available frameworks by language:".to_string(),
                "  Rust:       axum, actix".to_string(),
                "  Python:     fastapi, django".to_string(),
                "  TypeScript: react, vue, nextjs (frontend/fullstack)".to_string(),
                "              express, nestjs (backend)".to_string(),
                "".to_string(),
                "Run 'scarff list-frameworks' for more details".to_string(),
            ],

            Self::FrameworkLanguageMismatch {
                framework,
                language,
            } => {
                let correct_frameworks = match language.as_str() {
                    "rust" => "axum, actix",
                    "python" => "fastapi, django",
                    "typescript" => "react, vue, nextjs, express, nestjs",
                    _ => "unknown",
                };

                vec![
                    format!("{} is not available for {}", framework, language),
                    format!("Available {} frameworks: {}", language, correct_frameworks),
                    "".to_string(),
                    "Example:".to_string(),
                    format!(
                        "  scarff new my-app --language {} --framework {}",
                        language,
                        correct_frameworks.split(',').next().unwrap_or("").trim()
                    ),
                ]
            }

            Self::FrameworkProjectKindMismatch { framework, kind } => vec![
                format!("{} doesn't support {} projects", framework, kind),
                "".to_string(),
                "Framework compatibility:".to_string(),
                "  • axum/actix    → backend".to_string(),
                "  • fastapi       → backend".to_string(),
                "  • django        → fullstack".to_string(),
                "  • react/vue     → frontend".to_string(),
                "  • nextjs        → fullstack".to_string(),
                "  • express/nestjs → backend".to_string(),
            ],

            // Architecture errors
            Self::UnsupportedArchitecture { architecture } => vec![
                "Supported architectures:".to_string(),
                "  • layered - Layered architecture (most flexible)".to_string(),
                "  • mvc     - Model-View-Controller (traditional web apps)".to_string(),
                "  • clean   - Clean/Hexagonal architecture (complex domains)".to_string(),
                format!("You provided: {}", architecture),
            ],

            Self::ArchitectureProjectKindMismatch { architecture, kind } => vec![
                format!(
                    "{} architecture is not compatible with {} projects",
                    architecture, kind
                ),
                "".to_string(),
                "Recommended architectures:".to_string(),
                "  • CLI projects     → layered, clean".to_string(),
                "  • Backend projects → layered, clean".to_string(),
                "  • Frontend projects → layered".to_string(),
                "  • Fullstack (Django) → mvc".to_string(),
                "  • Worker projects  → layered, clean".to_string(),
            ],

            Self::ArchitectureFrameworkMismatch {
                architecture,
                framework,
            } => vec![
                format!(
                    "{} architecture is not compatible with {}",
                    architecture, framework
                ),
                "This combination hasn't been implemented yet".to_string(),
                "Try a different architecture or framework".to_string(),
            ],

            // Inference errors
            Self::CannotInfer { field, reason } => vec![
                format!("Cannot automatically determine {}", field),
                reason.clone(),
                format!("Please specify --{} explicitly", field.to_lowercase()),
            ],

            Self::AmbiguousIntent {
                reason,
                suggestions,
            } => {
                let mut result = vec![
                    "Multiple valid options available".to_string(),
                    reason.clone(),
                    "".to_string(),
                    "Suggestions:".to_string(),
                ];
                for suggestion in suggestions {
                    result.push(format!("  • {}", suggestion));
                }
                result
            }

            Self::InferenceRefused {
                message,
                suggestions,
            } => {
                let mut result = vec![message.clone(), "".to_string(), "Suggestions:".to_string()];
                for suggestion in suggestions {
                    result.push(format!("  • {}", suggestion));
                }
                result
            }

            // Template errors
            Self::TemplateEmptyTree { template_id } => vec![
                format!("Template '{}' has no files or directories", template_id),
                "This is a bug in the template definition".to_string(),
                "Please report this issue".to_string(),
            ],

            Self::TemplateDuplicatePath { template_id, path } => vec![
                format!(
                    "Template '{}' contains duplicate path: {}",
                    template_id,
                    path.display()
                ),
                "This is a bug in the template definition".to_string(),
                "Please report this issue".to_string(),
            ],

            Self::TemplateAbsolutePath { template_id, path } => vec![
                format!(
                    "Template '{}' contains absolute path: {}",
                    template_id,
                    path.display()
                ),
                "Templates must use relative paths only".to_string(),
                "This is a bug in the template definition".to_string(),
            ],

            Self::InvalidTemplate(reason) => vec![
                "Template validation failed".to_string(),
                reason.clone(),
                "This template may be corrupted or incorrectly defined".to_string(),
            ],

            Self::InvalidTemplateWithMetadata { name, reason } => vec![
                format!("Template '{}' is invalid", name),
                reason.clone(),
                "This template may be corrupted or incorrectly defined".to_string(),
            ],

            // ProjectStructure errors
            Self::ProjectStructureError(msg) => vec![
                "Project structure validation failed".to_string(),
                msg.clone(),
                "This may indicate a bug in template rendering".to_string(),
                "Please report this issue with your target configuration".to_string(),
            ],

            // General
            Self::NotSupported => vec![
                "This combination is not currently supported".to_string(),
                "Run 'scarff list-templates' to see what's available".to_string(),
                "Consider creating a custom template for this use case".to_string(),
            ],
        }
    }

    /// Get a short, user-friendly error category.
    pub fn category(&self) -> &'static str {
        match self {
            Self::UnsupportedLanguage { .. } => "Language Error",
            Self::UnsupportedProjectKind { .. } | Self::ProjectKindLanguageMismatch { .. } => {
                "Project Type Error"
            }
            Self::FrameworkRequired { .. }
            | Self::FrameworkLanguageMismatch { .. }
            | Self::FrameworkProjectKindMismatch { .. } => "Framework Error",
            Self::UnsupportedArchitecture { .. }
            | Self::ArchitectureProjectKindMismatch { .. }
            | Self::ArchitectureFrameworkMismatch { .. } => "Architecture Error",
            Self::CannotInfer { .. }
            | Self::AmbiguousIntent { .. }
            | Self::InferenceRefused { .. } => "Configuration Error",
            Self::TemplateEmptyTree { .. }
            | Self::TemplateDuplicatePath { .. }
            | Self::TemplateAbsolutePath { .. }
            | Self::InvalidTemplate(_)
            | Self::InvalidTemplateWithMetadata { .. } => "Template Error",
            Self::ProjectStructureError(_) => "Structure Error",
            Self::NotSupported => "Unsupported",
        }
    }

}

#[cfg(test)]
mod tests {
use super::\*;

    #[test]
    fn error_has_suggestions() {
        let err = DomainError::UnsupportedLanguage {
            language: "java".to_string(),
        };

        let suggestions = err.suggestions();
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("Supported languages"));
    }

    #[test]
    fn error_has_category() {
        let err = DomainError::FrameworkRequired {
            kind: "backend".to_string(),
        };

        assert_eq!(err.category(), "Framework Error");
    }

    #[test]
    fn framework_mismatch_suggests_correct_frameworks() {
        let err = DomainError::FrameworkLanguageMismatch {
            framework: "django".to_string(),
            language: "rust".to_string(),
        };

        let suggestions = err.suggestions();
        assert!(suggestions.iter().any(|s| s.contains("axum")));
    }

}

//domain/common.rs
use std::fmt;
use std::path::{Path, PathBuf};

/// A filesystem path guaranteed to be **relative**.
///
/// This type encodes an important invariant:
/// templates and project structures must never contain absolute paths.
///
/// Why?
/// - Absolute paths break portability
/// - They can overwrite arbitrary locations
/// - They are almost always a bug in scaffolding systems
///
/// `RelativePath` is a _semantic guardrail_, not a filesystem abstraction. #[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelativePath(PathBuf);

impl RelativePath {
/// Create a new relative path.
///
/// # Panics
/// Panics if the provided path is absolute.
pub fn new(path: impl Into<PathBuf>) -> Self {
let path = path.into();
assert!(
!path.is_absolute(),
"RelativePath cannot be absolute: {path:?}"
);
Self(path)
}

    /// Try to create a relative path.
    ///
    /// This is the non-panicking variant.
    pub fn try_new(path: impl Into<PathBuf>) -> Result<Self, PathBuf> {
        let path = path.into();
        if path.is_absolute() {
            Err(path)
        } else {
            Ok(Self(path))
        }
    }

    /// Join a path segment onto this relative path.
    ///
    /// # Panics
    /// Panics if the joined path is absolute.
    pub fn join(&self, segment: impl AsRef<Path>) -> Self {
        let segment = segment.as_ref();
        assert!(
            !segment.is_absolute(),
            "cannot join absolute path to RelativePath"
        );
        Self(self.0.join(segment))
    }

    /// Borrow as a `Path`.
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Consume into a `PathBuf`.
    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }

}

impl AsRef<Path> for RelativePath {
fn as_ref(&self) -> &Path {
&self.0
}
}

impl From<&str> for RelativePath {
fn from(s: &str) -> Self {
RelativePath::new(s)
}
}

impl From<String> for RelativePath {
fn from(s: String) -> Self {
RelativePath::new(s)
}
}

impl fmt::Display for RelativePath {
fn fmt(&self, f: &mut fmt::Formatter<'\_>) -> fmt::Result {
write!(f, "{}", self.0.display())
}
}

/// Simplified permission model for generated artifacts.
///
/// This is a **capability model**, not a Unix permission model.
///
/// It answers:
/// - Can this file be read?
/// - Can it be modified?
/// - Can it be executed or entered? #[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Permissions {
readable: bool,
writable: bool,
executable: bool,
}

impl Permissions {
/// Read-only permissions.
pub const fn read_only() -> Self {
Self {
readable: true,
writable: false,
executable: false,
}
}

    /// Read and write permissions.
    pub const fn read_write() -> Self {
        Self {
            readable: true,
            writable: true,
            executable: false,
        }
    }

    /// Read and execute permissions.
    pub const fn executable() -> Self {
        Self {
            readable: true,
            writable: false,
            executable: true,
        }
    }

    /// Full permissions.
    pub const fn full() -> Self {
        Self {
            readable: true,
            writable: true,
            executable: true,
        }
    }

    // Getters

    pub const fn readable(&self) -> bool {
        self.readable
    }

    pub const fn writable(&self) -> bool {
        self.writable
    }

    pub const fn executable_flag(&self) -> bool {
        self.executable
    }

}

impl Default for Permissions {
fn default() -> Self {
Self::read_write()
}
}

#[cfg(test)]
mod tests {
use super::\*;

    // ---------------------------------------------------------------------
    // RelativePath
    // ---------------------------------------------------------------------

    #[test]
    fn relative_path_accepts_relative() {
        let p = RelativePath::new("src/main.rs");
        assert_eq!(p.as_path(), Path::new("src/main.rs"));
    }

    #[test]
    #[should_panic]
    fn relative_path_rejects_absolute() {
        RelativePath::new("/etc/passwd");
    }

    #[test]
    fn try_new_rejects_absolute() {
        let result = RelativePath::try_new("/etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn join_relative_path() {
        let base = RelativePath::new("src");
        let joined = base.join("main.rs");
        assert_eq!(joined.as_path(), Path::new("src/main.rs"));
    }

    #[test]
    #[should_panic]
    fn join_rejects_absolute_segment() {
        let base = RelativePath::new("src");
        base.join("/etc/passwd");
    }

    // ---------------------------------------------------------------------
    // Permissions
    // ---------------------------------------------------------------------

    #[test]
    fn permissions_defaults() {
        let p = Permissions::default();
        assert!(p.readable());
        assert!(p.writable());
        assert!(!p.executable_flag());
    }

    #[test]
    fn permissions_read_only() {
        let p = Permissions::read_only();
        assert!(p.readable());
        assert!(!p.writable());
        assert!(!p.executable_flag());
    }

    #[test]
    fn permissions_executable() {
        let p = Permissions::executable();
        assert!(p.readable());
        assert!(!p.writable());
        assert!(p.executable_flag());
    }

    #[test]
    fn permissions_full() {
        let p = Permissions::full();
        assert!(p.readable());
        assert!(p.writable());
        assert!(p.executable_flag());
    }

}

//domain/validator.rs
//! Validation logic for domain models.
//!
//! This module provides validation for:
//! - Target: Project configuration validation
//! - Template: Template structure validation
//! - ProjectStructure: Output structure validation

use crate::domain::{
DomainError, ProjectStructure, Target, Template, TemplateRecord,
target::{ActivelySupported, Compatible, LangCapable},
};

// ============================================================================
// Target Validation
// ============================================================================

/// Validate a target configuration.
///
/// Checks:
/// - Language is supported
/// - Framework is compatible with language and kind
/// - Architecture is compatible with framework and kind
/// - All required fields are present
///
/// # Errors
///
/// Returns a `DomainError` if validation fails.
pub fn validate_target(target: &Target) -> Result<(), DomainError> {
// Language is always set by builder, but check it's supported
if !target.language().is_supported() {
return Err(DomainError::UnsupportedLanguage {
language: target.language().to_string(),
});
}

    // Project kind validation
    if !target.kind().is_supported() {
        return Err(DomainError::UnsupportedProjectKind {
            kind: target.kind().to_string(),
        });
    }

    // Check language-kind compatibility
    if !target.kind().lang_capable(target.language()) {
        return Err(DomainError::ProjectKindLanguageMismatch {
            kind: target.kind().to_string(),
            language: target.language().to_string(),
        });
    }

    // Framework validation (if present)
    if let Some(framework) = target.framework() {
        // Check framework is supported
        if !framework.is_supported() {
            return Err(DomainError::FrameworkLanguageMismatch {
                framework: framework.to_string(),
                language: target.language().to_string(),
            });
        }

        // Check framework-language compatibility
        if framework.language() != target.language() {
            return Err(DomainError::FrameworkLanguageMismatch {
                framework: framework.to_string(),
                language: target.language().to_string(),
            });
        }

        // Check framework-kind compatibility
        if !framework.is_compatible((target.language(), target.kind())) {
            return Err(DomainError::FrameworkProjectKindMismatch {
                framework: framework.to_string(),
                kind: target.kind().to_string(),
            });
        }
    } else if target.kind().requires_framework() {
        // Framework is required but not provided
        return Err(DomainError::FrameworkRequired {
            kind: target.kind().to_string(),
        });
    }

    // Architecture validation
    if !target.architecture().is_supported() {
        return Err(DomainError::UnsupportedArchitecture {
            architecture: target.architecture().to_string(),
        });
    }

    // Check architecture compatibility
    if !target
        .architecture()
        .is_compatible((target.language(), target.kind(), target.framework()))
    {
        return Err(DomainError::ArchitectureProjectKindMismatch {
            architecture: target.architecture().to_string(),
            kind: target.kind().to_string(),
        });
    }

    Ok(())

}

// ============================================================================
// Template Validation
// ============================================================================

/// Validate a template.
///
/// Checks:
/// - Metadata is valid (non-empty name, version)
/// - Tree is not empty
/// - All paths are relative
/// - No duplicate paths
/// - Matcher is valid
///
/// # Errors
///
/// Returns a `DomainError` if validation fails.
pub fn validate_template(template: &Template) -> Result<(), DomainError> {
// Validate ID
if template.id.name().is_empty() {
return Err(DomainError::InvalidTemplateWithMetadata {
name: "unknown".to_string(),
reason: "Template ID name cannot be empty".to_string(),
});
}

    if template.id.version().is_empty() {
        return Err(DomainError::InvalidTemplateWithMetadata {
            name: template.id.name().to_string(),
            reason: "Template ID version cannot be empty".to_string(),
        });
    }
    // Validate metadata
    if template.metadata.name.is_empty() {
        return Err(DomainError::InvalidTemplateWithMetadata {
            name: template.id.name.to_string(),
            reason: "Template metadata name cannot be empty".to_string(),
        });
    }

    if template.metadata.version.is_empty() {
        return Err(DomainError::InvalidTemplateWithMetadata {
            name: template.metadata.name.to_string(),
            reason: "Template metadata version cannot be empty".to_string(),
        });
    }

    // Validate tree is not empty
    if template.tree.nodes.is_empty() {
        return Err(DomainError::TemplateEmptyTree {
            template_id: template.metadata.name.to_string(),
        });
    }

    // Validate paths
    let mut seen_paths = std::collections::HashSet::new();

    for node in &template.tree.nodes {
        let path = match node {
            crate::domain::TemplateNode::File(spec) => spec.path.as_path(),
            crate::domain::TemplateNode::Directory(spec) => spec.path.as_path(),
        };

        // Check for duplicates
        if !seen_paths.insert(path) {
            return Err(DomainError::TemplateDuplicatePath {
                template_id: template.metadata.name.to_string(),
                path: path.to_path_buf(),
            });
        }

        // Check not absolute
        if path.is_absolute() {
            return Err(DomainError::TemplateAbsolutePath {
                template_id: template.metadata.name.to_string(),
                path: path.to_path_buf(),
            });
        }
    }

    Ok(())

}

/// Validate a template record.
///
/// Checks:
/// - UUID is valid (not nil)
/// - ID is valid (non-empty name and version)
/// - Template is valid
///
/// # Errors
///
/// Returns a `DomainError` if validation fails.
pub fn validate_template_record(record: &TemplateRecord) -> Result<(), DomainError> {
// Check UUID is not nil
if record.uuid.is_nil() {
return Err(DomainError::InvalidTemplateWithMetadata {
name: record.template.id.to_string(),
reason: "Template UUID cannot be nil".to_string(),
});
}

    // Validate the template itself
    validate_template(&record.template)?;

    Ok(())

}

// ============================================================================
// ProjectStructure Validation
// ============================================================================

/// Validate a project structure.
///
/// Checks:
/// - Structure is not empty
/// - All paths are relative
/// - No duplicate paths
/// - Root path is set
///
/// # Errors
///
/// Returns a `DomainError` if validation fails.
pub fn validate_project_structure(structure: &ProjectStructure) -> Result<(), DomainError> {
// Check structure is not empty
if structure.entries.is_empty() {
return Err(DomainError::ProjectStructureError(
"Project structure is empty - no files or directories to create".to_string(),
));
}

    // Check root path is set and not empty
    if structure.root.as_os_str().is_empty() {
        return Err(DomainError::ProjectStructureError(
            "Project root path cannot be empty".to_string(),
        ));
    }

    // Validate paths
    let mut seen_paths = std::collections::HashSet::new();

    for entry in &structure.entries {
        let path = match entry {
            crate::domain::FsEntry::File(f) => &f.path,
            crate::domain::FsEntry::Directory(d) => &d.path,
        };

        // Check for duplicates
        if !seen_paths.insert(path) {
            return Err(DomainError::ProjectStructureError(format!(
                "Duplicate path in structure: {}",
                path.display()
            )));
        }

        // Check path is relative
        if path.is_absolute() {
            return Err(DomainError::ProjectStructureError(format!(
                "Absolute path not allowed in structure: {}",
                path.display()
            )));
        }
    }

    Ok(())

}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
use super::\*;
use crate::domain::{
Architecture, Framework, Language, ProjectKind, RustFramework, TargetMatcher, TemplateId,
TemplateMetadata, TemplateTree,
};

    #[test]
    fn validate_valid_target() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap();

        assert!(validate_target(&target).is_ok());
    }

    #[test]
    fn validate_target_with_framework() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .framework(Framework::Rust(RustFramework::Axum))
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap();

        assert!(validate_target(&target).is_ok());
    }

    #[test]
    fn validate_target_missing_required_framework() {
        // This should fail at builder level, but test validation anyway
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .build()
            .unwrap();

        // CLI doesn't require framework, should pass
        assert!(validate_target(&target).is_ok());
    }

    #[test]
    fn validate_empty_template_fails() {
        let template = Template {
            id: TemplateId::new("test", "".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata::new("test"),
            tree: TemplateTree::new(),
        };

        let result = validate_template(&template);
        assert!(result.is_err());
    }

    #[test]
    fn validate_template_with_absolute_path_fails() {
        use crate::domain::{
            FileSpec, RelativePath, TemplateContent, TemplateNode, TemplateSource,
        };

        let mut tree = TemplateTree::new();
        // This would panic at RelativePath::new, so we can't really test this
        // The type system prevents it

        // Test with empty name instead
        let template = Template {
            id: TemplateId::new("test", "0.1.0".to_string()),
            matcher: TargetMatcher::builder().build(),
            metadata: TemplateMetadata {
                name: "",
                description: "",
                version: "1.0.0",
                author: "",
                tags: vec![],
            },
            tree: TemplateTree::new().with_node(TemplateNode::File(FileSpec::new(
                "test.txt",
                TemplateContent::Literal(TemplateSource::Static("")),
            ))),
        };

        let result = validate_template(&template);
        assert!(result.is_err());
    }

}

//domain/target.rs
//! Target modeling with typestate builder pattern.
//!
//! This module provides the [`Target`] type, which represents a fully validated
//! project configuration. Targets are constructed using a builder pattern that
//! enforces compile-time guarantees about required fields.
//!
//! # Examples
//!
//! `rust
//! use scarff_core::{Target, Language, ProjectKind};
//!
//! // Minimal target - other fields inferred
//! let target = Target::builder()
//!     .language(Language::Rust)
//!     .build()?;
//!
//! // Fully specified target
//! let target = Target::builder()
//!     .language(Language::Rust)
//!     .kind(ProjectKind::WebBackend)
//!     .framework(Framework::Rust(RustFramework::Axum))
//!     .architecture(Architecture::Layered)
//!     .build()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! `

use std::{fmt, marker::PhantomData};

use crate::DomainError;

// ============================================================================
// Target (Final, Always Valid)
// ============================================================================

/// A fully validated project configuration.
///
/// `Target` represents a complete, validated project specification that has
/// passed all compatibility checks. It cannot be constructed directly - use
/// [`Target::builder()`] instead.
///
/// # Invariants
///
/// - Language is always set and supported
/// - If framework is set, it's compatible with the language
/// - Architecture is compatible with both framework and project type
/// - All inferred values are deterministic and documented
///
/// # Examples
///
/// `rust
/// use scarff_core::{Target, Language, ProjectKind};
///
/// let target = Target::builder()
///     .language(Language::Rust)
///     .kind(ProjectKind::Cli)
///     .build()?;
///
/// assert_eq!(target.language(), Language::Rust);
/// assert_eq!(target.kind(), ProjectKind::Cli);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ` #[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
/// Language of the project
pub language: Language,
/// Project type
pub kind: ProjectKind,
/// Framework, if any
pub framework: Option<Framework>,
/// Architecture
pub architecture: Architecture,
}

impl Target {
/// Create a new builder to construct a Target. #[must_use]
pub fn builder() -> TargetBuilder<NoLanguage> {
TargetBuilder::new()
}

    /// Get the language of this target.
    #[must_use]
    pub const fn language(&self) -> Language {
        self.language
    }

    /// Get the project type of this target.
    #[must_use]
    pub const fn kind(&self) -> ProjectKind {
        self.kind
    }

    /// Get the framework, if any.
    #[must_use]
    pub const fn framework(&self) -> Option<Framework> {
        self.framework
    }

    /// Get the architecture of this target.
    #[must_use]
    pub const fn architecture(&self) -> Architecture {
        self.architecture
    }

    // Preset methods for common configurations

    /// Create a Rust CLI application target.
    ///
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn rust_cli() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)?
            .build()
    }

    /// Create a Rust web backend with Axum.
    ///
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn rust_backend_axum() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::Rust)
            .kind(ProjectKind::WebBackend)?
            .framework(Framework::Rust(RustFramework::Axum))?
            .build()
    }

    /// Create a Python web backend with FastAPI.
    ///
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn python_backend_fastapi() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::Python)
            .kind(ProjectKind::WebBackend)?
            .framework(Framework::Python(PythonFramework::FastApi))?
            .build()
    }

    /// Create a TypeScript frontend with React.
    ///
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn typescript_frontend_react() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::TypeScript)
            .kind(ProjectKind::WebFrontend)?
            .framework(Framework::TypeScript(TypeScriptFramework::React))?
            .build()
    }

    // TODO: validate method to validate self

}

impl fmt::Display for Target {
fn fmt(&self, f: &mut fmt::Formatter<'\_>) -> fmt::Result {
write!(
f,
"{} {} ({}{})",
self.language,
self.kind,
self.architecture,
self.framework
.as_ref()
.map(|framework| format!(" + {framework}"))
.unwrap_or_default()
)
}
}

// ============================================================================
// Typestate Markers
// ============================================================================

/// Marker type indicating the builder has no language set yet.
pub struct NoLanguage;

/// Marker type indicating the builder has a language set.
pub struct HasLanguage;

// ============================================================================
// TargetBuilder (Typestate)
// ============================================================================

/// Builder for constructing validated [`Target`] instances.
pub struct TargetBuilder<L> {
language: Option<Language>,
framework: Option<Framework>,
kind: Option<ProjectKind>,
architecture: Option<Architecture>,
\_language_state: PhantomData<L>,
}

impl TargetBuilder<NoLanguage> {
/// Create a new builder. Language must be set before calling `build()`. #[must_use]
pub const fn new() -> Self {
Self {
language: None,
framework: None,
kind: None,
architecture: None,
\_language_state: PhantomData,
}
}

    /// Set the programming language (required).
    #[must_use]
    pub fn language(self, language: Language) -> TargetBuilder<HasLanguage> {
        TargetBuilder {
            language: Some(language),
            framework: self.framework,
            kind: self.kind,
            architecture: self.architecture,
            _language_state: PhantomData,
        }
    }

}

impl Default for TargetBuilder<NoLanguage> {
fn default() -> Self {
Self::new()
}
}

impl TargetBuilder<HasLanguage> {
/// Set the framework (optional). #[must_use]
pub fn framework(mut self, framework: Framework) -> Result<Self, DomainError> {
// Validate immediately
if let Some(lang) = self.language
&& framework.language() != lang
{
Err(DomainError::FrameworkLanguageMismatch {
framework: framework.to_string(),
language: lang.to_string(),
})?;
}
// if framework is required then it must be provided
// else if kind.requires_framework() && fram
self.framework = Some(framework);
Ok(self)
}

    /// Set the project type (optional).
    #[must_use]
    pub fn kind(mut self, kind: ProjectKind) -> Result<Self, DomainError> {
        if let Some(lang) = self.language
            && (!kind.is_supported() || !kind.lang_capable(lang))
        {
            Err(DomainError::ProjectKindLanguageMismatch {
                kind: kind.to_string(),
                language: lang.to_string(),
            })?;
        }
        self.kind = Some(kind);
        Ok(self)
    }

    /// Set the architecture (optional).
    #[must_use]
    pub fn architecture(mut self, architecture: Architecture) -> Result<Self, DomainError> {
        if let Some(lang) = self.language
            && let Some(kind) = self.kind
            && (!architecture.is_supported()
                || !architecture.is_compatible((lang, kind, self.framework)))
        {
            Err(DomainError::ArchitectureProjectKindMismatch {
                architecture: architecture.to_string(),
                kind: kind.to_string(),
            })?;
        }
        self.architecture = Some(architecture);
        Ok(self)
    }

    /// Finalize the builder and construct a validated [`Target`].
    ///
    /// This performs all validation and inference:
    /// 1. Validates language is supported
    /// 2. Infers or validates project type
    /// 3. Infers or validates framework (optional)
    /// 4. Infers or validates architecture
    /// 5. Checks all compatibility constraints
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Language is not supported
    /// - Framework is incompatible with language
    /// - Framework doesn't support the project type
    /// - Architecture is incompatible with framework or project type
    /// - Required values cannot be inferred
    pub fn build(self) -> Result<Target, DomainError> {
        let language = self
            .language
            .expect("HasLanguage state guarantees language is set");

        // Step 1: Validate language is supported
        if !language.is_supported() {
            return Err(DomainError::UnsupportedLanguage {
                language: language.to_string(),
            });
        }

        let (kind, framework, architecture) = self.parse(language)?;

        Ok(Target {
            language,
            kind,
            framework,
            architecture,
        })
    }

    /// Internal parser to validate and infer `kind`, framework, architecture
    ///
    /// ## Inference Strategy
    ///
    /// The inference follows this priority:
    /// 1. **`ProjectKind`**: Infer from language if not provided
    /// 2. **Framework**: Try to infer from (language, kind), but allow None for CLI/Worker
    /// 3. **Architecture**: Infer from (language, kind, framework)
    ///
    /// ## Key Rule
    /// Framework is **optional** for some project types (CLI, Worker).
    /// We only error if framework inference fails AND it's required for that project type.
    fn parse(
        self,
        language: Language,
    ) -> Result<(ProjectKind, Option<Framework>, Architecture), DomainError> {
        // =====================
        // 1️⃣ ProjectKind
        // =====================
        let kind = match self.kind {
            Some(k) => {
                if !k.is_supported() || !k.lang_capable(language) {
                    return Err(DomainError::ProjectKindLanguageMismatch {
                        kind: k.to_string(),
                        language: language.to_string(),
                    });
                }
                k
            }
            None => ProjectKind::infer_from(language).ok_or_else(|| DomainError::CannotInfer {
                field: "kind".into(),
                reason: format!("No default project type for {language}"),
            })?,
        };

        // =====================
        // 2️⃣ Framework (OPTIONAL for some types)
        // =====================
        let framework = if let Some(fw) = self.framework {
            if !fw.is_supported() {
                return Err(DomainError::FrameworkRequired {
                    kind: kind.to_string(),
                });
            }

            if !fw.is_compatible((language, kind)) {
                return Err(DomainError::FrameworkProjectKindMismatch {
                    framework: fw.to_string(),
                    kind: kind.to_string(),
                });
            }

            Some(fw)
        } else {
            let inferred = Framework::infer_from((language, kind));

            // Check if this project type REQUIRES a framework
            if inferred.is_none() && kind.requires_framework() {
                return Err(DomainError::FrameworkRequired {
                    kind: kind.to_string(),
                });
            }

            inferred
        };

        // =====================
        // 3️⃣ Architecture
        // =====================
        let architecture = match self.architecture {
            Some(arch) => {
                if !arch.is_supported() {
                    return Err(DomainError::UnsupportedArchitecture {
                        architecture: arch.to_string(),
                    });
                }

                if !arch.is_compatible((language, kind, framework)) {
                    return Err(DomainError::ArchitectureFrameworkMismatch {
                        architecture: arch.to_string(),
                        framework: framework.map_or_else(|| "none".to_string(), |f| f.to_string()),
                    });
                }

                arch
            }
            None => Architecture::infer_from((language, kind, framework)).ok_or_else(|| {
                DomainError::CannotInfer {
                    field: "architecture".to_string(),
                    reason: format!(
                        "Cannot infer architecture for {} {} {}",
                        language,
                        kind,
                        framework.map_or_else(|| "none".to_string(), |f| f.to_string())
                    ),
                }
            })?,
        };

        Ok((kind, framework, architecture))
    }

}

// ============================================================================
// Language
// ============================================================================

/// Supported programming languages. #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
Rust,
Python,
TypeScript,
}

impl Language { #[must_use]
pub const fn as_str(self) -> &'static str {
match self {
Self::Rust => "rust",
Self::Python => "python",
Self::TypeScript => "typescript",
}
}

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "rust" | "rs" => Some(Self::Rust),
            "python" | "py" => Some(Self::Python),
            "typescript" | "ts" => Some(Self::TypeScript),
            _ => None,
        }
    }

}

impl fmt::Display for Language {
fn fmt(&self, f: &mut fmt::Formatter<'\_>) -> fmt::Result {
f.write_str(self.as_str())
}
}

impl From<Language> for String {
fn from(l: Language) -> Self {
l.to_string()
}
}

impl ActivelySupported for Language {
const ALL: &'static [Self] = &[Self::Rust, Self::Python, Self::TypeScript];
}

// ============================================================================
// ProjectKind
// ============================================================================

/// Type of project being scaffolded. #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectKind {
Cli,
WebBackend,
WebFrontend,
Fullstack,
Worker,
}

impl ProjectKind { #[must_use]
pub const fn as_str(self) -> &'static str {
match self {
Self::Cli => "cli",
Self::WebBackend => "web-backend",
Self::WebFrontend => "web-frontend",
Self::Fullstack => "fullstack",
Self::Worker => "worker",
}
}

    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "cli" => Some(Self::Cli),
            "web-backend" | "backend" | "api" => Some(Self::WebBackend),
            "web-frontend" | "frontend" => Some(Self::WebFrontend),
            "fullstack" => Some(Self::Fullstack),
            "worker" => Some(Self::Worker),
            _ => None,
        }
    }

    /// Check if this project type requires a framework.
    ///
    /// CLI and Worker projects don't require frameworks.
    /// Web projects (backend, frontend, fullstack) do.
    #[must_use]
    pub const fn requires_framework(self) -> bool {
        matches!(self, Self::WebBackend | Self::WebFrontend | Self::Fullstack)
    }

}

impl From<ProjectKind> for String {
fn from(value: ProjectKind) -> Self {
value.as_str().to_string()
}
}

impl fmt::Display for ProjectKind {
fn fmt(&self, f: &mut fmt::Formatter<'\_>) -> fmt::Result {
write!(f, "{}", self.as_str())
}
}

impl ActivelySupported for ProjectKind {
const ALL: &'static [Self] = &[
Self::Cli,
Self::WebBackend,
Self::WebFrontend,
Self::Fullstack,
Self::Worker,
];
}

// ============================================================================
// Framework
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Framework {
Rust(RustFramework),
Python(PythonFramework),
TypeScript(TypeScriptFramework),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustFramework {
Axum,
Actix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PythonFramework {
FastApi,
Django,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeScriptFramework {
Express,
NestJs,
React,
Vue,
NextJs,
}

impl Framework { #[must_use]
pub const fn as_str(self) -> &'static str {
match self {
Self::Rust(RustFramework::Axum) => "axum",
Self::Rust(RustFramework::Actix) => "actix",
Self::Python(PythonFramework::FastApi) => "fastapi",
Self::Python(PythonFramework::Django) => "django",
Self::TypeScript(TypeScriptFramework::Express) => "express",
Self::TypeScript(TypeScriptFramework::NestJs) => "nestjs",
Self::TypeScript(TypeScriptFramework::React) => "react",
Self::TypeScript(TypeScriptFramework::Vue) => "vue",
Self::TypeScript(TypeScriptFramework::NextJs) => "nextjs",
}
}

    #[must_use]
    pub const fn language(self) -> Language {
        match self {
            Self::Rust(_) => Language::Rust,
            Self::Python(_) => Language::Python,
            Self::TypeScript(_) => Language::TypeScript,
        }
    }

    // #[must_use]
    // depending on framework we can infer the kind of project user should wants to build?
    // pub const fn kind(self)-> ProjectKind{
    //     match self{
    //         Framework::Rust(_rust_framework) => ProjectKind::WebBackend,
    //         Framework::Python(_python_framework) => Proj,
    //         Framework::TypeScript(type_script_framework) => todo!(),
    //     }
    // }

}

impl From<Framework> for String {
fn from(value: Framework) -> Self {
value.as_str().to_string()
}
}

impl fmt::Display for Framework {
fn fmt(&self, f: &mut fmt::Formatter<'\_>) -> fmt::Result {
write!(f, "{}", self.as_str())
}
}

impl ActivelySupported for Framework {
const ALL: &'static [Self] = &[
Framework::Rust(RustFramework::Axum),
Framework::Rust(RustFramework::Actix),
Framework::TypeScript(TypeScriptFramework::Express),
Framework::TypeScript(TypeScriptFramework::NestJs),
Framework::TypeScript(TypeScriptFramework::React),
Framework::TypeScript(TypeScriptFramework::Vue),
Framework::TypeScript(TypeScriptFramework::NextJs),
Framework::Python(PythonFramework::Django),
Framework::Python(PythonFramework::FastApi),
];
}

// ============================================================================
// Architecture
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
Layered,
MVC,
Clean,
}

impl Architecture {
pub fn as_str(self) -> &'static str {
match self {
Architecture::Layered => "layered",
Architecture::MVC => "mvc",
Architecture::Clean => "clean",
}
}
}

impl From<Architecture> for String {
fn from(value: Architecture) -> Self {
value.as_str().to_string()
}
}

impl fmt::Display for Architecture {
fn fmt(&self, f: &mut fmt::Formatter<'\_>) -> fmt::Result {
write!(f, "{}", self.as_str())
}
}

impl ActivelySupported for Architecture {
const ALL: &'static [Self] = &[
Architecture::Layered,
Architecture::MVC,
Architecture::Clean,
];
}

// ============================================================================
// Traits
// ============================================================================

pub trait ActivelySupported: Sized + PartialEq + 'static {
const ALL: &'static [Self];

    fn is_supported(&self) -> bool {
        Self::ALL.contains(self)
    }

}

pub trait ActivelySupportedExt: ActivelySupported {
fn active() -> &'static [Self] {
Self::ALL
}
}

impl<T: ActivelySupported> ActivelySupportedExt for T {}

pub trait LangCapable {
fn lang_capable(&self, language: Language) -> bool;
fn capable_languages(self) -> Vec<Language>;
}

#[derive(Debug, PartialEq, Eq)]
struct LangCapableProjects {
language: Language,
p_types: &'static [ProjectKind],
}

const LANG_CAPABILITIES: &[LangCapableProjects] = &[
LangCapableProjects {
language: Language::Rust,
p_types: &[
ProjectKind::Cli,
ProjectKind::WebBackend,
ProjectKind::Worker,
],
},
LangCapableProjects {
language: Language::Python,
p_types: &[
ProjectKind::Cli,
ProjectKind::Fullstack,
ProjectKind::WebBackend,
ProjectKind::Worker,
],
},
LangCapableProjects {
language: Language::TypeScript,
p_types: &[
ProjectKind::WebFrontend,
ProjectKind::Fullstack,
ProjectKind::WebBackend,
ProjectKind::Worker,
],
},
];

impl LangCapable for ProjectKind {
fn lang_capable(&self, language: Language) -> bool {
if !language.is_supported() {
return false;
}

        if !self.is_supported() {
            return false;
        }

        LANG_CAPABILITIES
            .iter()
            .find(|cap| cap.language == language)
            .is_some_and(|cap| cap.p_types.contains(self))
    }

    fn capable_languages(self) -> Vec<Language> {
        LANG_CAPABILITIES
            .iter()
            .filter(|cap| cap.language.is_supported())
            .filter(|cap| cap.p_types.contains(&self))
            .map(|cap| cap.language)
            .collect()
    }

}

pub trait Compatible {
type Context;
fn is_compatible(&self, ctx: Self::Context) -> bool;
fn get_compatible(&self) -> Option<Vec<Self::Context>>;
}

impl Compatible for Framework {
type Context = (Language, ProjectKind);

    fn is_compatible(&self, ctx: Self::Context) -> bool {
        matches!(
            (self, ctx),
            (
                Framework::Rust(RustFramework::Axum | RustFramework::Actix),
                (Language::Rust, ProjectKind::WebBackend),
            ) | (
                Framework::TypeScript(TypeScriptFramework::Express | TypeScriptFramework::NestJs),
                (Language::TypeScript, ProjectKind::WebBackend),
            ) | (
                Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue),
                (Language::TypeScript, ProjectKind::WebFrontend),
            ) | (
                Framework::TypeScript(TypeScriptFramework::NextJs),
                (Language::TypeScript, ProjectKind::Fullstack),
            ) | (
                Framework::Python(PythonFramework::FastApi),
                (Language::Python, ProjectKind::WebBackend),
            ) | (
                Framework::Python(PythonFramework::Django),
                (Language::Python, ProjectKind::Fullstack),
            )
        )
    }

    fn get_compatible(&self) -> Option<Vec<Self::Context>> {
        let contexts = match self {
            Framework::Rust(RustFramework::Axum) => vec![(Language::Rust, ProjectKind::WebBackend)],
            Framework::Rust(RustFramework::Actix) => {
                vec![(Language::Rust, ProjectKind::WebBackend)]
            }
            Framework::TypeScript(TypeScriptFramework::Express | TypeScriptFramework::NestJs) => {
                vec![(Language::TypeScript, ProjectKind::WebBackend)]
            }
            Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue) => {
                vec![(Language::TypeScript, ProjectKind::WebFrontend)]
            }
            Framework::TypeScript(TypeScriptFramework::NextJs) => {
                vec![(Language::TypeScript, ProjectKind::Fullstack)]
            }
            Framework::Python(PythonFramework::FastApi) => {
                vec![(Language::Python, ProjectKind::WebBackend)]
            }
            Framework::Python(PythonFramework::Django) => {
                vec![(Language::Python, ProjectKind::Fullstack)]
            }
        };

        Some(contexts)
    }

}

impl Compatible for Architecture {
type Context = (Language, ProjectKind, Option<Framework>);

    fn is_compatible(&self, ctx: Self::Context) -> bool {
        match (self, ctx) {
            // Layered architecture - works with most combinations
            (
                Architecture::Layered,
                (Language::Rust, ProjectKind::Cli | ProjectKind::Worker, None),
            ) => true,
            (
                Architecture::Layered,
                (
                    Language::Rust,
                    ProjectKind::WebBackend,
                    Some(Framework::Rust(RustFramework::Axum | RustFramework::Actix)),
                ),
            ) => true,
            (
                Architecture::Layered,
                (
                    Language::TypeScript,
                    ProjectKind::WebBackend,
                    Some(Framework::TypeScript(
                        TypeScriptFramework::Express | TypeScriptFramework::NestJs,
                    )),
                ),
            ) => true,
            (
                Architecture::Layered,
                (
                    Language::TypeScript,
                    ProjectKind::Fullstack,
                    Some(Framework::TypeScript(TypeScriptFramework::NextJs)),
                ),
            ) => true,
            (
                Architecture::Layered,
                (
                    Language::Python,
                    ProjectKind::WebBackend,
                    Some(Framework::Python(PythonFramework::FastApi)),
                ),
            ) => true,

            // MVC - Django only
            (
                Architecture::MVC,
                (
                    Language::Python,
                    ProjectKind::Fullstack,
                    Some(Framework::Python(PythonFramework::Django)),
                ),
            ) => true,

            _ => false,
        }
    }

    fn get_compatible(&self) -> Option<Vec<Self::Context>> {
        let contexts = match self {
            Architecture::Layered => vec![
                (Language::Rust, ProjectKind::Cli, None),
                (Language::Rust, ProjectKind::Worker, None),
                (
                    Language::Rust,
                    ProjectKind::WebBackend,
                    Some(Framework::Rust(RustFramework::Axum)),
                ),
                (
                    Language::TypeScript,
                    ProjectKind::WebBackend,
                    Some(Framework::TypeScript(TypeScriptFramework::Express)),
                ),
                (
                    Language::Python,
                    ProjectKind::WebBackend,
                    Some(Framework::Python(PythonFramework::FastApi)),
                ),
            ],
            Architecture::MVC => vec![(
                Language::Python,
                ProjectKind::Fullstack,
                Some(Framework::Python(PythonFramework::Django)),
            )],
            Architecture::Clean => vec![],
        };

        Some(contexts)
    }

}

trait Infer {
type Context;
fn infer_from(ctx: Self::Context) -> Option<Self>
where
Self: Sized;
}

impl Infer for ProjectKind {
type Context = Language;

    fn infer_from(ctx: Self::Context) -> Option<Self> {
        match ctx {
            Language::Rust => Some(ProjectKind::Cli),
            Language::TypeScript => Some(ProjectKind::WebFrontend),
            Language::Python => Some(ProjectKind::WebBackend),
        }
    }

}

impl Infer for Framework {
type Context = (Language, ProjectKind);

    fn infer_from(ctx: Self::Context) -> Option<Self> {
        match ctx {
            // Rust
            (Language::Rust, ProjectKind::WebBackend) => Some(Framework::Rust(RustFramework::Axum)),
            (Language::Rust, ProjectKind::Cli | ProjectKind::Worker) => None, // No framework needed

            // TypeScript
            (Language::TypeScript, ProjectKind::WebBackend) => {
                Some(Framework::TypeScript(TypeScriptFramework::Express))
            }
            (Language::TypeScript, ProjectKind::WebFrontend) => {
                Some(Framework::TypeScript(TypeScriptFramework::React))
            }
            (Language::TypeScript, ProjectKind::Fullstack) => {
                Some(Framework::TypeScript(TypeScriptFramework::NextJs))
            }

            // Python
            (Language::Python, ProjectKind::WebBackend) => {
                Some(Framework::Python(PythonFramework::FastApi))
            }
            (Language::Python, ProjectKind::Fullstack) => {
                Some(Framework::Python(PythonFramework::Django))
            }
            (Language::Python, ProjectKind::Cli | ProjectKind::Worker) => None, // No framework needed

            _ => None,
        }
    }

}

impl Infer for Architecture {
type Context = (Language, ProjectKind, Option<Framework>);

    fn infer_from(ctx: Self::Context) -> Option<Self> {
        match ctx {
            // Rust - Layered for everything
            (Language::Rust, _, _) => Some(Architecture::Layered),

            // TypeScript
            (Language::TypeScript, _, Some(Framework::TypeScript(_))) => {
                Some(Architecture::Layered)
            }

            // Python
            (
                Language::Python,
                ProjectKind::Fullstack,
                Some(Framework::Python(PythonFramework::Django)),
            ) => Some(Architecture::MVC),
            (Language::Python, _, Some(Framework::Python(PythonFramework::FastApi))) => {
                Some(Architecture::Layered)
            }

            _ => Some(Architecture::Layered), // Default fallback
        }
    }

}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
use super::\*;

    #[test]
    fn target_builder_requires_language() {
        let target = Target::builder().language(Language::Rust).build().unwrap();
        println!("{target:?}");
        assert_eq!(target.language(), Language::Rust);
    }

    #[test]
    fn target_with_defaults() {
        let target = Target::builder().language(Language::Rust).build().unwrap();
        assert_eq!(target.language(), Language::Rust);
        assert_eq!(target.kind(), ProjectKind::Cli);
        assert_eq!(target.architecture(), Architecture::Layered);
        assert_eq!(target.framework(), None); // CLI doesn't need framework
    }

    #[test]
    fn target_explicit_all_fields() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .framework(Framework::Rust(RustFramework::Axum))
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(target.language(), Language::Rust);
        assert_eq!(target.kind(), ProjectKind::WebBackend);
        assert_eq!(
            target.framework(),
            Some(Framework::Rust(RustFramework::Axum))
        );
        assert_eq!(target.architecture(), Architecture::Layered);
    }

    #[test]
    #[should_panic]
    fn target_rejects_incompatible_framework() {
        let result = Target::builder()
            .language(Language::Rust)
            .framework(Framework::Python(PythonFramework::Django))
            .inspect_err(|e| eprintln!("{e:?}"))
            .expect("error building  result becaus eof framework mismatch")
            .build();

        println!("{result:?}");
        assert!(!result.is_err(), "because unwrapped");
    }

    #[test]
    fn language_parse() {
        assert_eq!(Language::parse("rust"), Some(Language::Rust));
        assert_eq!(Language::parse("rs"), Some(Language::Rust));
        assert_eq!(Language::parse("python"), Some(Language::Python));
        assert_eq!(Language::parse("py"), Some(Language::Python));
        assert_eq!(Language::parse("invalid"), None);
    }

    #[test]
    fn preset_rust_cli() {
        let target = Target::rust_cli().unwrap();
        assert_eq!(target.language(), Language::Rust);
        assert_eq!(target.kind(), ProjectKind::Cli);
    }

    #[test]
    fn preset_rust_backend_axum() {
        let target = Target::rust_backend_axum().unwrap();
        assert_eq!(target.language(), Language::Rust);
        assert_eq!(target.kind(), ProjectKind::WebBackend);
        assert_eq!(
            target.framework(),
            Some(Framework::Rust(RustFramework::Axum))
        );
    }

    #[test]
    fn build_partial_target_with_inference() {
        let target = TargetBuilder::new()
            .language(Language::TypeScript)
            .kind(ProjectKind::Fullstack)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(target.language, Language::TypeScript);
        assert_eq!(target.kind, ProjectKind::Fullstack);
        assert_eq!(
            target.framework,
            Some(Framework::TypeScript(TypeScriptFramework::NextJs))
        );
        assert!(matches!(target.architecture, Architecture::Layered));
    }

    #[test]
    fn infer_framework_for_web_backend() {
        let target = Target::builder()
            .language(Language::Python)
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            target.framework(),
            Some(Framework::Python(PythonFramework::FastApi))
        );
    }

    #[test]
    fn cli_does_not_require_framework() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(target.framework(), None);
    }

    #[test]
    fn worker_does_not_require_framework() {
        let target = Target::builder()
            .language(Language::Python)
            .kind(ProjectKind::Worker)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(target.framework(), None);
    }

    #[test]
    fn web_backend_requires_framework_if_not_inferable() {
        // This should succeed because FastAPI can be inferred
        let result = Target::builder()
            .language(Language::Python)
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .build();

        assert!(result.is_ok());
    }

}

//domain/template.rs
//! Template domain model and system.
//!
//! This module defines the complete template system for Scarff:
//! - Template definitions (what to generate)
//! - Template matching (when to use a template)
//! - Template metadata (for discovery and documentation)
//!
//! ## Architecture
//!
//! `text
//! TemplateRecord (storage wrapper)
//!   └─ Template (declarative recipe)
//!       ├─ TargetMatcher (when to apply)
//!       ├─ TemplateMetadata (human info)
//!       └─ TemplateTree (what to generate)
//!           └─ TemplateNode[] (files & dirs)
//! `
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
/// `rust,ignore
/// struct MyEngine;
///
/// impl TemplateEngine for MyEngine {
///     fn resolve(&self, target: &Target) -> Result<TemplateRecord, DomainError> {
///         // Find best matching template
///         todo!()
///     }
/// }
/// `
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
/// `rust
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
/// ` #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
/// _ `name` - Template name (e.g., "rust-cli-layered")
/// _ `version` - Semantic version string (e.g., "1.0.0")
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
fn fmt(&self, f: &mut fmt::Formatter<'\_>) -> fmt::Result {
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
/// internal consistency. #[derive(Debug, Clone)]
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
/// `rust,ignore
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
/// ` #[derive(Debug, Clone)]
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

/// Builder for constructing templates. #[derive(Default)]
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
/// `rust,ignore
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
/// ` #[derive(Debug, Clone, PartialEq, Eq)]
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
/// Post-MVP: Can be changed to `String` for dynamic templates. #[derive(Debug, Clone)]
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
/// Order matters for dependencies (e.g., create directories before files). #[derive(Debug, Clone, Default)]
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

/// A node in a template filesystem tree. #[derive(Debug, Clone)]
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
/// `rust,ignore
/// let spec = FileSpec::new(
///     "src/main.rs",
///     TemplateContent::Literal(TemplateSource::Static("fn main() {}"))
/// ).executable();
/// ` #[derive(Debug, Clone)]
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

/// Declarative specification for a generated directory. #[derive(Debug, Clone)]
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
/// - **External**: Reference to external template engine (Tera, Handlebars) #[derive(Debug, Clone)]
pub enum TemplateContent {
/// Static literal content (no variables).
Literal(TemplateSource),

    /// Content requiring variable substitution.
    Parameterized(TemplateSource),

    /// External template resolved by a rendering engine.
    External(ContentTemplateId),

}

/// Source of template content. #[derive(Debug, Clone)]
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

/// Identifier for external template engines. #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

//domain/project_structure.rs
//! ProjectStructure - the output of template rendering.

use std::path::PathBuf;

use crate::domain::{common::Permissions, validator};

use super::DomainError;

// ============================================================================
// ProjectStructure
// ============================================================================

/// Fully resolved, ready-to-write project layout.
///
/// This is the output of template resolution + rendering.
/// Contains everything needed to write a project to disk. #[derive(Debug, Clone)]
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
        permissions: Permissions,
    ) {
        self.entries.push(FsEntry::File(FileToWrite {
            path: path.into(),
            content,
            permissions,
        }));
    }

    /// Add a directory to the structure (mutable).
    pub(crate) fn add_directory(&mut self, path: impl Into<PathBuf>, permissions: Permissions) {
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
        permissions: Permissions,
    ) -> Self {
        self.add_file(path, content, permissions);
        self
    }

    /// Add a directory to the structure (builder style).
    pub(crate) fn with_directory(
        mut self,
        path: impl Into<PathBuf>,
        permissions: Permissions,
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
        // 4. Validate structure
        validator::validate_project_structure(self)?;

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

/// A filesystem entry (file or directory). #[derive(Debug, Clone)]
pub(crate) enum FsEntry {
File(FileToWrite),
Directory(DirectoryToCreate),
}

// ============================================================================
// FileToWrite
// ============================================================================

/// A file to be written to disk. #[derive(Debug, Clone)]
pub(crate) struct FileToWrite {
pub path: PathBuf,
pub content: String,
pub permissions: Permissions,
}

impl FileToWrite {
pub(crate) fn new(path: impl Into<PathBuf>, content: String, permissions: Permissions) -> Self {
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

/// A directory to be created on disk. #[derive(Debug, Clone)]
pub(crate) struct DirectoryToCreate {
pub path: PathBuf,
pub permissions: Permissions,
}

impl DirectoryToCreate {
pub(crate) fn new(path: impl Into<PathBuf>, permissions: Permissions) -> Self {
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
use super::\*;

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
            Permissions::read_write(),
        );

        assert_eq!(structure.entries.len(), 1);
        assert!(matches!(structure.entries[0], FsEntry::File(_)));
    }

    #[test]
    fn project_structure_add_directory() {
        let mut structure = ProjectStructure::new("/tmp/test");
        structure.add_directory("src", Permissions::read_write());

        assert_eq!(structure.entries.len(), 1);
        assert!(matches!(structure.entries[0], FsEntry::Directory(_)));
    }

    #[test]
    fn project_structure_builder_style() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", Permissions::read_write())
            .with_file(
                "src/main.rs",
                "fn main() {}".to_string(),
                Permissions::read_write(),
            )
            .with_file(
                "Cargo.toml",
                "[package]".to_string(),
                Permissions::read_write(),
            );

        assert_eq!(structure.entries.len(), 3);
        assert_eq!(structure.file_count(), 2);
        assert_eq!(structure.directory_count(), 1);
    }

    #[test]
    fn project_structure_validate_success() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", Permissions::read_write())
            .with_file("src/main.rs", "".to_string(), Permissions::read_write());

        assert!(structure.validate().is_ok());
    }

    #[test]
    fn project_structure_validate_duplicate_fails() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_file("main.rs", "".to_string(), Permissions::read_write())
            .with_file("main.rs", "".to_string(), Permissions::read_write());

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
        structure.add_file("/absolute/path", "".to_string(), Permissions::read_write());

        let result = structure.validate();
        assert!(result.is_err());
    }

    #[test]
    fn project_structure_files_iterator() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", Permissions::read_write())
            .with_file("main.rs", "".to_string(), Permissions::read_write())
            .with_file("lib.rs", "".to_string(), Permissions::read_write());

        let files: Vec<_> = structure.files().collect();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn project_structure_directories_iterator() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", Permissions::read_write())
            .with_directory("tests", Permissions::read_write())
            .with_file("main.rs", "".to_string(), Permissions::read_write());

        let dirs: Vec<_> = structure.directories().collect();
        assert_eq!(dirs.len(), 2);
    }

    #[test]
    fn file_to_write_new() {
        let file = FileToWrite::new("test.txt", "content".to_string(), Permissions::read_write());
        assert_eq!(file.path, PathBuf::from("test.txt"));
        assert_eq!(file.content, "content");
        assert!(!file.is_empty());
        assert_eq!(file.size(), 7);
    }

    #[test]
    fn file_to_write_empty() {
        let file = FileToWrite::new("empty.txt", String::new(), Permissions::read_write());
        assert!(file.is_empty());
        assert_eq!(file.size(), 0);
    }

    #[test]
    fn directory_to_create_new() {
        let dir = DirectoryToCreate::new("src", Permissions::read_write());
        assert_eq!(dir.path, PathBuf::from("src"));
    }

    #[test]
    fn project_structure_entry_count() {
        let structure = ProjectStructure::new("/tmp/test")
            .with_directory("src", Permissions::read_write())
            .with_file("main.rs", "".to_string(), Permissions::read_write())
            .with_file("lib.rs", "".to_string(), Permissions::read_write());

        assert_eq!(structure.entry_count(), 3);
        assert_eq!(structure.file_count(), 2);
        assert_eq!(structure.directory_count(), 1);
    }

}

//domain/render_context.rs
//! Context for rendering templates with variables.

use std::collections::HashMap;

/// Context containing variables for template rendering.
///
/// Provides standard variables (project name, year, etc.) plus custom variables. #[derive(Debug, Clone)]
pub struct RenderContext {
variables: HashMap<String, String>,
}

impl RenderContext {
/// Create a new render context with a project name.
///
/// Standard variables are automatically populated:
/// - `PROJECT_NAME`: Original project name
/// - `PROJECT_NAME_SNAKE`: snake_case version
/// - `PROJECT_NAME_KEBAB`: kebab-case version
/// - `PROJECT_NAME_PASCAL`: PascalCase version
/// - `YEAR`: Current year (for copyright notices)
pub fn new(project_name: impl Into<String>) -> Self {
let mut variables = HashMap::new();
let project_name = project_name.into();

        // Core variables
        variables.insert("PROJECT_NAME".to_string(), project_name.clone());
        variables.insert(
            "PROJECT_NAME_SNAKE".to_string(),
            to_snake_case(&project_name),
        );
        variables.insert(
            "PROJECT_NAME_KEBAB".to_string(),
            to_kebab_case(&project_name),
        );
        variables.insert(
            "PROJECT_NAME_PASCAL".to_string(),
            to_pascal_case(&project_name),
        );
        variables.insert("YEAR".to_string(), current_year());

        Self { variables }
    }

    /// Add a custom variable.
    ///
    /// Builder-style method for chaining.
    pub fn with_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(key.into(), value.into());
        self
    }

    /// Set a custom variable (mutable method).
    pub fn set_var(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }

    /// Get a variable value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }

    /// Check if a variable exists.
    pub fn has(&self, key: &str) -> bool {
        self.variables.contains_key(key)
    }

    /// Get all variables as a map (for template engines).
    pub fn all(&self) -> &HashMap<String, String> {
        &self.variables
    }

    /// Render a template string by replacing {{VARIABLE}} placeholders.
    ///
    /// Simple implementation for MVP. Can be replaced with a proper template engine later.
    pub fn render(&self, template: &str) -> String {
        let mut result = template.to_string();

        for (key, value) in &self.variables {
            let placeholder = format!("{{{{{key}}}}}");
            result = result.replace(&placeholder, value);
        }

        result
    }

}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert a string to snake*case.
///
/// Rules:
/// - Replace hyphens and spaces with underscores
/// - Convert to lowercase
fn to_snake_case(s: &str) -> String {
split_words(s).join("*")
}

/// Convert a string to kebab-case.
///
/// Rules:
/// - Replace underscores and spaces with hyphens
/// - Convert to lowercase
fn to_kebab_case(s: &str) -> String {
split_words(s).join("-")
}

/// Convert a string to PascalCase.
///
/// Rules:
/// - Split on hyphens, underscores, and spaces
/// - Capitalize first letter of each word
/// - Join without separators
fn to_pascal_case(s: &str) -> String {
split_words(s)
.into_iter()
.map(|w| {
let mut chars = w.chars();
match chars.next() {
Some(first) => {
let mut out = String::new();
out.extend(first.to_uppercase());
out.push_str(chars.as_str());
out
}
None => String::new(),
}
})
.collect()
}

fn split_words(input: &str) -> Vec<String> {
let mut words = Vec::new();
let mut current = String::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '_' || c == '-' || c.is_whitespace() {
            if !current.is_empty() {
                words.push(current.to_lowercase());
                current.clear();
            }
            continue;
        }

        // Word boundary: lower -> upper (myAwesome)
        if let Some(next) = chars.peek() {
            if c.is_lowercase() && next.is_uppercase() {
                current.push(c);
                words.push(current.to_lowercase());
                current.clear();
                continue;
            }

            // Acronym boundary: HTTPServer -> HTTP + Server
            if c.is_uppercase()
                && next.is_uppercase()
                && chars.clone().nth(1).map_or(false, |n| n.is_lowercase())
            {
                current.push(c);
                words.push(current.to_lowercase());
                current.clear();
                continue;
            }
        }

        current.push(c);
    }

    if !current.is_empty() {
        words.push(current.to_lowercase());
    }

    words

}

/// Get the current year as a string.
///
/// For now, returns a placeholder. In production, use `chrono` or `time` crate.
fn current_year() -> String {
// TODO: Use chrono::Utc::now().year() in production
"2026".to_string()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
use super::\*;

    #[test]
    fn render_context_standard_variables() {
        let ctx = RenderContext::new("my-awesome-project");

        assert_eq!(ctx.get("PROJECT_NAME"), Some("my-awesome-project"));
        assert_eq!(ctx.get("PROJECT_NAME_SNAKE"), Some("my_awesome_project"));
        assert_eq!(ctx.get("PROJECT_NAME_KEBAB"), Some("my-awesome-project"));
        assert_eq!(ctx.get("PROJECT_NAME_PASCAL"), Some("MyAwesomeProject"));
        assert_eq!(ctx.get("YEAR"), Some("2026"));
    }

    #[test]
    fn render_context_custom_variables() {
        let ctx = RenderContext::new("test-project")
            .with_var("AUTHOR", "John Doe")
            .with_var("LICENSE", "MIT");

        assert_eq!(ctx.get("AUTHOR"), Some("John Doe"));
        assert_eq!(ctx.get("LICENSE"), Some("MIT"));
    }

    #[test]
    fn render_context_set_var_mutable() {
        let mut ctx = RenderContext::new("test");
        ctx.set_var("CUSTOM", "value");
        assert_eq!(ctx.get("CUSTOM"), Some("value"));
    }

    #[test]
    fn render_context_has() {
        let ctx = RenderContext::new("test");
        assert!(ctx.has("PROJECT_NAME"));
        assert!(!ctx.has("NONEXISTENT"));
    }

    #[test]
    fn render_simple_template() {
        let ctx = RenderContext::new("my-project");
        let template = "Project: {{PROJECT_NAME}}, Year: {{YEAR}}";
        let result = ctx.render(template);
        assert_eq!(result, "Project: my-project, Year: 2026");
    }

    #[test]
    fn render_multiple_occurrences() {
        let ctx = RenderContext::new("test");
        let template = "{{PROJECT_NAME}} {{PROJECT_NAME}} {{PROJECT_NAME}}";
        let result = ctx.render(template);
        assert_eq!(result, "test test test");
    }

    #[test]
    fn to_snake_case_conversions() {
        assert_eq!(to_snake_case("my-project"), "my_project");
        assert_eq!(to_snake_case("my project"), "my_project");
        assert_eq!(to_snake_case("MyProject"), "my_project");
        assert_eq!(to_snake_case("my_project"), "my_project");
    }

    #[test]
    fn to_kebab_case_conversions() {
        assert_eq!(to_kebab_case("my_project"), "my-project");
        assert_eq!(to_kebab_case("my project"), "my-project");
        assert_eq!(to_kebab_case("MyProject"), "my-project");
        assert_eq!(to_kebab_case("my-project"), "my-project");
    }

    #[test]
    fn to_pascal_case_conversions() {
        assert_eq!(to_pascal_case("my-project"), "MyProject");
        assert_eq!(to_pascal_case("my_project"), "MyProject");
        assert_eq!(to_pascal_case("my project"), "MyProject");
        assert_eq!(to_pascal_case("MyProject"), "MyProject"); // Normalizes
    }

    #[test]
    fn to_pascal_case_multiple_words() {
        assert_eq!(to_pascal_case("hello-world-app"), "HelloWorldApp");
        assert_eq!(to_pascal_case("foo_bar_baz"), "FooBarBaz");
    }

}
