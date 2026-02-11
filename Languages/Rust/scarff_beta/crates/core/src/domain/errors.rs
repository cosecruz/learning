use std::path::PathBuf;

use thiserror::Error;

/// Domain-specific errors for Scarff's core types.
#[derive(Debug, Error, Clone)]
pub enum DomainError {
    // ========================================================================
    // Language errors
    // ========================================================================
    /// Unsupported programming language
    #[error("Unsupported language '{language}'. Supported: rust, python, typescript")]
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
    /// ```rust,ignore
    /// match result {
    ///     Err(e) => {
    ///         eprintln!("Error: {}", e);
    ///         for suggestion in e.suggestions() {
    ///             eprintln!("  💡 {}", suggestion);
    ///         }
    ///     }
    ///     Ok(v) => { /* ... */ }
    /// }
    /// ```
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
    use super::*;

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
