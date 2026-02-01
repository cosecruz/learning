use thiserror::Error;

use crate::infra::db::DatabaseError;

/// Infrastructure errors
#[derive(Debug, Error)]
pub enum InfrastructureError {
    // Database Error
    #[error("Database error")]
    Database(#[from] DatabaseError),
}
