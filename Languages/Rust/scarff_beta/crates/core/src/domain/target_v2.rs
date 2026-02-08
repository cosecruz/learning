//! Target modeling with typestate builder pattern.
//!
//! This module provides the [`Target`] type, which represents a fully validated
//! project configuration. Targets are constructed using a builder pattern that
//! enforces compile-time guarantees about required fields.
//!
//! # Examples
//!
//! ```rust
//! use scarff_core::{Target, Language, ProjectKind};
//!
//! // Minimal target - other fields inferred
//! let target = Target::builder()
//!     .language(Language::Rust)
//!     .build()?;
//!
//! // Fully specified target
//! let target = Target::builder()
//!     .language(Language::Rust)
//!     .kind(ProjectKind::WebBackend)
//!     .framework(Framework::Rust(RustFramework::Axum))
//!     .architecture(Architecture::Layered)
//!     .build()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//!
use std::{fmt, marker::PhantomData};

use crate::DomainError;

// ======================================================
// region: Target and `TargetBuilder`
// ======================================================

// ============================================================================
// Target (Final, Always Valid)
// ============================================================================

/// A fully validated project configuration.
///
/// `Target` represents a complete, validated project specification that has
/// passed all compatibility checks. It cannot be constructed directly - use
/// [`Target::builder()`] instead.
///
/// # Invariants
///
/// - Language is always set and supported
/// - If framework is set, it's compatible with the language
/// - Architecture is compatible with both framework and project type
/// - All inferred values are deterministic and documented
///
/// # Examples
///
/// ```rust
/// use scarff_core::{Target, Language, ProjectKind};
///
/// let target = Target::builder()
///     .language(Language::Rust)
///     .kind(ProjectKind::Cli)
///     .build()?;
///
/// assert_eq!(target.language(), Language::Rust);
/// assert_eq!(target.kind(), ProjectKind::Cli);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    /// Language of the project
    pub language: Language,
    /// Project type
    pub kind: ProjectKind,
    /// Framework, if any
    pub framework: Option<Framework>,
    /// Architecture, if any
    pub architecture: Architecture,
}

impl Target {
    /// Create a new builder to construct a Target.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::Target;
    ///
    /// let builder = Target::builder();
    /// ```
    #[must_use]
    pub fn builder() -> TargetBuilder<NoLanguage> {
        TargetBuilder::new()
    }

    /// Get the language of this target.
    #[must_use]
    pub const fn language(&self) -> Language {
        self.language
    }

    /// Get the project type of this target.
    #[must_use]
    pub const fn kind(&self) -> ProjectKind {
        self.kind
    }

    /// Get the framework, if any.
    #[must_use]
    pub const fn framework(&self) -> Option<Framework> {
        self.framework
    }

    /// Get the architecture of this target.
    #[must_use]
    pub const fn architecture(&self) -> Architecture {
        self.architecture
    }

    // Preset methods for common configurations

    /// Create a Rust CLI application target.
    ///
    /// # Examples
    ///
    /// ```rust:no_run
    /// use scarff_core::Target;
    ///
    /// let target = Target::rust_cli();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn rust_cli() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .build()
    }

    /// Create a Rust web backend with Axum.
    ///
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn rust_backend_axum() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::Rust)
            .kind(ProjectKind::WebBackend)
            .framework(Framework::Rust(RustFramework::Axum))
            .build()
    }

    /// Create a Python web backend with FastAPI.
    ///
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn python_backend_fastapi() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::Python)
            .kind(ProjectKind::WebBackend)
            .framework(Framework::Python(PythonFramework::FastApi))
            .build()
    }

    /// Create a TypeScript frontend with React.
    ///
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn typescript_frontend_react() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::TypeScript)
            .kind(ProjectKind::WebFrontend)
            .framework(Framework::TypeScript(TypeScriptFramework::React))
            .build()
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} ({}{})",
            self.language,
            self.kind,
            self.architecture,
            self.framework
                .as_ref()
                .map(|framework| format!(" + {framework}"))
                .unwrap_or_default()
        )
    }
}

// ============================================================================
// Typestate Markers
// ============================================================================

/// Marker type indicating the builder has no language set yet.
///
/// This prevents calling `.build()` without first setting a language.
pub struct NoLanguage;

/// Marker type indicating the builder has a language set.
///
/// Once in this state, the builder can be finalized with `.build()`.
pub struct HasLanguage;

// ============================================================================
// TargetBuilder (Typestate)
// ============================================================================

