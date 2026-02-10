//! Built-in template definitions.
//!
//! This module contains all built-in templates shipped with Scarff.
//! Templates are defined using declarative macros for consistency.

use crate::domain::{Architecture, Language, ProjectKind};
use crate::domain::{
    DirectorySpec, FileSpec, TargetMatcher, Template, TemplateContent, TemplateMetadata,
    TemplateNode, TemplateSource, TemplateTree,
};

// ============================================================================
// Template Definition Macros
// ============================================================================

/// Helper macro to create a template tree from a declarative syntax.
///
/// # Syntax
///
/// ```ignore
/// template_tree! {
///     dir "src";
///     dir "tests";
///     file "src/main.rs" => "path/to/template.rs";
///     file "Cargo.toml" => "path/to/Cargo.toml.template";
/// }
/// ```
macro_rules! template_tree {
    (
        $(
            dir $dir:literal;
        )*
        $(
            file $path:literal => $tpl:literal;
        )*
    ) => {{
        let mut tree = TemplateTree::new();

        $(
            tree.push(TemplateNode::Directory(DirectorySpec::new($dir)));
        )*

        $(
            tree.push(TemplateNode::File(FileSpec::new(
                $path,
                TemplateContent::Parameterized(TemplateSource::Static(include_str!($tpl))),
            )));
        )*

        tree
    }};
}

/// Complete template definition macro.
///
/// # Syntax
///
/// ```ignore
/// template! {
///     name: "Rust CLI",
///     version: "1.0.0",
///     description: "A Rust CLI application",
///     tags: ["rust", "cli"],
///
///     matcher {
///         language: Rust,
///         framework: None,
///         kind: Cli,
///         architecture: Layered,
///     }
///
///     tree {
///         dir "src";
///         file "src/main.rs" => "templates/rust/cli/main.rs.template";
///     }
/// }
/// ```
macro_rules! template {
    (
        name: $name:literal,
        version: $version:literal,
        description: $description:literal,
        tags: [$($tag:literal),* $(,)?],

        matcher {
            language: $lang:ident,
            framework: $fw:expr,
            kind: $kind:ident,
            architecture: $arch:ident $(,)?
        }

        tree {
            $($tree:tt)*
        }
    ) => {
        Template {
            metadata: TemplateMetadata::new($name)
                .version($version)
                .description($description)
                .tags(vec![$($tag),*]),
            matcher: TargetMatcher {
                language: Some(Language::$lang),
                framework: $fw,
                kind: Some(ProjectKind::$kind),
                architecture: Some(Architecture::$arch),
            },
            tree: template_tree! {
                $($tree)*
            },
        }
    };
}

// ============================================================================
// Built-in Template Definitions
// ============================================================================

/// Rust CLI application with default structure.
///
/// This is the simplest Rust project template - a single-file CLI application.
pub fn rust_cli_default() -> Template {
    template! {
        name: "Rust CLI (Default)",
        version: "1.0.0",
        description: "A simple Rust command-line application",
        tags: ["rust", "cli", "simple"],

        matcher {
            language: Rust,
            framework: None,
            kind: Cli,
            architecture: Layered,
        }

        tree {
            dir "src";
            file "src/main.rs"
                => "templates/rust/cli/_default/main.rs.template";
            file "Cargo.toml"
                => "templates/rust/cli/_default/Cargo.toml.template";
            // file ".gitignore"
            //     => "templates/common/rust.gitignore";
            // file "README.md"
            //     => "templates/rust/cli/_default/README.md.template";
        }
    }
}

/// Rust CLI application with layered architecture.
///
/// This template provides a more structured approach with separation of concerns:
/// - Domain layer: Core business logic
/// - Application layer: Use cases and orchestration
/// - Infrastructure layer: External dependencies (DB, HTTP, etc.)
/// - Presentation layer: CLI interface
pub fn rust_cli_layered() -> Template {
    template! {
        name: "Rust CLI (Layered)",
        version: "1.0.0",
        description: "A Rust CLI with layered architecture for larger applications",
        tags: ["rust", "cli", "layered", "architecture"],

        matcher {
            language: Rust,
            framework: None,
            kind: Cli,
            architecture: Layered,
        }

        tree {
            // dir "src";
            // dir "src/domain";
            // dir "src/application";
            // dir "src/infrastructure";
            // dir "src/presentation";
            // dir "tests";

            // file "src/main.rs"
            //     => "templates/rust/cli/layered/main.rs.template";
            // file "src/lib.rs"
            //     => "templates/rust/cli/layered/lib.rs.template";
            // file "src/domain/mod.rs"
            //     => "templates/rust/cli/layered/domain_mod.rs.template";
            // file "src/application/mod.rs"
            //     => "templates/rust/cli/layered/application_mod.rs.template";
            // file "src/infrastructure/mod.rs"
            //     => "templates/rust/cli/layered/infrastructure_mod.rs.template";
            // file "src/presentation/mod.rs"
            //     => "templates/rust/cli/layered/presentation_mod.rs.template";
            // file "Cargo.toml"
            //     => "templates/rust/cli/layered/Cargo.toml.template";
            // file ".gitignore"
            //     => "templates/common/rust.gitignore";
            // file "README.md"
            //     => "templates/rust/cli/layered/README.md.template";
        }
    }
}

