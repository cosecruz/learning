//! Error types for the CLI.
//!
//! This module defines CLI-specific errors that provide helpful,
//! user-friendly error messages.

use owo_colors::OwoColorize;
use scarff_core::CoreError;
use thiserror::Error;

/// CLI-specific errors with user-friendly messages.
#[derive(Debug, Error)]
pub enum CliError {
    /// Unsupported programming language
    #[error("Unsupported language '{0}'")]
    UnsupportedLanguage(String),

    /// Unsupported project type
    #[error("Unsupported project type '{0}'")]
    UnsupportedProjectKind(String),

    /// Unsupported architecture
    #[error("Unsupported architecture '{0}'")]
    UnsupportedArchitecture(String),

    /// Framework not compatible with language
    #[error("Framework '{framework}' is not available for {language}")]
    FrameworkNotAvailable { framework: String, language: String },

    /// Invalid project name
    #[error("Invalid project name: {reason}")]
    InvalidProjectName { reason: String },

    /// Project directory already exists
    #[error("Project directory already exists: {path}")]
    ProjectExists { path: String },

    /// Core library error (wrapped)
    #[error("Scaffolding error: {0}")]
    Core(#[from] CoreError),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// User cancelled the operation
    #[error("Operation cancelled by user")]
    Cancelled,

    /// Generic error from anyhow
    #[error("{0}")]
    Other(&'static str),
}

impl CliError {
    ///format the error with helpful suggestions.
    pub fn format_with_suggestions(&self) -> String {
        let error_msg = format!("{} {}", "Error:".red().bold(), self);

        let suggestions = match self {
            CliError::UnsupportedLanguage(lang) => {
                format!(
                    "\n\n{}\n  - rust\n  - python\n  - typescript (or 'ts')\n\n{}\n  scarff new my-project --lang rust --type cli --arch layered",
                    "Supported languages:".yellow(),
                    "Example:".cyan()
                )
            }
            CliError::UnsupportedProjectKind(pt) => {
                format!(
                    "\n\n{}\n  - cli (command-line application)\n  - backend (web service)\n  - frontend (web app)\n  - fullstack (frontend + backend)\n  - worker (background jobs)\n\n{}\n  scarff new my-api --lang python --type backend --arch layered",
                    "Supported project types:".yellow(),
                    "Example:".cyan()
                )
            }
            CliError::UnsupportedArchitecture(arch) => {
                format!(
                    "\n\n{}\n  - layered (domain, application, infrastructure)\n  - mvc (model-view-controller)\n  - modular (feature-based modules)\n  - app-router (Next.js app router)\n\n{}\n  scarff new my-app --lang rust --type backend --arch layered",
                    "Supported architectures:".yellow(),
                    "Example:".cyan()
                )
            }
            CliError::FrameworkNotAvailable {
                framework,
                language,
            } => {
                let suggestions = get_framework_suggestions(language);
                if suggestions.is_empty() {
                    String::new()
                } else {
                    format!(
                        "\n\n{} {}\n{}\n\n{}\n  scarff new my-project --lang {} --framework {}",
                        "Available frameworks for".yellow(),
                        language.cyan(),
                        suggestions
                            .iter()
                            .map(|s| format!("  - {}", s))
                            .collect::<Vec<_>>()
                            .join("\n"),
                        "Example:".cyan(),
                        language,
                        suggestions[0]
                    )
                }
            }
            CliError::ProjectExists { path } => {
                format!(
                    "\n\n{}\n  1. Use a different project name\n  2. Remove the existing directory: rm -rf {}\n  3. Use --force to overwrite (not recommended)",
                    "Options:".yellow(),
                    path
                )
            }
            CliError::InvalidProjectName { reason } => {
                format!(
                    "\n\n{}\n  - Use alphanumeric characters, hyphens, and underscores\n  - Start with a letter or number\n  - Avoid special characters and spaces\n\n{}\n  my-project, my_app, project123",
                    "Project name requirements:".yellow(),
                    "Valid examples:".cyan()
                )
            }
            _ => String::new(),
        };

        format!("{}{}", error_msg, suggestions)
    }
}

// Convert from anyhow::Error to CliError
impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> Self {
        // Try to downcast to CoreError first
        if let Some(core_err) = err.downcast_ref::<CoreError>() {
            return CliError::Core(core_err.clone());
        }

        // Try to downcast to CliError (in case it's already a CliError)
        if let Ok(cli_err) = err.downcast::<CliError>() {
            return cli_err;
        }

        // Otherwise, convert to Other
        // CliError::Other(err.to_string())
        CliError::Other("")
    }
}

/// Get framework suggestions for a given language.
fn get_framework_suggestions(language: &str) -> Vec<&'static str> {
    match language.to_lowercase().as_str() {
        "rust" => vec!["axum", "actix"],
        "python" => vec!["fastapi", "django"],
        "typescript" => vec!["express", "nestjs", "nextjs", "react", "vue"],
        _ => vec![],
    }
}