/// Builder for constructing validated [`Target`] instances.
///
/// Uses the typestate pattern to enforce that language must be set before
/// building. This catches configuration errors at compile time.
///
/// # Type Parameters
///
/// - `L`: Language state marker (either [`NoLanguage`] or [`HasLanguage`])
///
/// # Examples
///
/// ```rust
/// use scarff_core::{Target, Language, ProjectKind};
///
/// // This compiles - language is set
/// let target = Target::builder()
///     .language(Language::Rust)
///     .build()?;
///
/// // This won't compile - language not set
/// // let target = Target::builder().build()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::TargetBuilder;
    ///
    /// let builder = TargetBuilder::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
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
    /// This transitions the builder to [`HasLanguage`] state, allowing it
    /// to be finalized with `.build()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::{TargetBuilder, Language};
    ///
    /// let builder = TargetBuilder::new()
    ///     .language(Language::Rust);
    /// ```
    #[must_use]
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

// Configuration methods (only available after language is set)
impl TargetBuilder<HasLanguage> {
    /// Set the framework (optional).
    ///
    /// If not set, a default will be inferred based on language and project type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::{Target, Language, Framework, RustFramework};
    ///
    /// let target = Target::builder()
    ///     .language(Language::Rust)
    ///     .framework(Framework::Rust(RustFramework::Axum))
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn framework(mut self, framework: Framework) -> Self {
        self.framework = Some(framework);
        self
    }

    /// Set the project type (optional).
    ///
    /// If not set, a default will be inferred based on the language.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::{Target, Language, ProjectKind};
    ///
    /// let target = Target::builder()
    ///     .language(Language::Rust)
    ///     .kind(ProjectKind::WebBackend)
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn kind(mut self, kind: ProjectKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Set the architecture (optional).
    ///
    /// If not set, a default will be inferred based on framework and project type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::{Target, Language, Architecture};
    ///
    /// let target = Target::builder()
    ///     .language(Language::Rust)
    ///     .architecture(Architecture::Layered)
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn architecture(mut self, architecture: Architecture) -> Self {
        self.architecture = Some(architecture);
        self
    }

    /// Finalize the builder and construct a validated [`Target`].
    ///
    /// This performs all validation and inference:
    /// 1. Validates language is supported
    /// 2. Infers or validates project type
    /// 3. Infers or validates framework
    /// 4. Infers or validates architecture
    /// 5. Checks all compatibility constraints
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Language is not supported
    /// - Framework is incompatible with language
    /// - Framework doesn't support the project type
    /// - Architecture is incompatible with framework or project type
    /// - Required values cannot be inferred
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::{Target, Language};
    ///
    /// let target = Target::builder()
    ///     .language(Language::Rust)
    ///     .build()?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn build(self) -> Result<Target, DomainError> {
        let language = self
            .language
            .expect("HasLanguage state guarantees language is set");

        // Step 1: Validate language is supported
        if !language.is_supported() {
            return Err(DomainError::UnsupportedLanguage {
                language: language.to_string(),
            });
        }

        let (kind, framework, architecture) = self.parse(language)?;

        Ok(Target {
            language,
            kind,
            framework,
            architecture,
        })
    }

    /// Internal parser to validate and infer `kind`, framework, architecture
    fn parse(
        self,
        language: Language,
    ) -> Result<(ProjectKind, Option<Framework>, Architecture), DomainError> {
        // =====================
        // 1️⃣ ProjectKind
        // =====================
        let kind = match self.kind {
            Some(k) => {
                if !k.is_supported() || !k.lang_capable(language) {
                    return Err(DomainError::ProjectKindLanguageMismatch {
                        kind: k.to_string(),
                        language: language.to_string(),
                    });
                }
                k
            }
            None => ProjectKind::infer_from(language).ok_or_else(|| DomainError::CannotInfer {
                field: "kind".into(),
                reason: format!("No default project type for {language}"),
            })?,
        };

        // =====================
        // 2️⃣ Framework
        // =====================
        let framework = match self.framework {
            Some(framework) => {
                // framework must be compatible with both language and project type
                if !framework.is_supported() || !framework.is_compatible((language, kind)) {
                    return Err(DomainError::FrameworkProjectKindMismatch {
                        framework: framework.to_string(),
                        kind: kind.to_string(),
                    });
                }
                framework
            }
            None => Framework::infer_from((language, kind)).ok_or(DomainError::CannotInfer {
                field: "framework".to_string(),
                reason: format!(
                    "cannot infer default for language '{language}' + project type '{kind}'"
                ),
            })?,
        };

        // =====================
        // 3️⃣ Architecture
        // =====================
        let architecture = match self.architecture {
            Some(architecture) => {
                if !architecture.is_supported()||!architecture.is_compatible((language, kind, framework)) {
                    return Err(DomainError::ArchitectureFrameworkMismatch {
                        architecture: architecture.to_string(),
                        framework: framework.to_string(),
                    });
                }
                architecture
            }
            None => Architecture::infer_from((language, kind, framework)).ok_or(DomainError::CannotInfer {
                field: "architecture".to_string(),
                reason: format!(
                    "cannot infer default for language '{language}', project type '{kind}', framework '{framework}'"
                ),
            })?,
        };

        Ok((kind, Some(framework), architecture))
    }
}

