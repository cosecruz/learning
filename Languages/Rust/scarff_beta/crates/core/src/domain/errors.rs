use thiserror::Error;

/// Domain-specific errors for Scarff's core types.
#[derive(Debug, Error)]
pub enum DomainError {
    // ========================================================================
    // Language errors
    // ========================================================================
    /// Unsupported programming language
    #[error("Unsupported language '{language}'. Supported: rust, python, typescript")]
    UnsupportedLanguage { language: String },

    // ========================================================================
    // ProjectType errors
    // ========================================================================
    /// Unsupported project type
    #[error(
        "Unsupported project type '{project_type}'. Supported: cli, backend, frontend, fullstack, worker"
    )]
    UnsupportedProjectType { project_type: String },

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
        "Framework '{framework}' does not support project type '{project_type}'. Choose a compatible framework or different project type"
    )]
    FrameworkProjectTypeMismatch {
        framework: String,
        project_type: String,
    },

    /// Framework is required for this project type but was not provided
    #[error(
        "A framework is required for project type '{project_type}'. Specify a framework using --framework or choose a different project type"
    )]
    FrameworkRequired { project_type: String },

    // ========================================================================
    // Architecture errors
    // ========================================================================
    /// Unsupported architecture style
    #[error(
        "Unsupported architecture '{architecture}'. Supported: layered, mvc, modular, app-router"
    )]
    UnsupportedArchitecture { architecture: String },

    /// Architecture is incompatible with the project type
    #[error(
        "Architecture '{architecture}' is not compatible with project type '{project_type}'. Choose a compatible architecture or different project type"
    )]
    ArchitectureProjectTypeMismatch {
        architecture: String,
        project_type: String,
    },

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
    // Template errors (placeholder for future use)
    // ========================================================================
    /// Template-related errors (to be expanded)
    #[error("Template error: {0}")]
    TemplateError(String),

    // ========================================================================
    // ProjectStructure errors (placeholder for future use)
    // ========================================================================
    /// Project structure errors (to be expanded)
    #[error("Project structure error: {0}")]
    ProjectStructureError(String),

    // ========================================================================
    // General/Shared errors
    // ========================================================================
    /// Generic "not supported" error (fallback)
    #[error("This combination is not supported")]
    NotSupported,
}
