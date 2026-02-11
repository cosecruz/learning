// crates/cli/src/args.rs
//! Command-line argument definitions and parsing.
//!
//! This module defines the CLI interface using clap's derive API.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::commands;
use anyhow::Result;

// ============================================================================
// Root CLI
// ============================================================================

#[derive(Debug, Parser)]
#[command(
    name = "scarff",
    version,
    author,
    about = "Project scaffolding made instant ⚡",
    long_about = "Scarff is a powerful project scaffolding tool that generates \
                  production-ready project structures based on your specifications.\n\n\
                  It supports multiple languages (Rust, Python, TypeScript), frameworks, \
                  and architectural patterns.",
    after_help = "EXAMPLES:\n  \
        # Create a Rust CLI project\n  \
        scarff new my-cli --lang rust --type cli --arch layered\n\n  \
        # Create a Python backend with FastAPI\n  \
        scarff new my-api --lang=python --type=web_api --framework=fastapi\n\n  \
        # Short form with output directory\n  \
        scarff new ../my-app -l rust -t web_api -a layered -f axum\n\n  \
        # Interactive mode (future feature)\n  \
        scarff new my-project --interactive\n\n\
        For more information, visit: https://github.com/yourusername/scarff"
)]
pub struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output (shows progress and detailed information)
    #[arg(
        short = 'v',
        long = "verbose",
        global = true,
        conflicts_with = "quiet",
        help = "Show detailed progress information"
    )]
    pub verbose: bool,

    /// Suppress all non-error output
    #[arg(
        short = 'q',
        long = "quiet",
        global = true,
        conflicts_with = "verbose",
        help = "Only show errors"
    )]
    pub quiet: bool,

    /// Disable colored output
    #[arg(
        long = "no-color",
        global = true,
        env = "NO_COLOR",
        help = "Disable colored output"
    )]
    pub no_color: bool,
}

impl Cli {
    /// Execute the parsed command.
    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::New(cmd) => commands::new::execute(cmd, self.verbose, self.quiet),
        }
    }
}

// ============================================================================
// Subcommands
// ============================================================================

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new project from a template
    #[command(
        visible_alias = "n",
        after_help = "EXAMPLES:\n  \
            # Basic usage\n  \
            scarff new my-project --lang rust --type cli --arch layered\n\n  \
            # With framework\n  \
            scarff new my-api --lang python --type backend --framework fastapi\n\n  \
            # Output to specific directory\n  \
            scarff new ../projects/my-app -l rust -t backend -f axum\n\n  \
            # Skip confirmation prompt\n  \
            scarff new my-cli -l rust -t cli -a layered --yes"
    )]
    New(NewCommand),
}

// ============================================================================
// New Command
// ============================================================================

#[derive(Debug, Args)]
pub struct NewCommand {
    /// Project name or path
    ///
    /// Can be a simple name (e.g., "my-project") which creates ./my-project,
    /// or a path (e.g., "../my-project", "/tmp/my-project").
    #[arg(
        value_name = "NAME",
        help = "Project name or path (e.g., 'my-project', '../my-project')"
    )]
    pub name: String,

    /// Programming language
    #[arg(
        short = 'l',
        long = "lang",
        value_name = "LANGUAGE",
        value_enum,
        help = "Programming language for the project"
    )]
    pub language: Language,

    /// Project type
    #[arg(
        short = 't',
        long = "type",
        value_name = "TYPE",
        value_enum,
        help = "Type of project to generate"
    )]
    pub kind: ProjectKind,

    /// Architecture style
    #[arg(
        short = 'a',
        long = "arch",
        value_name = "ARCHITECTURE",
        value_enum,
        help = "Architectural pattern to use"
    )]
    pub architecture: Architecture,

    /// Framework (optional)
    #[arg(
        short = 'f',
        long = "framework",
        value_name = "FRAMEWORK",
        help = "Framework to use (e.g., axum, fastapi, react)"
    )]
    pub framework: Option<String>,

    /// Output directory (defaults to current directory)
    #[arg(
        short = 'o',
        long = "output",
        value_name = "DIR",
        help = "Output directory (default: current directory)"
    )]
    pub output: Option<PathBuf>,

    /// Skip confirmation prompt
    #[arg(
        short = 'y',
        long = "yes",
        help = "Skip confirmation and create project immediately"
    )]
    pub yes: bool,

    /// Overwrite existing directory (use with caution)
    #[arg(long = "force", help = "Overwrite existing project directory")]
    pub force: bool,

    /// Dry run (show what would be created without creating it)
    #[arg(
        long = "dry-run",
        help = "Show what would be created without actually creating it"
    )]
    pub dry_run: bool,
}

