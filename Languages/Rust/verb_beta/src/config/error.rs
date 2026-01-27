use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable '{key}' error: {reason}")]
    EnvironmentVariable { key: String, reason: String },

    #[error("Invalid port value '{value}': {source}")]
    InvalidPort {
        value: String,
        #[source]
        source: std::num::ParseIntError,
    },

    #[error("Required field '{field}' is missing in {environment} environment")]
    MissingRequiredField { field: String, environment: String },

    #[error("Failed to resolve host '{host}:{port}'")]
    HostResolution {
        host: String,
        port: u16,
        #[source]
        source: std::io::Error,
    },

    #[error("Host '{host}:{port}' did not resolve to any socket addresses")]
    NoResolvedAddresses { host: String, port: u16 },
}

// impl fmt::Display for ConfigError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::EnvironmentVariable { key, reason } => {
//                 write!(f, "Environment variable '{key}' error: {reason}")
//             }
//             Self::InvalidPort { value, source } => {
//                 write!(f, "Invalid port value '{value}': {source}")
//             }
//             Self::InvalidSocketAddr { host, port, source } => {
//                 write!(f, "Invalid socket address '{host}:{port}': {source}")
//             }
//             Self::MissingRequiredField { field, environment } => {
//                 write!(
//                     f,
//                     "Required field '{field}' is missing in {environment} environment"
//                 )
//             }

//             // --- NEW ---
//             Self::HostResolution { host, port, source } => {
//                 write!(f, "Failed to resolve host '{host}:{port}': {source}")
//             }
//             Self::NoResolvedAddresses { host, port } => {
//                 write!(
//                     f,
//                     "Host '{host}:{port}' did not resolve to any socket addresses"
//                 )
//             }
//         }
//     }
// }

// impl std::error::Error for ConfigError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             Self::InvalidPort { source, .. } => Some(source),
//             Self::InvalidSocketAddr { source, .. } => Some(source),
//             Self::HostResolution { source, .. } => Some(source),
//             _ => None,
//         }
//     }
// }
