use std::{fmt, str::FromStr};

use super::DomainError;

// target
#[derive(Debug, Clone)]
pub struct Target {
    ///Language e.g: Rust, Python
    lang: Language,

    ///Type of Project: backend: cli, api, web-api, worker,  frontend, devops and infra, scripting
    project_type: ProjectType,

    // TODO: Best design choices;
    /// Framework: can be Some() or None. None represents using default
    /// E.g Axum, Node, Express, Actix, FastApi, React, Vue, NextJs, Nest.js, Django,
    /// because typescript an be used fro both front and backend it becomes a bit cumbersome so needs to be designed well
    framework: Option<Framework>,

    // TODO: some frameworks are very opinionated and may not support some architectures
    ///Architecture: can be SOme() or None. layered, hexagonal, cqrs, event, blah blah blah;
    arch: Architecture,
    // TODO: some project types will force user to design in some certain way

    // allow only language as compulsory the rest optional
    // design defaults if nothing is passed form interface
    // a lot of type state compile time enforcement
    // enum + metadata
    // scoped options
}

impl Target {
    /// public facing method to create a Target Object
    pub fn new(
        language: Language,
        framework: Option<Framework>,
        architecture: Option<Architecture>,
        project_type: Option<ProjectType>,
    ) -> Result<Self, DomainError> {
        // validate
        // create defaults as per design for some with options
        // return object or error
        // now on the cli interface depending on the error we can tell client its not compatible and offer better compatible options
        todo!()
    }

    //
    fn validate() {
        // validate language
        // validate framework: against language support and project_type
        // validate architecture: against framework and project_type
    }
}

// ============================================================
// region: language

#[derive(Debug, Clone)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
}

impl FromStr for Language {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "rust" => Ok(Language::Rust),
            "python" => Ok(Language::Python),
            "typescript" | "ts" => Ok(Language::TypeScript),
            _ => Err(DomainError::LanguageNotSupported),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::TypeScript => "typescript",
        };

        f.write_str(s)
    }
}
// endregion: language

// ==================================================================
// region: ProjectType

#[derive(Debug, Clone)]
pub enum ProjectType {
    Cli,
    Backend,
    Frontend,
    Fullstack,
    Worker,
}
// endregion: ProjectType

impl ProjectType {
    ///infer default id project_type is None in Target::new or Target builder
    /// project_type is mostly inferred from language and framework
    /// ProjectType INfer Table
    /// Language.............|Framework........| Result........
    /// Rust                 |None             | Backend
    /// Rust                 |Axum|Actix|Rocket| Backend
    /// Python...............|None|Django|FastApi| Backend
    /// TypeScript...........|None             | Frontend?
    /// Typescrpt            |Node....
    pub fn infer_default(language: Language, framework: Option<Framework>) -> Self {
        todo!()
    }
}

// ==============================================================
// region: framework
// TODO: Designing framework
// frameworks are decided by the language and maybe type  of project

#[derive(Debug, Clone)]
pub enum Framework {
    // Rust
    Axum,
    Actix,

    // Python
    FastApi,
    Django,

    // TypeScript
    Express,
    NestJs,
    NextJs,
    React,
    Vue,
}

impl Framework {
    pub fn language_support(&self) -> Language {
        match self {
            // Rust supported frameworks
            Framework::Actix | Framework::Axum => Language::Rust,
            // Python supported frameworks
            Framework::Django | Framework::FastApi => Language::Python,
            // Typescript supported frameworks
            Framework::Express
            | Framework::NestJs
            | Framework::NextJs
            | Framework::React
            | Framework::Vue => Language::TypeScript,
        }
    }

    pub fn supports(&self, project: &ProjectType) -> bool {
        matches!(
            (self, project),
            //Backend project support
            (Framework::Axum, ProjectType::Backend)
                | (Framework::Actix, ProjectType::Backend)
                | (Framework::FastApi, ProjectType::Backend)
                | (Framework::Django, ProjectType::Backend)
                | (Framework::Express, ProjectType::Backend)
                | (Framework::NestJs, ProjectType::Backend)

                // Fullstack project support
                | (
                    Framework::NextJs,
                    ProjectType::Frontend | ProjectType::Fullstack
                )

                // frontend project support
                | (Framework::React, ProjectType::Frontend)
                | (Framework::Vue, ProjectType::Frontend)
        )
    }
}

// endregion: framework

// ==================================================================
// region: Architecture

#[derive(Debug, Clone)]
pub enum Architecture {}
// endregion: Architecture

// region
// endregion

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
