//! # Web Interface Layer for HTTP
//!
//! This module represents the **HTTP interface layer** of the system.
//! It is responsible for adapting external HTTP requests into
//! application-level use case invocations, and translating application
//! responses back into HTTP responses.

mod dto;
mod error;
mod handlers;
mod routes;

use std::sync::Arc;

pub use routes::app;

use crate::{application::VerbFacade, infra::db::Database};

/// HTTP boundary dependency container.
///
/// `AppState` holds **application-level dependencies** required by HTTP
/// handlers and middleware.
///
/// ## Why Generic Over Database?
///
/// The `AppState` is generic over `D: Database` because:
/// 1. The `VerbFacade` is generic over database type
/// 2. We want compile-time type safety
/// 3. Allows swapping database implementations
///
/// ## Cloning Semantics
///
/// `AppState` is **cheap to clone** because it only holds `Arc` pointers.
/// Axum clones state for each request, but this just increments reference counts.
#[derive(Clone)]
pub struct AppState<D: Database> {
    /// Application facade for verb operations
    pub verb_facade: Arc<VerbFacade<D>>,
}

impl<D: Database> AppState<D> {
    /// Create new application state
    pub fn new(verb_facade: VerbFacade<D>) -> Self {
        Self {
            verb_facade: Arc::new(verb_facade),
        }
    }
}
