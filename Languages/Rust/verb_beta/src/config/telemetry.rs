use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use super::{Config, Environment};

// Initialize the tracing subscriber based on configuration
///
/// This function configures logging/tracing output based on the application
/// environment. Production uses JSON for log aggregators, while development
/// uses pretty-printed output for human readability.
///
/// # Panics
///
/// Panics if called more than once (global subscriber can only be set once)
pub fn init_tracing(cfg: &Config) {
    // --Get Environment from cfg.environment
    let environment = cfg.environment;

    // Build filter from RUST_LOG, with environment-aware defaults
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let default_level = match environment {
            Environment::Production => "info",
            Environment::UAT => "debug",
            Environment::Development => "debug",
        };

        EnvFilter::new(format!(
            "verb={default_level},tower_http=debug,axum=debug,{default_level}"
        ))
    });

    match environment {
        Environment::Production => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    fmt::layer()
                        .json()
                        .with_target(false)
                        .with_thread_ids(false)
                        .with_thread_names(false)
                        .with_file(false)
                        .with_line_number(false)
                        .with_ansi(false),
                )
                .init();
        }
        _ => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    fmt::layer()
                        .pretty()
                        .with_target(true)
                        .with_file(true)
                        .with_line_number(true),
                )
                .init();
        }
    }

    // Log that tracing is initialized (using the newly installed subscriber)
    info!(
        environment = %environment,
        "Tracing initialized"
    );
}
