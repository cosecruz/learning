//! Target modeling with typestate builder pattern.
//!
//! This module provides the [`Target`] type, which represents a fully validated
//! project configuration. Targets are constructed using a builder pattern that
//! enforces compile-time guarantees about required fields.
//!
//! # Examples
//!
//! ```rust
//! use scarff_core::{Target, Language, ProjectType};
//!
//! // Minimal target - other fields inferred
//! let target = Target::builder()
//!     .language(Language::Rust)
//!     .build()?;
//!
//! // Fully specified target
//! let target = Target::builder()
//!     .language(Language::Rust)
//!     .project_type(ProjectType::WebBackend)
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
/// use scarff_core::{Target, Language, ProjectType};
///
/// let target = Target::builder()
///     .language(Language::Rust)
///     .project_type(ProjectType::Cli)
///     .build()?;
///
/// assert_eq!(target.language(), Language::Rust);
/// assert_eq!(target.project_type(), ProjectType::Cli);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Target {
    /// Language of the project
    pub language: Lang,
    /// Project type
    pub project_type: PT,
    /// Framework, if any
    pub framework: FW,
    /// Architecture, if any
    pub architecture: Arch,
}

/// Marker type indicating the builder has **no language set** yet.
pub struct NoLang;

/// Marker type indicating the builder **has a language**.
pub struct HasLang;

/// Builder for `Target` using typestate pattern to ensure `Lang` is set first.
pub struct TargetBuilder<L> {
    lang: Option<Lang>,
    p_type: Option<PT>,
    fw: Option<FW>,
    arch: Option<Arch>,
    _lang_state: PhantomData<L>,
}

impl TargetBuilder<NoLang> {
    /// Create a new builder without language.
    pub fn new() -> Self {
        Self {
            lang: None,
            p_type: None,
            fw: None,
            arch: None,
            _lang_state: PhantomData,
        }
    }

    /// Set the language and move to `HasLang` state.
    pub fn with_lang(self, lang: Lang) -> TargetBuilder<HasLang> {
        TargetBuilder {
            lang: Some(lang),
            p_type: self.p_type,
            fw: self.fw,
            arch: self.arch,
            _lang_state: PhantomData,
        }
    }
}

impl Default for TargetBuilder<NoLang> {
    fn default() -> Self {
        Self::new()
    }
}

impl TargetBuilder<HasLang> {
    /// Set the framework
    pub fn with_fw(mut self, fw: FW) -> Self {
        self.fw = Some(fw);
        self
    }

    /// Set the project type
    pub fn with_p_type(mut self, p_type: PT) -> Self {
        self.p_type = Some(p_type);
        self
    }

    /// Set the architecture
    pub fn with_arch(mut self, arch: Arch) -> Self {
        self.arch = Some(arch);
        self
    }

    /// Build the final `Target`, validating and inferring missing values
    pub fn build(self) -> Result<Target, DomainError> {
        let lang = self.lang.ok_or(DomainError::UnsupportedLanguage {
            language: String::new(),
        })?;

        if !lang.actively_supported() {
            return Err(DomainError::UnsupportedLanguage {
                language: lang.into(),
            });
        }

        let (p_type, fw, arch) = self.parse(lang)?;

        Ok(Target {
            language: lang,
            project_type: p_type,
            framework: fw,
            architecture: arch,
        })
    }

