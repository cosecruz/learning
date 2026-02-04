use thiserror::Error;

// errors: domain specific errors
#[derive(Debug, Error)]
pub enum DomainError {
    /// target model errors
    #[error("framework `{framework}` does not support language `{language}`")]
    FrameworkLanguageMismatch { framework: String, language: String },

    #[error("framework `{framework}` does not support project type `{project_type:?}`")]
    FrameworkProjectTypeMismatch {
        framework: String,
        project_type: String,
    },

    #[error("error inferring defaults")]
    CannotInfer,

    // template
    // project_structure
    // shared errors
    #[error("not supported")]
    NotSupported,
}