// ============================================================================
// Value Enums
// ============================================================================

/// Supported programming languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum Language {
    /// Rust programming language
    Rust,
    /// Python programming language
    Python,
    /// TypeScript programming language
    #[value(alias = "ts")]
    TypeScript,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::Python => write!(f, "python"),
            Language::TypeScript => write!(f, "typescript"),
        }
    }
}

/// Supported project types
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum ProjectKind {
    /// Command-line interface application
    Cli,
    /// Backend web service
    #[value(name = "web_api")]
    WebApi,
    /// Frontend web application
    #[value(name = "web_fe")]
    WebFrontend,
    /// Full-stack application (frontend + backend)
    Fullstack,
    /// Background worker/job processor
    Worker,
}

impl std::fmt::Display for ProjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectKind::Cli => write!(f, "cli"),
            ProjectKind::WebApi => write!(f, "web_api"),
            ProjectKind::WebFrontend => write!(f, "web_frontend"),
            ProjectKind::Fullstack => write!(f, "fullstack"),
            ProjectKind::Worker => write!(f, "worker"),
        }
    }
}

/// Supported architectural patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum Architecture {
    /// Layered architecture (domain, application, infrastructure)
    Layered,
    /// Model-View-Controller pattern
    Mvc,
    /// Modular architecture
    Modular,
    /// App Router (Next.js specific)
    #[value(name = "app-router")]
    AppRouter,
    ///Clean / Hexagonal Architecture
    Clean,
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::Layered => write!(f, "layered"),
            Architecture::Mvc => write!(f, "mvc"),
            Architecture::Modular => write!(f, "modular"),
            Architecture::AppRouter => write!(f, "app-router"),
            Architecture::Clean => write!(f, "clean"),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_structure() {
        // This test ensures the CLI structure is valid
        Cli::command().debug_assert();
    }

    #[test]
    fn test_language_value_enum() {
        // Test that all languages can be parsed
        assert_eq!(Language::from_str("rust", true).unwrap(), Language::Rust);
        assert_eq!(
            Language::from_str("python", true).unwrap(),
            Language::Python
        );
        assert_eq!(
            Language::from_str("typescript", true).unwrap(),
            Language::TypeScript
        );
        assert_eq!(
            Language::from_str("ts", true).unwrap(),
            Language::TypeScript
        ); // alias
    }

    #[test]
    fn test_kind_value_enum() {
        let p = ProjectKind::from_str("web_api", true);

        println!("{p:?}");
        assert_eq!(
            ProjectKind::from_str("cli", true).unwrap(),
            ProjectKind::Cli
        );
        assert_eq!(
            ProjectKind::from_str("web_api", true).unwrap(),
            ProjectKind::WebApi
        );
        assert_eq!(
            ProjectKind::from_str("frontend", true).unwrap(),
            ProjectKind::WebFrontend
        );
        assert_eq!(
            ProjectKind::from_str("fullstack", true).unwrap(),
            ProjectKind::Fullstack
        );
        assert_eq!(
            ProjectKind::from_str("worker", true).unwrap(),
            ProjectKind::Worker
        );
    }

    #[test]
    fn test_architecture_value_enum() {
        assert_eq!(
            Architecture::from_str("layered", true).unwrap(),
            Architecture::Layered
        );
        assert_eq!(
            Architecture::from_str("mvc", true).unwrap(),
            Architecture::Mvc
        );
        assert_eq!(
            Architecture::from_str("modular", true).unwrap(),
            Architecture::Modular
        );
        assert_eq!(
            Architecture::from_str("app-router", true).unwrap(),
            Architecture::AppRouter
        );
    }

    #[test]
    fn verbose_and_quiet_conflict() {
        // Ensure verbose and quiet flags conflict
        let result = Cli::try_parse_from([
            "scarff", "new", "test", "-l", "rust", "-t", "cli", "-a", "layered", "-v", "-q",
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn short_and_long_forms_work() {
        // Test short form
        let cli = Cli::try_parse_from([
            "scarff", "new", "test", "-l", "rust", "-t", "cli", "-a", "layered",
        ])
        .unwrap();

        if let Commands::New(cmd) = cli.command {
            assert_eq!(cmd.language, Language::Rust);
            assert_eq!(cmd.kind, ProjectKind::Cli);
            assert_eq!(cmd.architecture, Architecture::Layered);
        } else {
            panic!("Expected New command");
        }

        // Test long form
        let cli = Cli::try_parse_from([
            "scarff", "new", "test", "--lang", "rust", "--type", "cli", "--arch", "layered",
        ])
        .unwrap();

        if let Commands::New(cmd) = cli.command {
            assert_eq!(cmd.language, Language::Rust);
            assert_eq!(cmd.kind, ProjectKind::Cli);
            assert_eq!(cmd.architecture, Architecture::Layered);
        } else {
            panic!("Expected New command");
        }
    }

    #[test]
    fn equals_syntax_works() {
        // Test --key=value syntax
        let cli = Cli::try_parse_from([
            "scarff",
            "new",
            "test",
            "--lang=rust",
            "--type=cli",
            "--arch=layered",
        ])
        .unwrap();

        if let Commands::New(cmd) = cli.command {
            assert_eq!(cmd.language, Language::Rust);
        } else {
            panic!("Expected New command");
        }
    }

    #[test]
    fn framework_is_optional() {
        let cli = Cli::try_parse_from([
            "scarff", "new", "test", "-l", "rust", "-t", "cli", "-a", "layered",
        ])
        .unwrap();

        if let Commands::New(cmd) = cli.command {
            assert!(cmd.framework.is_none());
        } else {
            panic!("Expected New command");
        }
    }

    #[test]
    fn framework_can_be_specified() {
        let cli = Cli::try_parse_from([
            "scarff", "new", "test", "-l", "rust", "-t", "backend", "-a", "layered", "-f", "axum",
        ])
        .unwrap();

        println!("{cli:?}");
        if let Commands::New(cmd) = cli.command {
            assert_eq!(cmd.framework, Some("axum".to_string()));
        } else {
            panic!("Expected New command");
        }
    }

    #[test]
    fn output_directory_can_be_specified() {
        let cli = Cli::try_parse_from([
            "scarff",
            "new",
            "test",
            "-l",
            "rust",
            "-t",
            "cli",
            "-a",
            "layered",
            "-o",
            "/tmp/projects",
        ])
        .unwrap();

        if let Commands::New(cmd) = cli.command {
            assert_eq!(cmd.output, Some(PathBuf::from("/tmp/projects")));
        } else {
            panic!("Expected New command");
        }
    }

    #[test]
    fn yes_flag_works() {
        let cli = Cli::try_parse_from([
            "scarff", "new", "test", "-l", "rust", "-t", "cli", "-a", "layered", "--yes",
        ])
        .unwrap();

        if let Commands::New(cmd) = cli.command {
            assert!(cmd.yes);
        } else {
            panic!("Expected New command");
        }
    }

    #[test]
    fn help_shows_examples() {
        let mut cmd = Cli::command();
        let help = cmd.render_help().to_string();
        assert!(help.contains("EXAMPLES:"));
    }

    #[test]
    fn subcommand_alias_works() {
        // 'n' is an alias for 'new'
        let cli = Cli::try_parse_from(&[
            "scarff", "n", "test", "-l", "rust", "-t", "cli", "-a", "layered",
        ])
        .unwrap();

        assert!(matches!(cli.command, Commands::New(_)));
    }
}
