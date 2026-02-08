// // crates/core/src/scaffold/validator.rs
// //! Input validation for scaffolding operations.
// //!
// //! This module provides validation for targets before scaffolding begins.
// //! While [`Target`] itself is constructed via a builder that enforces validity,
// //! this validator provides an additional defensive layer and generates
// //! user-friendly error messages.

// use tracing::{debug, instrument};

// use crate::{CoreResult, Target, scaffold::errors::ScaffoldError};

// /// Validates inputs before scaffolding.
// ///
// /// This validator checks that the target configuration is valid and
// /// provides helpful error messages when it's not.
// pub struct Validator;

// impl Validator {
//     ///Create a new validator
//     pub(crate) fn new() -> Self {
//         Self
//     }

//     ///validator as string literal
//     fn as_str(&self) -> &'static str {
//         "scaffold validator"
//     }

//     /// Validate a target configuration.
//     ///
//     /// # Errors
//     ///
//     /// Returns an error if the target configuration is invalid.
//     /// While the Target builder should prevent most invalid states,
//     /// this provides an extra defensive check.
//     ///
//     /// # Examples
//     ///
//     /// ```rust,no_run
//     /// use scarff_core::{Target, Language, ProjectKind, Architecture};
//     /// use scarff_core::scaffold::Validator;
//     ///
//     /// let validator = Validator::new();
//     /// let target = Target::builder()
//     ///     .language(Language::Rust)
//     ///     .kind(ProjectKind::Cli)
//     ///     .architecture(Architecture::Layered)
//     ///     .build()?;
//     ///
//     /// validator.validate(&target)?;
//     /// # Ok::<(), Box<dyn std::error::Error>>(())
//     /// ```
//     ///
//     #[instrument(skip(self), fields(target = %target))]
//     pub(crate) fn validate(&self, target: &Target) -> CoreResult<()> {
//         debug!("Validating target configuration");

//         // Check language-framework compatibility
//         self.validate_framework(target)?;

//         // Check framework-project type compatibility
//         self.validate_kind(target)?;

//         // Check architecture compatibility
//         self.validate_architecture(target)?;

//         debug!("Target validation successful");
//         Ok(())
//     }

//     ///validate kind os compatible with language
//     fn validate_kind(&self, target: &Target) -> CoreResult<()> {
//         // kind is valid when:
//         // it is actively supported
//         // when language is capable of implementing it

//         let kind  = target.kind()
//         if let kind = target.kind()
//             && kind.requires_framework()
//         {
//             let suggestions = self.suggest_frameworks(target);
//             return Err(ScaffoldError::InvalidTarget {
//                 reason: format!(
//                     "Framework '{}' does not support project type '{}'",
//                     fw,
//                     target.kind()
//                 ),
//                 source_error: None,
//             }
//             .with_suggestions(suggestions))?;
//         }

//         Ok(())
//     }

//      ///Validate framework is compatible with language
//     fn validate_framework(&self, target: &Target) -> CoreResult<()> {
//         // framework is validated when
//         //  kind, language
//         // if framework exists in
//         if let Some(framework) = target.framework()
//             && framework.language() != target.language()
//         {
//             return Err(ScaffoldError::InvalidTarget {
//                 reason: format!(
//                     "Framework '{}' cannot be used with language '{}'",
//                     framework,
//                     target.language()
//                 ),
//                 source_error: Some(self.as_str().to_string()),
//             })?;
//         }
//         Ok(())
//     }

//     /// Validate architecture is compatible with framework and project type.
//     fn validate_architecture(&self, target: &Target) -> CoreResult<()> {
//         let arch = target.architecture();

//         // Check architecture-project type compatibility
//         if !arch.supports(target.kind()) {
//             let suggestions = self.suggest_architectures(target);

//             return Err(ScaffoldError::InvalidTarget {
//                 reason: format!(
//                     "Architecture '{}' is not compatible with project type '{}'",
//                     arch,
//                     target.kind()
//                 ),
//                 source_error: None,
//             }
//             .with_suggestions(suggestions))?;
//         }

//         // Check architecture-framework compatibility
//         if let Some(framework) = target.framework()
//             && !arch.supports_framework(framework)
//         {
//             return Err(ScaffoldError::InvalidTarget {
//                 reason: format!(
//                     "Architecture '{}' is not compatible with framework '{}'",
//                     arch, framework
//                 ),
//                 source_error: None,
//             })
//             .context("failed to validate architecture compatibility with framework")?;
//         }

//         Ok(())
//     }

//     /// Suggest compatible frameworks for the given target
//     fn suggest_frameworks(&self, target: &Target) -> Vec<String> {
//         use crate::domain::*;

//         let mut suggestions = Vec::new();

