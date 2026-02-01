use thiserror::Error;

use crate::{application::ApplicationError, infra::error::InfrastructureError};

///* ✅ The rule (final form)
///
///✔ At boundaries and orchestrators
/// use anyhow::Result<T> (or your alias AppResult<T>)
///
/// ✔ Inside layers (domain, use cases, infra)
/// use Result<T, LayerSpecificError>
///
// Global result type used in Application
pub type AppResult<T> = anyhow::Result<T>;

#[derive(Debug, Error)]
pub enum AppError {
    /// configuration errors
    #[error("Configuration Error")]
    Config(#[from] crate::config::ConfigError),

    ///Application errors
    #[error("Application Error")]
    Application(#[from] ApplicationError),

    /// Infrastructure errors
    #[error("Application Error")]
    Infra(#[from] InfrastructureError),
}

// * thiserror will be used on client facing errors?
// * add proper html error response
// * do i need an API response helper
//*  at boundaries and orchestrators use anyhow::Result or AppResult<T>
// * at domain, infra, usecase level code use Result<T, [Layer Specific Error]>