// ======================================================
// endregion: Target and TargetBuilder
// ======================================================

//=====================================================
// region: Language
//=====================================================

/// Supported programming languages.
///
/// Each language has specific project types, frameworks, and architectures
/// that work well with it.
///
/// # Examples
///
/// ```rust
/// use scarff_core::Language;
///
/// let lang = Language::Rust;
/// assert_eq!(lang.as_str(), "rust");
/// assert!(lang.is_supported());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Rust programming language
    Rust,
    /// Python programming language
    Python,
    /// TypeScript programming language
    TypeScript,
}

impl Language {
    /// Get the canonical string identifier for this language.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::Language;
    ///
    /// assert_eq!(Language::Rust.as_str(), "rust");
    /// assert_eq!(Language::Python.as_str(), "python");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
        }
    }

    /// Parse a language from a string.
    ///
    /// Accepts both full names and common abbreviations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::Language;
    ///
    /// assert_eq!(Language::parse("rust"), Some(Language::Rust));
    /// assert_eq!(Language::parse("rs"), Some(Language::Rust));
    /// assert_eq!(Language::parse("python"), Some(Language::Python));
    /// assert_eq!(Language::parse("py"), Some(Language::Python));
    /// assert_eq!(Language::parse("invalid"), None);
    /// ```
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "rust" | "rs" => Some(Self::Rust),
            "python" | "py" => Some(Self::Python),
            "typescript" | "ts" => Some(Self::TypeScript),
            _ => None,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Language> for String {
    fn from(l: Language) -> Self {
        l.to_string()
    }
}

impl ActivelySupported for Language {
    /// Get all supported languages.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scarff_core::Language;
    ///
    /// let languages = Language::All;
    /// assert!(languages.contains(&Language::Rust));
    ///
    const ALL: &'static [Self] = &[Self::Rust, Self::Python, Self::TypeScript];
}

// endregion: Language
//=====================================================

//=====================================================
// region: ProjectKind
//=====================================================

/// Type of project being scaffolded.
///
/// Each project type has specific requirements and best practices.
///
/// # Examples
///
/// ```rust
/// use scarff_core::ProjectKind;
///
/// let pt = ProjectKind::Cli;
/// assert_eq!(pt.as_str(), "cli");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectKind {
    /// Command-line interface application
    Cli,
    /// Web backend API
    WebBackend,
    /// Web frontend (SPA)
    WebFrontend,
    /// Fullstack web application
    Fullstack,
    /// Background worker/job processor
    Worker,
}

impl ProjectKind {
    /// Get the canonical string identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::WebBackend => "web-backend",
            Self::WebFrontend => "web-frontend",
            Self::Fullstack => "fullstack",
            Self::Worker => "worker",
        }
    }

    /// Parse a project type from a string.
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "cli" => Some(Self::Cli),
            "web-backend" | "backend" | "api" => Some(Self::WebBackend),
            "web-frontend" | "frontend" => Some(Self::WebFrontend),
            "fullstack" => Some(Self::Fullstack),
            "worker" => Some(Self::Worker),
            _ => None,
        }
    }
}
impl From<ProjectKind> for String {
    fn from(value: ProjectKind) -> Self {
        value.as_str().to_string()
    }
}

impl fmt::Display for ProjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ActivelySupported for ProjectKind {
    /// Project types that are stable and production-ready in the system
    const ALL: &'static [Self] = &[
        Self::Cli,
        Self::WebBackend,
        Self::WebFrontend,
        Self::Fullstack,
        Self::Worker,
    ];
}

// endregion: ProjectKind
//=====================================================

//=====================================================
// region: Framework
//=====================================================

/// Web framework or library.
///
/// Frameworks are language-specific and project-type specific.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Framework {
    /// Rust framework
    Rust(RustFramework),
    /// Python framework
    Python(PythonFramework),
    /// TypeScript framework
    TypeScript(TypeScriptFramework),
}

/// Rust-specific frameworks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustFramework {
    /// Axum web framework
    Axum,
    /// Actix web framework
    Actix,
}

/// Python-specific frameworks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PythonFramework {
    /// FastAPI web framework
    FastApi,
    /// Django web framework
    Django,
}

/// TypeScript-specific frameworks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeScriptFramework {
    /// Express.js backend framework
    Express,
    /// NestJS backend framework
    NestJs,
    /// React frontend library
    React,
    /// Vue.js frontend framework
    Vue,
    /// Next.js fullstack framework
    NextJs,
}

