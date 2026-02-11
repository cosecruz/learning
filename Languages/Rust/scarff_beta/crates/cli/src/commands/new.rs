//! Implementation of the `scarff new` command.
//!
//! This module handles creating new projects from templates.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use scarff_core::{
    Architecture as CoreArchitecture, Engine, Framework as CoreFramework, Language as CoreLanguage,
    ProjectKind as CoreProjectKind, PythonFramework, RustFramework, Target, TypeScriptFramework,
};

use crate::{
    args::{Architecture, Language, NewCommand, ProjectKind},
    error::{CliError, CliResul, IntoCli},
    output,
};

/// Execute the `new` command.
///
/// # Arguments
///
/// * `cmd` - Parsed command arguments
/// * `verbose` - Whether to show verbose output
/// * `quiet` - Whether to suppress non-error output
pub fn execute(cmd: NewCommand, verbose: bool, quiet: bool) -> CliResul<()> {
    debug!("Executing new command with: {:#?}", cmd);

    // 1. Resolve project path
    let (project_name, output_dir) = resolve_project_path(&cmd.name, cmd.output.as_deref())?;
    validate_project_name(&project_name)?;

    info!("Project name: {}", project_name);
    info!("Output directory: {}", output_dir.display());

    // 2. Build target configuration
    let target = build_target(&cmd).context("Failed to build target configuration")?;

    debug!("Built target: {}", target);

    // 3. Show configuration and get confirmation (unless --yes or --quiet)
    if !cmd.yes && !quiet {
        output::show_configuration(&target, &project_name, &output_dir)?;
        output::confirm()?;
    }

    // 4. Check if project already exists
    let project_path = output_dir.join(&project_name);
    if project_path.exists() && !cmd.force {
        return Err(CliError::ProjectExists {
            path: project_path.display().to_string(),
        }
        .into());
    }

    // 5. Handle dry run
    if cmd.dry_run {
        output::show_dry_run(&target, &project_name, &output_dir)?;
        return Ok(());
    }

    // 6. Create engine and scaffold
    let engine = Engine::new();

    if !quiet {
        output::show_progress("Scaffolding project", || {
            Ok(engine.scaffold(target, &project_name, &output_dir)?)
        })?
    } else {
        engine
            .scaffold(target, &project_name, &output_dir)
            .into_cli()?;
    }

    // 7. Show success message
    if !quiet {
        output::show_success(&project_name, &project_path, verbose)?;
    }

    Ok(())
}

/// Resolve the project path into (name, output_directory).
///
/// # Examples
///
/// - "my-project" → ("my-project", "./")
/// - "../my-project" → ("my-project", "../")
/// - "/tmp/my-project" → ("my-project", "/tmp/")
/// - "my-project" with output="/tmp" → ("my-project", "/tmp/")
fn resolve_project_path(
    name_or_path: &str,
    output_override: Option<&Path>,
) -> Result<(String, PathBuf)> {
    let path = PathBuf::from(name_or_path);

    // If output is explicitly provided, use it
    if let Some(output) = output_override {
        let project_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| CliError::InvalidProjectName {
                reason: "Could not extract project name from path".to_string(),
            })?
            .to_string();

        return Ok((project_name, output.to_path_buf()));
    }

    // Otherwise, split the path
    if path.parent().is_some() && path.parent() != Some(Path::new("")) {
        // Has a parent directory component
        let project_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| CliError::InvalidProjectName {
                reason: "Invalid path format".to_string(),
            })?
            .to_string();

        let output_dir = path.parent().unwrap().to_path_buf();

        Ok((project_name, output_dir))
    } else {
        // Just a name, no path component
        Ok((name_or_path.to_string(), PathBuf::from(".")))
    }
}

