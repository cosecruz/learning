// crates/core/src/domain/mod.rs
//! Domain models for Scarff.
//!
//! This module contains the core business types and logic:
//! - Target: A validated project configuration
//! - Template: A reusable project recipe
//! - Project Structure: The output ready for writing
//! - Common types: Shared utilities and types

mod common;
mod errors;
mod project_structure;
mod render_context;
mod target;
mod target_v2;
mod template;

// Re-export common types for convenience
pub(crate) use common::{FilePermissions, RelativePath};
pub use errors::DomainError;

// Re exporting project_structure
pub(crate) use project_structure::{
    FsEntry,
    // project structure
    ProjectStructure,
};
pub(crate) use render_context::RenderContext;
pub use target::{
    Architecture, Framework, HasLanguage, Language, NoLanguage, ProjectType, PythonFramework,
    RustFramework, Target, TargetBuilder, TypeScriptFramework,
};
pub(crate) use template::{
    DirectorySpec, FileSpec, TargetMatcher, Template, TemplateContent, TemplateId,
    TemplateMetadata, TemplateNode, TemplateTree,
};