impl Framework {
    /// Get the canonical string identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust(RustFramework::Axum) => "axum",
            Self::Rust(RustFramework::Actix) => "actix",
            Self::Python(PythonFramework::FastApi) => "fastapi",
            Self::Python(PythonFramework::Django) => "django",
            Self::TypeScript(TypeScriptFramework::Express) => "express",
            Self::TypeScript(TypeScriptFramework::NestJs) => "nestjs",
            Self::TypeScript(TypeScriptFramework::React) => "react",
            Self::TypeScript(TypeScriptFramework::Vue) => "vue",
            Self::TypeScript(TypeScriptFramework::NextJs) => "nextjs",
        }
    }

    /// Get the language this framework belongs to.
    #[must_use]
    pub const fn language(self) -> Language {
        match self {
            Self::Rust(_) => Language::Rust,
            Self::Python(_) => Language::Python,
            Self::TypeScript(_) => Language::TypeScript,
        }
    }
}

impl From<Framework> for String {
    fn from(value: Framework) -> Self {
        value.as_str().to_string()
    }
}

impl fmt::Display for Framework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ActivelySupported for Framework {
    /// Frameworks that have first-class templates and inference rules
    const ALL: &'static [Self] = &[
        Framework::Rust(RustFramework::Axum),
        Framework::Rust(RustFramework::Actix),
        Framework::TypeScript(TypeScriptFramework::Express),
        Framework::TypeScript(TypeScriptFramework::NestJs),
        Framework::TypeScript(TypeScriptFramework::React),
        Framework::TypeScript(TypeScriptFramework::Vue),
        Framework::TypeScript(TypeScriptFramework::NextJs),
        Framework::Python(PythonFramework::Django),
        Framework::Python(PythonFramework::FastApi),
    ];
}

// endregion: Framework
//=====================================================

//=====================================================
// region: Architecture
//=====================================================

/// Architectural patterns supported by the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    Layered,
    MVC,
    Clean,
}

impl Architecture {
    pub fn as_str(self) -> &'static str {
        match self {
            Architecture::Layered => "layered",
            Architecture::MVC => "mvc",
            Architecture::Clean => "clean",
        }
    }
}

impl From<Architecture> for String {
    fn from(value: Architecture) -> Self {
        value.as_str().to_string()
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ActivelySupported for Architecture {
    /// Architectures that have concrete templates
    const ALL: &'static [Self] = &[Architecture::Layered];
}

// endregion: Architecture
//=====================================================

// =======================================================
// region: ActivelySupported
// =======================================================
pub trait ActivelySupported: Sized + PartialEq + 'static {
    /// JUST BECAUSE IT IS LISTED AS AN ENUM DOES NOT MEAN IT IS SUPPORTED
    /// this is the list of actual currently actively supported from ENUMS
    /// this is a general list of all actively supported
    const ALL: &'static [Self];

    /// returns true if the type is actively supported
    fn is_supported(&self) -> bool {
        Self::ALL.contains(self)
    }
}

pub trait ActivelySupportedExt: ActivelySupported {
    ///lists out actively supported values
    fn active() -> &'static [Self] {
        Self::ALL
    }
}

impl<T: ActivelySupported> ActivelySupportedExt for T {}

// endregion: ActivelySupported===================================================

//=======================================================
// region:Capabilities and Compatibility
//=======================================================

trait LangCapable {
    fn lang_capable(&self, language: Language) -> bool;
    fn capable_languages(self) -> Vec<Language>;
}

#[derive(Debug, PartialEq, Eq)]
struct LangCapableProjects {
    language: Language,
    p_types: &'static [ProjectKind],
}

const LANG_CAPABILITIES: &[LangCapableProjects] = &[
    // project types rust is capable of doing
    LangCapableProjects {
        language: Language::Rust,
        p_types: &[
            ProjectKind::Cli,
            ProjectKind::Worker,
            ProjectKind::WebBackend,
            ProjectKind::Worker,
        ],
    },
    // project types python is capable of doing
    LangCapableProjects {
        language: Language::Python,
        p_types: &[
            ProjectKind::Cli,
            ProjectKind::Fullstack,
            ProjectKind::WebBackend,
            ProjectKind::Worker,
        ],
    },
    // project types typescript is capable of doing
    LangCapableProjects {
        language: Language::TypeScript,
        p_types: &[
            ProjectKind::WebFrontend,
            ProjectKind::Fullstack,
            ProjectKind::WebBackend,
            ProjectKind::WebFrontend,
            ProjectKind::Worker,
        ],
    },
];

impl LangCapable for ProjectKind {
    ///is it lang_capable: is language capable of doing this project type
    fn lang_capable(&self, language: Language) -> bool {
        if !language.is_supported() {
            return false;
        }

        if !self.is_supported() {
            return false;
        }

        LANG_CAPABILITIES
            .iter()
            .find(|cap| cap.language == language)
            .is_some_and(|cap| cap.p_types.contains(self))
    }

