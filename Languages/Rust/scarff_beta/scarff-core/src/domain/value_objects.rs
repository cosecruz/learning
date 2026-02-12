use super::DomainError;
use std::fmt;
use std::str::FromStr;

/// Supported programming languages.
///
/// Value Object: Immutable, equality-based, no identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    Go, // Added for completeness
}

impl Language {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::Go => "go",
        }
    }

    pub const fn file_extension(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Python => "py",
            Self::TypeScript => "ts",
            Self::Go => "go",
        }
    }

    /// Check if this language supports the given project kind natively.
    /// support defined for MVP:
    /// as project grows, more support will be added
    pub fn supports(self, kind: ProjectKind) -> bool {
        matches!(
            (self, kind),
            (
                Self::Rust,
                ProjectKind::Cli | ProjectKind::WebBackend | ProjectKind::Worker
            ) | (
                Self::Python,
                ProjectKind::Cli
                    | ProjectKind::WebBackend
                    | ProjectKind::Worker
                    | ProjectKind::Fullstack
            ) | (
                Self::TypeScript,
                ProjectKind::WebFrontend
                    | ProjectKind::WebBackend
                    | ProjectKind::Fullstack
                    | ProjectKind::Worker
            ) | (
                Self::Go,
                ProjectKind::Cli | ProjectKind::WebBackend | ProjectKind::Worker
            )
        )
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Language {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "rust" | "rs" => Ok(Self::Rust),
            "python" | "py" => Ok(Self::Python),
            "typescript" | "ts" => Ok(Self::TypeScript),
            "go" | "golang" => Ok(Self::Go),
            _ => Err(DomainError::InvalidTarget(format!(
                "Unknown language: {}",
                s
            ))),
        }
    }
}

/// Type of project being scaffolded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectKind {
    Cli,
    WebBackend,
    WebFrontend,
    Fullstack,
    Worker,
    Library, // Added
}

impl ProjectKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::WebBackend => "web-backend",
            Self::WebFrontend => "web-frontend",
            Self::Fullstack => "fullstack",
            Self::Worker => "worker",
            Self::Library => "library",
        }
    }

    /// Check if this project kind requires a framework.
    pub const fn requires_framework(self) -> bool {
        matches!(self, Self::WebBackend | Self::WebFrontend | Self::Fullstack)
    }

    /// Get default project kind for a language.
    pub fn default_for(language: Language) -> Self {
        match language {
            Language::Rust => Self::Cli,
            Language::Python => Self::WebBackend,
            Language::TypeScript => Self::WebFrontend,
            Language::Go => Self::Cli,
        }
    }
}

impl fmt::Display for ProjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ProjectKind {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "cli" => Ok(Self::Cli),
            "web-backend" | "backend" | "api" => Ok(Self::WebBackend),
            "web-frontend" | "frontend" => Ok(Self::WebFrontend),
            "fullstack" => Ok(Self::Fullstack),
            "worker" => Ok(Self::Worker),
            "library" | "lib" => Ok(Self::Library),
            _ => Err(DomainError::InvalidTarget(format!(
                "Unknown project kind: {}",
                s
            ))),
        }
    }
}

