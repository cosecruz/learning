// crates/core/src/domain/mod.rs
//! Domain models for Scarff.
//!
//! This module contains the core business types and logic:
//! - Target: A validated project configuration
//! - Template: A reusable project recipe
//! - ProjectStructure: The output ready for writing
//! - Common types: Shared utilities and types

pub mod common;
pub mod errors;
pub mod project_structure;
pub mod render_context;
pub mod target;
pub mod template;

// Re-export common types for convenience
pub use common::{FilePermissions, RelativePath};
pub use errors::DomainError;
pub use project_structure::{DirectoryToCreate, FileToWrite, FsEntry, ProjectStructure};
pub use render_context::RenderContext;
pub use target::{
    Architecture, Framework, HasLanguage, Language, NoLanguage, ProjectType, PythonFramework,
    RustFramework, Target, TargetBuilder, TypeScriptFramework,
};
pub use template::{
    ContentTemplateId, DirectorySpec, FileSpec, TargetMatcher, Template, TemplateContent,
    TemplateId, TemplateMetadata, TemplateNode, TemplateTree,
};