    fn capable_languages(self) -> Vec<Language> {
        LANG_CAPABILITIES
            .iter()
            .filter(|cap| cap.language.is_supported())
            .filter(|cap| cap.p_types.contains(&self))
            .map(|cap| cap.language)
            .collect()
    }
}

// =====================Compatibility=================
/// Trait to define compatibility between an item and a context.
/// This is useful to check if a framework or architecture works with a given language/project type combo.
pub trait Compatible {
    /// The type of context the item can be checked against.
    type Context;

    /// Returns `true` if `self` is compatible with the given `ctx`.
    fn is_compatible(&self, ctx: Self::Context) -> bool;

    /// Returns a list of all contexts `self` is compatible with.
    fn get_compatible(&self) -> Option<Vec<Self::Context>>;
}

// =====================================================
// Implementation for Frameworks
// =====================================================
impl Compatible for Framework {
    type Context = (Language, ProjectKind);

    /// Checks if the framework is compatible with a specific (Lang, PT) pair.
    fn is_compatible(&self, ctx: Self::Context) -> bool {
        match (self, ctx) {
            (
                Framework::Rust(RustFramework::Axum | RustFramework::Actix),
                (Language::Rust, ProjectKind::WebBackend),
            )
            | (
                Framework::TypeScript(TypeScriptFramework::Express | TypeScriptFramework::NestJs),
                (Language::TypeScript, ProjectKind::WebBackend),
            )
            | (
                Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue),
                (Language::TypeScript, ProjectKind::WebFrontend),
            )
            | (
                Framework::TypeScript(TypeScriptFramework::NextJs),
                (Language::TypeScript, ProjectKind::Fullstack),
            )
            | (
                Framework::Python(PythonFramework::FastApi),
                (Language::Python, ProjectKind::WebBackend),
            )
            | (
                Framework::Python(PythonFramework::Django),
                (Language::Python, ProjectKind::Fullstack),
            ) => true,

            // None / unmatched
            _ => false,
        }
    }

    /// Returns all contexts compatible with this framework.
    fn get_compatible(&self) -> Option<Vec<Self::Context>> {
        let contexts = match self {
            Framework::Rust(RustFramework::Axum) => vec![(Language::Rust, ProjectKind::WebBackend)],
            Framework::Rust(RustFramework::Actix) => {
                vec![(Language::Rust, ProjectKind::WebBackend)]
            }

            Framework::TypeScript(TypeScriptFramework::Express | TypeScriptFramework::NestJs) => {
                vec![(Language::TypeScript, ProjectKind::WebBackend)]
            }
            Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue) => {
                vec![(Language::TypeScript, ProjectKind::WebFrontend)]
            }
            Framework::TypeScript(TypeScriptFramework::NextJs) => {
                vec![(Language::TypeScript, ProjectKind::Fullstack)]
            }

            Framework::Python(PythonFramework::FastApi) => {
                vec![(Language::Python, ProjectKind::WebBackend)]
            }
            Framework::Python(PythonFramework::Django) => {
                vec![(Language::Python, ProjectKind::Fullstack)]
            }
        };

        Some(contexts)
    }
}

// =====================================================
// Implementation for Architectures
// =====================================================
impl Compatible for Architecture {
    type Context = (Language, ProjectKind, Framework);

    /// Checks if the architecture is compatible with a specific (Lang, PT, FW) context.
    fn is_compatible(&self, ctx: Self::Context) -> bool {
        matches!(
            (self, ctx),
            (
                Architecture::Layered,
                (
                    Language::Rust,
                    ProjectKind::WebBackend,
                    Framework::Rust(RustFramework::Axum | RustFramework::Actix),
                ) | (Language::Rust, ProjectKind::Cli | ProjectKind::Worker, _)
                    | (
                        Language::TypeScript,
                        ProjectKind::WebBackend,
                        Framework::TypeScript(
                            TypeScriptFramework::Express | TypeScriptFramework::NestJs,
                        ),
                    )
                    | (
                        Language::TypeScript,
                        ProjectKind::Fullstack,
                        Framework::TypeScript(TypeScriptFramework::NextJs),
                    )
                    | (
                        Language::Python,
                        ProjectKind::WebBackend,
                        Framework::Python(PythonFramework::FastApi),
                    ),
            ) | (
                Architecture::MVC,
                (
                    Language::Python,
                    ProjectKind::Fullstack,
                    Framework::Python(PythonFramework::Django),
                ),
            )
        )
    }

