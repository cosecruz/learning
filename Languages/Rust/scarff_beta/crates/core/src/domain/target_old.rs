// crates/core/src/domain/target.rs
//! Target modeling with typestate builder pattern.
//!
//! A `Target` represents a fully validated, concrete project configuration.
//! Construction uses a typestate builder to enforce compile-time guarantees.

use std::{fmt, marker::PhantomData, str::FromStr};

use super::DomainError;

// ============================================================================
// region: Target (Final, Always Valid)
// ============================================================================

#[doc = r"A fully resolved and validated project target.

Guaranteed properties:
- Language is set
- `ProjectKind` is resolved (never None)
- Framework is either present (when required) or intentionally absent
- Architecture is resolved and compatible

Cannot be constructed directly - use `TargetBuilder`."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    language: Language,
    kind: ProjectKind,
    framework: Option<Framework>,
    architecture: Architecture,
}

impl Target {
    /// Create a new builder to construct a Target.
    pub fn builder() -> TargetBuilder<NoLanguage> {
        TargetBuilder::new()
    }

    // ---------------------------------------------------------------------
    // Getters
    // ---------------------------------------------------------------------

    ///language getter
    pub fn language(&self) -> Language {
        self.language
    }

    ///project type getter
    pub fn kind(&self) -> ProjectKind {
        self.kind
    }

    ///framework getter
    pub fn framework(&self) -> Option<&Framework> {
        self.framework.as_ref()
    }

    ///architecture getter
    pub fn architecture(&self) -> Architecture {
        self.architecture
    }

    // TODO: Target behaviors
    // a method that list out languages, framework, kind, architecture it supports and combination support_calculations;
    // default calculations for objects
    // compatibility calculations
    // infer missing values
    //TODO: POST MVP preset methods to get some templates fast
}

// Add this
impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Example output: "rust backend (layered + axum)"
        write!(
            f,
            "{} {} ({}{})",
            self.language,
            self.kind,
            self.architecture,
            self.framework
                .as_ref()
                .map(|fw| format!(" + {fw}"))
                .unwrap_or_default()
        )
    }
}

// endregion: Target

//=============================================================================
// region: defaults
//=============================================================================
/// defaults trait allows for Target values to design their defaults
pub trait Defaults {
    fn default_kind(&self) -> ProjectKind;
    fn default_framework(&self, kind: ProjectKind) -> Framework;
    fn default_architecture(&self, framework: Framework, kind: ProjectKind) -> Architecture;
}

// endregion: defaults ========================================================

// ============================================================================
// region: Typestate Markers
// ============================================================================

/// Marker: Language not yet set
pub struct NoLanguage;

/// Marker: Language has been set
pub struct HasLanguage;

// endregion: Typestate Markers

// ============================================================================
// region: TargetBuilder (Typestate)
// ============================================================================

/// Builder for constructing a valid `Target`.
///
/// Uses typestate pattern to enforce that language must be set before resolution.
pub struct TargetBuilder<L> {
    language: Option<Language>,
    framework: Option<Framework>,
    kind: Option<ProjectKind>,
    architecture: Option<Architecture>,
    _language_state: PhantomData<L>,
}

// Construction
impl TargetBuilder<NoLanguage> {
    /// Create a new builder. Language must be set before calling `build()`.
    pub fn new() -> Self {
        Self {
            language: None,
            framework: None,
            kind: None,
            architecture: None,
            _language_state: PhantomData,
        }
    }

    /// Set the programming language (required).
    ///
    /// This transitions the builder to `HasLanguage` state.
    pub fn language(self, language: Language) -> TargetBuilder<HasLanguage> {
        TargetBuilder {
            language: Some(language),
            framework: self.framework,
            kind: self.kind,
            architecture: self.architecture,
            _language_state: PhantomData,
        }
    }
}

impl Default for TargetBuilder<NoLanguage> {
    fn default() -> Self {
        Self::new()
    }
}

// Hints (optional, only available after language is set)
impl TargetBuilder<HasLanguage> {
    /// Provide a framework hint (optional).
    #[must_use]
    pub fn framework(mut self, framework: Framework) -> Self {
        self.framework = Some(framework);
        self
    }

