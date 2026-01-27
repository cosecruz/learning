mod environment;
pub mod telemetry;

use self::environment::Environment;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub rust_log: String,
    pub environment: Environment,
    // db_url
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let environment = Environment::from_env();

        // Load .env only in non-production
        if environment != Environment::Production {
            dotenvy::dotenv().ok(); // silently ignore errors
        }

        let host = env("APP_HOST").unwrap_or_else(|| "127.0.0.1".to_string());

        let port = match env("APP_PORT") {
            Some(s) => s
                .parse()
                .map_err(|_| "Port must be a valid u16".to_string())?,
            None => match environment {
                Environment::Production => {
                    return Err("Port must be set in production".to_string());
                }
                _ => 3000,
            },
        };

        let rust_log = env("RUST_LOG").unwrap_or_else(|| "info".to_string());

        // Validate that host + port forms a valid SocketAddr
        let addr_str = format!("{host}:{port}");
        let _addr: SocketAddr = addr_str
            .parse()
            .map_err(|e| format!("Invalid host/port combination: {e}"))?;

        Ok(Self {
            host,
            port,
            rust_log,
            environment,
        })
    }

    /// Return the SocketAddr for server binding
    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Config validated; this should never fail")
    }
}

fn env(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

// logging level

// debug endpoints

// tracing verbosity

// CORS rules

// feature flags

// TODO: unit tests for config
