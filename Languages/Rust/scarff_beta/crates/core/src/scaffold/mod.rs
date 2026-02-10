//! Scaffold orchestration module.
//!
//! This module coordinates the entire scaffolding process:
//! - Engine: Main orchestrator
//! - Writer: Filesystem operations
//! - Filesystem: Abstraction for testability

pub mod engine;
pub mod errors;
pub mod filesystem;
pub(crate) mod writer;

pub use engine::{Engine, TemplateInfo};
pub use errors::ScaffoldError;
pub(crate) use writer::{FileWriter, Writer};
