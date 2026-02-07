// crates/core/src/scaffold/validator.rs
//! Input validation for scaffolding operations.
//!
//! This module provides validation for targets before scaffolding begins.
//! While [`Target`] itself is constructed via a builder that enforces validity,
//! this validator provides an additional defensive layer and generates
//! user-friendly error messages.

use anyhow::Context;
use tracing::{debug, instrument};

use crate::{CoreResult, Target, scaffold::errors::ScaffoldError};

/// Validates inputs before scaffolding.
///
/// This validator checks that the target configuration is valid and
/// provides helpful error messages when it's not.
pub struct Validator;

impl Validator {
    ///Create a new validator
    pub(crate) fn new() -> Self {
        Self
    }

    /// Validate a target configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the target configuration is invalid.
    /// While the Target builder should prevent most invalid states,
    /// this provides an extra defensive check.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use scarff_core::{Target, Language, ProjectType, Architecture};
    /// use scarff_core::scaffold::Validator;
    ///
    /// let validator = Validator::new();
    /// let target = Target::builder()
    ///     .language(Language::Rust)
    ///     .project_type(ProjectType::Cli)
    ///     .architecture(Architecture::Layered)
    ///     .build()?;
    ///
    /// validator.validate(&target)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    #[instrument(skip(self), fields(target = %target))]
    pub(crate) fn validate(&self, target: &Target) -> CoreResult<()> {
        debug!("Validating target configuration");

        // Check language-framework compatibility
        self.validate_framework(target)?;

        // Check framework-project type compatibility
        self.validate_project_type(target)?;

        // Check architecture compatibility
        self.validate_architecture(target)?;

        debug!("Target validation successful");
        Ok(())
    }

    ///Validate framework is compatible with language
    fn validate_framework(&self, target: &Target) -> CoreResult<()> {
        if let Some(framework) = target.framework()
            && framework.language() != target.language()
        {
            return Err(ScaffoldError::InvalidTarget {
                reason: format!(
                    "Framework '{}' cannot be used with language '{}'",
                    framework,
                    target.language()
                ),
                source_error: None,
            })
            .context("failed to validate framework compatibility with language")?;
        }
        Ok(())
    }

    ///validate project_type os compatible with framework
    fn validate_project_type(&self, target: &Target) -> CoreResult<()> {
        if let Some(fw) = target.framework()
            && !fw.supports(target.project_type())
        {
            let suggestions = self.suggest_frameworks(target);
            return Err(ScaffoldError::InvalidTarget {
                reason: format!(
                    "Framework '{}' does not support project type '{}'",
                    fw,
                    target.project_type()
                ),
                source_error: None,
            }
            .with_suggestions(suggestions))
            .context("failed to validate project type compatibility with framework")?;
        }

        Ok(())
    }

    /// Validate architecture is compatible with framework and project type.
    fn validate_architecture(&self, target: &Target) -> CoreResult<()> {
        let arch = target.architecture();

        // Check architecture-project type compatibility
        if !arch.supports(target.project_type()) {
            let suggestions = self.suggest_architectures(target);

            return Err(ScaffoldError::InvalidTarget {
                reason: format!(
                    "Architecture '{}' is not compatible with project type '{}'",
                    arch,
                    target.project_type()
                ),
                source_error: None,
            }
            .with_suggestions(suggestions))?;
        }

        // Check architecture-framework compatibility
        if let Some(framework) = target.framework()
            && !arch.supports_framework(framework)
        {
            return Err(ScaffoldError::InvalidTarget {
                reason: format!(
                    "Architecture '{}' is not compatible with framework '{}'",
                    arch, framework
                ),
                source_error: None,
            })
            .context("failed to validate architecture compatibility with framework")?;
        }

        Ok(())
    }

