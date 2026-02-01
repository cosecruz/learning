//! Domain Layer
//! This layer contains the entities and behaviors that form the business logic for this system
//! Contains:
//!  - model: which are the entities and some of their behaviors
//!  - repository: a adapter or port to application use cases as it relates to persistence
//!  - error: domain  and application specific errors
//!
pub mod error;
pub mod model;
pub mod repository;

pub use error::DomainError;
