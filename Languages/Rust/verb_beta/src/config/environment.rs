use std::env;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Environment {
    Development,
    UAT,
    Production,
}

impl Environment {
    pub(crate) fn from_env() -> Self {
        match env::var("APP_ENV")
            .unwrap_or_else(|_| "development".into())
            .as_str()
        {
            "production" => Self::Production,
            "uat" => Self::UAT,
            _ => Self::Development,
        }
    }
}