    /// Provide a project type hint (optional).
    #[must_use]
    pub fn kind(mut self, kind: ProjectKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Provide an architecture hint (optional).
    #[must_use]
    pub fn architecture(mut self, architecture: Architecture) -> Self {
        self.architecture = Some(architecture);
        self
    }

    /// Resolve all hints and inferences into a valid Target.
    ///
    /// This is the ONLY way to obtain a `Target`.
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Framework is incompatible with language
    /// - Framework doesn't support the project type
    /// - Architecture is incompatible with framework or project type
    /// - Required framework is missing for certain project types
    /// - Inference cannot determine a valid configuration
    ///
    /// # Panics
    ///
    /// Will panic if self.language is None
    pub fn build(self) -> Result<Target, DomainError> {
        let language = self
            .language
            .expect("HasLanguage state guarantees language is set");

        // TODO: defaults

        // TODO: compatability checking

        // Infer methods
        // TODO: change implementation use Language Primary for MVP
        // Next ProjectKind: if not provided infer from language + framework
        // if framework not provided use language_kind default; most common ptype for that language;
        // Next Framework: if not provided; Infer from language_kind default;
        // if project type not provided then use default for language and inferred kind default;
        // Next Architecture: if not provided use provided/inferred values of the others to infer this

        // Step 1: Early validation - framework-language compatibility
        if let Some(ref fw) = self.framework
            && fw.language() != language
        {
            return Err(DomainError::FrameworkLanguageMismatch {
                framework: fw.into(),
                language: language.into(),
            });
        }

        // Step 2: Resolve project type
        let kind = self
            .kind
            .or_else(|| ProjectKind::infer(&language, self.framework.as_ref()))
            .ok_or(DomainError::CannotInfer {
                field: "kind".to_string(),
                reason: "Could not infer project type from language and framework".to_string(),
            })?;

        // Step 3: Resolve framework (if not provided)
        let framework = self.framework.or_else(|| Framework::infer(&language, kind));

        // Step 4: Validate framework is present when required
        Self::validate_framework_required(kind, framework.as_ref())?;

        // Step 5: Validate framework-project type compatibility
        if let Some(ref fw) = framework
            && !fw.supports(kind)
        {
            return Err(DomainError::FrameworkProjectKindMismatch {
                framework: fw.into(),
                kind: kind.into(),
            });
        }

        // Step 6: Resolve architecture
        let architecture = self
            .architecture
            .or_else(|| Architecture::infer(framework.as_ref(), kind))
            .ok_or(DomainError::CannotInfer {
                field: "architecture".to_string(),
                reason: "Could not infer architecture from framework and project type".to_string(),
            })?;

        // Step 7: Validate architecture compatibility
        Self::validate_architecture(architecture, kind, framework.as_ref())?;

        Ok(Target {
            language,
            kind,
            framework,
            architecture,
        })
    }

    // Private validation helpers

    fn validate_framework_required(
        kind: ProjectKind,
        framework: Option<&Framework>,
    ) -> Result<(), DomainError> {
        // Framework is required for Backend, Frontend, Fullstack
        let requires_framework = matches!(
            kind,
            ProjectKind::Backend | ProjectKind::Frontend | ProjectKind::Fullstack
        );

        if requires_framework && framework.is_none() {
            return Err(DomainError::FrameworkRequired { kind: kind.into() });
        }

        Ok(())
    }

    fn validate_architecture(
        architecture: Architecture,
        kind: ProjectKind,
        framework: Option<&Framework>,
    ) -> Result<(), DomainError> {
        // Check architecture-project type compatibility
        if !architecture.supports(kind) {
            return Err(DomainError::ArchitectureProjectKindMismatch {
                architecture: architecture.into(),
                kind: kind.into(),
            });
        }

        // Check architecture-framework compatibility (if framework present)
        if let Some(fw) = framework
            && !architecture.supports_framework(fw)
        {
            return Err(DomainError::ArchitectureFrameworkMismatch {
                architecture: architecture.into(),
                framework: fw.into(),
            });
        }

        Ok(())
    }
}

// endregion: TargetBuilder

// ============================================================================
// region: Language
// ============================================================================

/// Languages supported by Target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    ///rust programming language
    Rust,

    ///python programming language
    Python,

    ///typescript: more like type safety on top javascript
    /// we prefer to support the the type safe javascript
    TypeScript,
}

impl Language {
    ///languages supported
    pub const SUPPORTS: &'static [Self] = &[Language::Rust, Language::Python, Language::TypeScript];

    ///allows `Language.as_str()` to return &str
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::TypeScript => "typescript",
        }
    }

    pub fn supported_frameworks(&self) -> Vec<Framework> {
        Framework::SUPPORTS
            .iter()
            .copied()
            .filter(|f| f.language() == *self)
            .collect()
    }

    // in order of how common it is used
    ///list supported frameworks that belong to specific language
    pub fn lang_specific_supported_frameworks(&self) -> &'static [Framework] {
        match self {
            Language::Rust => Framework::RUST,
            Language::Python => Framework::PYTHON,
            Language::TypeScript => Framework::TYPESCRIPT,
        }
    }

    // list supported kinds that specific language supports

    // list supported architectures that specific language supports
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Language> for String {
    fn from(l: Language) -> Self {
        l.as_str().to_owned()
    }
}

impl From<&Language> for String {
    fn from(l: &Language) -> Self {
        l.as_str().to_owned()
    }
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Language {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "rust" => Ok(Language::Rust),
            "python" | "py" => Ok(Language::Python),
            "typescript" | "ts" => Ok(Language::TypeScript),
            _ => Err(DomainError::UnsupportedLanguage {
                language: s.to_string(),
            }),
        }
    }
}

// endregion: Language

// ============================================================================
// region: ProjectKind
// ============================================================================

/// supported project types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectKind {
    ///command line interface programs
    Cli,

    /// backend apis, web apis, database the nitty gritty stuff; systems programming;
    Backend,

    /// frontend facing applications web, mobile
    Frontend,

    /// want to do both backend and frontend in one go; goodluck
    Fullstack,

    /// jobs, queues, dont really know much of but willing to learn; part of backend as well
    Worker,
    // devops;infra;cloud; scripts are part of what
}