    /// Internal parser to validate and infer `project_type`, framework, architecture
    fn parse(self, lang: Lang) -> Result<(PT, FW, Arch), DomainError> {
        // =====================
        // 1️⃣ ProjectType
        // =====================
        let p_type = match self.p_type {
            Some(pt) => {
                if !pt.actively_supported() || !pt.lang_capable(lang) {
                    return Err(DomainError::ProjectTypeLanguageMismatch {
                        project_type: pt.to_string(),
                        language: lang.to_string(),
                    });
                }
                pt
            }
            None => PT::infer_default(lang).ok_or(DomainError::CannotInfer {
                field: "project_type".to_string(),
                reason: "cannot infer default for this language".to_string(),
            })?,
        };

        // =====================
        // 2️⃣ Framework
        // =====================
        let fw = match self.fw {
            Some(fw) => {
                // framework must be compatible with both lang and project type
                if !fw.actively_supported() || !fw.is_compatible((lang, p_type)) {
                    return Err(DomainError::FrameworkProjectTypeMismatch {
                        framework: fw.to_string(),
                        project_type: p_type.to_string(),
                    });
                }
                fw
            }
            None => FW::infer_default((lang, p_type)).ok_or(DomainError::CannotInfer {
                field: "framework".to_string(),
                reason: format!(
                    "cannot infer default for language '{lang}' + project type '{p_type}'"
                ),
            })?,
        };

        // =====================
        // 3️⃣ Architecture
        // =====================
        let arch = match self.arch {
            Some(arch) => {
                if !arch.actively_supported()||!arch.is_compatible((lang, p_type, fw)) {
                    return Err(DomainError::ArchitectureFrameworkMismatch {
                        architecture: arch.to_string(),
                        framework: fw.to_string(),
                    });
                }
                arch
            }
            None => Arch::infer_default((lang, p_type, fw)).ok_or(DomainError::CannotInfer {
                field: "architecture".to_string(),
                reason: format!(
                    "cannot infer default for language '{lang}', project type '{p_type}', framework '{fw}'"
                ),
            })?,
        };

        Ok((p_type, fw, arch))
    }
}

// ======================================================
// endregion: Target and TargetBuilder
// ======================================================

//=====================================================
// region: Language
//=====================================================

/// Programming languages supported by the system.
///
/// `None` represents either:
/// - unknown input
/// - unsupported language
/// - user omission (to be inferred later)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    Rust,
    TS,
    PY,
    None,
}

impl Lang {
    /// Canonical lowercase identifier used in CLI, config, and templates
    pub fn as_str(self) -> &'static str {
        match self {
            Lang::Rust => "rust",
            Lang::TS => "typescript",
            Lang::PY => "python",
            Lang::None => "none",
        }
    }
}

impl From<String> for Lang {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "rust" | "rs" => Self::Rust,
            "typescript" | "ts" => Self::TS,
            "python" | "py" => Self::PY,
            _ => Self::None,
        }
    }
}

impl From<Lang> for String {
    fn from(value: Lang) -> Self {
        value.as_str().to_string()
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ActivelySupported for Lang {
    /// Languages that are first-class citizens in the system
    const ACTIVELY_SUPPORTED: &'static [Self] = &[Self::Rust, Self::PY, Self::TS];
}

// endregion: Language
//=====================================================

//=====================================================
// region: ProjectType
//=====================================================

/// High-level classification of what kind of project is being built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PT {
    Cli,
    WebApi,
    Api,
    WebFE,
    MobileFE,
    DesktopFE,
    TUI,
    Fullstack,
    Worker,
    None,
}

impl PT {
    /// Canonical identifier used in CLI flags and templates
    pub fn as_str(self) -> &'static str {
        match self {
            PT::Cli => "cli",
            PT::WebApi => "web_api",
            PT::Api => "api",
            PT::WebFE => "web_fe",
            PT::MobileFE => "mobile_fe",
            PT::DesktopFE => "desktop_fe",
            PT::TUI => "tui",
            PT::Fullstack => "fullstack",
            PT::Worker => "worker",
            PT::None => "none",
        }
    }
}

impl From<String> for PT {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "cli" => Self::Cli,
            "webapi" | "web_api" => Self::WebApi,
            "api" => Self::Api,
            "webfe" | "web_fe" => Self::WebFE,
            "fullstack" => Self::Fullstack,
            _ => Self::None,
        }
    }
}

impl From<PT> for String {
    fn from(value: PT) -> Self {
        value.as_str().to_string()
    }
}

