use std::{fmt, str::FromStr};

use super::DomainError;

/// Represents a fully resolved and valid project target.
///
/// A `Target` is guaranteed to be internally consistent:
/// - Language, framework, project type, and architecture are compatible
/// - Defaults are inferred if not explicitly provided
#[derive(Debug, Clone)]
pub struct Target {
    pub language: Language,
    pub project_type: ProjectType,
    pub framework: Option<Framework>,
    pub architecture: Architecture,
}

impl Target {
    /// Create a new `Target`.
    ///
    /// Only `language` is mandatory. All other fields are inferred if missing.
    pub fn new(
        language: Language,
        framework: Option<Framework>,
        architecture: Option<Architecture>,
        project_type: Option<ProjectType>,
    ) -> Result<Self, DomainError> {
        // 1. Validate framework-language compatibility (early)
        if let Some(ref fw) = framework
            && fw.language() != language
        {
            return Err(DomainError::FrameworkLanguageMismatch {
                framework: fw.into(),
                language: language.into(),
            });
        }

        // 2. Infer project type
        let project_type = project_type
            .or_else(|| ProjectType::infer(language, framework))
            .ok_or(DomainError::CannotInfer)?;

        // 3. Infer framework
        let framework = framework.or_else(|| Framework::infer(language, &project_type));

        // 4. Infer architecture
        let architecture = architecture
            .or_else(|| Architecture::infer(framework.as_ref(), &project_type))
            .ok_or(DomainError::CannotInfer)?;

        // 5. Final validation
        Self::validate(&language, &framework, &project_type, &architecture)?;

        Ok(Self {
            language,
            framework,
            project_type,
            architecture,
        })
    }

    fn validate(
        language: &Language,
        framework: &Option<Framework>,
        project_type: &ProjectType,
        architecture: &Architecture,
    ) -> Result<(), DomainError> {
        if let Some(fw) = framework {
            if fw.language() != *language {
                return Err(DomainError::FrameworkLanguageMismatch {
                    language: language.to_string(),
                    framework: fw.into(),
                });
            }

            if !fw.supports(project_type) {
                return Err(DomainError::FrameworkProjectTypeMismatch {
                    framework: fw.into(),
                    project_type: project_type.into(),
                });
            }
        }

        // Architecture rules go here later

        Ok(())
    }
}

// ============================================================
// region: language

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::TypeScript => "typescript",
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
            "python" => Ok(Language::Python),
            "typescript" | "ts" => Ok(Language::TypeScript),
            _ => Err(DomainError::NotSupported),
        }
    }
}

// endregion: language

// ==================================================================
// region: ProjectType

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectType {
    Cli,
    Backend,
    Frontend,
    Fullstack,
    Worker,
}

impl ProjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectType::Cli => "cli",
            ProjectType::Backend => "backend",
            ProjectType::Frontend => "frontend",
            ProjectType::Fullstack => "fullstack",
            ProjectType::Worker => "worker",
        }
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<ProjectType> for String {
    fn from(p: ProjectType) -> Self {
        p.as_str().to_owned()
    }
}

impl From<&ProjectType> for String {
    fn from(p: &ProjectType) -> Self {
        p.as_str().to_owned()
    }
}

impl AsRef<str> for ProjectType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for ProjectType {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "cli" => Ok(ProjectType::Cli),
            "backend" => Ok(ProjectType::Backend),
            "frontend" => Ok(ProjectType::Frontend),
            "fullstack" => Ok(ProjectType::Fullstack),
            "worker" => Ok(ProjectType::Worker),
            _ => Err(DomainError::NotSupported),
        }
    }
}
// endregion: ProjectType

// ==============================================================
// region: framework
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RustFramework {
    Axum,
    Actix,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PythonFramework {
    FastApi,
    Django,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeScriptFramework {
    Express,
    NestJs,
    NextJs,
    React,
    Vue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Framework {
    Rust(RustFramework),
    Python(PythonFramework),
    TypeScript(TypeScriptFramework),
}

impl Framework {
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

impl Framework {
    pub fn language(&self) -> Language {
        match self {
            Framework::Rust(_) => Language::Rust,
            Framework::Python(_) => Language::Python,
            Framework::TypeScript(_) => Language::TypeScript,
        }
    }
}

// endregion: framework

// ==================================================================
// region: Architecture
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Architecture {
    Layered,
    Mvc,
    Modular,
    AppRouter,
}

impl Architecture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Architecture::Layered => "layered",
            Architecture::Mvc => "mvc",
            Architecture::Modular => "modular",
            Architecture::AppRouter => "app-router",
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
            _ => Err(DomainError::NotSupported),
        }
    }
}

// endregion: Architecture

// =====================================================================
// region: infer method
impl ProjectType {
    pub fn infer(language: Language, framework: Option<Framework>) -> Option<Self> {
        match (language, framework) {
            (Language::Rust, _) => Some(ProjectType::Backend),
            (Language::Python, _) => Some(ProjectType::Backend),

            (Language::TypeScript, Some(Framework::TypeScript(TypeScriptFramework::NextJs))) => {
                Some(ProjectType::Fullstack)
            }

            (
                Language::TypeScript,
                Some(Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue)),
            ) => Some(ProjectType::Frontend),

            (Language::TypeScript, Some(_)) => Some(ProjectType::Backend),
            (Language::TypeScript, None) => Some(ProjectType::Frontend),
        }
    }
}

