//! Domain models for Scarff.
//!
//! This module contains the core business types and logic:
//! - Target: A validated project configuration
//! - Template: A reusable project recipe
//! - [`ProjectStructure`]: The output ready for writing
//! - Common types: Shared utilities

pub mod common;
mod errors;
mod project_structure;
mod render_context;
mod target;
mod template;
pub(crate) mod validator;

// Re-export common types
pub(crate) use common::{Permissions, RelativePath};
pub use errors::DomainError;

// Re-export project structure
pub(crate) use project_structure::{DirectoryToCreate, FileToWrite, FsEntry, ProjectStructure};

// Re-export render context
pub(crate) use render_context::RenderContext;

// Re-export target types
pub use target::{
    Architecture, Framework, HasLanguage, Language, NoLanguage, ProjectKind, PythonFramework,
    RustFramework, Target, TargetBuilder, TypeScriptFramework,
};

// Re-export template types
pub(crate) use template::{
    ContentTemplateId, DirectorySpec, FileSpec, TargetMatcher, TargetMatcherBuilder, Template,
    TemplateBuilder, TemplateContent, TemplateEngine, TemplateId, TemplateMetadata, TemplateNode,
    TemplateRecord, TemplateSource, TemplateTree,
};