    /// Suggest compatible frameworks for the given target
    fn suggest_frameworks(&self, target: &Target) -> Vec<String> {
        use crate::domain::*;

        let mut suggestions = Vec::new();

        match (target.language(), target.project_type()) {
            // TODO: remove hardcoded stuff; there should be a function that returns
            // TODO: Framework for a language and language_project_type just have to loop through it and push to suggestions
            // this will avoid mistakes of pushing not compatible suggestions and easier if options becomes numerous
            (Language::Rust, ProjectType::Backend | ProjectType::Worker) => {
                suggestions.push(Framework::Rust(RustFramework::Actix).to_string());
                suggestions.push(Framework::Rust(RustFramework::Axum).to_string());
            }
            (Language::Python, ProjectType::Backend) => {
                suggestions.push(Framework::Python(PythonFramework::FastApi).to_string());
                suggestions.push(Framework::Python(PythonFramework::Django).to_string());
            }
            (Language::TypeScript, ProjectType::Frontend) => {
                suggestions.push(Framework::TypeScript(TypeScriptFramework::React).to_string());
                suggestions.push(Framework::TypeScript(TypeScriptFramework::Vue).to_string());
            }
            (Language::TypeScript, ProjectType::Backend) => {
                suggestions.push(Framework::TypeScript(TypeScriptFramework::Express).to_string());
                suggestions.push(Framework::TypeScript(TypeScriptFramework::NestJs).to_string());
            }
            (Language::TypeScript, ProjectType::Fullstack) => {
                suggestions.push(Framework::TypeScript(TypeScriptFramework::NextJs).to_string());
            }
            _ => {}
        }

        suggestions
    }

    /// Suggest compatible architectures for the given target.
    fn suggest_architectures(&self, target: &Target) -> Vec<String> {
        use crate::domain::*;

        let mut suggestions = Vec::new();

        // TODO: remove hardcode
        match target.project_type() {
            ProjectType::Cli => {
                suggestions.push("layered".to_string());
            }
            ProjectType::Backend => {
                suggestions.push("layered".to_string());
                suggestions.push("mvc".to_string());
                suggestions.push("modular".to_string());
            }
            ProjectType::Frontend => {
                suggestions.push("layered".to_string());
            }
            ProjectType::Fullstack => {
                if let Some(Framework::TypeScript(TypeScriptFramework::NextJs)) = target.framework()
                {
                    suggestions.push("app-router".to_string());
                }
                suggestions.push("layered".to_string());
                suggestions.push("mvc".to_string());
            }
            ProjectType::Worker => {
                suggestions.push("layered".to_string());
                suggestions.push("modular".to_string());
            }
        }

        suggestions
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;

    #[test]
    fn validator_accepts_valid_target() {
        let validator = Validator::new();

        let target = Target::builder()
            .language(Language::Rust)
            .project_type(ProjectType::Cli)
            .architecture(Architecture::Layered)
            .build()
            .unwrap();

        assert!(validator.validate(&target).is_ok());
    }

    #[test]
    fn validator_provides_framework_suggestions() {
        let validator = Validator::new();

        let target = Target::builder()
            .language(Language::Rust)
            .project_type(ProjectType::Backend)
            .architecture(Architecture::Layered)
            .build()
            .unwrap();

        let suggestions = validator.suggest_frameworks(&target);
        assert!(suggestions.contains(&"axum".to_string()));
        assert!(suggestions.contains(&"actix".to_string()));
    }

    #[test]
    fn validator_provides_architecture_suggestions() {
        let validator = Validator::new();

        let target = Target::builder()
            .language(Language::Rust)
            .project_type(ProjectType::Backend)
            .architecture(Architecture::Layered)
            .build()
            .unwrap();

        let suggestions = validator.suggest_architectures(&target);
        assert!(suggestions.contains(&"layered".to_string()));
        assert!(suggestions.contains(&"mvc".to_string()));
        assert!(suggestions.contains(&"modular".to_string()));
    }

    #[test]
    #[should_panic]
    fn validator_rejects_incompatible_architecture() {
        let validator = Validator::new();

        // AppRouter only works with Fullstack
        let target = Target::builder()
            .language(Language::Rust)
            .project_type(ProjectType::Cli)
            .architecture(Architecture::AppRouter)
            .build()
            .unwrap();

        // this test should fail because builder fails to build and returns ArchitectureProjectTypeMismatch { architecture: "app-router", project_type: "cli" }

        let result = validator.validate(&target);
        println!("{result:?}");
        assert!(result.is_err());
    }
}
