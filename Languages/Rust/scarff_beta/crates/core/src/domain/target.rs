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
//! ```

use std::{fmt, marker::PhantomData};

use crate::DomainError;

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
    /// Architecture
    pub architecture: Architecture,
}

impl Target {
    /// Create a new builder to construct a Target.
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
    /// # Errors
    ///
    /// This should not fail as it uses a known-good configuration.
    pub fn rust_cli() -> Result<Self, DomainError> {
        Self::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)?
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
            .kind(ProjectKind::WebBackend)?
            .framework(Framework::Rust(RustFramework::Axum))?
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
            .kind(ProjectKind::WebBackend)?
            .framework(Framework::Python(PythonFramework::FastApi))?
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
            .kind(ProjectKind::WebFrontend)?
            .framework(Framework::TypeScript(TypeScriptFramework::React))?
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
pub struct NoLanguage;

/// Marker type indicating the builder has a language set.
pub struct HasLanguage;

// ============================================================================
// TargetBuilder (Typestate)
// ============================================================================

/// Builder for constructing validated [`Target`] instances.
pub struct TargetBuilder<L> {
    language: Option<Language>,
    framework: Option<Framework>,
    kind: Option<ProjectKind>,
    architecture: Option<Architecture>,
    _language_state: PhantomData<L>,
}

impl TargetBuilder<NoLanguage> {
    /// Create a new builder. Language must be set before calling `build()`.
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

impl TargetBuilder<HasLanguage> {
    /// Set the framework (optional).
    #[must_use]
    pub fn framework(mut self, framework: Framework) -> Result<Self, DomainError> {
        // Validate immediately
        if let Some(lang) = self.language
            && framework.language() != lang
        {
            Err(DomainError::FrameworkLanguageMismatch {
                framework: framework.to_string(),
                language: lang.to_string(),
            })?;
        }
        // if framework is required then it must be provided
        // else if kind.requires_framework() && fram
        self.framework = Some(framework);
        Ok(self)
    }

    /// Set the project type (optional).
    #[must_use]
    pub fn kind(mut self, kind: ProjectKind) -> Result<Self, DomainError> {
        if let Some(lang) = self.language
            && (!kind.is_supported() || !kind.lang_capable(lang))
        {
            Err(DomainError::ProjectKindLanguageMismatch {
                kind: kind.to_string(),
                language: lang.to_string(),
            })?;
        }
        self.kind = Some(kind);
        Ok(self)
    }

    /// Set the architecture (optional).
    #[must_use]
    pub fn architecture(mut self, architecture: Architecture) -> Result<Self, DomainError> {
        if let Some(lang) = self.language
            && let Some(kind) = self.kind
            && (!architecture.is_supported()
                || !architecture.is_compatible((lang, kind, self.framework)))
        {
            Err(DomainError::ArchitectureProjectKindMismatch {
                architecture: architecture.to_string(),
                kind: kind.to_string(),
            })?;
        }
        self.architecture = Some(architecture);
        Ok(self)
    }

    /// Finalize the builder and construct a validated [`Target`].
    ///
    /// This performs all validation and inference:
    /// 1. Validates language is supported
    /// 2. Infers or validates project type
    /// 3. Infers or validates framework (optional)
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
    ///
    /// ## Inference Strategy
    ///
    /// The inference follows this priority:
    /// 1. **`ProjectKind`**: Infer from language if not provided
    /// 2. **Framework**: Try to infer from (language, kind), but allow None for CLI/Worker
    /// 3. **Architecture**: Infer from (language, kind, framework)
    ///
    /// ## Key Rule
    /// Framework is **optional** for some project types (CLI, Worker).
    /// We only error if framework inference fails AND it's required for that project type.
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
        // 2️⃣ Framework (OPTIONAL for some types)
        // =====================
        let framework = if let Some(fw) = self.framework {
            if !fw.is_supported() {
                return Err(DomainError::FrameworkRequired {
                    kind: kind.to_string(),
                });
            }

            if !fw.is_compatible((language, kind)) {
                return Err(DomainError::FrameworkProjectKindMismatch {
                    framework: fw.to_string(),
                    kind: kind.to_string(),
                });
            }

            Some(fw)
        } else {
            let inferred = Framework::infer_from((language, kind));

            // Check if this project type REQUIRES a framework
            if inferred.is_none() && kind.requires_framework() {
                return Err(DomainError::FrameworkRequired {
                    kind: kind.to_string(),
                });
            }

            inferred
        };