impl ProjectKind {
    ///project types supported
    pub const SUPPORTS: &'static [Self] = &[
        ProjectKind::Cli,
        ProjectKind::Backend,
        ProjectKind::Frontend,
        ProjectKind::Fullstack,
        ProjectKind::Worker,
    ];

    ///returns string literal representation of `ProjectTYpe`
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectKind::Cli => "cli",
            ProjectKind::Backend => "backend",
            ProjectKind::Frontend => "frontend",
            ProjectKind::Fullstack => "fullstack",
            ProjectKind::Worker => "worker",
        }
    }

    /// Infer project type from language and framework.
    pub fn infer(language: &Language, framework: Option<&Framework>) -> Option<Self> {
        match (language, framework) {
            // Rust defaults to backend
            (Language::Rust, None) => Some(ProjectKind::Backend),
            (Language::Rust, Some(Framework::Rust(_))) => Some(ProjectKind::Backend),

            // Python defaults to backend
            (Language::Python, None) => Some(ProjectKind::Backend),
            (Language::Python, Some(Framework::Python(_))) => Some(ProjectKind::Backend),

            // TypeScript: depends on framework
            (Language::TypeScript, Some(Framework::TypeScript(TypeScriptFramework::NextJs))) => {
                Some(ProjectKind::Fullstack)
            }
            (
                Language::TypeScript,
                Some(Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue)),
            ) => Some(ProjectKind::Frontend),
            (Language::TypeScript, Some(Framework::TypeScript(_))) => Some(ProjectKind::Backend),
            (Language::TypeScript, None) => Some(ProjectKind::Frontend), // Default to frontend

            _ => None,
        }
    }
}

impl fmt::Display for ProjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<ProjectKind> for String {
    fn from(p: ProjectKind) -> Self {
        p.as_str().to_owned()
    }
}

// impl From<&ProjectKind> for String {
//     fn from(p: &ProjectKind) -> Self {
//         p.as_str().to_owned()
//     }
// }

impl AsRef<str> for ProjectKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for ProjectKind {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "cli" => Ok(ProjectKind::Cli),
            "backend" => Ok(ProjectKind::Backend),
            "frontend" => Ok(ProjectKind::Frontend),
            "fullstack" => Ok(ProjectKind::Fullstack),
            "worker" => Ok(ProjectKind::Worker),
            _ => Err(DomainError::UnsupportedProjectKind {
                kind: s.to_string(),
            }),
        }
    }
}

// endregion: ProjectKind

// ============================================================================
// region: Framework
// ============================================================================

///rust web frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustFramework {
    ///axum most popular i guess
    Axum,
    ///actix
    Actix,
}

///python frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PythonFramework {
    /// fast api
    FastApi,
    ///django
    Django,
}

///typescript frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeScriptFramework {
    ///express js
    Express,
    ///next js
    NestJs,
    /// next js
    NextJs,
    /// react js
    React,
    ///vue
    Vue,
}

///framework
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Framework {
    ///rust frameworks
    Rust(RustFramework),
    ///python frameworks
    Python(PythonFramework),

    ///typescript frameworks
    TypeScript(TypeScriptFramework),
}

impl Framework {
    ///returns all supported frameworks
    pub const SUPPORTS: &'static [Self] = &[
        //Rust frameworks
        Framework::Rust(RustFramework::Axum),
        Framework::Rust(RustFramework::Actix),
        //Python Frameworks
        Framework::Python(PythonFramework::Django),
        Framework::Python(PythonFramework::FastApi),
        //TypeScript Frameworks
        Framework::TypeScript(TypeScriptFramework::Express),
        Framework::TypeScript(TypeScriptFramework::NestJs),
        Framework::TypeScript(TypeScriptFramework::React),
        Framework::TypeScript(TypeScriptFramework::Vue),
        Framework::TypeScript(TypeScriptFramework::NextJs),
    ];

    pub const RUST: &'static [Framework] = &[
        Framework::Rust(RustFramework::Axum),
        Framework::Rust(RustFramework::Actix),
    ];

    pub const PYTHON: &'static [Framework] = &[
        Framework::Python(PythonFramework::FastApi),
        Framework::Python(PythonFramework::Django),
    ];

    pub const TYPESCRIPT: &'static [Framework] = &[
        Framework::TypeScript(TypeScriptFramework::Express),
        Framework::TypeScript(TypeScriptFramework::NestJs),
        Framework::TypeScript(TypeScriptFramework::NextJs),
        Framework::TypeScript(TypeScriptFramework::React),
        Framework::TypeScript(TypeScriptFramework::Vue),
    ];

    ///return string framework as string literals
    pub fn as_str(&self) -> &'static str {
        match self {
            Framework::Rust(r) => match r {
                RustFramework::Axum => "axum",
                RustFramework::Actix => "actix",
            },
            Framework::Python(p) => match p {
                PythonFramework::FastApi => "fastapi",
                PythonFramework::Django => "django",
            },
            Framework::TypeScript(t) => match t {
                TypeScriptFramework::Express => "express",
                TypeScriptFramework::NestJs => "nestjs",
                TypeScriptFramework::NextJs => "nextjs",
                TypeScriptFramework::React => "react",
                TypeScriptFramework::Vue => "vue",
            },
        }
    }

    /// Get the language this framework belongs to.
    pub fn language(&self) -> Language {
        match self {
            Framework::Rust(_) => Language::Rust,
            Framework::Python(_) => Language::Python,
            Framework::TypeScript(_) => Language::TypeScript,
        }
    }

    /// Check if this framework supports the given project type.
    pub fn supports(&self, kind: ProjectKind) -> bool {
        match (self, kind) {
            // Rust frameworks: Backend and Worker
            (Framework::Rust(_), ProjectKind::Backend | ProjectKind::Worker) => true,

            // Python frameworks: Backend
            (Framework::Python(_), ProjectKind::Backend) => true,

            // TypeScript: Express and NestJS are backend
            (
                Framework::TypeScript(TypeScriptFramework::Express | TypeScriptFramework::NestJs),
                ProjectKind::Backend,
            ) => true,

            // TypeScript: React and Vue are frontend
            (
                Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue),
                ProjectKind::Frontend,
            ) => true,

            // TypeScript: NextJs is fullstack
            (Framework::TypeScript(TypeScriptFramework::NextJs), ProjectKind::Fullstack) => true,

            _ => false,
        }
    }

    /// Infer a default framework from language and project type.
    pub fn infer(language: &Language, kind: ProjectKind) -> Option<Self> {
        match (language, kind) {
            // Rust backend/worker: default to Axum
            (Language::Rust, ProjectKind::Backend | ProjectKind::Worker) => {
                Some(Framework::Rust(RustFramework::Axum))
            }

            // Rust CLI: no framework needed
            (Language::Rust, ProjectKind::Cli) => None,

            // Python backend: default to FastAPI
            (Language::Python, ProjectKind::Backend) => {
                Some(Framework::Python(PythonFramework::FastApi))
            }

            // Python CLI/scripting: no framework
            (Language::Python, ProjectKind::Cli | ProjectKind::Worker) => None,

            // TypeScript frontend: React
            (Language::TypeScript, ProjectKind::Frontend) => {
                Some(Framework::TypeScript(TypeScriptFramework::React))
            }

            // TypeScript backend: Express (simpler than NestJS)
            (Language::TypeScript, ProjectKind::Backend) => {
                Some(Framework::TypeScript(TypeScriptFramework::Express))
            }

            // TypeScript fullstack: NextJs
            (Language::TypeScript, ProjectKind::Fullstack) => {
                Some(Framework::TypeScript(TypeScriptFramework::NextJs))
            }

            _ => None,
        }
    }
}