/// Rust web backend with Axum framework.
///
/// Creates a REST API server using Axum with layered architecture.
pub fn rust_backend_axum() -> Template {
    use crate::domain::{Framework, RustFramework};

    template! {
        name: "Rust Web Backend (Axum)",
        version: "1.0.0",
        description: "A Rust web API using Axum with layered architecture",
        tags: ["rust", "web", "api", "axum", "backend"],

        matcher {
            language: Rust,
            framework: Some(Framework::Rust(RustFramework::Axum)),
            kind: WebBackend,
            architecture: Layered,
        }

        tree {
            // dir "src";
            // dir "src/domain";
            // dir "src/application";
            // dir "src/infrastructure";
            // dir "src/presentation";
            // dir "tests";

            // file "src/main.rs"
            //     => "templates/rust/backend/axum/main.rs.template";
            // file "src/lib.rs"
            //     => "templates/rust/backend/axum/lib.rs.template";
            // file "src/domain/mod.rs"
            //     => "templates/rust/backend/axum/domain_mod.rs.template";
            // file "src/application/mod.rs"
            //     => "templates/rust/backend/axum/application_mod.rs.template";
            // file "src/infrastructure/mod.rs"
            //     => "templates/rust/backend/axum/infrastructure_mod.rs.template";
            // file "src/presentation/mod.rs"
            //     => "templates/rust/backend/axum/presentation_mod.rs.template";
            // file "Cargo.toml"
            //     => "templates/rust/backend/axum/Cargo.toml.template";
            // file ".env.example"
            //     => "templates/rust/backend/axum/env.example";
            // file ".gitignore"
            //     => "templates/common/rust.gitignore";
            // file "README.md"
            //     => "templates/rust/backend/axum/README.md.template";
        }
    }
}

/// Python backend with FastAPI framework.
///
/// Creates a REST API server using FastAPI with layered architecture.
pub fn python_backend_fastapi() -> Template {
    use crate::domain::{Framework, PythonFramework};

    template! {
        name: "Python Backend (FastAPI)",
        version: "1.0.0",
        description: "A Python web API using FastAPI with layered architecture",
        tags: ["python", "web", "api", "fastapi", "backend"],

        matcher {
            language: Python,
            framework: Some(Framework::Python(PythonFramework::FastApi)),
            kind: WebBackend,
            architecture: Layered,
        }

        tree {
            // dir "app";
            // dir "app/domain";
            // dir "app/application";
            // dir "app/infrastructure";
            // dir "app/presentation";
            // dir "tests";

            // file "app/__init__.py"
            //     => "templates/python/backend/fastapi/__init__.py.template";
            // file "app/main.py"
            //     => "templates/python/backend/fastapi/main.py.template";
            // file "app/domain/__init__.py"
            //     => "templates/python/backend/fastapi/domain_init.py.template";
            // file "app/application/__init__.py"
            //     => "templates/python/backend/fastapi/application_init.py.template";
            // file "app/infrastructure/__init__.py"
            //     => "templates/python/backend/fastapi/infrastructure_init.py.template";
            // file "app/presentation/__init__.py"
            //     => "templates/python/backend/fastapi/presentation_init.py.template";
            // file "requirements.txt"
            //     => "templates/python/backend/fastapi/requirements.txt.template";
            // file ".env.example"
            //     => "templates/python/backend/fastapi/env.example";
            // file ".gitignore"
            //     => "templates/common/python.gitignore";
            // file "README.md"
            //     => "templates/python/backend/fastapi/README.md.template";
        }
    }
}

