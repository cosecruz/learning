// crates/core/src/lib.rs
//! # Scarff Core
//!
//! Core business logic for the Scarff project scaffolding tool.
//!
//! ## Overview
//!
//! Scarff Core provides the fundamental building blocks for generating project
//! scaffolds from templates. It follows a layered architecture:
//!
//! ```text
//! ┌─────────────────┐
//! │   Public API    │  ← You are here (Engine, Target, etc.)
//! ├─────────────────┤
//! │   Scaffold      │  ← Orchestration layer
//! ├─────────────────┤
//! │   Template      │  ← Template management
//! ├─────────────────┤
//! │   Domain        │  ← Core types and validation
//! └─────────────────┘
//! ```
//!
//! ## Quick Start
//!
//! The main entry point is the [`Engine`] struct:
//!
//! ```rust,no_run
//! use scarff_core::{Engine, Target, Language, ProjectType, Architecture};
//!
//! // Create an engine
//! let engine = Engine::new();
//!
//! // Define what you want to scaffold
//! let target = Target::builder()
//!     .language(Language::Rust)
//!     .project_type(ProjectType::Cli)
//!     .architecture(Architecture::Layered)
//!     .build()?;
//!
//! // Generate the project
//! engine.scaffold(target, "my-project", "./output")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Using Presets
//!
//! For common configurations, use preset methods:
//!
//! ```rust,no_run
//! use scarff_core::{Engine, Target};
//!
//! let engine = Engine::new();
//!
//! // Rust CLI application
//! engine.scaffold(Target::rust_cli(), "my-cli", "./output")?;
//!
//! // Rust web backend with Axum
//! engine.scaffold(Target::rust_backend_axum(), "my-api", "./output")?;
//!
//! // Python backend with FastAPI
//! engine.scaffold(Target::python_backend_fastapi(), "my-api", "./output")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - **Domain** (`domain`): Core types like [`Target`], [`Language`], [`ProjectType`]
//! - **Template** (`template`): Template resolution and rendering (internal)
//! - **Scaffold** (`scaffold`): Orchestration and filesystem operations (internal)
//!
//! Most of these modules are internal implementation details. The public API
//! is carefully curated to expose only what users need.
//!
//! ## Error Handling
//!
//! All fallible operations return [`CoreResult<T>`], which is an alias for
//! `Result<T, CoreError>`. Errors are designed to be actionable:
//!
//! ```rust,no_run
//! use scarff_core::{Target, Language};
//!
//! // Invalid configuration will give a clear error
//! let result = Target::builder()
//!     .language(Language::Rust)
//!     .framework(Framework::Python(PythonFramework::Django))  // Wrong!
//!     .build();
//!
//! match result {
//!     Err(e) => {
//!         eprintln!("Error: {}", e);
//!         // Prints: "Framework django is not available for rust"
//!     }
//!     _ => {}
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Features
//!
//! - `logging`: Enable tracing support (disabled by default for library users)
//!
//! ## Examples
//!
//! See the `examples/` directory for more usage examples.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

// ============================================================================
// Private Modules
// ============================================================================

// Private modules (not exposed to users)
mod domain;
mod errors;
mod scaffold;
mod template;

// ============================================================================
// Public API: Domain Types
// ============================================================================

pub use domain::{
    // Core target types
    Architecture,
    // Domain errors
    DomainError,
    Framework,
    // Builder pattern
    HasLanguage,
    Language,
    NoLanguage,
    ProjectType,
    PythonFramework,
    RustFramework,
    Target,
    TargetBuilder,
    TypeScriptFramework,
};

// ============================================================================
// Public API: Errors
// ============================================================================
pub use errors::{CoreError, CoreResult};

// ============================================================================
// Public API: Scaffolding
// ============================================================================
pub use scaffold::{Engine, ScaffoldError, TemplateInfo};

// ============================================================================
// Re-exports for convenience
// ============================================================================

/// Prelude module for convenient imports.
///
/// This module re-exports the most commonly used types so users can
/// import everything they need with a single `use` statement:
///
/// ```rust
/// use scarff_core::prelude::*;
///
/// let target = Target::builder()
///     .language(Language::Rust)
///     .project_type(ProjectType::Cli)
///     .build()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub mod prelude {
    pub use crate::{
        Architecture, CoreError, CoreResult, DomainError, Engine, Framework, HasLanguage, Language,
        NoLanguage, ProjectType, PythonFramework, RustFramework, ScaffoldError, Target,
        TargetBuilder, TemplateInfo, TypeScriptFramework,
    };
}

// ============================================================================
// Library Configuration
// ============================================================================

/// Initialize logging for the library.
///
/// This is only available when the `logging` feature is enabled.
/// CLI applications should handle their own logging setup.
///
/// # Examples
///
/// ```rust,no_run
/// #[cfg(feature = "logging")]
/// scarff_core::init_logging();
/// ```
#[cfg(feature = "logging")]
pub fn init_logging() {
    use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("scarff_core=info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prelude_imports_work() {
        use crate::prelude::*;

        // Should be able to use all common types
        let _target: Target;
        let _lang: Language = Language::Rust;
        let _project_type: ProjectType = ProjectType::Cli;
        let _arch: Architecture = Architecture::Layered;
    }

    #[test]
    fn api_is_usable() {
        // This test verifies the public API is actually usable
        let engine = Engine::new();

        let target = Target::builder()
            .language(Language::Rust)
            .project_type(ProjectType::Cli)
            .architecture(Architecture::Layered)
            .build()
            .expect("Should build valid target");

        // Getting templates shouldn't panic
        let templates = engine.find_templates(&target);
        assert!(templates.is_ok());
    }

    // #[test]
    // fn preset_methods_work() {
    //     // Verify preset methods create valid targets
    //     let _rust_cli = Target::rust_cli();
    //     let _rust_backend = Target::rust_backend_axum();
    //     let _python_backend = Target::python_backend_fastapi();
    //     let _typescript_frontend = Target::typescript_frontend_react();
    // }
}