impl fmt::Display for PT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ActivelySupported for PT {
    /// Project types that are stable and production-ready in the system
    const ACTIVELY_SUPPORTED: &'static [Self] =
        &[Self::Cli, Self::WebApi, Self::WebFE, Self::Fullstack];
}

// endregion: ProjectType
//=====================================================

//=====================================================
// region: Framework
//=====================================================

/// Frameworks supported by the system.
///
/// Frameworks are **language + domain specific**.
/// Naming encodes intent to avoid ambiguity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FW {
    None,

    // ======================
    // Rust
    // ======================
    RustWebAxum,
    RustWebActix,

    // ======================
    // TypeScript
    // ======================
    TSWebExpress,
    TSWebNestJS,
    TSWebFEReact,
    TSWebFEVue,
    TSFullStackNext,

    // ======================
    // Python
    // ======================
    PYWebFastApi,
    PYWebDjango,
}

impl FW {
    /// Canonical identifier used in templates and CLI
    pub fn as_str(self) -> &'static str {
        match self {
            FW::None => "none",

            FW::RustWebAxum => "axum",
            FW::RustWebActix => "actix",

            FW::TSWebExpress => "express",
            FW::TSWebNestJS => "nestjs",
            FW::TSWebFEReact => "react",
            FW::TSWebFEVue => "vue",
            FW::TSFullStackNext => "nextjs",

            FW::PYWebFastApi => "fastapi",
            FW::PYWebDjango => "django",
        }
    }
}

impl From<FW> for String {
    fn from(value: FW) -> Self {
        value.as_str().to_string()
    }
}

impl fmt::Display for FW {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ActivelySupported for FW {
    /// Frameworks that have first-class templates and inference rules
    const ACTIVELY_SUPPORTED: &'static [Self] = &[
        FW::RustWebAxum,
        FW::TSWebExpress,
        FW::TSWebFEReact,
        FW::TSFullStackNext,
        FW::PYWebFastApi,
        FW::PYWebDjango,
    ];
}

// endregion: Framework
//=====================================================

//=====================================================
// region: Architecture
//=====================================================

/// Architectural patterns supported by the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    Layered,
    MVC,
    Clean,
    None,
}

impl Arch {
    pub fn as_str(self) -> &'static str {
        match self {
            Arch::Layered => "layered",
            Arch::MVC => "mvc",
            Arch::Clean => "clean",
            Arch::None => "none",
        }
    }
}

impl From<Arch> for String {
    fn from(value: Arch) -> Self {
        value.as_str().to_string()
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ActivelySupported for Arch {
    /// Architectures that have concrete templates
    const ACTIVELY_SUPPORTED: &'static [Self] = &[Arch::Layered];
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
    const ACTIVELY_SUPPORTED: &'static [Self];

    /// returns true if the type is actively supported
    fn actively_supported(&self) -> bool {
        Self::ACTIVELY_SUPPORTED.contains(self)
    }
}

pub trait ActivelySupportedExt: ActivelySupported {
    ///lists out actively supported values
    fn active() -> &'static [Self] {
        Self::ACTIVELY_SUPPORTED
    }
}

impl<T: ActivelySupported> ActivelySupportedExt for T {}

// endregion: ActivelySupported===================================================

//=======================================================
// region:Capabilities and Compatibility
//=======================================================

trait LangCapable {
    fn lang_capable(&self, lang: Lang) -> bool;
    fn capable_languages(self) -> Vec<Lang>;
}

#[derive(Debug, PartialEq, Eq)]
struct LangCapableProjects {
    lang: Lang,
    p_types: &'static [PT],
}

const LANG_CAPABILITIES: &[LangCapableProjects] = &[
    // project types rust is capable of doing
    LangCapableProjects {
        lang: Lang::Rust,
        p_types: &[PT::Cli, PT::Api, PT::WebApi, PT::Worker],
    },
    // project types python is capable of doing
    LangCapableProjects {
        lang: Lang::PY,
        p_types: &[PT::Api, PT::Fullstack, PT::WebApi, PT::Worker],
    },
    // project types typescript is capable of doing
    LangCapableProjects {
        lang: Lang::TS,
        p_types: &[PT::Api, PT::Fullstack, PT::WebApi, PT::WebFE, PT::Worker],
    },
];

impl LangCapable for PT {
    ///is it lang_capable: is lang capable of doing this project type
    fn lang_capable(&self, lang: Lang) -> bool {
        if !lang.actively_supported() {
            return false;
        }

        if !self.actively_supported() {
            return false;
        }

        LANG_CAPABILITIES
            .iter()
            .find(|cap| cap.lang == lang)
            .is_some_and(|cap| cap.p_types.contains(self))
    }

