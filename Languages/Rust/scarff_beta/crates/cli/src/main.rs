//! # Scarff CLI
//!
//! Command-line interface for the Scarff project scaffolding tool.
//!
//! ## Quick Start
//!
//! ```bash
//! # Create a new Rust CLI project
//! scarff new my-cli --lang rust --type cli --arch layered
//!
//! # Create a Python backend with FastAPI
//! scarff new my-api --lang=python --type=backend --framework=fastapi
//!
//! # Short form
//! scarff new my-app -l rust -t backend -a layered -f axum
//! ```

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod args;
mod commands;
mod error;
mod output;

use args::Cli;

fn main() -> Result<()> {
    // Parse CLI arguments first (this will handle --help, --version, etc.)
    let cli = Cli::parse();

    // Initialize logging based on verbosity flags
    init_logging(&cli)?;

    // Execute the command
    cli.execute()
}

/// Initialize tracing/logging based on CLI flags.
///
/// Logging behavior:
/// - Default: Only errors are shown
/// - `-v` (verbose): Info-level messages (progress, major steps)
/// - `-q` (quiet): No output except critical errors
/// - `RUST_LOG` env var: Overrides CLI flags
fn init_logging(cli: &Cli) -> Result<()> {
    // Determine log level from flags
    let default_filter = if cli.quiet {
        "error"
    } else if cli.verbose {
        "scarff=info,scarff_core=info"
    } else {
        "warn"
    };

    // Build the filter
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default_filter))
        .unwrap();

    // Set up subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(false) // Don't show module paths
                .with_writer(std::io::stderr) // Log to stderr
                .without_time() // Don't show timestamps (cleaner for CLI)
                .with_ansi(!cli.no_color), // Respect --no-color flag
        )
        .init();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn help_message_includes_examples() {
        use clap::CommandFactory;
        let help = Cli::command().render_help().to_string();
        assert!(help.contains("EXAMPLES:"));
    }

    #[test]
    fn version_is_set() {
        use clap::CommandFactory;
        let cmd = Cli::command();
        assert!(cmd.get_version().is_some());
    }
}