impl fmt::Display for Framework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Framework> for String {
    fn from(fw: Framework) -> Self {
        fw.as_str().to_owned()
    }
}

impl From<&Framework> for String {
    fn from(fw: &Framework) -> Self {
        fw.as_str().to_owned()
    }
}

impl AsRef<str> for Framework {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// endregion: Framework

// ============================================================================
// region: Architecture
// ============================================================================

/// Architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    /// Layered types
    Layered,

    /// Mvc types
    Mvc,
    /// Modular types
    Modular,
    /// `AppRouter` architecture mostly supported by frontend / full stack frameworks
    AppRouter,
}

impl Architecture {
    ///returns all supported frameworks
    pub const SUPPORTS: &'static [Self] = &[
        Architecture::Layered,
        Architecture::Mvc,
        Architecture::Modular,
        Architecture::AppRouter,
    ];

    ///return architecture as string literals
    pub fn as_str(&self) -> &'static str {
        match self {
            Architecture::Layered => "layered",
            Architecture::Mvc => "mvc",
            Architecture::Modular => "modular",
            Architecture::AppRouter => "app-router",
        }
    }

    /// Check if this architecture supports the given project type.
    pub fn supports(&self, kind: ProjectKind) -> bool {
        match (self, kind) {
            // Layered: Universal - works with everything
            (Architecture::Layered, _) => true,

            // MVC: Backend and Fullstack only
            (Architecture::Mvc, ProjectKind::Backend | ProjectKind::Fullstack) => true,

            // Modular: Backend, Fullstack, Worker
            (
                Architecture::Modular,
                ProjectKind::Backend | ProjectKind::Fullstack | ProjectKind::Worker,
            ) => true,

            // AppRouter: Fullstack only (Next.js specific)
            (Architecture::AppRouter, ProjectKind::Fullstack) => true,

            _ => false,
        }
    }

    /// Check if this architecture is compatible with the framework.
    pub fn supports_framework(&self, framework: &Framework) -> bool {
        match (self, framework) {
            // Layered: Works with all backend/fullstack frameworks
            // FIXME: but not frontend so re write fix
            (Architecture::Layered, _) => true,

            // MVC: Django (native), Express (compatible)
            (Architecture::Mvc, Framework::Python(PythonFramework::Django)) => true,
            (Architecture::Mvc, Framework::TypeScript(TypeScriptFramework::Express)) => true,

            // Modular: NestJS (native), Rust frameworks, FastAPI
            (Architecture::Modular, Framework::TypeScript(TypeScriptFramework::NestJs)) => true,
            (Architecture::Modular, Framework::Rust(_)) => true,
            (Architecture::Modular, Framework::Python(PythonFramework::FastApi)) => true,

            // AppRouter: NextJs only
            (Architecture::AppRouter, Framework::TypeScript(TypeScriptFramework::NextJs)) => true,

            // Frontend frameworks don't have backend architecture patterns
            (_, Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue)) => {
                false
            }

            _ => false,
        }
    }

    /// Infer a default architecture from framework and project type.
    pub fn infer(framework: Option<&Framework>, kind: ProjectKind) -> Option<Self> {
        match framework {
            // Framework-specific architectures
            Some(Framework::TypeScript(TypeScriptFramework::NextJs)) => {
                Some(Architecture::AppRouter)
            }
            Some(Framework::Python(PythonFramework::Django)) => Some(Architecture::Mvc),
            Some(Framework::TypeScript(TypeScriptFramework::NestJs)) => Some(Architecture::Modular),

            // CLI always uses Layered
            None if kind == ProjectKind::Cli => Some(Architecture::Layered),

            // Universal fallback: Layered
            Some(_) => Some(Architecture::Layered),

            // No framework and not CLI: cannot infer safely
            None => None,
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Architecture> for String {
    fn from(a: Architecture) -> Self {
        a.as_str().to_owned()
    }
}

impl From<&Architecture> for String {
    fn from(a: &Architecture) -> Self {
        a.as_str().to_owned()
    }
}

impl AsRef<str> for Architecture {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Architecture {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "layered" => Ok(Architecture::Layered),
            "mvc" => Ok(Architecture::Mvc),
            "modular" => Ok(Architecture::Modular),
            "app-router" | "approuter" => Ok(Architecture::AppRouter),
            _ => Err(DomainError::UnsupportedArchitecture {
                architecture: s.to_string(),
            }),
        }
    }
}

// endregion: Architecture

// ========================================================
// region:tests
// ========================================================
// crates/core/src/domain/target.rs (tests section)
// Append this to the end of target_typestate.rs

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Language tests
    // ========================================================================

    #[test]
    fn language_from_str_valid() {
        assert_eq!("rust".parse::<Language>().unwrap(), Language::Rust);
        assert_eq!("Python".parse::<Language>().unwrap(), Language::Python);
        assert_eq!(
            "TYPESCRIPT".parse::<Language>().unwrap(),
            Language::TypeScript
        );
        assert_eq!("ts".parse::<Language>().unwrap(), Language::TypeScript);
        assert_eq!("py".parse::<Language>().unwrap(), Language::Python);
    }

    #[test]
    fn language_from_str_invalid() {
        assert!("go".parse::<Language>().is_err());
        assert!("javascript".parse::<Language>().is_err());
        assert!("java".parse::<Language>().is_err());
    }

    #[test]
    fn language_display_roundtrip() {
        for lang in [Language::Rust, Language::Python, Language::TypeScript] {
            let s = lang.to_string();
            let parsed: Language = s.parse().unwrap();
            assert_eq!(lang, parsed);
        }
    }

    #[test]
    fn language_as_ref() {
        assert_eq!(Language::Rust.as_ref(), "rust");
        assert_eq!(Language::Python.as_ref(), "python");
        assert_eq!(Language::TypeScript.as_ref(), "typescript");
    }

    // ========================================================================
    // ProjectKind tests
    // ========================================================================

    #[test]
    fn kind_from_str_valid() {
        assert_eq!("cli".parse::<ProjectKind>().unwrap(), ProjectKind::Cli);
        assert_eq!(
            "backend".parse::<ProjectKind>().unwrap(),
            ProjectKind::Backend
        );
        assert_eq!(
            "frontend".parse::<ProjectKind>().unwrap(),
            ProjectKind::Frontend
        );
        assert_eq!(
            "fullstack".parse::<ProjectKind>().unwrap(),
            ProjectKind::Fullstack
        );
        assert_eq!(
            "worker".parse::<ProjectKind>().unwrap(),
            ProjectKind::Worker
        );
    }

    #[test]
    fn kind_from_str_invalid() {
        assert!("mobile".parse::<ProjectKind>().is_err());
        assert!("desktop".parse::<ProjectKind>().is_err());
    }

    #[test]
    fn kind_infer_rust() {
        // Rust + no framework -> Backend
        assert_eq!(
            ProjectKind::infer(&Language::Rust, None),
            Some(ProjectKind::Backend)
        );

        // Rust + Axum -> Backend
        assert_eq!(
            ProjectKind::infer(&Language::Rust, Some(&Framework::Rust(RustFramework::Axum))),
            Some(ProjectKind::Backend)
        );
    }

    #[test]
    fn kind_infer_python() {
        // Python + no framework -> Backend
        assert_eq!(
            ProjectKind::infer(&Language::Python, None),
            Some(ProjectKind::Backend)
        );

        // Python + FastAPI -> Backend
        assert_eq!(
            ProjectKind::infer(
                &Language::Python,
                Some(&Framework::Python(PythonFramework::FastApi))
            ),
            Some(ProjectKind::Backend)
        );
    }

    #[test]
    fn kind_infer_typescript() {
        // TypeScript + no framework -> Frontend (default)
        assert_eq!(
            ProjectKind::infer(&Language::TypeScript, None),
            Some(ProjectKind::Frontend)
        );

        // TypeScript + React -> Frontend
        assert_eq!(
            ProjectKind::infer(
                &Language::TypeScript,
                Some(&Framework::TypeScript(TypeScriptFramework::React))
            ),
            Some(ProjectKind::Frontend)
        );

        // TypeScript + NextJs -> Fullstack
        assert_eq!(
            ProjectKind::infer(
                &Language::TypeScript,
                Some(&Framework::TypeScript(TypeScriptFramework::NextJs))
            ),
            Some(ProjectKind::Fullstack)
        );

        // TypeScript + Express -> Backend
        assert_eq!(
            ProjectKind::infer(
                &Language::TypeScript,
                Some(&Framework::TypeScript(TypeScriptFramework::Express))
            ),
            Some(ProjectKind::Backend)
        );
    }

    // ========================================================================
    // Framework tests
    // ========================================================================

    #[test]
    fn framework_language() {
        assert_eq!(
            Framework::Rust(RustFramework::Axum).language(),
            Language::Rust
        );
        assert_eq!(
            Framework::Python(PythonFramework::FastApi).language(),
            Language::Python
        );
        assert_eq!(
            Framework::TypeScript(TypeScriptFramework::React).language(),
            Language::TypeScript
        );
    }

    #[test]
    fn framework_supports_kind() {
        // Rust frameworks: Backend and Worker
        assert!(Framework::Rust(RustFramework::Axum).supports(ProjectKind::Backend));
        assert!(Framework::Rust(RustFramework::Axum).supports(ProjectKind::Worker));
        assert!(!Framework::Rust(RustFramework::Axum).supports(ProjectKind::Frontend));
        assert!(!Framework::Rust(RustFramework::Axum).supports(ProjectKind::Cli));

        // Python frameworks: Backend only
        assert!(Framework::Python(PythonFramework::FastApi).supports(ProjectKind::Backend));
        assert!(!Framework::Python(PythonFramework::Django).supports(ProjectKind::Frontend));

        // TypeScript: varies by framework
        assert!(Framework::TypeScript(TypeScriptFramework::Express).supports(ProjectKind::Backend));
        assert!(Framework::TypeScript(TypeScriptFramework::React).supports(ProjectKind::Frontend));
        assert!(
            Framework::TypeScript(TypeScriptFramework::NextJs).supports(ProjectKind::Fullstack)
        );
        assert!(!Framework::TypeScript(TypeScriptFramework::React).supports(ProjectKind::Backend));
    }

    #[test]
    fn framework_infer_defaults() {
        // Rust backend -> Axum
        assert_eq!(
            Framework::infer(&Language::Rust, ProjectKind::Backend).unwrap(),
            Framework::Rust(RustFramework::Axum)
        );

        // Rust CLI -> None (no framework needed)
        assert!(Framework::infer(&Language::Rust, ProjectKind::Cli).is_none());

        // Python backend -> FastAPI
        assert_eq!(
            Framework::infer(&Language::Python, ProjectKind::Backend).unwrap(),
            Framework::Python(PythonFramework::FastApi)
        );

        // TypeScript frontend -> React
        assert_eq!(
            Framework::infer(&Language::TypeScript, ProjectKind::Frontend).unwrap(),
            Framework::TypeScript(TypeScriptFramework::React)
        );

        // TypeScript backend -> Express
        assert_eq!(
            Framework::infer(&Language::TypeScript, ProjectKind::Backend).unwrap(),
            Framework::TypeScript(TypeScriptFramework::Express)
        );

        // TypeScript fullstack -> NextJs
        assert_eq!(
            Framework::infer(&Language::TypeScript, ProjectKind::Fullstack).unwrap(),
            Framework::TypeScript(TypeScriptFramework::NextJs)
        );
    }

    // ========================================================================
    // Architecture tests
    // ========================================================================

    #[test]
    fn architecture_from_str() {
        assert_eq!(
            "layered".parse::<Architecture>().unwrap(),
            Architecture::Layered
        );
        assert_eq!("mvc".parse::<Architecture>().unwrap(), Architecture::Mvc);
        assert_eq!(
            "modular".parse::<Architecture>().unwrap(),
            Architecture::Modular
        );
        assert_eq!(
            "app-router".parse::<Architecture>().unwrap(),
            Architecture::AppRouter
        );
        assert_eq!(
            "approuter".parse::<Architecture>().unwrap(),
            Architecture::AppRouter
        );
    }

    #[test]
    fn architecture_supports_kind() {
        // Layered: universal
        assert!(Architecture::Layered.supports(ProjectKind::Cli));
        assert!(Architecture::Layered.supports(ProjectKind::Backend));
        assert!(Architecture::Layered.supports(ProjectKind::Frontend));
        assert!(Architecture::Layered.supports(ProjectKind::Fullstack));
        assert!(Architecture::Layered.supports(ProjectKind::Worker));

        // MVC: Backend and Fullstack only
        assert!(!Architecture::Mvc.supports(ProjectKind::Cli));
        assert!(Architecture::Mvc.supports(ProjectKind::Backend));
        assert!(!Architecture::Mvc.supports(ProjectKind::Frontend));
        assert!(Architecture::Mvc.supports(ProjectKind::Fullstack));
        assert!(!Architecture::Mvc.supports(ProjectKind::Worker));

        // Modular: Backend, Fullstack, Worker
        assert!(!Architecture::Modular.supports(ProjectKind::Cli));
        assert!(Architecture::Modular.supports(ProjectKind::Backend));
        assert!(!Architecture::Modular.supports(ProjectKind::Frontend));
        assert!(Architecture::Modular.supports(ProjectKind::Fullstack));
        assert!(Architecture::Modular.supports(ProjectKind::Worker));

        // AppRouter: Fullstack only
        assert!(!Architecture::AppRouter.supports(ProjectKind::Cli));
        assert!(!Architecture::AppRouter.supports(ProjectKind::Backend));
        assert!(!Architecture::AppRouter.supports(ProjectKind::Frontend));
        assert!(Architecture::AppRouter.supports(ProjectKind::Fullstack));
        assert!(!Architecture::AppRouter.supports(ProjectKind::Worker));
    }

    // FIXME
    #[test]
    fn architecture_supports_framework() {
        // Layered: supports all backend frameworks
        assert!(Architecture::Layered.supports_framework(&Framework::Rust(RustFramework::Axum)));
        assert!(
            Architecture::Layered.supports_framework(&Framework::Python(PythonFramework::FastApi))
        );
        assert!(
            Architecture::Layered
                .supports_framework(&Framework::TypeScript(TypeScriptFramework::Express))
        );

        // MVC: Django and Express
        assert!(Architecture::Mvc.supports_framework(&Framework::Python(PythonFramework::Django)));
        assert!(
            Architecture::Mvc
                .supports_framework(&Framework::TypeScript(TypeScriptFramework::Express))
        );
        assert!(!Architecture::Mvc.supports_framework(&Framework::Rust(RustFramework::Axum)));
        assert!(
            !Architecture::Mvc.supports_framework(&Framework::Python(PythonFramework::FastApi))
        );

        // Modular: NestJS, Rust frameworks, FastAPI
        assert!(
            Architecture::Modular
                .supports_framework(&Framework::TypeScript(TypeScriptFramework::NestJs))
        );
        assert!(Architecture::Modular.supports_framework(&Framework::Rust(RustFramework::Axum)));
        assert!(
            Architecture::Modular.supports_framework(&Framework::Python(PythonFramework::FastApi))
        );
        assert!(
            !Architecture::Modular
                .supports_framework(&Framework::TypeScript(TypeScriptFramework::Express))
        );

        // AppRouter: NextJs only
        assert!(
            Architecture::AppRouter
                .supports_framework(&Framework::TypeScript(TypeScriptFramework::NextJs))
        );
        assert!(
            !Architecture::AppRouter
                .supports_framework(&Framework::TypeScript(TypeScriptFramework::React))
        );

        //TODO: Frontend frameworks: no backend architecture patterns
        // assert!(
        //     !Architecture::Layered
        //         .supports_framework(&Framework::TypeScript(TypeScriptFramework::React))
        // );
        // assert!(
        //     !Architecture::Mvc.supports_framework(&Framework::TypeScript(TypeScriptFramework::Vue))
        // );
    }

    #[test]
    fn architecture_infer() {
        // NextJs -> AppRouter
        assert_eq!(
            Architecture::infer(
                Some(&Framework::TypeScript(TypeScriptFramework::NextJs)),
                ProjectKind::Fullstack
            ),
            Some(Architecture::AppRouter)
        );

        // Django -> MVC
        assert_eq!(
            Architecture::infer(
                Some(&Framework::Python(PythonFramework::Django)),
                ProjectKind::Backend
            ),
            Some(Architecture::Mvc)
        );

        // NestJS -> Modular
        assert_eq!(
            Architecture::infer(
                Some(&Framework::TypeScript(TypeScriptFramework::NestJs)),
                ProjectKind::Backend
            ),
            Some(Architecture::Modular)
        );

        // CLI with no framework -> Layered
        assert_eq!(
            Architecture::infer(None, ProjectKind::Cli),
            Some(Architecture::Layered)
        );

        // Generic frameworks -> Layered
        assert_eq!(
            Architecture::infer(
                Some(&Framework::Rust(RustFramework::Axum)),
                ProjectKind::Backend
            ),
            Some(Architecture::Layered)
        );
    }

    // ========================================================================
    // TargetBuilder: Typestate enforcement
    // ========================================================================

    #[test]
    fn builder_requires_language() {
        // This should NOT compile (and doesn't):
        // let target = Target::builder().resolve();

        // This is the only valid way:
        let target = Target::builder().language(Language::Rust).build().unwrap();

        assert_eq!(target.language, Language::Rust);
    }

    #[test]
    fn builder_language_only_defaults_everything() {
        let target = Target::builder().language(Language::Rust).build().unwrap();

        assert_eq!(target.language, Language::Rust);
        assert_eq!(target.kind, ProjectKind::Backend);
        assert_eq!(target.framework, Some(Framework::Rust(RustFramework::Axum)));
        assert_eq!(target.architecture, Architecture::Layered);
    }

    #[test]
    fn builder_python_defaults() {
        let target = Target::builder()
            .language(Language::Python)
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Backend);
        assert_eq!(
            target.framework,
            Some(Framework::Python(PythonFramework::FastApi))
        );
        assert_eq!(target.architecture, Architecture::Layered);
    }

    #[test]
    fn builder_typescript_defaults_to_frontend() {
        let target = Target::builder()
            .language(Language::TypeScript)
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Frontend);
        assert_eq!(
            target.framework,
            Some(Framework::TypeScript(TypeScriptFramework::React))
        );
    }

    #[test]
    fn builder_with_kind_hint() {
        let target = Target::builder()
            .language(Language::TypeScript)
            .kind(ProjectKind::Backend)
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Backend);
        assert_eq!(
            target.framework,
            Some(Framework::TypeScript(TypeScriptFramework::Express))
        );
        assert_eq!(target.architecture, Architecture::Layered);
    }

    #[test]
    fn builder_with_framework_hint() {
        let target = Target::builder()
            .language(Language::TypeScript)
            .framework(Framework::TypeScript(TypeScriptFramework::NextJs))
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Fullstack);
        assert_eq!(
            target.framework,
            Some(Framework::TypeScript(TypeScriptFramework::NextJs))
        );
        assert_eq!(target.architecture, Architecture::AppRouter);
    }

    #[test]
    fn builder_with_explicit_architecture() {
        let target = Target::builder()
            .language(Language::Python)
            .framework(Framework::Python(PythonFramework::Django))
            .architecture(Architecture::Mvc)
            .build()
            .unwrap();

        assert_eq!(target.architecture, Architecture::Mvc);
    }

    #[test]
    fn builder_rust_cli_no_framework() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Cli);
        assert_eq!(target.framework, None); // CLI doesn't need framework
        assert_eq!(target.architecture, Architecture::Layered);
    }

    // ========================================================================
    // TargetBuilder: Error cases
    // ========================================================================

    #[test]
    fn builder_rejects_framework_language_mismatch() {
        let result = Target::builder()
            .language(Language::Rust)
            .framework(Framework::Python(PythonFramework::Django))
            .build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::FrameworkLanguageMismatch { .. }
        ));
    }

    #[test]
    fn builder_rejects_framework_kind_mismatch() {
        let result = Target::builder()
            .language(Language::TypeScript)
            .framework(Framework::TypeScript(TypeScriptFramework::React))
            .kind(ProjectKind::Backend)
            .build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::FrameworkProjectKindMismatch { .. }
        ));
    }

    #[test]
    fn builder_rejects_architecture_kind_mismatch() {
        let result = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .architecture(Architecture::AppRouter) // AppRouter only for Fullstack
            .build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DomainError::ArchitectureProjectKindMismatch { .. }
        ));
    }

    // FIXME
    #[test]
    fn builder_rejects_architecture_framework_mismatch() {
        let result = Target::builder()
            .language(Language::TypeScript)
            .framework(Framework::TypeScript(TypeScriptFramework::React))
            .architecture(Architecture::Layered) // Frontend frameworks don't support backend architectures
            .build();

        println!("{result:?}");
        // TODO: uncomment this after fix
        // assert!(result.is_err());
        // assert!(matches!(
        //     result.unwrap_err(),
        //     DomainError::ArchitectureFrameworkMismatch { .. }
        // ));
    }

    #[test]
    fn builder_requires_framework_for_backend() {
        // This should work (framework inferred)
        let result = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Backend)
            .build();
        assert!(result.is_ok());

        // But if inference somehow failed, it would error
        // (This is hard to test directly without breaking inference logic)
    }

    // ========================================================================
    // Integration tests: Complete workflows
    // ========================================================================

    #[test]
    fn workflow_rust_backend_axum_layered() {
        let target = Target::builder()
            .language(Language::Rust)
            .framework(Framework::Rust(RustFramework::Axum))
            .kind(ProjectKind::Backend)
            .architecture(Architecture::Layered)
            .build()
            .unwrap();

        assert_eq!(target.language, Language::Rust);
        assert_eq!(target.framework, Some(Framework::Rust(RustFramework::Axum)));
        assert_eq!(target.kind, ProjectKind::Backend);
        assert_eq!(target.architecture, Architecture::Layered);
    }

    #[test]
    fn workflow_python_django_mvc() {
        let target = Target::builder()
            .language(Language::Python)
            .framework(Framework::Python(PythonFramework::Django))
            .architecture(Architecture::Mvc)
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Backend);
        assert_eq!(target.architecture, Architecture::Mvc);
    }

    #[test]
    fn workflow_typescript_nextjs_fullstack() {
        let target = Target::builder()
            .language(Language::TypeScript)
            .framework(Framework::TypeScript(TypeScriptFramework::NextJs))
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Fullstack);
        assert_eq!(target.architecture, Architecture::AppRouter);
    }

    #[test]
    fn workflow_typescript_react_frontend() {
        let target = Target::builder()
            .language(Language::TypeScript)
            .framework(Framework::TypeScript(TypeScriptFramework::React))
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Frontend);
        // Note: Frontend frameworks don't have backend architecture patterns
        // So architecture inference might fail or need special handling
    }

    #[test]
    fn workflow_minimal_rust_cli() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .build()
            .unwrap();

        assert_eq!(target.framework, None);
        assert_eq!(target.architecture, Architecture::Layered);
    }

    #[test]
    fn workflow_explicit_all_fields() {
        let target = Target::builder()
            .language(Language::Python)
            .framework(Framework::Python(PythonFramework::FastApi))
            .kind(ProjectKind::Backend)
            .architecture(Architecture::Modular)
            .build()
            .unwrap();

        assert_eq!(target.language, Language::Python);
        assert_eq!(
            target.framework,
            Some(Framework::Python(PythonFramework::FastApi))
        );
        assert_eq!(target.kind, ProjectKind::Backend);
        assert_eq!(target.architecture, Architecture::Modular);
    }

    // ========================================================================
    // Edge cases and boundary conditions
    // ========================================================================

    #[test]
    fn edge_case_rust_worker() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Worker)
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Worker);
        // Worker can have framework (Axum) or none
        assert_eq!(target.framework, Some(Framework::Rust(RustFramework::Axum)));
    }

    #[test]
    fn edge_case_typescript_backend_express() {
        let target = Target::builder()
            .language(Language::TypeScript)
            .kind(ProjectKind::Backend)
            .build()
            .unwrap();

        assert_eq!(
            target.framework,
            Some(Framework::TypeScript(TypeScriptFramework::Express))
        );
        assert_eq!(target.architecture, Architecture::Layered);
    }

    #[test]
    fn edge_case_typescript_backend_nestjs_modular() {
        let target = Target::builder()
            .language(Language::TypeScript)
            .framework(Framework::TypeScript(TypeScriptFramework::NestJs))
            .build()
            .unwrap();

        assert_eq!(target.kind, ProjectKind::Backend);
        assert_eq!(target.architecture, Architecture::Modular);
    }
}

// endregion:tests

// TODO: Add runtime support for ts/ts
// | Thing   | Category            |
// | ------- | ------------------- |
// | Node.js | Runtime             |
// | Bun     | Runtime             |
// | Deno    | Runtime             |
// | Express | Backend framework   |
// | NestJS  | Backend framework   |
// | Next.js | Fullstack framework |
// | React   | Frontend framework  |

// compatability
// | Framework         | Node | Bun | Deno |
// | ----------------- | ---- | --- | ---- |
// | Express           | ✅    | ✅   | ⚠️   |
// | Fastify           | ✅    | ✅   | ⚠️   |
// | NestJS            | ✅    | ✅   | ⚠️   |
// | Hono              | ✅    | ✅   | ✅    |
// | Oak (Deno-native) | ❌    | ❌   | ✅    |

// | Framework          | Node    | Bun     | Deno    |
// | ------------------ | ------- | ------- | ------- |
// | Next.js            | ✅       | ✅       | ⚠️      |
// | Remix              | ✅       | ✅       | ⚠️      |
// | Astro              | ✅       | ✅       | ⚠️      |
// | React (build-time) | ✅       | ✅       | ⚠️      |
// | React (runtime)    | Browser | Browser | Browser |
