use thiserror::Error;

/// Infrastructure errors
#[derive(Debug, Error)]
pub enum InfraError {
    // Database Error
    // #[error("Database error: {0}")]
    // Database(#[from] sqlx::Error),
}