        // =====================
        // 3️⃣ Architecture
        // =====================
        let architecture = match self.architecture {
            Some(arch) => {
                if !arch.is_supported() {
                    return Err(DomainError::UnsupportedArchitecture {
                        architecture: arch.to_string(),
                    });
                }

                if !arch.is_compatible((language, kind, framework)) {
                    return Err(DomainError::ArchitectureFrameworkMismatch {
                        architecture: arch.to_string(),
                        framework: framework.map_or_else(|| "none".to_string(), |f| f.to_string()),
                    });
                }

                arch
            }
            None => Architecture::infer_from((language, kind, framework)).ok_or_else(|| {
                DomainError::CannotInfer {
                    field: "architecture".to_string(),
                    reason: format!(
                        "Cannot infer architecture for {} {} {}",
                        language,
                        kind,
                        framework.map_or_else(|| "none".to_string(), |f| f.to_string())
                    ),
                }
            })?,
        };

        Ok((kind, framework, architecture))
    }
}

// ============================================================================
// Language
// ============================================================================

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
}

impl Language {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
        }
    }

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
    const ALL: &'static [Self] = &[Self::Rust, Self::Python, Self::TypeScript];
}

// ============================================================================
// ProjectKind
// ============================================================================

/// Type of project being scaffolded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectKind {
    Cli,
    WebBackend,
    WebFrontend,
    Fullstack,
    Worker,
}

impl ProjectKind {
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

    /// Check if this project type requires a framework.
    ///
    /// CLI and Worker projects don't require frameworks.
    /// Web projects (backend, frontend, fullstack) do.
    #[must_use]
    pub const fn requires_framework(self) -> bool {
        matches!(self, Self::WebBackend | Self::WebFrontend | Self::Fullstack)
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
    const ALL: &'static [Self] = &[
        Self::Cli,
        Self::WebBackend,
        Self::WebFrontend,
        Self::Fullstack,
        Self::Worker,
    ];
}

// ============================================================================
// Framework
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Framework {
    Rust(RustFramework),
    Python(PythonFramework),
    TypeScript(TypeScriptFramework),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustFramework {
    Axum,
    Actix,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PythonFramework {
    FastApi,
    Django,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeScriptFramework {
    Express,
    NestJs,
    React,
    Vue,
    NextJs,
}

impl Framework {
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

    #[must_use]
    pub const fn language(self) -> Language {
        match self {
            Self::Rust(_) => Language::Rust,
            Self::Python(_) => Language::Python,
            Self::TypeScript(_) => Language::TypeScript,
        }
    }

    // #[must_use]
    // depending on framework we can infer the kind of project user should wants to build?
    // pub const fn kind(self)-> ProjectKind{
    //     match self{
    //         Framework::Rust(_rust_framework) => ProjectKind::WebBackend,
    //         Framework::Python(_python_framework) => Proj,
    //         Framework::TypeScript(type_script_framework) => todo!(),
    //     }
    // }
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

// ============================================================================
// Architecture
// ============================================================================

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
    const ALL: &'static [Self] = &[
        Architecture::Layered,
        Architecture::MVC,
        Architecture::Clean,
    ];
}

// ============================================================================
// Traits
// ============================================================================

pub trait ActivelySupported: Sized + PartialEq + 'static {
    const ALL: &'static [Self];

    fn is_supported(&self) -> bool {
        Self::ALL.contains(self)
    }
}

pub trait ActivelySupportedExt: ActivelySupported {
    fn active() -> &'static [Self] {
        Self::ALL
    }
}

impl<T: ActivelySupported> ActivelySupportedExt for T {}

pub trait LangCapable {
    fn lang_capable(&self, language: Language) -> bool;
    fn capable_languages(self) -> Vec<Language>;
}

#[derive(Debug, PartialEq, Eq)]
struct LangCapableProjects {
    language: Language,
    p_types: &'static [ProjectKind],
}

