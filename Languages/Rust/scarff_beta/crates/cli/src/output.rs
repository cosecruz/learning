//! User-facing output formatting and display.
//!
//! This module handles all terminal output, including:
//! - Colored and formatted messages
//! - Progress indicators
//! - Confirmation prompts
//! - Success/error displays

use anyhow::Result;
use console::{Term, style};
use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::OwoColorize;
use scarff_core::Target;
use std::io::{self, Write};
use std::path::Path;

// ============================================================================
// Configuration Display
// ============================================================================

/// Show the scaffold configuration to the user.
pub fn show_configuration(target: &Target, name: &str, output: &Path) -> Result<()> {
    let term = Term::stdout();

    term.write_line("")?;
    term.write_line(&format!("{}", "╭─ Scaffold Configuration ".cyan().bold()))?;
    term.write_line(&format!("│"))?;
    term.write_line(&format!(
        "│ {} {}",
        "Project Name:".dimmed(),
        style(name).cyan().bold()
    ))?;
    term.write_line(&format!(
        "│ {} {}",
        "Language:    ".dimmed(),
        style(target.language()).green()
    ))?;
    term.write_line(&format!(
        "│ {} {}",
        "Type:        ".dimmed(),
        style(target.project_type()).green()
    ))?;
    term.write_line(&format!(
        "│ {} {}",
        "Architecture:".dimmed(),
        style(target.architecture()).green()
    ))?;

    if let Some(fw) = target.framework() {
        term.write_line(&format!(
            "│ {} {}",
            "Framework:   ".dimmed(),
            style(fw).green()
        ))?;
    }

    let full_path = output.join(name);
    term.write_line(&format!(
        "│ {} {}",
        "Location:    ".dimmed(),
        style(full_path.display()).cyan()
    ))?;
    term.write_line(&format!("│"))?;
    term.write_line(&format!("{}", "╰─".cyan()))?;
    term.write_line("")?;

    Ok(())
}

/// Show dry-run output (what would be created).
pub fn show_dry_run(target: &Target, name: &str, output: &Path) -> Result<()> {
    let term = Term::stdout();

    show_configuration(target, name, output)?;

    term.write_line(&format!(
        "{} Dry run - nothing will be created",
        "ℹ".blue().bold()
    ))?;
    term.write_line("")?;

    Ok(())
}

// ============================================================================
// Confirmation
// ============================================================================

/// Prompt the user for confirmation.
///
/// Returns Ok(()) if user confirms, Err if they cancel.
pub fn confirm() -> Result<()> {
    let term = Term::stdout();

    term.write_line(&format!(
        "{} Press {} to continue or {} to cancel...",
        "?".yellow().bold(),
        "Enter".green(),
        "Ctrl+C".red()
    ))?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}

// ============================================================================
// Progress Indicators
// ============================================================================

/// Show progress while executing a function.
pub fn show_progress<F, T>(message: &str, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let result = f();

    pb.finish_and_clear();

    match result {
        Ok(val) => {
            println!("{} {}", "✓".green().bold(), message);
            Ok(val)
        }
        Err(e) => {
            println!("{} {}", "✗".red().bold(), message);
            Err(e)
        }
    }
}

// ============================================================================
// Success Messages
// ============================================================================

/// Show success message after scaffolding.
pub fn show_success(name: &str, project_path: &Path, verbose: bool) -> Result<()> {
    let term = Term::stdout();

    term.write_line("")?;
    term.write_line(&format!(
        "{} Project created successfully!",
        "✓".green().bold()
    ))?;
    term.write_line("")?;

    // Show next steps
    term.write_line(&format!("{}", "Next steps:".cyan().bold()))?;
    term.write_line(&format!("  {} cd {}", "1.".dimmed(), style(name).cyan()))?;

    if verbose {
        term.write_line(&format!(
            "  {} Open in your editor: {} {}",
            "2.".dimmed(),
            "code".yellow(),
            name
        ))?;
        term.write_line(&format!(
            "  {} Start coding! {}",
            "3.".dimmed(),
            "🚀".to_string()
        ))?;
    } else {
        term.write_line(&format!("  {} Start coding!", "2.".dimmed()))?;
    }

    term.write_line("")?;

    Ok(())
}

// ============================================================================
// Error Display
// ============================================================================

/// Display an error with helpful formatting.
pub fn show_error(error: &anyhow::Error) -> Result<()> {
    let term = Term::stderr();

    term.write_line("")?;
    term.write_line(&format!("{} Error", "✗".red().bold()))?;
    term.write_line("")?;

    // Show the error chain
    for (i, cause) in error.chain().enumerate() {
        if i == 0 {
            term.write_line(&format!("  {}", cause.to_string().red()))?;
        } else {
            term.write_line(&format!("  {} {}", "→".dimmed(), cause))?;
        }
    }

    term.write_line("")?;

    Ok(())
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create a styled header.
pub fn header(text: &str) -> String {
    format!("{}", text.cyan().bold())
}

/// Create a styled value.
pub fn value(text: &str) -> String {
    format!("{}", text.green())
}

/// Create a styled path.
pub fn path(text: &str) -> String {
    format!("{}", text.cyan())
}

/// Create a styled warning.
pub fn warning(text: &str) -> String {
    format!("{} {}", "⚠".yellow().bold(), text.yellow())
}

/// Create a styled info message.
pub fn info(text: &str) -> String {
    format!("{} {}", "ℹ".blue().bold(), text)
}

/// Create a styled success message.
pub fn success(text: &str) -> String {
    format!("{} {}", "✓".green().bold(), text.green())
}

/// Create a styled error message.
pub fn error(text: &str) -> String {
    format!("{} {}", "✗".red().bold(), text.red())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_helpers() {
        // Just verify they don't panic
        let _ = header("Test");
        let _ = value("Test");
        let _ = path("Test");
        let _ = warning("Test");
        let _ = info("Test");
        let _ = success("Test");
        let _ = error("Test");
    }

    #[test]
    fn test_show_progress() {
        // Test successful execution
        let result = show_progress("Testing", || Ok(42));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        // Test failed execution
        let result: Result<()> = show_progress("Testing", || Err(anyhow::anyhow!("test error")));
        assert!(result.is_err());
    }
}
