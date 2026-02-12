//! Built-in template definitions.
//!
//! Moved from scarff-core to here because these are infrastructure/configuration,
//! not domain logic. They define the actual file contents and structure.

use scarff_core::domain::{
    Architecture, DirectorySpec, FileSpec, Framework, Language, ProjectKind, PythonFramework,
    RustFramework, TargetMatcher, Template, TemplateContent, TemplateId, TemplateMetadata,
    TemplateNode, TemplateSource, TemplateTree, TypeScriptFramework,
};

/// Get all built-in templates.
pub fn all_templates() -> Vec<Template> {
    vec![
        rust_cli_default(),
        rust_cli_layered(),
        rust_backend_axum(),
        python_backend_fastapi(),
        typescript_frontend_react(),
    ]
}

fn rust_cli_default() -> Template {
    Template {
        id: TemplateId::new("rust-cli-default", "1.0.0"),
        matcher: TargetMatcher {
            language: Some(Language::Rust),
            framework: None,
            kind: Some(ProjectKind::Cli),
            architecture: Some(Architecture::Layered),
        },
        metadata: TemplateMetadata::new("Rust CLI (Default)")
            .version("1.0.0")
            .description("A simple Rust command-line application")
            .tags(vec!["rust", "cli", "simple"]),
        tree: TemplateTree::new()
            .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/main.rs",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"fn main() {
    println!("Hello, {{PROJECT_NAME}}!");
}"#,
                )),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "Cargo.toml",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"[package]
name = "{{PROJECT_NAME_KEBAB}}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#,
                )),
            ))),
    }
}

fn rust_cli_layered() -> Template {
    Template {
        id: TemplateId::new("rust-cli-layered", "1.0.0"),
        matcher: TargetMatcher {
            language: Some(Language::Rust),
            framework: None,
            kind: Some(ProjectKind::Cli),
            architecture: Some(Architecture::Layered),
        },
        metadata: TemplateMetadata::new("Rust CLI (Layered)")
            .version("1.0.0")
            .description("A Rust CLI with layered architecture")
            .tags(vec!["rust", "cli", "layered"]),
        tree: TemplateTree::new()
            .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
            .with_node(TemplateNode::Directory(DirectorySpec::new("src/domain")))
            .with_node(TemplateNode::Directory(DirectorySpec::new(
                "src/application",
            )))
            .with_node(TemplateNode::Directory(DirectorySpec::new(
                "src/infrastructure",
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/main.rs",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"mod domain;
mod application;
mod infrastructure;

fn main() {
    println!("Hello, {{PROJECT_NAME}}!");
}"#,
                )),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/domain/mod.rs",
                TemplateContent::Literal(TemplateSource::Static("// Domain layer")),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/application/mod.rs",
                TemplateContent::Literal(TemplateSource::Static("// Application layer")),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/infrastructure/mod.rs",
                TemplateContent::Literal(TemplateSource::Static("// Infrastructure layer")),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "Cargo.toml",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"[package]
name = "{{PROJECT_NAME_KEBAB}}"
version = "0.1.0"
edition = "2024"

[dependencies]
"#,
                )),
            ))),
    }
}

fn rust_backend_axum() -> Template {
    use scarff_core::domain::Framework;

    Template {
        id: TemplateId::new("rust-backend-axum", "1.0.0"),
        matcher: TargetMatcher {
            language: Some(Language::Rust),
            framework: Some(Framework::Rust(RustFramework::Axum)),
            kind: Some(ProjectKind::WebBackend),
            architecture: Some(Architecture::Layered),
        },
        metadata: TemplateMetadata::new("Rust Web Backend (Axum)")
            .version("1.0.0")
            .description("A Rust web API using Axum")
            .tags(vec!["rust", "web", "api", "axum"]),
        tree: TemplateTree::new()
            .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/main.rs",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, {{PROJECT_NAME}}!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}"#,
                )),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "Cargo.toml",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"[package]
name = "{{PROJECT_NAME_KEBAB}}"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
"#,
                )),
            ))),
    }
}

fn python_backend_fastapi() -> Template {
    use scarff_core::domain::Framework;

    Template {
        id: TemplateId::new("python-backend-fastapi", "1.0.0"),
        matcher: TargetMatcher {
            language: Some(Language::Python),
            framework: Some(Framework::Python(PythonFramework::FastApi)),
            kind: Some(ProjectKind::WebBackend),
            architecture: Some(Architecture::Layered),
        },
        metadata: TemplateMetadata::new("Python Backend (FastAPI)")
            .version("1.0.0")
            .description("A Python web API using FastAPI")
            .tags(vec!["python", "web", "api", "fastapi"]),
        tree: TemplateTree::new()
            .with_node(TemplateNode::Directory(DirectorySpec::new("app")))
            .with_node(TemplateNode::File(FileSpec::new(
                "app/main.py",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"from fastapi import FastAPI

app = FastAPI(title="{{PROJECT_NAME}}")

@app.get("/")
async def root():
    return {"message": "Hello {{PROJECT_NAME}}"}
"#,
                )),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "requirements.txt",
                TemplateContent::Literal(TemplateSource::Static("fastapi\nuvicorn")),
            ))),
    }
}

fn typescript_frontend_react() -> Template {
    use scarff_core::domain::Framework;

    Template {
        id: TemplateId::new("typescript-frontend-react", "1.0.0"),
        matcher: TargetMatcher {
            language: Some(Language::TypeScript),
            framework: Some(Framework::TypeScript(TypeScriptFramework::React)),
            kind: Some(ProjectKind::WebFrontend),
            architecture: Some(Architecture::Layered),
        },
        metadata: TemplateMetadata::new("TypeScript Frontend (React)")
            .version("1.0.0")
            .description("A React application with TypeScript")
            .tags(vec!["typescript", "react", "frontend"]),
        tree: TemplateTree::new()
            .with_node(TemplateNode::Directory(DirectorySpec::new("src")))
            .with_node(TemplateNode::File(FileSpec::new(
                "src/App.tsx",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"function App() {
  return (
    <div className="App">
      <h1>Welcome to {{PROJECT_NAME}}</h1>
    </div>
  );
}

export default App;"#,
                )),
            )))
            .with_node(TemplateNode::File(FileSpec::new(
                "package.json",
                TemplateContent::Parameterized(TemplateSource::Static(
                    r#"{
  "name": "{{PROJECT_NAME_KEBAB}}",
  "version": "0.1.0",
  "private": true,
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  }
}"#,
                )),
            ))),
    }
}
