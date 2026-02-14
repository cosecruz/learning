//! Implementation of the `scarff new` command.

use std::path::{Path, PathBuf};

use anyhow::Context;
use tracing::{debug, info, instrument};

use scarff_adapters::{InMemoryStore, LocalFilesystem, SimpleRenderer};
use scarff_core::{
    application::{ScaffoldService, TemplateService},
    domain::{
        Architecture as CoreArch, Framework as CoreFramework, Language as CoreLanguage,
        ProjectKind as CoreKind, Target, TargetBuilder,
    },
};

use crate::{
    cli::{Architecture, Language, NewArgs, ProjectKind, global::GlobalArgs},
    config::AppConfig,
    error::{CliError, CliResult, IntoCli},
    output::OutputManager,
};

/// Execute the `new` command.
#[instrument(skip(args, global, config, output))]
pub async fn execute(
    args: NewArgs,
    global: GlobalArgs,
    config: AppConfig,
    output: OutputManager,
) -> CliResult<()> {
    debug!("Executing new command with args: {:?}", args);

    // 1. Resolve project path
    let (project_name, output_dir) = resolve_project_path(&args.name, args.output.as_deref())?;
    validate_project_name(&project_name)?;

    info!(
        "Project: {}, Output: {}",
        project_name,
        output_dir.display()
    );

    // 2. Build target
    let target = build_target(&args, &config)?;
    info!(target = ?target );

    // 3. Show configuration
    if !global.quiet && !args.yes {
        show_configuration(&target, &project_name, &output_dir, &output)?;
        if !confirm()? {
            return Err(CliError::Cancelled);
        }
    }

    // 4. Check for existing directory
    let project_path = output_dir.join(&project_name);
    if project_path.exists() && !args.force {
        return Err(CliError::ProjectExists { path: project_path });
    }

    // 5. Dry-run: describe but do not write.
    if args.dry_run {
        output.info(&format!(
            "Dry run: would create project at {}",
            project_path.display(),
        ))?;
        return Ok(());
    }

    // 6. Create adapters and service
    let store = Box::new(InMemoryStore::with_builtin().map_err(CliError::Core)?);
    let renderer = Box::new(SimpleRenderer::new());
    let filesystem = Box::new(LocalFilesystem::new());

    let service = ScaffoldService::new(store, renderer, filesystem);

    // 7. Execute scaffolding
    output.header(&format!("Creating project '{}'...", project_name))?;

    service
        .scaffold(target, &project_name, &output_dir)
        .map_err(CliError::Core)?;

    // 8. Success
    output.success(&format!("Project '{}' created successfully!", project_name))?;

    if !global.quiet {
        output.print("")?;
        output.print("Next steps:")?;
        output.print(&format!("  cd {}", project_name))?;
        output.print("  # Start coding!")?;
    }

    Ok(())
}

// Helper functions...

fn resolve_project_path(name: &str, output: Option<&Path>) -> CliResult<(String, PathBuf)> {
    let path = PathBuf::from(name);

    if let Some(out) = output {
        let name = path.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
            CliError::InvalidProjectName {
                name: name.into(),
                reason: "Invalid path".into(),
            }
        })?;
        return Ok((name.into(), out.into()));
    }

    if let Some(parent) = path.parent().filter(|p| p != &Path::new("")) {
        let name = path.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
            CliError::InvalidProjectName {
                name: name.into(),
                reason: "Invalid path".into(),
            }
        })?;
        Ok((name.into(), parent.into()))
    } else {
        Ok((name.into(), PathBuf::from(".")))
    }
}

fn validate_project_name(name: &str) -> CliResult<()> {
    if name.is_empty() {
        return Err(CliError::InvalidProjectName {
            name: name.into(),
            reason: "Cannot be empty".into(),
        });
    }
    if name.starts_with('.') {
        return Err(CliError::InvalidProjectName {
            name: name.into(),
            reason: "Cannot start with '.'".into(),
        });
    }
    if name.contains('/') || name.contains('\\') {
        return Err(CliError::InvalidProjectName {
            name: name.into(),
            reason: "Cannot contain path separators".into(),
        });
    }
    Ok(())
}

fn build_target(args: &NewArgs, _config: &AppConfig) -> CliResult<Target> {
    let lang = convert_language(args.language);

    let mut builder = Target::builder().language(lang);

    info!(args = ?args);

    // kind
    if let Some(kind) = args.kind {
        let k = convert_kind(kind);
        builder = builder.kind(k).map_err(|e| CliError::Core(e.into()))?;
    }
    info!(args = ?args);

    // framework
    if let Some(fw_str) = &args.framework {
        let fw = parse_framework(args.language, fw_str)?;
        builder = builder
            .framework(fw)
            .map_err(|e| CliError::Core(e.into()))?;
    }

    // architecture
    if let Some(arch) = args.architecture {
        let arch = convert_architecture(arch);
        builder = builder.architecture(arch)
    }
    info!(args = ?args);

    builder.build().map_err(|e| CliError::Core(e.into()))
}

fn convert_language(lang: Language) -> CoreLanguage {
    match lang {
        Language::Rust => CoreLanguage::Rust,
        Language::Python => CoreLanguage::Python,
        Language::TypeScript => CoreLanguage::TypeScript,
    }
}

fn convert_kind(kind: ProjectKind) -> CoreKind {
    match kind {
        ProjectKind::Cli => CoreKind::Cli,
        ProjectKind::Backend => CoreKind::WebBackend,
        ProjectKind::Frontend => CoreKind::WebFrontend,
        ProjectKind::Fullstack => CoreKind::Fullstack,
        ProjectKind::Worker => CoreKind::Worker,
    }
}

