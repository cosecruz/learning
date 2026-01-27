use thiserror::Error;

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
pub enum _AppError {
    // errors that happen in configuration phase
    #[error("Configuration Error")]
    Config(#[from] crate::config::ConfigError),
    // // errors that happen in Domain- Models and Entities: contains core business logic
    // DomainError,

    // // errors that happen in Application: contains use cases
    // ApplicationError,

    // // INTERFACE ADAPTERS (Controllers, Presenters, Gateways)
    // InterfaceErrors,

    // // FRAMEWORKS & DRIVERS (Web, DB, UI, External interfaces)
    // InfraError,
}

// * thiserror will be used on client facing errors?
// * add proper html error response
// * do i need an API response helper
//*  at boundaries and orchestrators use anyhow::Result or AppResult<T>
// * at domain, infra, usecase level code use Result<T, [Layer Specific Error]>