    /// Returns all contexts compatible with this architecture.
    fn get_compatible(&self) -> Option<Vec<Self::Context>> {
        let contexts = match self {
            Architecture::Layered => vec![
                // Rust
                (
                    Language::Rust,
                    ProjectKind::WebBackend,
                    Framework::Rust(RustFramework::Axum),
                ),
                (
                    Language::Rust,
                    ProjectKind::WebBackend,
                    Framework::Rust(RustFramework::Actix),
                ),
                // TypeScript
                (
                    Language::TypeScript,
                    ProjectKind::WebBackend,
                    Framework::TypeScript(TypeScriptFramework::Express),
                ),
                (
                    Language::TypeScript,
                    ProjectKind::WebBackend,
                    Framework::TypeScript(TypeScriptFramework::NestJs),
                ),
                (
                    Language::TypeScript,
                    ProjectKind::Fullstack,
                    Framework::TypeScript(TypeScriptFramework::NextJs),
                ),
                // Python
                (
                    Language::Python,
                    ProjectKind::WebBackend,
                    Framework::Python(PythonFramework::FastApi),
                ),
            ],
            Architecture::MVC => {
                vec![(
                    Language::Python,
                    ProjectKind::Fullstack,
                    Framework::Python(PythonFramework::Django),
                )]
            }
            Architecture::Clean => vec![],
        };

        Some(contexts)
    }
}

// endregion: Capabilities===================================================

// ==========================================================================
// region: Infer
// ==========================================================================

trait Infer {
    type Context;

    fn infer_from(ctx: Self::Context) -> Option<Self>
    where
        Self: Sized;
}

impl Infer for ProjectKind {
    type Context = Language;

    fn infer_from(ctx: Self::Context) -> Option<Self>
    where
        Self: Sized,
    {
        match ctx {
            Language::Rust => Some(ProjectKind::Cli),
            Language::TypeScript => Some(ProjectKind::WebFrontend),
            Language::Python => Some(ProjectKind::WebBackend),
        }
    }
}

impl Infer for Framework {
    type Context = (Language, ProjectKind);

    fn infer_from(ctx: Self::Context) -> Option<Self>
    where
        Self: Sized,
    {
        match ctx {
            // ======================
            // Rust
            // ======================
            (Language::Rust, ProjectKind::WebBackend) => Some(Framework::Rust(RustFramework::Axum)),

            // ======================
            // TypeScript
            // ======================
            (Language::TypeScript, ProjectKind::WebBackend) => {
                Some(Framework::TypeScript(TypeScriptFramework::Express))
            }
            (Language::TypeScript, ProjectKind::WebFrontend) => {
                Some(Framework::TypeScript(TypeScriptFramework::React))
            }
            (Language::TypeScript, ProjectKind::Fullstack) => {
                Some(Framework::TypeScript(TypeScriptFramework::NextJs))
            }

            // ======================
            // Python
            // ======================
            (Language::Python, ProjectKind::WebBackend) => {
                Some(Framework::Python(PythonFramework::FastApi))
            }
            (Language::Python, ProjectKind::Fullstack) => {
                Some(Framework::Python(PythonFramework::Django))
            }

            // ======================
            // Explicitly unsupported (MVP)
            // ======================
            (
                Language::Rust,
                ProjectKind::Cli
                | ProjectKind::WebFrontend
                | ProjectKind::Fullstack
                | ProjectKind::Worker,
            )
            | (Language::TypeScript, ProjectKind::Cli | ProjectKind::Worker)
            | (
                Language::Python,
                ProjectKind::Cli | ProjectKind::WebFrontend | ProjectKind::Worker,
            ) => None,
        }
    }
}

impl Infer for Architecture {
    type Context = (Language, ProjectKind, Framework);

