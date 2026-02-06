//! Template contains the following;
//! - TemplateStore: In-memory store that stores and loads builtin templates #MVP
//!
//! - TemplateResolver: resolves Target to a Template. This is where the matching happens
//!
//! - TemplateRenderer: renders Template to a ProjectStructure.
//!   The project structure it outputs is what will be written to the file system.
//!
//! - TemplateError: template specific errors
//!
//! - templates: In memory templates to scaffold if matched #MVP

// crates/core/src/template/mod.rs

// Public interface
pub(crate) use errors::TemplateError;
pub(crate) use renderer::TemplateRenderer;
pub(crate) use resolver::TemplateResolver;
pub(crate) use store::{InMemoryStore, Store};

// Private implementation details
mod built_in_templates;
mod errors;
mod renderer;
mod resolver;
mod store;
