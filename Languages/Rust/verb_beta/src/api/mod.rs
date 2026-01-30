//! # Web Interface Layer for HTTP
//!
//! This module represents the **HTTP interface layer** of the system.
//! It is responsible for adapting external HTTP requests into
//! application-level use case invocations, and translating application
//! responses back into HTTP responses.
//!
//! ## Architectural Context
//!
//! This project follows a Clean / Layered Architecture with the following layers:
//!
//! - **Domain**
//!   - The innermost layer
//!   - Contains core business rules, entities, and value objects
//!   - Has no dependency on any other layer
//!
//! - **Application**
//!   - Contains use cases and application flow orchestration
//!   - Coordinates domain objects to fulfill business requirements
//!   - Defines *ports* (traits) that outer layers must implement
//!
//! - **Interface (this module)**
//!   - Adapters such as HTTP handlers, controllers, and presenters
//!   - Translates between transport-level concerns (HTTP, JSON)
//!     and application-level concepts (commands, queries, results)
//!   - Contains **no business logic**
//!
//! - **Infrastructure**
//!   - Frameworks, drivers, and external services
//!   - Web servers, databases, message brokers, external APIs
//!   - Implements interfaces (ports) defined by inner layers
//!
//! ## Dependency Rule
//!
//! Dependencies always point **inward**:
//!
//! ```text
//! Web (Interface)
//!   → Application
//!     → Domain
//! ```
//!
//! Infrastructure depends on Application interfaces but is never depended on
//! by inner layers.

mod dto;
mod error;
mod handlers;
mod middlewares;
mod routes;

pub use routes::app;

/// HTTP boundary dependency container.
///
/// `AppState` holds **application-level dependencies** required by HTTP
/// handlers and middleware.
///
/// ### Invariant
///
/// - `AppState` is **cheap to clone**
/// - It contains **handles**, not data
/// - It depends only on **application ports (use case traits)**
/// - It never contains:
///   - Domain entities
///   - Request-scoped data
///   - Infrastructure implementations
///
/// ### Ownership & Concurrency
///
/// Handlers receive an **owned clone** of `AppState`. Internally, this
/// typically means cloning `Arc` pointers, not duplicating memory.
///
/// ```text
/// ┌───────────────┐
/// │   Handlers    │  ← clone AppState per request
/// ├───────────────┤
/// │  Use Cases    │  ← trait-based, application logic
/// ├───────────────┤
/// │ Repositories  │  ← trait objects behind Arc
/// ├───────────────┤
/// │ Infrastructure│  ← DB, Web, External APIs
/// └───────────────┘
/// ```
///
/// ### Example (future shape)
///
/// ```ignore
/// #[derive(Clone)]
/// pub struct AppState {
///     pub verb_use_case: Arc<dyn VerbUseCase>,
///     pub auth_use_case: Arc<dyn AuthUseCase>,
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AppState {
    // Intentionally empty for now.
    // Fields will be added as application ports are introduced.
}

// / ## Design Notes
// /
// / Traits are appropriate for:
// / - Application services / use cases
// / - Repositories
// / - Gateways
// / - External service clients
// /
// / Traits are **not** appropriate for:
// / - Routers
// / - Route composition
// / - HTTP wiring logic
// /
// / Routers should remain declarative and compositional,
// / constructed via functions rather than abstracted behind traits.
