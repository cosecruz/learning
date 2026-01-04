use std::borrow::Cow;
use std::fmt;

#[derive(Debug)]
struct Config {
    port: u16,
    host: String,
    max_connections: usize,
    timeout_ms: u16,
}

#[derive(Debug)]
struct ValidationError {
    field: Cow<'static, str>,
    message: Cow<'static, str>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

type Errors = Vec<ValidationError>;

/// Exact function pointer type (NO generics allowed here)
type Rule = fn(&Config, &mut Errors);

/* ---------------- RULE IMPLEMENTATIONS ---------------- */

fn validate_port(config: &Config, errors: &mut Errors) {
    if config.port < 1024 {
        errors.push(ValidationError {
            field: Cow::Borrowed("port"),
            message: Cow::Borrowed("port must be >= 1024"),
        });
    }
}

fn validate_host(config: &Config, errors: &mut Errors) {
    if config.host.trim().is_empty() {
        errors.push(ValidationError {
            field: Cow::Borrowed("host"),
            message: Cow::Borrowed("host must not be empty"),
        });
    }
}

fn validate_max_connections(config: &Config, errors: &mut Errors) {
    if config.max_connections == 0 || config.max_connections >= 10_000 {
        errors.push(ValidationError {
            field: Cow::Borrowed("max_connections"),
            message: Cow::Borrowed("must be > 0 and < 10000"),
        });
    }
}

fn validate_timeout(config: &Config, errors: &mut Errors) {
    if config.timeout_ms < 100 || config.timeout_ms > 60_000 {
        errors.push(ValidationError {
            field: Cow::Borrowed("timeout_ms"),
            message: Cow::Borrowed("timeout must be between 100ms and 60000ms"),
        });
    }
}

/* ---------------- RULE REGISTRY ---------------- */

const RULES: &[Rule] = &[
    validate_port,
    validate_host,
    validate_max_connections,
    validate_timeout,
];

/* ---------------- VALIDATION ENTRY POINT ---------------- */

fn validate_config(config: &Config) -> Errors {
    let mut errors = Vec::new();

    for rule in RULES {
        rule(config, &mut errors);
    }

    errors
}

/* ---------------- MAIN ---------------- */

fn main() {
    let config = Config {
        port: 80,
        host: "".into(),
        max_connections: 0,
        timeout_ms: 50,
    };

    let errors = validate_config(&config);

    for error in errors {
        println!("{}", error);
    }
}