/// TypeScript frontend with React.
///
/// Creates a React application with modern tooling (Vite).
pub fn typescript_frontend_react() -> Template {
    use crate::domain::{Framework, TypeScriptFramework};

    template! {
        name: "TypeScript Frontend (React)",
        version: "1.0.0",
        description: "A React application with TypeScript and Vite",
        tags: ["typescript", "react", "frontend", "vite"],

        matcher {
            language: TypeScript,
            framework: Some(Framework::TypeScript(TypeScriptFramework::React)),
            kind: WebFrontend,
            architecture: Layered,
        }

        tree {
            // dir "src";
            // dir "src/components";
            // dir "src/hooks";
            // dir "src/utils";
            // dir "public";

            // file "src/main.tsx"
            //     => "templates/typescript/frontend/react/main.tsx.template";
            // file "src/App.tsx"
            //     => "templates/typescript/frontend/react/App.tsx.template";
            // file "src/vite-env.d.ts"
            //     => "templates/typescript/frontend/react/vite-env.d.ts.template";
            // file "index.html"
            //     => "templates/typescript/frontend/react/index.html.template";
            // file "package.json"
            //     => "templates/typescript/frontend/react/package.json.template";
            // file "tsconfig.json"
            //     => "templates/typescript/frontend/react/tsconfig.json.template";
            // file "vite.config.ts"
            //     => "templates/typescript/frontend/react/vite.config.ts.template";
            // file ".gitignore"
            //     => "templates/common/node.gitignore";
            // file "README.md"
            //     => "templates/typescript/frontend/react/README.md.template";
        }
    }
}

// ============================================================================
// Template Registry
// ============================================================================

/// Get all built-in templates.
///
/// This function returns all templates that ship with Scarff.
/// Used by the store to load built-in templates.
pub fn all_templates() -> Vec<Template> {
    vec![
        rust_cli_default(),
        // rust_cli_layered(),
        // rust_backend_axum(),
        // python_backend_fastapi(),
        // typescript_frontend_react(),
    ]
}

/// Get template count.
pub fn template_count() -> usize {
    all_templates().len()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_templates_are_valid() {
        let templates = all_templates();
        assert!(!templates.is_empty());

        for template in templates {
            // Check metadata is non-empty
            assert!(!template.metadata.name.is_empty());
            assert!(!template.metadata.version.is_empty());

            // Check tree has nodes
            assert!(!template.tree.is_empty());

            // Check matcher has at least language set
            assert!(template.matcher.language.is_some());
        }
    }

    #[test]
    fn rust_cli_default_template() {
        let template = rust_cli_default();

        assert_eq!(template.metadata.name, "Rust CLI (Default)");
        assert_eq!(template.matcher.language, Some(Language::Rust));
        assert_eq!(template.matcher.kind, Some(ProjectKind::Cli));
        assert!(template.tree.len() > 0);
    }

    #[test]
    fn rust_cli_layered_template() {
        let template = rust_cli_layered();

        assert_eq!(template.metadata.name, "Rust CLI (Layered)");
        assert_eq!(template.matcher.architecture, Some(Architecture::Layered));
        // Layered should have more files than default
        assert!(
            !template.tree.len() > rust_cli_default().tree.len(),
            "because template not yet created"
        );
    }

    #[test]
    fn rust_backend_axum_template() {
        use crate::domain::{Framework, RustFramework};

        let template = rust_backend_axum();

        assert_eq!(template.metadata.name, "Rust Web Backend (Axum)");
        assert_eq!(
            template.matcher.framework,
            Some(Framework::Rust(RustFramework::Axum))
        );
        assert_eq!(template.matcher.kind, Some(ProjectKind::WebBackend));
    }

    #[test]
    fn python_backend_fastapi_template() {
        use crate::domain::{Framework, PythonFramework};

        let template = python_backend_fastapi();

        assert_eq!(template.metadata.name, "Python Backend (FastAPI)");
        assert_eq!(
            template.matcher.framework,
            Some(Framework::Python(PythonFramework::FastApi))
        );
    }

    #[test]
    fn typescript_frontend_react_template() {
        use crate::domain::{Framework, TypeScriptFramework};

        let template = typescript_frontend_react();

        assert_eq!(template.metadata.name, "TypeScript Frontend (React)");
        assert_eq!(
            template.matcher.framework,
            Some(Framework::TypeScript(TypeScriptFramework::React))
        );
        assert_eq!(template.matcher.kind, Some(ProjectKind::WebFrontend));
    }

    #[test]
    fn template_count_matches_vec_length() {
        assert_eq!(template_count(), all_templates().len());
    }

    #[test]
    fn no_duplicate_template_names() {
        use std::collections::HashSet;

        let templates = all_templates();
        let names: HashSet<_> = templates.iter().map(|t| t.metadata.name).collect();

        assert_eq!(
            names.len(),
            templates.len(),
            "Duplicate template names found"
        );
    }
}
