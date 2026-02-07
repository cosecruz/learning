//! This is Scaffold Engine module
//! It orchestrates and coordinates the entire scaffolding process
//! Consists of :
//! - engine: the main orchestrator
//! - filesystem: abstract adapter/port I/O that the actual external filesystem will use

pub mod engine;
pub mod errors;
pub mod filesystem;
mod validator;
mod writer;

pub use engine::{Engine, TemplateInfo};
pub use errors::ScaffoldError;