//         match (target.language(), target.kind()) {
//             // TODO: remove hardcoded stuff; there should be a function that returns
//             // TODO: Framework for a language and language_kind just have to loop through it and push to suggestions
//             // this will avoid mistakes of pushing not compatible suggestions and easier if options becomes numerous
//             (Language::Rust, ProjectKind::Backend | ProjectKind::Worker) => {
//                 suggestions.push(Framework::Rust(RustFramework::Actix).to_string());
//                 suggestions.push(Framework::Rust(RustFramework::Axum).to_string());
//             }
//             (Language::Python, ProjectKind::Backend) => {
//                 suggestions.push(Framework::Python(PythonFramework::FastApi).to_string());
//                 suggestions.push(Framework::Python(PythonFramework::Django).to_string());
//             }
//             (Language::TypeScript, ProjectKind::Frontend) => {
//                 suggestions.push(Framework::TypeScript(TypeScriptFramework::React).to_string());
//                 suggestions.push(Framework::TypeScript(TypeScriptFramework::Vue).to_string());
//             }
//             (Language::TypeScript, ProjectKind::Backend) => {
//                 suggestions.push(Framework::TypeScript(TypeScriptFramework::Express).to_string());
//                 suggestions.push(Framework::TypeScript(TypeScriptFramework::NestJs).to_string());
//             }
//             (Language::TypeScript, ProjectKind::Fullstack) => {
//                 suggestions.push(Framework::TypeScript(TypeScriptFramework::NextJs).to_string());
//             }
//             _ => {}
//         }

//         suggestions
//     }

//     /// Suggest compatible architectures for the given target.
//     fn suggest_architectures(&self, target: &Target) -> Vec<String> {
//         use crate::domain::*;

//         let mut suggestions = Vec::new();

//         // TODO: remove hardcode
//         match target.kind() {
//             ProjectKind::Cli => {
//                 suggestions.push("layered".to_string());
//             }
//             ProjectKind::Backend => {
//                 suggestions.push("layered".to_string());
//                 suggestions.push("mvc".to_string());
//                 suggestions.push("modular".to_string());
//             }
//             ProjectKind::Frontend => {
//                 suggestions.push("layered".to_string());
//             }
//             ProjectKind::Fullstack => {
//                 if let Some(Framework::TypeScript(TypeScriptFramework::NextJs)) = target.framework()
//                 {
//                     suggestions.push("app-router".to_string());
//                 }
//                 suggestions.push("layered".to_string());
//                 suggestions.push("mvc".to_string());
//             }
//             ProjectKind::Worker => {
//                 suggestions.push("layered".to_string());
//                 suggestions.push("modular".to_string());
//             }
//         }

//         suggestions
//     }
// }

// impl Default for Validator {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::*;

//     #[test]
//     fn validator_accepts_valid_target() {
//         let validator = Validator::new();

//         let target = Target::builder()
//             .language(Language::Rust)
//             .kind(ProjectKind::Cli)
//             .architecture(Architecture::Layered)
//             .build()
//             .unwrap();

//         assert!(validator.validate(&target).is_ok());
//     }

//     #[test]
//     fn validator_provides_framework_suggestions() {
//         let validator = Validator::new();

//         let target = Target::builder()
//             .language(Language::Rust)
//             .kind(ProjectKind::Backend)
//             .architecture(Architecture::Layered)
//             .build()
//             .unwrap();

//         let suggestions = validator.suggest_frameworks(&target);
//         assert!(suggestions.contains(&"axum".to_string()));
//         assert!(suggestions.contains(&"actix".to_string()));
//     }

//     #[test]
//     fn validator_provides_architecture_suggestions() {
//         let validator = Validator::new();

//         let target = Target::builder()
//             .language(Language::Rust)
//             .kind(ProjectKind::Backend)
//             .architecture(Architecture::Layered)
//             .build()
//             .unwrap();

//         let suggestions = validator.suggest_architectures(&target);
//         assert!(suggestions.contains(&"layered".to_string()));
//         assert!(suggestions.contains(&"mvc".to_string()));
//         assert!(suggestions.contains(&"modular".to_string()));
//     }

//     #[test]
//     #[should_panic]
//     fn validator_rejects_incompatible_architecture() {
//         let validator = Validator::new();

//         // AppRouter only works with Fullstack
//         let target = Target::builder()
//             .language(Language::Rust)
//             .kind(ProjectKind::Cli)
//             .architecture(Architecture::AppRouter)
//             .build()
//             .unwrap();

//         // this test should fail because builder fails to build and returns ArchitectureProjectKindMismatch { architecture: "app-router", kind: "cli" }

//         let result = validator.validate(&target);
//         println!("{result:?}");
//         assert!(result.is_err());
//     }
// }