/// Validate that the project name is acceptable.
///
/// Rules:
/// - Not empty
/// - Doesn't contain path separators
/// - Doesn't start with . (hidden files)
/// - Only contains safe characters
fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(CliError::InvalidProjectName {
            reason: "Project name cannot be empty".to_string(),
        }
        .into());
    }

    if name.starts_with('.') {
        return Err(CliError::InvalidProjectName {
            reason: "Project name cannot start with '.' (hidden directory)".to_string(),
        }
        .into());
    }

    if name.contains('/') || name.contains('\\') {
        return Err(CliError::InvalidProjectName {
            reason: "Project name cannot contain path separators. Use --output for the directory."
                .to_string(),
        }
        .into());
    }

    // Check for invalid characters (very permissive, just basic safety)
    let invalid_chars = ['<', '>', ':', '"', '|', '?', '*', '\0'];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        return Err(CliError::InvalidProjectName {
            reason: format!(
                "Project name contains invalid characters: {}",
                invalid_chars
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
        .into());
    }

    Ok(())
}

/// Build a Target from command arguments.
fn build_target(cmd: &NewCommand) -> CliResul<Target> {
    // Convert CLI enums to core enums
    let language = convert_language(cmd.language);
    let kind = convert_kind(cmd.kind);
    let architecture = convert_architecture(cmd.architecture);

    // Start building target
    let mut builder = Target::builder()
        .language(language)
        .kind(kind)?
        .architecture(architecture)?;

    // Add framework if provided
    if let Some(ref framework_str) = cmd.framework {
        let framework = parse_framework(cmd.language, framework_str)?;
        builder = builder.framework(framework)?;
    }

    // Build and validate
    Ok(builder.build()?)
}

/// Convert CLI Language to core Language.
fn convert_language(lang: Language) -> CoreLanguage {
    match lang {
        Language::Rust => CoreLanguage::Rust,
        Language::Python => CoreLanguage::Python,
        Language::TypeScript => CoreLanguage::TypeScript,
    }
}

/// Convert CLI ProjectKind to core ProjectKind.
fn convert_kind(pt: ProjectKind) -> CoreProjectKind {
    match pt {
        ProjectKind::Cli => CoreProjectKind::Cli,
        ProjectKind::WebApi => CoreProjectKind::WebBackend,
        ProjectKind::WebFrontend => CoreProjectKind::WebFrontend,
        ProjectKind::Fullstack => CoreProjectKind::Fullstack,
        ProjectKind::Worker => CoreProjectKind::Worker,
    }
}

/// Convert CLI Architecture to core Architecture.
fn convert_architecture(arch: Architecture) -> CoreArchitecture {
    match arch {
        Architecture::Layered => CoreArchitecture::Layered,
        Architecture::Mvc => CoreArchitecture::MVC,
        Architecture::Clean => CoreArchitecture::Clean,
        Architecture::Modular => todo!(),
        Architecture::AppRouter => todo!(),
    }
}