fn convert_architecture(arch: Architecture) -> CoreArch {
    match arch {
        Architecture::Layered => CoreArch::Layered,
        Architecture::Clean => CoreArch::Clean,
        // TODO: create for onion on scarff-core
        Architecture::Onion => CoreArch::Clean,
        Architecture::Modular => CoreArch::FeatureModular,
        Architecture::Mvc => CoreArch::Mvc,
    }
}

fn parse_framework(lang: Language, fw: &str) -> CliResult<CoreFramework> {
    use scarff_core::domain::{PythonFramework, RustFramework, TypeScriptFramework};

    let fw_lower = fw.to_lowercase();

    match lang {
        Language::Rust => match fw_lower.as_str() {
            "axum" => Ok(CoreFramework::Rust(RustFramework::Axum)),
            "actix" => Ok(CoreFramework::Rust(RustFramework::Actix)),
            _ => Err(CliError::FrameworkNotAvailable {
                framework: fw.into(),
                language: lang.to_string(),
            }),
        },
        Language::Python => match fw_lower.as_str() {
            "fastapi" => Ok(CoreFramework::Python(PythonFramework::FastApi)),
            "django" => Ok(CoreFramework::Python(PythonFramework::Django)),
            _ => Err(CliError::FrameworkNotAvailable {
                framework: fw.into(),
                language: lang.to_string(),
            }),
        },
        Language::TypeScript => match fw_lower.as_str() {
            "express" => Ok(CoreFramework::TypeScript(TypeScriptFramework::Express)),
            "nestjs" => Ok(CoreFramework::TypeScript(TypeScriptFramework::NestJs)),
            "nextjs" => Ok(CoreFramework::TypeScript(TypeScriptFramework::NextJs)),
            "react" => Ok(CoreFramework::TypeScript(TypeScriptFramework::React)),
            "vue" => Ok(CoreFramework::TypeScript(TypeScriptFramework::Vue)),
            _ => Err(CliError::FrameworkNotAvailable {
                framework: fw.into(),
                language: lang.to_string(),
            }),
        },
    }
}

fn show_configuration(
    target: &Target,
    name: &str,
    output: &Path,
    out: &OutputManager,
) -> CliResult<()> {
    out.header("Configuration:")?;
    out.print(&format!("  Project:  {}", name))?;
    out.print(&format!("  Language: {}", target.language()))?;
    out.print(&format!("  Type:     {}", target.kind()))?;
    out.print(&format!("  Arch:     {}", target.architecture()))?;
    if let Some(fw) = target.framework() {
        out.print(&format!("  Framework: {}", fw))?;
    }
    out.print(&format!("  Location: {}", output.join(name).display()))?;
    out.print("")?;
    Ok(())
}

fn confirm() -> CliResult<bool> {
    use std::io::{self, Write};

    print!("Continue? [Y/n] ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| CliError::IoError {
            message: "Failed to read input".into(),
            source: e,
        })?;

    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input == "y" || input == "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── resolve_project_path ──────────────────────────────────────────────

    #[test]
    fn simple_name_resolves_to_cwd() {
        let (name, dir) = resolve_project_path("my-app", None).unwrap();
        assert_eq!(name, "my-app");
        assert_eq!(dir, PathBuf::from("."));
    }

    #[test]
    fn path_name_extracts_leaf_and_parent() {
        let (name, dir) = resolve_project_path("../my-app", None).unwrap();
        assert_eq!(name, "my-app");
        assert_eq!(dir, PathBuf::from(".."));
    }

    #[test]
    fn explicit_output_overrides_path() {
        let (name, dir) = resolve_project_path("my-app", Some(Path::new("/tmp"))).unwrap();
        assert_eq!(name, "my-app");
        assert_eq!(dir, PathBuf::from("/tmp"));
    }

    // ── validate_project_name ─────────────────────────────────────────────

    #[test]
    fn empty_name_is_invalid() {
        assert!(matches!(
            validate_project_name(""),
            Err(CliError::InvalidProjectName { .. })
        ));
    }

    #[test]
    fn dotfile_name_is_invalid() {
        assert!(matches!(
            validate_project_name(".hidden"),
            Err(CliError::InvalidProjectName { .. })
        ));
    }

    #[test]
    fn slash_in_name_is_invalid() {
        assert!(matches!(
            validate_project_name("a/b"),
            Err(CliError::InvalidProjectName { .. })
        ));
    }

    #[test]
    fn valid_names_pass() {
        for name in &["my-project", "my_app", "project123", "MyApp"] {
            assert!(validate_project_name(name).is_ok(), "failed for: {name}");
        }
    }

    // ── parse_framework ───────────────────────────────────────────────────

    #[test]
    fn rust_axum_parses() {
        let fw = parse_framework(Language::Rust, "axum").unwrap();
        assert!(matches!(
            fw,
            CoreFramework::Rust(scarff_core::domain::RustFramework::Axum)
        ));
    }

    #[test]
    fn wrong_framework_for_language_is_error() {
        assert!(matches!(
            parse_framework(Language::Rust, "django"),
            Err(CliError::FrameworkNotAvailable { .. })
        ));
    }

    #[test]
    fn framework_matching_is_case_insensitive() {
        assert!(parse_framework(Language::Rust, "AXUM").is_ok());
        assert!(parse_framework(Language::Rust, "Axum").is_ok());
    }
}
