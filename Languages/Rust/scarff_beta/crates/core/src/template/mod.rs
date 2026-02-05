//! Template contains the following;
//! - TemplateStore: In-memory store that stores templates and retrieves build in templates #MVP
//!
//! - TemplateResolver: resolves Target to a Template. This is where the matching happens
//!
//! - TemplateRenderer: renders Template to a ProjectStructure.
//!   The project structure it outputs is what will be written to the file system.
//!
//! - TemplateError: template specific errors
//!
//! - templates: In memory templates to scaffold if matched #MVP

pub mod errors;
pub mod store;

pub use errors::TemplateError;
