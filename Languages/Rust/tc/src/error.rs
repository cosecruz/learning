use std::fmt;

#[derive(Debug)]
pub enum TCError {
    CliError,
    ParseError(String),
}

impl fmt::Display for TCError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TCError::CliError => write!(f, "CLI error"),
            TCError::ParseError(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}