impl Framework {
    pub fn infer(language: Language, project_type: &ProjectType) -> Option<Self> {
        match (language, project_type) {
            (Language::Rust, ProjectType::Backend | ProjectType::Worker) => {
                Some(Framework::Rust(RustFramework::Axum))
            }

            (Language::Python, ProjectType::Backend) => {
                Some(Framework::Python(PythonFramework::FastApi))
            }

            (Language::TypeScript, ProjectType::Frontend) => {
                Some(Framework::TypeScript(TypeScriptFramework::React))
            }

            (Language::TypeScript, ProjectType::Backend) => {
                Some(Framework::TypeScript(TypeScriptFramework::Express))
            }

            (Language::TypeScript, ProjectType::Fullstack) => {
                Some(Framework::TypeScript(TypeScriptFramework::NextJs))
            }

            _ => None,
        }
    }

    pub fn supports(&self, project: &ProjectType) -> bool {
        matches!(
            (self, project),
            (
                Framework::Rust(_),
                ProjectType::Backend | ProjectType::Worker
            ) | (Framework::Python(_), ProjectType::Backend)
                | (
                    Framework::TypeScript(TypeScriptFramework::React | TypeScriptFramework::Vue),
                    ProjectType::Frontend
                )
                | (
                    Framework::TypeScript(
                        TypeScriptFramework::Express | TypeScriptFramework::NestJs
                    ),
                    ProjectType::Backend
                )
                | (
                    Framework::TypeScript(TypeScriptFramework::NextJs),
                    ProjectType::Fullstack
                )
        )
    }
}

impl Architecture {
    pub fn infer(framework: Option<&Framework>, _: &ProjectType) -> Option<Self> {
        match framework {
            Some(Framework::Rust(_)) => Some(Architecture::Layered),
            Some(Framework::Python(PythonFramework::Django)) => Some(Architecture::Mvc),
            Some(Framework::TypeScript(TypeScriptFramework::NestJs)) => Some(Architecture::Modular),
            Some(Framework::TypeScript(TypeScriptFramework::NextJs)) => {
                Some(Architecture::AppRouter)
            }
            Some(_) => Some(Architecture::Layered),
            None => None,
        }
    }
}

// endregion: infer method