/// Extension trait for converting Result types to CLI errors.
///
/// This trait provides convenient conversion from various Result types
/// to Result<T, CliError>.
pub trait IntoCli<T> {
    /// Convert this Result into a CLI Result.
    fn into_cli(self) -> Result<T, CliError>;
}

// Implementation for anyhow::Result
impl<T> IntoCli<T> for anyhow::Result<T> {
    fn into_cli(self) -> Result<T, CliError> {
        self.map_err(CliError::from)
    }
}

// Implementation for Result<T, CoreError>
impl<T> IntoCli<T> for Result<T, CoreError> {
    fn into_cli(self) -> Result<T, CliError> {
        self.map_err(CliError::Core)
    }
}

// Implementation for Result<T, std::io::Error>
impl<T> IntoCli<T> for Result<T, std::io::Error> {
    fn into_cli(self) -> Result<T, CliError> {
        self.map_err(CliError::Io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_language_error_has_suggestions() {
        let err = CliError::UnsupportedLanguage("go".to_string());
        let formatted = err.format_with_suggestions();
        assert!(formatted.contains("Supported languages"));
        assert!(formatted.contains("rust"));
        assert!(formatted.contains("python"));
        assert!(formatted.contains("typescript"));
    }

    #[test]
    fn unsupported_kind_error_has_suggestions() {
        let err = CliError::UnsupportedProjectKind("mobile".to_string());
        let formatted = err.format_with_suggestions();
        assert!(formatted.contains("Supported project types"));
        assert!(formatted.contains("cli"));
        assert!(formatted.contains("backend"));
    }

    #[test]
    fn framework_suggestions_for_rust() {
        let suggestions = get_framework_suggestions("rust");
        assert!(suggestions.contains(&"axum"));
        assert!(suggestions.contains(&"actix"));
    }

    #[test]
    fn framework_suggestions_for_python() {
        let suggestions = get_framework_suggestions("python");
        assert!(suggestions.contains(&"fastapi"));
        assert!(suggestions.contains(&"django"));
    }

    #[test]
    fn framework_suggestions_for_typescript() {
        let suggestions = get_framework_suggestions("typescript");
        assert!(suggestions.contains(&"express"));
        assert!(suggestions.contains(&"react"));
    }

    #[test]
    fn project_exists_error_has_suggestions() {
        let err = CliError::ProjectExists {
            path: "./my-project".to_string(),
        };
        let formatted = err.format_with_suggestions();
        assert!(formatted.contains("Options"));
        assert!(formatted.contains("--force"));
    }

    #[test]
    fn into_cli_converts_anyhow_error() {
        let anyhow_err: anyhow::Result<()> = Err(anyhow::anyhow!("test error"));
        let cli_result = anyhow_err.into_cli();
        assert!(cli_result.is_err());
    }

    #[test]
    fn into_cli_converts_io_error() {
        let io_err: Result<(), std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "not found",
        ));
        let cli_result = io_err.into_cli();
        assert!(cli_result.is_err());
        assert!(matches!(cli_result.unwrap_err(), CliError::Io(_)));
    }

    #[test]
    fn from_anyhow_error() {
        let anyhow_err = anyhow::anyhow!("test error");
        let cli_err = CliError::from(anyhow_err);
        assert!(matches!(cli_err, CliError::Other(_)));
    }
}