const LANG_CAPABILITIES: &[LangCapableProjects] = &[
    LangCapableProjects {
        language: Language::Rust,
        p_types: &[
            ProjectKind::Cli,
            ProjectKind::WebBackend,
            ProjectKind::Worker,
        ],
    },
    LangCapableProjects {
        language: Language::Python,
        p_types: &[
            ProjectKind::Cli,
            ProjectKind::Fullstack,
            ProjectKind::WebBackend,
            ProjectKind::Worker,
        ],
    },
    LangCapableProjects {
        language: Language::TypeScript,
        p_types: &[
            ProjectKind::WebFrontend,
            ProjectKind::Fullstack,
            ProjectKind::WebBackend,
            ProjectKind::Worker,
        ],
    },
];

impl LangCapable for ProjectKind {
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

pub trait Compatible {
    type Context;
    fn is_compatible(&self, ctx: Self::Context) -> bool;
    fn get_compatible(&self) -> Option<Vec<Self::Context>>;
}

impl Compatible for Framework {
    type Context = (Language, ProjectKind);

    fn is_compatible(&self, ctx: Self::Context) -> bool {
        matches!(
            (self, ctx),
            (
                Framework::Rust(RustFramework::Axum | RustFramework::Actix),
                (Language::Rust, ProjectKind::WebBackend),
            ) | (
                Framework::TypeScript(TypeScriptFramework::Express | TypeScriptFramework::NestJs),
                (Language::TypeScript, ProjectKind::WebBackend),
            ) | (
                Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue),
                (Language::TypeScript, ProjectKind::WebFrontend),
            ) | (
                Framework::TypeScript(TypeScriptFramework::NextJs),
                (Language::TypeScript, ProjectKind::Fullstack),
            ) | (
                Framework::Python(PythonFramework::FastApi),
                (Language::Python, ProjectKind::WebBackend),
            ) | (
                Framework::Python(PythonFramework::Django),
                (Language::Python, ProjectKind::Fullstack),
            )
        )
    }

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

impl Compatible for Architecture {
    type Context = (Language, ProjectKind, Option<Framework>);

    fn is_compatible(&self, ctx: Self::Context) -> bool {
        match (self, ctx) {
            // Layered architecture - works with most combinations
            (
                Architecture::Layered,
                (Language::Rust, ProjectKind::Cli | ProjectKind::Worker, None),
            ) => true,
            (
                Architecture::Layered,
                (
                    Language::Rust,
                    ProjectKind::WebBackend,
                    Some(Framework::Rust(RustFramework::Axum | RustFramework::Actix)),
                ),
            ) => true,
            (
                Architecture::Layered,
                (
                    Language::TypeScript,
                    ProjectKind::WebBackend,
                    Some(Framework::TypeScript(
                        TypeScriptFramework::Express | TypeScriptFramework::NestJs,
                    )),
                ),
            ) => true,
            (
                Architecture::Layered,
                (
                    Language::TypeScript,
                    ProjectKind::Fullstack,
                    Some(Framework::TypeScript(TypeScriptFramework::NextJs)),
                ),
            ) => true,
            (
                Architecture::Layered,
                (
                    Language::Python,
                    ProjectKind::WebBackend,
                    Some(Framework::Python(PythonFramework::FastApi)),
                ),
            ) => true,

            // MVC - Django only
            (
                Architecture::MVC,
                (
                    Language::Python,
                    ProjectKind::Fullstack,
                    Some(Framework::Python(PythonFramework::Django)),
                ),
            ) => true,

            _ => false,
        }
    }

    fn get_compatible(&self) -> Option<Vec<Self::Context>> {
        let contexts = match self {
            Architecture::Layered => vec![
                (Language::Rust, ProjectKind::Cli, None),
                (Language::Rust, ProjectKind::Worker, None),
                (
                    Language::Rust,
                    ProjectKind::WebBackend,
                    Some(Framework::Rust(RustFramework::Axum)),
                ),
                (
                    Language::TypeScript,
                    ProjectKind::WebBackend,
                    Some(Framework::TypeScript(TypeScriptFramework::Express)),
                ),
                (
                    Language::Python,
                    ProjectKind::WebBackend,
                    Some(Framework::Python(PythonFramework::FastApi)),
                ),
            ],
            Architecture::MVC => vec![(
                Language::Python,
                ProjectKind::Fullstack,
                Some(Framework::Python(PythonFramework::Django)),
            )],
            Architecture::Clean => vec![],
        };

        Some(contexts)
    }
}

trait Infer {
    type Context;
    fn infer_from(ctx: Self::Context) -> Option<Self>
    where
        Self: Sized;
}

impl Infer for ProjectKind {
    type Context = Language;