// ========================================================
// tests
// ========================================================
#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================
    // Language

    #[test]
    fn parse_language() {
        assert_eq!("rust".parse::<Language>().unwrap(), Language::Rust);
        assert_eq!("Python".parse::<Language>().unwrap(), Language::Python);
        assert_eq!("ts".parse::<Language>().unwrap(), Language::TypeScript);
        assert!("go".parse::<Language>().is_err());
    }

    #[test]
    fn language_display_roundtrip() {
        for lang in [Language::Rust, Language::Python, Language::TypeScript] {
            let s = lang.to_string();
            let parsed: Language = s.parse().unwrap();
            assert_eq!(lang, parsed);
        }
    }

    // ============================================================
    // ProjectType

    #[test]
    fn parse_project_type() {
        assert_eq!(
            "backend".parse::<ProjectType>().unwrap(),
            ProjectType::Backend
        );
        assert_eq!(
            "frontend".parse::<ProjectType>().unwrap(),
            ProjectType::Frontend
        );
        assert!("mobile".parse::<ProjectType>().is_err());
    }

    #[test]
    fn infer_project_type() {
        assert_eq!(
            ProjectType::infer(Language::Rust, None),
            Some(ProjectType::Backend)
        );

        assert_eq!(
            ProjectType::infer(
                Language::TypeScript,
                Some(Framework::TypeScript(TypeScriptFramework::React))
            ),
            Some(ProjectType::Frontend)
        );

        assert_eq!(
            ProjectType::infer(
                Language::TypeScript,
                Some(Framework::TypeScript(TypeScriptFramework::NextJs))
            ),
            Some(ProjectType::Fullstack)
        );
    }

    // ============================================================
    // Framework

    #[test]
    fn framework_language_mapping() {
        let cases = [
            (Framework::Rust(RustFramework::Axum), Language::Rust),
            (
                Framework::Python(PythonFramework::FastApi),
                Language::Python,
            ),
            (
                Framework::TypeScript(TypeScriptFramework::React),
                Language::TypeScript,
            ),
        ];

        for (fw, lang) in cases {
            assert_eq!(fw.language(), lang);
        }
    }

    #[test]
    fn infer_framework_defaults() {
        assert_eq!(
            Framework::infer(Language::Rust, &ProjectType::Backend)
                .unwrap()
                .to_string(),
            "axum"
        );

        assert_eq!(
            Framework::infer(Language::TypeScript, &ProjectType::Frontend)
                .unwrap()
                .to_string(),
            "react"
        );

        assert_eq!(
            Framework::infer(Language::TypeScript, &ProjectType::Fullstack)
                .unwrap()
                .to_string(),
            "nextjs"
        );
    }

    #[test]
    fn framework_supports_project_type() {
        let cases = [
            (
                Framework::Rust(RustFramework::Axum),
                ProjectType::Backend,
                true,
            ),
            (
                Framework::Rust(RustFramework::Axum),
                ProjectType::Frontend,
                false,
            ),
            (
                Framework::TypeScript(TypeScriptFramework::React),
                ProjectType::Frontend,
                true,
            ),
            (
                Framework::TypeScript(TypeScriptFramework::React),
                ProjectType::Backend,
                false,
            ),
            (
                Framework::TypeScript(TypeScriptFramework::NextJs),
                ProjectType::Fullstack,
                true,
            ),
        ];

        for (fw, pt, expected) in cases {
            assert_eq!(fw.supports(&pt), expected);
        }
    }

    // ============================================================
    // Architecture

    #[test]
    fn infer_architecture() {
        assert_eq!(
            Architecture::infer(
                Some(&Framework::Rust(RustFramework::Axum)),
                &ProjectType::Backend
            )
            .unwrap(),
            Architecture::Layered
        );

        assert_eq!(
            Architecture::infer(
                Some(&Framework::Python(PythonFramework::Django)),
                &ProjectType::Backend
            )
            .unwrap(),
            Architecture::Mvc
        );

        assert_eq!(
            Architecture::infer(
                Some(&Framework::TypeScript(TypeScriptFramework::NextJs)),
                &ProjectType::Fullstack
            )
            .unwrap(),
            Architecture::AppRouter
        );
    }

    // ============================================================
    // Target::new (integration tests)

    #[test]
    fn target_with_only_language_is_valid() {
        let target = Target::new(Language::Rust, None, None, None).unwrap();

        assert_eq!(target.language, Language::Rust);
        assert_eq!(target.project_type, ProjectType::Backend);
        assert!(target.framework.is_some());
        assert_eq!(target.architecture, Architecture::Layered);
    }

    #[test]
    fn target_infers_everything_from_language_and_framework() {
        let target = Target::new(
            Language::TypeScript,
            Some(Framework::TypeScript(TypeScriptFramework::NextJs)),
            None,
            None,
        )
        .unwrap();

        assert_eq!(target.project_type, ProjectType::Fullstack);
        assert_eq!(target.architecture, Architecture::AppRouter);
    }

    #[test]
    fn framework_language_mismatch_is_error() {
        let err = Target::new(
            Language::Rust,
            Some(Framework::Python(PythonFramework::FastApi)),
            None,
            None,
        )
        .unwrap_err();

        matches!(err, DomainError::FrameworkLanguageMismatch { .. });
    }

    #[test]
    fn framework_project_type_mismatch_is_error() {
        let err = Target::new(
            Language::TypeScript,
            Some(Framework::TypeScript(TypeScriptFramework::React)),
            None,
            Some(ProjectType::Backend),
        )
        .unwrap_err();

        matches!(err, DomainError::FrameworkProjectTypeMismatch { .. });
    }

    #[test]
    fn explicit_values_override_inference() {
        let target = Target::new(
            Language::TypeScript,
            Some(Framework::TypeScript(TypeScriptFramework::Express)),
            Some(Architecture::Layered),
            Some(ProjectType::Backend),
        )
        .unwrap();

        assert_eq!(target.project_type, ProjectType::Backend);
        assert_eq!(target.architecture, Architecture::Layered);
    }
}

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