/// Framework selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Framework {
    Rust(RustFramework),
    Python(PythonFramework),
    TypeScript(TypeScriptFramework),
    Go(GoFramework),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RustFramework {
    Axum,
    Actix,
    Rocket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PythonFramework {
    FastApi,
    Django,
    Flask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeScriptFramework {
    Express,
    NestJs,
    React,
    Vue,
    NextJs,
    Svelte,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoFramework {
    Gin,
    Echo,
    Stdlib,
}

impl Framework {
    pub fn language(&self) -> Language {
        match self {
            Self::Rust(_) => Language::Rust,
            Self::Python(_) => Language::Python,
            Self::TypeScript(_) => Language::TypeScript,
            Self::Go(_) => Language::Go,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rust(RustFramework::Axum) => "axum",
            Self::Rust(RustFramework::Actix) => "actix",
            Self::Rust(RustFramework::Rocket) => "rocket",
            Self::Python(PythonFramework::FastApi) => "fastapi",
            Self::Python(PythonFramework::Django) => "django",
            Self::Python(PythonFramework::Flask) => "flask",
            Self::TypeScript(TypeScriptFramework::Express) => "express",
            Self::TypeScript(TypeScriptFramework::NestJs) => "nestjs",
            Self::TypeScript(TypeScriptFramework::React) => "react",
            Self::TypeScript(TypeScriptFramework::Vue) => "vue",
            Self::TypeScript(TypeScriptFramework::NextJs) => "nextjs",
            Self::TypeScript(TypeScriptFramework::Svelte) => "svelte",
            Self::Go(GoFramework::Gin) => "gin",
            Self::Go(GoFramework::Echo) => "echo",
            Self::Go(GoFramework::Stdlib) => "stdlib",
        }
    }

    /// Check compatibility with language and project kind.
    pub fn is_compatible_with(self, language: Language, kind: ProjectKind) -> bool {
        if self.language() != language {
            return false;
        }

        matches!(
            (self, kind),
            // Rust frameworks
            (Self::Rust(RustFramework::Axum | RustFramework::Actix | RustFramework::Rocket), ProjectKind::WebBackend) |
            (Self::Rust(RustFramework::Rocket), ProjectKind::Fullstack) |

            // Python frameworks
            (Self::Python(PythonFramework::FastApi | PythonFramework::Flask), ProjectKind::WebBackend | ProjectKind::Worker) |
            (Self::Python(PythonFramework::Django), ProjectKind::Fullstack | ProjectKind::WebBackend) |

            // TypeScript frameworks
            (Self::TypeScript(TypeScriptFramework::Express | TypeScriptFramework::NestJs), ProjectKind::WebBackend | ProjectKind::Worker) |
            (Self::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue | TypeScriptFramework::Svelte), ProjectKind::WebFrontend) |
            (Self::TypeScript(TypeScriptFramework::NextJs | TypeScriptFramework::Svelte), ProjectKind::Fullstack) |

            // Go frameworks
            (Self::Go(GoFramework::Gin | GoFramework::Echo | GoFramework::Stdlib), ProjectKind::WebBackend | ProjectKind::Worker | ProjectKind::Cli)
        )
    }

    /// Infer framework from language and kind.
    pub fn infer(language: Language, kind: ProjectKind) -> Option<Self> {
        match (language, kind) {
            (Language::Rust, ProjectKind::WebBackend) => Some(Self::Rust(RustFramework::Axum)),
            (Language::TypeScript, ProjectKind::WebBackend) => {
                Some(Self::TypeScript(TypeScriptFramework::Express))
            }
            (Language::TypeScript, ProjectKind::WebFrontend) => {
                Some(Self::TypeScript(TypeScriptFramework::React))
            }
            (Language::TypeScript, ProjectKind::Fullstack) => {
                Some(Self::TypeScript(TypeScriptFramework::NextJs))
            }
            (Language::Python, ProjectKind::WebBackend) => {
                Some(Self::Python(PythonFramework::FastApi))
            }
            (Language::Python, ProjectKind::Fullstack) => {
                Some(Self::Python(PythonFramework::Django))
            }
            (Language::Go, ProjectKind::WebBackend) => Some(Self::Go(GoFramework::Gin)),
            _ => None,
        }
    }
}

impl fmt::Display for Framework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Architectural patterns supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Architecture {
    Layered,
    Mvc,
    Clean, // Hexagonal / Clean / Onion (all similar)
    FeatureModular,
}

impl Architecture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Layered => "layered",
            Self::Mvc => "mvc",
            Self::Clean => "clean",
            Self::FeatureModular => "feature-modular",
        }
    }

    /// Check if this architecture is compatible with the given constraints.
    pub fn is_compatible_with(
        self,
        _language: Language,
        kind: ProjectKind,
        framework: Option<Framework>,
    ) -> bool {
        match (self, kind, framework) {
            // MVC only works with Django fullstack
            (
                Self::Mvc,
                ProjectKind::Fullstack,
                Some(Framework::Python(PythonFramework::Django)),
            ) => true,
            (Self::Mvc, _, _) => false,

            // Clean architecture works with everything except when MVC is required
            (Self::Clean, _, _) => true,

            // Layered works with everything
            (Self::Layered, _, _) => true,

            // Feature modular works with larger projects
            (
                Self::FeatureModular,
                ProjectKind::WebBackend | ProjectKind::Fullstack | ProjectKind::WebFrontend,
                _,
            ) => true,
            (Self::FeatureModular, _, _) => false,
        }
    }

    /// Infer architecture from constraints.
    pub fn infer(language: Language, kind: ProjectKind, framework: Option<Framework>) -> Self {
        match (language, kind, framework) {
            // Django fullstack -> MVC
            (_, ProjectKind::Fullstack, Some(Framework::Python(PythonFramework::Django))) => {
                Self::Mvc
            }

            // Large TypeScript projects -> Feature Modular
            (Language::TypeScript, ProjectKind::WebBackend | ProjectKind::Fullstack, _) => {
                Self::FeatureModular
            }

            // Everything else -> Layered (safest default)
            _ => Self::Layered,
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Extended architecture patterns for advanced usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchitecturePattern {
    Layered,
    Hexagonal, // Explicit hexagonal/ports-and-adapters
    Onion,     // Concentric circles
    Clean,     // Uncle Bob's Clean Architecture
    FeatureModular,
    Microkernel,
}

impl ArchitecturePattern {
    /// Convert to basic Architecture enum.
    pub fn to_architecture(self) -> Architecture {
        match self {
            Self::Layered => Architecture::Layered,
            Self::Hexagonal | Self::Onion | Self::Clean => Architecture::Clean,
            Self::FeatureModular => Architecture::FeatureModular,
            Self::Microkernel => Architecture::Layered, // Maps to layered for simplicity
        }
    }

    /// Get default directory structure for this pattern.
    pub fn default_structure(&self) -> Vec<&'static str> {
        match self {
            Self::Layered => vec![
                "src/presentation",
                "src/application",
                "src/domain",
                "src/infrastructure",
            ],
            Self::Hexagonal => vec![
                "src/domain",
                "src/application/ports",
                "src/application/services",
                "src/adapters/in",
                "src/adapters/out",
                "src/configuration",
            ],
            Self::Onion => vec![
                "src/core/domain",
                "src/core/use_cases",
                "src/interfaces",
                "src/infrastructure",
            ],
            Self::Clean => vec![
                "src/entities",
                "src/use_cases",
                "src/interface_adapters",
                "src/frameworks",
            ],
            Self::FeatureModular => vec!["src/features", "src/shared"],
            Self::Microkernel => vec!["src/core", "src/plugins", "src/platform"],
        }
    }
}