    fn capable_languages(self) -> Vec<Lang> {
        LANG_CAPABILITIES
            .iter()
            .filter(|cap| cap.lang.actively_supported())
            .filter(|cap| cap.p_types.contains(&self))
            .map(|cap| cap.lang)
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
impl Compatible for FW {
    type Context = (Lang, PT);

    /// Checks if the framework is compatible with a specific (Lang, PT) pair.
    fn is_compatible(&self, ctx: Self::Context) -> bool {
        match (self, ctx) {
            // Rust
            (FW::RustWebAxum, (Lang::Rust, PT::WebApi)) => true,
            (FW::RustWebActix, (Lang::Rust, PT::WebApi)) => true,

            // TypeScript
            (FW::TSWebExpress, (Lang::TS, PT::WebApi | PT::Api)) => true,
            (FW::TSWebNestJS, (Lang::TS, PT::WebApi | PT::Api)) => true,
            (FW::TSWebFEReact, (Lang::TS, PT::WebFE)) => true,
            (FW::TSWebFEVue, (Lang::TS, PT::WebFE)) => true,
            (FW::TSFullStackNext, (Lang::TS, PT::Fullstack)) => true,

            // Python
            (FW::PYWebFastApi, (Lang::PY, PT::WebApi | PT::Api)) => true,
            (FW::PYWebDjango, (Lang::PY, PT::Fullstack)) => true,

            // None / unmatched
            _ => false,
        }
    }

    /// Returns all contexts compatible with this framework.
    fn get_compatible(&self) -> Option<Vec<Self::Context>> {
        let contexts = match self {
            FW::None => vec![],

            // Rust
            FW::RustWebAxum => vec![(Lang::Rust, PT::WebApi)],
            FW::RustWebActix => vec![(Lang::Rust, PT::WebApi)],

            // TypeScript
            FW::TSWebExpress => vec![(Lang::TS, PT::WebApi), (Lang::TS, PT::Api)],
            FW::TSWebNestJS => vec![(Lang::TS, PT::WebApi), (Lang::TS, PT::Api)],
            FW::TSWebFEReact => vec![(Lang::TS, PT::WebFE)],
            FW::TSWebFEVue => vec![(Lang::TS, PT::WebFE)],
            FW::TSFullStackNext => vec![(Lang::TS, PT::Fullstack)],

            // Python
            FW::PYWebFastApi => vec![(Lang::PY, PT::WebApi), (Lang::PY, PT::Api)],
            FW::PYWebDjango => vec![(Lang::PY, PT::Fullstack)],
        };

        Some(contexts)
    }
}

// =====================================================
// Implementation for Architectures
// =====================================================
impl Compatible for Arch {
    type Context = (Lang, PT, FW);

    /// Checks if the architecture is compatible with a specific (Lang, PT, FW) context.
    fn is_compatible(&self, ctx: Self::Context) -> bool {
        match (self, ctx) {
            // =====================================================
            // Rust
            // =====================================================
            (
                Arch::Layered,
                (Lang::Rust, PT::WebApi, FW::RustWebAxum | FW::RustWebActix)
                | (Lang::Rust, PT::Cli | PT::Worker, _),
            ) => true,

            // =====================================================
            // TypeScript
            // =====================================================
            (
                Arch::Layered,
                (Lang::TS, PT::WebApi | PT::Api, FW::TSWebExpress | FW::TSWebNestJS)
                | (Lang::TS, PT::Fullstack, FW::TSFullStackNext),
            ) => true,

            // =====================================================
            // Python
            // =====================================================
            (Arch::Layered, (Lang::PY, PT::WebApi | PT::Api, FW::PYWebFastApi)) => true,
            (Arch::MVC, (Lang::PY, PT::Fullstack, FW::PYWebDjango)) => true,

            _ => false,
        }
    }

    /// Returns all contexts compatible with this architecture.
    fn get_compatible(&self) -> Option<Vec<Self::Context>> {
        let contexts = match self {
            Arch::Layered => vec![
                // Rust
                (Lang::Rust, PT::WebApi, FW::RustWebAxum),
                (Lang::Rust, PT::WebApi, FW::RustWebActix),
                (Lang::Rust, PT::Cli, FW::None),
                (Lang::Rust, PT::Worker, FW::None),
                // TypeScript
                (Lang::TS, PT::WebApi, FW::TSWebExpress),
                (Lang::TS, PT::WebApi, FW::TSWebNestJS),
                (Lang::TS, PT::Api, FW::TSWebExpress),
                (Lang::TS, PT::Api, FW::TSWebNestJS),
                (Lang::TS, PT::Fullstack, FW::TSFullStackNext),
                // Python
                (Lang::PY, PT::WebApi, FW::PYWebFastApi),
                (Lang::PY, PT::Api, FW::PYWebFastApi),
            ],
            Arch::MVC => vec![(Lang::PY, PT::Fullstack, FW::PYWebDjango)],
            Arch::Clean | Arch::None => vec![],
        };

        Some(contexts)
    }
}

// endregion: Capabilities===================================================

// ==========================================================================
// region: Infer Defaults
// ==========================================================================

trait InferDefault {
    type Context;

    fn infer_default(ctx: Self::Context) -> Option<Self>
    where
        Self: Sized;
}

impl InferDefault for PT {
    type Context = Lang;

    fn infer_default(ctx: Self::Context) -> Option<Self>
    where
        Self: Sized,
    {
        match ctx {
            Lang::Rust => Some(PT::Cli),
            Lang::TS => Some(PT::WebFE),
            Lang::PY => Some(PT::WebApi),
            Lang::None => None,
        }
    }
}

impl InferDefault for FW {
    type Context = (Lang, PT);

    fn infer_default(ctx: Self::Context) -> Option<Self>
    where
        Self: Sized,
    {
        match ctx {
            // ======================
            // Rust
            // ======================
            (Lang::Rust, PT::WebApi) => Some(FW::RustWebAxum),

            // ======================
            // TypeScript
            // ======================
            (Lang::TS, PT::WebApi | PT::Api) => Some(FW::TSWebExpress),
            (Lang::TS, PT::WebFE | PT::None) => Some(FW::TSWebFEReact),
            (Lang::TS, PT::Fullstack) => Some(FW::TSFullStackNext),

            // ======================
            // Python
            // ======================
            (Lang::PY, PT::WebApi | PT::Api) => Some(FW::PYWebFastApi),
            (Lang::PY, PT::Fullstack) => Some(FW::PYWebDjango),

            // ======================
            // Explicitly unsupported (MVP)
            // ======================
            (
                Lang::Rust,
                PT::Cli
                | PT::Api
                | PT::WebFE
                | PT::MobileFE
                | PT::DesktopFE
                | PT::TUI
                | PT::Fullstack
                | PT::Worker
                | PT::None,
            )
            | (Lang::TS, PT::Cli | PT::MobileFE | PT::DesktopFE | PT::TUI | PT::Worker)
            | (
                Lang::PY,
                PT::Cli
                | PT::WebFE
                | PT::MobileFE
                | PT::DesktopFE
                | PT::TUI
                | PT::Worker
                | PT::None,
            )
            | (Lang::None, _) => None,
        }
    }
}

impl InferDefault for Arch {
    type Context = (Lang, PT, FW);

    fn infer_default(ctx: Self::Context) -> Option<Self> {
        match ctx {
            // =====================================================
            // Rust
            // =====================================================

            // Rust Web APIs → Layered is the safest default
            (Lang::Rust, PT::WebApi, FW::RustWebAxum | FW::RustWebActix) => {
                // FUTURE:
                // - Clean Architecture (enterprise services)
                // - Hexagonal (ports/adapters)
                Some(Arch::Layered)
            }

            // Rust CLI tools → Layered
            (Lang::Rust, PT::Cli, _) => {
                // FUTURE:
                // - Single-module for tiny CLIs
                // - Clean for large tools
                Some(Arch::Layered)
            }

            // Rust Workers
            (Lang::Rust, PT::Worker, _) => {
                // FUTURE:
                // - Clean Architecture for long-running workers
                Some(Arch::Layered)
            }

            // Rust Frontend / UI
            (Lang::Rust, PT::WebFE | PT::MobileFE | PT::DesktopFE | PT::TUI, _) => {
                // FUTURE:
                // - MVU-style architectures
                // - Feature-based layouts
                None
            }

            // Rust Fullstack
            (Lang::Rust, PT::Fullstack, _) => {
                // FUTURE:
                // - Clean Architecture
                // - Modular monolith
                None
            }

            // =====================================================
            // TypeScript
            // =====================================================

            // TS Web APIs
            (Lang::TS, PT::WebApi | PT::Api, FW::TSWebExpress | FW::TSWebNestJS) => {
                // NestJS strongly implies layered
                Some(Arch::Layered)
            }

            // TS Web Frontend
            (Lang::TS, PT::WebFE, FW::TSWebFEReact | FW::TSWebFEVue) => {
                // FUTURE:
                // - Feature-based
                // - Atomic design
                None
            }

            // TS Fullstack → Next.js
            (Lang::TS, PT::Fullstack, FW::TSFullStackNext) => {
                // FUTURE:
                // - App Router architecture
                // - Domain-driven modules
                Some(Arch::Layered)
            }

            // TS CLI / Worker
            (Lang::TS, PT::Cli | PT::Worker, _) => {
                // FUTURE:
                // - Clean Architecture for tooling
                None
            }

            // =====================================================
            // Python
            // =====================================================

            // Python Web APIs
            (Lang::PY, PT::WebApi | PT::Api, FW::PYWebFastApi) => {
                // FastAPI commonly uses layered routers/services
                Some(Arch::Layered)
            }

            // Python Fullstack
            (Lang::PY, PT::Fullstack, FW::PYWebDjango) => {
                // Django enforces MVC
                Some(Arch::MVC)
            }

            // Python CLI / Worker
            (Lang::PY, PT::Cli | PT::Worker, _) => {
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

// ==========================================================================
// ==========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ====================================================
    // ActivelySupported tests
    // ====================================================
    #[test]
    fn check_actively_supported_pt() {
        let p_type_list = [PT::Api, PT::Cli, PT::WebApi, PT::Fullstack];
        let expected = [false, true, true, true];

        let actual: Vec<bool> = p_type_list
            .iter()
            .map(super::ActivelySupported::actively_supported)
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn check_actively_supported_lang() {
        let inactive = [Lang::None];
        let active = [Lang::PY, Lang::Rust, Lang::TS];

        for l in inactive {
            assert!(
                !l.actively_supported(),
                "{l:?} should NOT be actively supported"
            );
        }
        for l in active {
            assert!(l.actively_supported(), "{l:?} should be actively supported");
        }
    }

    #[test]
    fn list_active_langs() {
        let active = Lang::active();
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
            (PT::Cli, Lang::Rust, true),
            (PT::Cli, Lang::TS, false),
            (PT::WebApi, Lang::Rust, true),
            (PT::WebFE, Lang::TS, true),
            (PT::WebFE, Lang::PY, false),
        ];

        for (pt, lang, expected) in tests {
            assert_eq!(
                pt.lang_capable(lang),
                expected,
                "Mismatch: {lang:?} capable of {pt:?} should be {expected}"
            );
        }
    }

    #[test]
    fn check_capable_languages_list() {
        let capable = PT::WebApi.capable_languages();
        assert!(capable.contains(&Lang::Rust));
        assert!(capable.contains(&Lang::TS));
        assert!(capable.contains(&Lang::PY));
    }

    // ====================================================
    // InferDefault tests
    // ====================================================
    #[test]
    fn infer_pt_defaults() {
        let inferred = PT::infer_default(Lang::Rust).unwrap();
        assert!(
            inferred.actively_supported(),
            "Inferred PT must be supported"
        );
    }

    #[test]
    fn infer_fw_defaults() {
        let inferred = FW::infer_default((Lang::Rust, PT::WebApi)).unwrap();
        assert_eq!(inferred, FW::RustWebAxum);
    }

    #[test]
    fn infer_arch_defaults() {
        let inferred = Arch::infer_default((Lang::Rust, PT::WebApi, FW::RustWebAxum)).unwrap();
        assert!(matches!(inferred, Arch::Layered | Arch::MVC | Arch::Clean));
    }

    // ====================================================
    // Compatible trait tests
    // ====================================================
    #[test]
    fn fw_compatibility() {
        assert!(FW::RustWebAxum.is_compatible((Lang::Rust, PT::WebApi)));
        assert!(!FW::RustWebAxum.is_compatible((Lang::TS, PT::WebApi)));
    }

    #[test]
    fn arch_compatibility() {
        let arch = Arch::Layered;
        assert!(arch.is_compatible((Lang::Rust, PT::WebApi, FW::RustWebAxum)));
    }

    // ====================================================
    // TargetBuilder end-to-end tests
    // ====================================================
    #[test]
    fn build_complete_target() {
        let target = TargetBuilder::new()
            .with_lang(Lang::Rust)
            .with_p_type(PT::WebApi)
            .with_fw(FW::RustWebAxum)
            .with_arch(Arch::Layered)
            .build()
            .unwrap();

        assert_eq!(target.language, Lang::Rust);
        assert_eq!(target.project_type, PT::WebApi);
        assert_eq!(target.framework, FW::RustWebAxum);
        assert_eq!(target.architecture, Arch::Layered);
    }

    #[test]
    fn build_partial_target_with_inference() {
        let target = TargetBuilder::new()
            .with_lang(Lang::TS)
            .with_p_type(PT::Fullstack)
            .build()
            .unwrap();

        // Should have inferred framework and architecture
        assert_eq!(target.language, Lang::TS);
        assert_eq!(target.project_type, PT::Fullstack);
        assert_eq!(target.framework, FW::TSFullStackNext);
        assert!(matches!(
            target.architecture,
            Arch::Layered | Arch::MVC | Arch::Clean
        ));
    }

    #[test]
    fn build_target_error_on_incompatible_pt() {
        let result = TargetBuilder::new()
            .with_lang(Lang::TS)
            .with_p_type(PT::Cli)
            .with_fw(FW::TSWebFEReact)
            .build();

        println!("{:?}", result.as_ref());

        assert!(matches!(
            result,
            Err(DomainError::FrameworkProjectTypeMismatch { .. })
        ));
    }

    #[test]
    fn build_target_error_on_unsupported_lang() {
        let result = TargetBuilder::new()
            .with_lang(Lang::None)
            .with_p_type(PT::Cli)
            .build();

        println!("{:?}", result.as_ref());

        assert!(matches!(
            result,
            Err(DomainError::UnsupportedLanguage { .. })
        ));
    }

    #[test]
    fn build_target_error_on_inference_failure() {
        let result = TargetBuilder::new().with_lang(Lang::PY).build();

        println!("{:?}", result.as_ref());

        // Python CLI has no framework default → cannot infer
        assert!(
            matches!(result, Err(DomainError::CannotInfer { field, .. }) if field == "project_type" || field == "framework")
        );
    }
}
