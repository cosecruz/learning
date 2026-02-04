use thiserror::Error;

// errors: domain specific errors
#[derive(Debug, Error)]
pub enum DomainError {
    // target
    #[error("language not supported")]
    LanguageNotSupported,

    #[error("framework `{framework}` does not support language `{language}`")]
    FrameworkLanguageMismatch { framework: String, language: String },

    #[error("framework `{framework}` does not support project type `{project_type:?}`")]
    FrameworkProjectTypeMismatch {
        framework: String,
        project_type: String,
    },
    // template
    // project_structure
    // shared
}
