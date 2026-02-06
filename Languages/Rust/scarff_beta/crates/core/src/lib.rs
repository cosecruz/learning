// crates/core/src/lib.rs
//! # Scarff Core
//!
//! Core business logic for project scaffolding.
//!
//! ## Architecture
//!
//! This crate is organized into three main layers:
//!
//! - **Domain**: Pure domain types (`Target`, `Template`, etc.)
//! - **Template**: Template management (resolution, rendering)
//! - **Scaffold**: Orchestration and filesystem operations
//!
//! ## Usage
//!
//! ```rust,no_run
//! use scarff_core::{Engine, Target, Language, ProjectType, Architecture};
//!
//! let engine = Engine::new();
//! let target = Target::builder()
//!     .language(Language::Rust)
//!     .project_type(ProjectType::Cli)
//!     .architecture(Architecture::Layered)
//!     .build()?;
//!
//! engine.scaffold(target, "my-project", "./output")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// Private modules (not exposed to users)
mod domain;
mod errors;
mod scaffold;
mod template;

// Public re-exports (only what users need)
pub use domain::{
    Architecture,
    // Error types
    DomainError,
    Framework,

    Language,
    ProjectType,
    // Core types for building targets
    Target,
    // Builder pattern
    // TargetBuilder,
};

// Re-export error types
pub use errors::{CoreError, CoreResult};

// Internal types (DO NOT expose these)
// - FilePermissions (implementation detail)
// - RelativePath (implementation detail)
// - TemplateNode (implementation detail)
// - RenderContext (implementation detail)

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
