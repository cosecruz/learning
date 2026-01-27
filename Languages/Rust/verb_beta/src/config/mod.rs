mod environment;
mod error;
pub mod telemetry;

pub use error::ConfigError;

use self::environment::Environment;
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    // pub rust_log: String,
    pub environment: Environment,
    // db_url
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        // Load dotenv early (safe no-op in prod)
        dotenvy::dotenv().ok();

        let environment = Environment::from_env();

        let host = env("APP_HOST").unwrap_or_else(|| "127.0.0.1".to_string());

        let port = match env("APP_PORT") {
            Some(raw) => raw.parse().map_err(|e| ConfigError::InvalidPort {
                value: raw,
                source: e,
            })?,
            None if environment.is_production() => {
                return Err(ConfigError::MissingRequiredField {
                    field: "APP_PORT".into(),
                    environment: "production".into(),
                });
            }
            None => 3000,
        };

        Ok(Self {
            host,
            port,
            environment,
        })
    }

    /// Select a single address for binding (IPv4 preferred)
    pub fn bind_addr(&self) -> Result<SocketAddr, ConfigError> {
        let addrs = self.socket_addrs()?;

        Ok(addrs
            .iter()
            .find(|a| a.is_ipv4())
            .copied()
            .unwrap_or(addrs[0]))
    }

    /// Resolve host into concrete socket addresses (IPv4 + IPv6)
    fn socket_addrs(&self) -> Result<Vec<SocketAddr>, ConfigError> {
        let addrs: Vec<SocketAddr> = (self.host.as_str(), self.port)
            .to_socket_addrs()
            .map_err(|e| ConfigError::HostResolution {
                host: self.host.clone(),
                port: self.port,
                source: e,
            })?
            .collect();

        if addrs.is_empty() {
            return Err(ConfigError::NoResolvedAddresses {
                host: self.host.clone(),
                port: self.port,
            });
        }

        Ok(addrs)
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

// TODO: use a builder pattern to allow better testing
// TODO: unit tests for config
