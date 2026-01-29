//! # Service Module
//!
//! A service represents a **business capability** and groups together:
//!
//! - **Domain logic** (entities, states, invariants, rules)
//! - **Application logic** (use cases / workflows)
//! - **Ports** (traits defining required external capabilities)
//! - **Adapters** (protocol- or technology-specific integrations)
//!
//! ## Responsibilities
//!
//! A service defines *what it needs*, not *how those needs are fulfilled*.
//! External systems interact with a service exclusively through its exposed
//! interfaces (HTTP, CLI, IPC, messaging, etc.).
//!
//! ## Dependency Model
//!
//! Services are **agnostic to concrete infrastructure implementations**:
//!
//! - Persistence is accessed via injected repository traits.
//!   Any implementation (in-memory, SQLite, Postgres, etc.) may be supplied,
//!   provided it satisfies the required contract.
//! - Caching is modeled as an optional capability and may be backed by
//!   Redis, Valkey, or omitted entirely without affecting correctness.
//! - Communication protocols (HTTP, gRPC, CLI, workers) are adapters at the
//!   boundary and do not influence core business logic.
//!
//! All dependencies flow *inward*:
//! concrete implementations depend on service-defined abstractions.
//!
//! ## Optional Infrastructure
//!
//! Some capabilities (such as caching or telemetry) are optional and may be
//! absent at runtime. Core business persistence, when required by the domain,
//! is still modeled as a mandatory port and may be fulfilled by a no-op or
//! in-memory implementation when needed.
//!
//! ## Inter-service Communication
//!
//! Services may communicate with other services either:
//!
//! - synchronously via request/response interfaces, or
//! - asynchronously via domain events.
//!
//! In both cases, communication is expressed through ports and implemented
//! by adapters.
//!
//! ## Service Structure
//!
//! ```text
//! SERVICE (business capability)
//! ├─ Domain        // verbs, states, invariants, rules
//! ├─ Application   // use cases and orchestration
//! ├─ Ports         // required external capabilities
//! └─ Adapters      // protocols and integrations
//! ```
//!
//! ## Services (current)
//!
mod core;

use core::error::CoreError;
pub(crate) mod error {
    use thiserror::Error;

    use crate::services::CoreError;

    #[derive(Debug, Error)]
    pub enum ServiceError {
        ///Core Error
        #[error("Core Error")]
        Core(#[from] CoreError),
    }
}
