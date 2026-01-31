use thiserror::Error;

use crate::domain::DomainError;

/// Application-level errors wrap domain and infrastructure errors.
///
/// This layer translates between domain concepts and the outside world.
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Verb not found")]
    NotFound,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Transaction error: {0}")]
    Transaction(String),
}

impl ApplicationError {
    /// Convert infrastructure errors to application errors
    pub fn from_infra<E: std::error::Error>(err: E) -> Self {
        ApplicationError::Database(err.to_string())
    }
}

// /application error wraps and propagatess all the errors that can occur while executing use cases
// / Application error wraps and propagates all errors
// / that can occur while executing use cases
// #[derive(Debug, Error)]
// pub enum ApplicationError {
//     #[error("Error executing use case `{use_case}`")]
//     UseCase {
//         use_case: &'static str,

//         #[source]
//         source: Box<dyn std::error::Error + Send + Sync + 'static>,
//     },

//     #[error("Placeholder errors")]
//     Placeholder,
// }

// impl ApplicationError {
//     pub fn use_case(
//         use_case: impl Into<String>,
//         source: impl std::error::Error + Send + Sync + 'static,
//     ) -> Self {
//         Self::UseCase {
//             use_case: use_case.into(),
//             source: Box::new(source),
//         }
//     }
// }
