use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

use super::{Config, Environment};

pub fn init_tracing(cfg: &Config) {
    // --Get Environment from cfg.environment
    let environment = cfg.environment;

    // Step 1: Create the filter
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // --Build subscriber based on environment
    match environment {
        Environment::Production => {
            // JSON output, minimal metadata
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
            // Pretty output for humans
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
}