    fn infer_from(ctx: Self::Context) -> Option<Self> {
        match ctx {
            Language::Rust => Some(ProjectKind::Cli),
            Language::TypeScript => Some(ProjectKind::WebFrontend),
            Language::Python => Some(ProjectKind::WebBackend),
        }
    }
}

impl Infer for Framework {
    type Context = (Language, ProjectKind);

    fn infer_from(ctx: Self::Context) -> Option<Self> {
        match ctx {
            // Rust
            (Language::Rust, ProjectKind::WebBackend) => Some(Framework::Rust(RustFramework::Axum)),
            (Language::Rust, ProjectKind::Cli | ProjectKind::Worker) => None, // No framework needed

            // TypeScript
            (Language::TypeScript, ProjectKind::WebBackend) => {
                Some(Framework::TypeScript(TypeScriptFramework::Express))
            }
            (Language::TypeScript, ProjectKind::WebFrontend) => {
                Some(Framework::TypeScript(TypeScriptFramework::React))
            }
            (Language::TypeScript, ProjectKind::Fullstack) => {
                Some(Framework::TypeScript(TypeScriptFramework::NextJs))
            }

            // Python
            (Language::Python, ProjectKind::WebBackend) => {
                Some(Framework::Python(PythonFramework::FastApi))
            }
            (Language::Python, ProjectKind::Fullstack) => {
                Some(Framework::Python(PythonFramework::Django))
            }
            (Language::Python, ProjectKind::Cli | ProjectKind::Worker) => None, // No framework needed

            _ => None,
        }
    }
}

impl Infer for Architecture {
    type Context = (Language, ProjectKind, Option<Framework>);

    fn infer_from(ctx: Self::Context) -> Option<Self> {
        match ctx {
            // Rust - Layered for everything
            (Language::Rust, _, _) => Some(Architecture::Layered),

            // TypeScript
            (Language::TypeScript, _, Some(Framework::TypeScript(_))) => {
                Some(Architecture::Layered)
            }

            // Python
            (
                Language::Python,
                ProjectKind::Fullstack,
                Some(Framework::Python(PythonFramework::Django)),
            ) => Some(Architecture::MVC),
            (Language::Python, _, Some(Framework::Python(PythonFramework::FastApi))) => {
                Some(Architecture::Layered)
            }

            _ => Some(Architecture::Layered), // Default fallback
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_builder_requires_language() {
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
        assert_eq!(target.framework(), None); // CLI doesn't need framework
    }

    #[test]
    fn target_explicit_all_fields() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .framework(Framework::Rust(RustFramework::Axum))
            .unwrap()
            .architecture(Architecture::Layered)
            .unwrap()
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
            .inspect_err(|e| eprintln!("{e:?}"))
            .expect("error building  result becaus eof framework mismatch")
            .build();

        println!("{result:?}");
        assert!(!result.is_err(), "because unwrapped");
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

    #[test]
    fn build_partial_target_with_inference() {
        let target = TargetBuilder::new()
            .language(Language::TypeScript)
            .kind(ProjectKind::Fullstack)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(target.language, Language::TypeScript);
        assert_eq!(target.kind, ProjectKind::Fullstack);
        assert_eq!(
            target.framework,
            Some(Framework::TypeScript(TypeScriptFramework::NextJs))
        );
        assert!(matches!(target.architecture, Architecture::Layered));
    }

    #[test]
    fn infer_framework_for_web_backend() {
        let target = Target::builder()
            .language(Language::Python)
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            target.framework(),
            Some(Framework::Python(PythonFramework::FastApi))
        );
    }

    #[test]
    fn cli_does_not_require_framework() {
        let target = Target::builder()
            .language(Language::Rust)
            .kind(ProjectKind::Cli)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(target.framework(), None);
    }

    #[test]
    fn worker_does_not_require_framework() {
        let target = Target::builder()
            .language(Language::Python)
            .kind(ProjectKind::Worker)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(target.framework(), None);
    }

    #[test]
    fn web_backend_requires_framework_if_not_inferable() {
        // This should succeed because FastAPI can be inferred
        let result = Target::builder()
            .language(Language::Python)
            .kind(ProjectKind::WebBackend)
            .unwrap()
            .build();

        assert!(result.is_ok());
    }
}