    fn infer_from(ctx: Self::Context) -> Option<Self> {
        match ctx {
            // =====================================================
            // Rust
            // =====================================================

            // Rust Web APIs → Layered is the safest default
            (
                Language::Rust,
                ProjectKind::WebBackend,
                Framework::Rust(RustFramework::Axum | RustFramework::Actix),
            ) => {
                // FUTURE:
                // - Clean Architecture (enterprise services)
                // - Hexagonal (ports/adapters)
                Some(Architecture::Layered)
            }

            // Rust CLI tools → Layered
            (Language::Rust, ProjectKind::Cli, _) => {
                // FUTURE:
                // - Single-module for tiny CLIs
                // - Clean for large tools
                Some(Architecture::Layered)
            }

            // Rust Workers
            (Language::Rust, ProjectKind::Worker, _) => {
                // FUTURE:
                // - Clean Architecture for long-running workers
                Some(Architecture::Layered)
            }

            // Rust Frontend / UI
            (Language::Rust, ProjectKind::WebFrontend, _) => {
                // FUTURE:
                // - MVU-style architectures
                // - Feature-based layouts
                None
            }

            // Rust Fullstack
            (Language::Rust, ProjectKind::Fullstack, _) => {
                // FUTURE:
                // - Clean Architecture
                // - Modular monolith
                None
            }

            // =====================================================
            // TypeScript
            // =====================================================

            // TS Web APIs
            (
                Language::TypeScript,
                ProjectKind::WebBackend,
                Framework::TypeScript(TypeScriptFramework::Express | TypeScriptFramework::NestJs),
            ) => {
                // NestJS strongly implies layered
                Some(Architecture::Layered)
            }

            // TS Web Frontend
            (
                Language::TypeScript,
                ProjectKind::WebFrontend,
                Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue),
            ) => {
                // FUTURE:
                // - Feature-based
                // - Atomic design
                None
            }

            // TS Fullstack → Next.js
            (
                Language::TypeScript,
                ProjectKind::Fullstack,
                Framework::TypeScript(TypeScriptFramework::NextJs),
            ) => {
                // FUTURE:
                // - App Router architecture
                // - Domain-driven modules
                Some(Architecture::Layered)
            }

            // TS CLI / Worker
            (Language::TypeScript, ProjectKind::Cli | ProjectKind::Worker, _) => {
                // FUTURE:
                // - Clean Architecture for tooling
                None
            }

            // =====================================================
            // Python
            // =====================================================

            // Python Web APIs
            (
                Language::Python,
                ProjectKind::WebBackend,
                Framework::Python(PythonFramework::FastApi),
            ) => {
                // FastAPI commonly uses layered routers/services
                Some(Architecture::Layered)
            }

            // Python Fullstack
            (
                Language::Python,
                ProjectKind::Fullstack,
                Framework::Python(PythonFramework::Django),
            ) => {
                // Django enforces MVC
                Some(Architecture::MVC)
            }

            // Python CLI / Worker
            (Language::Python, ProjectKind::Cli | ProjectKind::Worker, _) => {
                // FUTURE:
                // - Clean Architecture for workers
                None
            }

            // =====================================================
            // Unsupported / incomplete inputs
            // =====================================================
            _ => None,
        }
    }
}
// endregion: Infer defaults================================================

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ====================================================
    // ActivelySupported tests
    // ====================================================
    #[test]
    fn check_actively_supported_pt() {
        let p_type_list = [
            ProjectKind::Cli,
            ProjectKind::WebBackend,
            ProjectKind::Fullstack,
        ];
        let expected = [true, true, true];

        let actual: Vec<bool> = p_type_list
            .iter()
            .map(super::ActivelySupported::is_supported)
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn check_actively_supported_lang() {
        let active = [Language::Python, Language::Rust, Language::TypeScript];

        for l in active {
            assert!(l.is_supported(), "{l:?} should be actively supported");
        }
    }

    #[test]
    fn list_active_langs() {
        let active = Language::active();
        assert!(
            !active.is_empty(),
            "there must be at least one actively supported language"
        );
        for l in active {
            println!("Active language: {l:?}");
        }
    }

    // ====================================================
    // lang_capable tests
    // ====================================================
    #[test]
    fn check_lang_capable_pt() {
        let tests = [
            (ProjectKind::Cli, Language::Rust, true),
            (ProjectKind::Cli, Language::TypeScript, false),
            (ProjectKind::WebBackend, Language::Rust, true),
            (ProjectKind::WebFrontend, Language::TypeScript, true),
            (ProjectKind::WebFrontend, Language::Python, false),
        ];

        for (pt, language, expected) in tests {
            assert_eq!(
                pt.lang_capable(language),
                expected,
                "Mismatch: {language:?} capable of {pt:?} should be {expected}"
            );
        }
    }

    #[test]
    fn check_capable_languages_list() {
        let capable = ProjectKind::WebBackend.capable_languages();
        assert!(capable.contains(&Language::Rust));
        assert!(capable.contains(&Language::TypeScript));
        assert!(capable.contains(&Language::Python));
    }

    // ====================================================
    // InferDefault tests
    // ====================================================
    #[test]
    fn infer_pt_defaults() {
        let inferred = ProjectKind::infer_from(Language::Rust).unwrap();
        assert!(inferred.is_supported(), "Inferred PT must be supported");
    }

    #[test]
    fn infer_fw_defaults() {
        let inferred = Framework::infer_from((Language::Rust, ProjectKind::WebBackend)).unwrap();
        assert_eq!(inferred, Framework::Rust(RustFramework::Axum));
    }

    #[test]
    fn infer_arch_defaults() {
        let inferred = Architecture::infer_from((
            Language::Rust,
            ProjectKind::WebBackend,
            Framework::Rust(RustFramework::Axum),
        ))
        .unwrap();
        assert!(matches!(
            inferred,
            Architecture::Layered | Architecture::MVC | Architecture::Clean
        ));
    }

    // ====================================================
    // Compatible trait tests
    // ====================================================
    #[test]
    fn fw_compatibility() {
        assert!(
            Framework::Rust(RustFramework::Axum)
                .is_compatible((Language::Rust, ProjectKind::WebBackend))
        );
        assert!(
            !Framework::Rust(RustFramework::Axum)
                .is_compatible((Language::TypeScript, ProjectKind::WebBackend))
        );
    }

    #[test]
    fn arch_compatibility() {
        let architecture = Architecture::Layered;
        assert!(architecture.is_compatible((
            Language::Rust,
            ProjectKind::WebBackend,
            Framework::Rust(RustFramework::Axum)
        )));
    }

    // ====================================================
    // TargetBuilder end-to-end tests
    // ====================================================
    #[test]
    fn build_complete_target() {
        let target = TargetBuilder::new()
            .language(Language::Rust)
            .kind(ProjectKind::WebBackend)
            .framework(Framework::Rust(RustFramework::Axum))
            .architecture(Architecture::Layered)
            .build()
            .unwrap();

        assert_eq!(target.language, Language::Rust);
        assert_eq!(target.kind, ProjectKind::WebBackend);
        assert_eq!(target.framework, Some(Framework::Rust(RustFramework::Axum)));
        assert_eq!(target.architecture, Architecture::Layered);
    }

    #[test]
    fn build_partial_target_with_inference() {
        let target = TargetBuilder::new()
            .language(Language::TypeScript)
            .kind(ProjectKind::Fullstack)
            .build()
            .unwrap();

        // Should have inferred framework and architecture
        assert_eq!(target.language, Language::TypeScript);
        assert_eq!(target.kind, ProjectKind::Fullstack);
        assert_eq!(
            target.framework,
            Some(Framework::TypeScript(TypeScriptFramework::NextJs))
        );
        assert!(matches!(
            target.architecture,
            Architecture::Layered | Architecture::MVC | Architecture::Clean
        ));
    }

    #[test]
    fn build_target_error_on_incompatible_kind() {
        let result = TargetBuilder::new()
            .language(Language::TypeScript)
            .kind(ProjectKind::Cli)
            .framework(Framework::TypeScript(TypeScriptFramework::React))
            .build();

        println!("{:?}", result.as_ref());

        assert!(matches!(
            result,
            Err(DomainError::ProjectKindLanguageMismatch { .. })
        ));
    }

    #[test]
    fn build_target_error_on_inference_failure() {
        let result = TargetBuilder::new().language(Language::TypeScript).build();

        println!("{:?}", result.as_ref());

        // Python CLI has no framework default → can infer
        assert!(
            matches!(result, Err(DomainError::CannotInfer { field, .. }) if field == "architecture" || field == "framework")
        );
    }

    #[test]
    fn target_builder_requires_language() {
        // This compiles
        let target = Target::builder().language(Language::Rust).build().unwrap();

        println!("{target:?}");

        assert_eq!(target.language(), Language::Rust);
    }

    #[test]
    fn target_with_defaults() {
        let target = Target::builder().language(Language::Rust).build().unwrap();

        assert_eq!(target.language(), Language::Rust);
        assert_eq!(target.kind(), ProjectKind::Cli);
        assert_eq!(target.architecture(), Architecture::Layered);
    }

    #[test]
    fn target_explicit_all_fields() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::WebBackend)
            .framework(Framework::Rust(RustFramework::Axum))
            .architecture(Architecture::Layered)
            .build()
            .unwrap();

        assert_eq!(target.language(), Language::Rust);
        assert_eq!(target.kind(), ProjectKind::WebBackend);
        assert_eq!(
            target.framework(),
            Some(Framework::Rust(RustFramework::Axum))
        );
        assert_eq!(target.architecture(), Architecture::Layered);
    }

    #[test]
    fn target_rejects_incompatible_framework() {
        let result = Target::builder()
            .language(Language::Rust)
            .framework(Framework::Python(PythonFramework::Django))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn language_parse() {
        assert_eq!(Language::parse("rust"), Some(Language::Rust));
        assert_eq!(Language::parse("rs"), Some(Language::Rust));
        assert_eq!(Language::parse("python"), Some(Language::Python));
        assert_eq!(Language::parse("py"), Some(Language::Python));
        assert_eq!(Language::parse("invalid"), None);
    }

    #[test]
    fn preset_rust_cli() {
        let target = Target::rust_cli().unwrap();
        assert_eq!(target.language(), Language::Rust);
        assert_eq!(target.kind(), ProjectKind::Cli);
    }

    #[test]
    fn preset_rust_backend_axum() {
        let target = Target::rust_backend_axum().unwrap();
        assert_eq!(target.language(), Language::Rust);
        assert_eq!(target.kind(), ProjectKind::WebBackend);
        assert_eq!(
            target.framework(),
            Some(Framework::Rust(RustFramework::Axum))
        );
    }
}
