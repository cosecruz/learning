use std::{env, fmt, str::FromStr};

use crate::config::ConfigError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    UAT,
    Production,
}

impl Environment {
    /// Load environment from APP_ENV variable, defaulting to Development
    pub fn from_env() -> Self {
        env::var("APP_ENV")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::Development)
    }

    /// Check if this is a production environment
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }

    /// Check if this is a development environment
    pub fn is_development(&self) -> bool {
        matches!(self, Self::Development)
    }

    /// Returns the string representation (lowercase)
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::UAT => "uat",
            Self::Production => "production",
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Environment {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "production" | "prod" => Ok(Self::Production),
            "uat" | "staging" => Ok(Self::UAT),
            "development" | "dev" => Ok(Self::Development),
            _ => Err(ConfigError::EnvironmentVariable {
                key: "APP_ENV".into(),
                reason: "Invalid environment".into(),
            }),
        }
    }
}

impl AsRef<str> for Environment {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