/// Parse framework string into core Framework enum.
fn parse_framework(language: Language, framework: &str) -> Result<CoreFramework, CliError> {
    let framework_lower = framework.to_lowercase();

    match language {
        Language::Rust => match framework_lower.as_str() {
            "axum" => Ok(CoreFramework::Rust(RustFramework::Axum)),
            "actix" => Ok(CoreFramework::Rust(RustFramework::Actix)),
            _ => Err(CliError::FrameworkNotAvailable {
                framework: framework.to_string(),
                language: language.to_string(),
            }),
        },
        Language::Python => match framework_lower.as_str() {
            "fastapi" => Ok(CoreFramework::Python(PythonFramework::FastApi)),
            "django" => Ok(CoreFramework::Python(PythonFramework::Django)),
            _ => Err(CliError::FrameworkNotAvailable {
                framework: framework.to_string(),
                language: language.to_string(),
            }),
        },
        Language::TypeScript => match framework_lower.as_str() {
            "express" => Ok(CoreFramework::TypeScript(TypeScriptFramework::Express)),
            "nestjs" => Ok(CoreFramework::TypeScript(TypeScriptFramework::NestJs)),
            "nextjs" => Ok(CoreFramework::TypeScript(TypeScriptFramework::NextJs)),
            "react" => Ok(CoreFramework::TypeScript(TypeScriptFramework::React)),
            "vue" => Ok(CoreFramework::TypeScript(TypeScriptFramework::Vue)),
            _ => Err(CliError::FrameworkNotAvailable {
                framework: framework.to_string(),
                language: language.to_string(),
            }),
        },
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_simple_name() {
        let (name, dir) = resolve_project_path("my-project", None).unwrap();
        assert_eq!(name, "my-project");
        assert_eq!(dir, PathBuf::from("."));
    }

    #[test]
    fn resolve_relative_path() {
        let (name, dir) = resolve_project_path("../my-project", None).unwrap();
        assert_eq!(name, "my-project");
        assert_eq!(dir, PathBuf::from(".."));
    }

    #[test]
    fn resolve_nested_path() {
        let (name, dir) = resolve_project_path("foo/bar/my-project", None).unwrap();
        assert_eq!(name, "my-project");
        assert_eq!(dir, PathBuf::from("foo/bar"));
    }

    #[test]
    fn resolve_with_output_override() {
        let (name, dir) = resolve_project_path("my-project", Some(Path::new("/tmp"))).unwrap();
        assert_eq!(name, "my-project");
        assert_eq!(dir, PathBuf::from("/tmp"));
    }

    #[test]
    fn resolve_path_with_output_override() {
        let (name, dir) = resolve_project_path("foo/my-project", Some(Path::new("/tmp"))).unwrap();
        assert_eq!(name, "my-project");
        assert_eq!(dir, PathBuf::from("/tmp"));
    }

    #[test]
    fn validate_good_names() {
        assert!(validate_project_name("my-project").is_ok());
        assert!(validate_project_name("my_project").is_ok());
        assert!(validate_project_name("project123").is_ok());
        assert!(validate_project_name("MyProject").is_ok());
    }

    #[test]
    fn validate_rejects_empty() {
        assert!(validate_project_name("").is_err());
    }

    #[test]
    fn validate_rejects_hidden() {
        assert!(validate_project_name(".hidden").is_err());
    }

    #[test]
    fn validate_rejects_path_separators() {
        assert!(validate_project_name("foo/bar").is_err());
        assert!(validate_project_name("foo\\bar").is_err());
    }

    #[test]
    fn validate_rejects_invalid_chars() {
        assert!(validate_project_name("foo:bar").is_err());
        assert!(validate_project_name("foo|bar").is_err());
        assert!(validate_project_name("foo*bar").is_err());
    }

    #[test]
    fn parse_rust_frameworks() {
        assert!(parse_framework(Language::Rust, "axum").is_ok());
        assert!(parse_framework(Language::Rust, "actix").is_ok());
        assert!(parse_framework(Language::Rust, "AXUM").is_ok()); // Case insensitive
        assert!(parse_framework(Language::Rust, "fastapi").is_err()); // Wrong language
    }

    #[test]
    fn parse_python_frameworks() {
        assert!(parse_framework(Language::Python, "fastapi").is_ok());
        assert!(parse_framework(Language::Python, "django").is_ok());
        assert!(parse_framework(Language::Python, "axum").is_err()); // Wrong language
    }

    #[test]
    fn parse_typescript_frameworks() {
        assert!(parse_framework(Language::TypeScript, "express").is_ok());
        assert!(parse_framework(Language::TypeScript, "nestjs").is_ok());
        assert!(parse_framework(Language::TypeScript, "nextjs").is_ok());
        assert!(parse_framework(Language::TypeScript, "react").is_ok());
        assert!(parse_framework(Language::TypeScript, "vue").is_ok());
    }

    #[test]
    fn convert_languages() {
        assert!(matches!(
            convert_language(Language::Rust),
            CoreLanguage::Rust
        ));
        assert!(matches!(
            convert_language(Language::Python),
            CoreLanguage::Python
        ));
        assert!(matches!(
            convert_language(Language::TypeScript),
            CoreLanguage::TypeScript
        ));
    }

    #[test]
    fn convert_kinds() {
        assert!(matches!(
            convert_kind(ProjectKind::Cli),
            CoreProjectKind::Cli
        ));
        assert!(matches!(
            convert_kind(ProjectKind::WebApi),
            CoreProjectKind::WebBackend
        ));
    }

    #[test]
    fn convert_architectures() {
        assert!(matches!(
            convert_architecture(Architecture::Layered),
            CoreArchitecture::Layered
        ));
        assert!(matches!(
            convert_architecture(Architecture::Mvc),
            CoreArchitecture::MVC
        ));
    }
}
