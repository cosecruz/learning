use std::{collections::HashMap, fmt};

use anyhow::{Context, Result};
use scarff_core::Target;
use thiserror::Error;

/* ===========================
CLI ERRORS
=========================== */

#[derive(Debug, Error)]
pub enum CliError {
    #[error("invalid arguments")]
    InvalidArgument,

    #[error("invalid command `{0}`")]
    InvalidCommand(String),

    #[error("failure parsing arguments")]
    ParseError,
}

pub type CliResult<T> = Result<T>;

/* ===========================
COMMANDS
=========================== */

#[derive(Debug, Clone)]
pub enum Command {
    New(NewArgs),
}

impl Command {
    pub fn as_str(&self) -> &str {
        match self {
            Command::New(_) => "new",
        }
    }

    pub fn run(&self) -> CliResult<()> {
        match self {
            Command::New(args) => {
                println!("Running `new` command with:");
                println!("{args:#?}");

                // let target = Target::builder()
                //     .language(scarff_core::Language::Rust)
                //     .project_type(scarff_core::ProjectType::Cli)
                //     .resolve()?;

                // let scarff_engine = scarff_core::Engine::new();
                // scarff_engine.scaffold(target, "test_scarff", "./")
                Ok(())
            }
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/* ===========================
NEW COMMAND ARGS
=========================== */

#[derive(Debug, Clone)]
pub struct NewArgs {
    pub language: String,
    pub project_type: Option<String>,
    pub framework: Option<String>,
    pub architecture: Option<String>,
}

impl NewArgs {
    pub fn from(args: HashMap<String, String>) -> CliResult<Self> {
        let language = args
            .get("language")
            .cloned()
            .ok_or(CliError::ParseError)
            .context("`--language` is required")?;

        Ok(Self {
            language,
            project_type: args.get("project_type").cloned(),
            framework: args.get("framework").cloned(),
            architecture: args.get("architecture").cloned(),
        })
    }
}

/* ===========================
ARG PARSER
=========================== */

fn parse_args(args: &[String]) -> CliResult<Command> {
    if args.is_empty() {
        return Err(CliError::InvalidArgument).context("no command provided");
    }

    let command = args[0].as_str();
    let flags = &args[1..];

    match command {
        "new" => {
            let kv = parse_flags(flags)?;
            let args = NewArgs::from(kv)?;
            Ok(Command::New(args))
        }
        other => Err(CliError::InvalidCommand(other.to_string()))?,
    }
}

fn parse_flags(args: &[String]) -> CliResult<HashMap<String, String>> {
    let mut map = HashMap::new();
    let mut iter = args.iter().peekable();

    while let Some(arg) = iter.next() {
        if !arg.starts_with("--") {
            return Err(CliError::InvalidArgument).context(format!("unexpected argument `{arg}`"));
        }

        // --key=value form
        if let Some((k, v)) = arg.split_once('=') {
            let key = normalize_key(k)?;
            insert_unique(&mut map, key, v.to_string())?;
            continue;
        }

        // --key value form
        let key = normalize_key(arg)?;
        let value = iter
            .next()
            .ok_or(CliError::InvalidArgument)
            .context(format!("missing value for `{key}`"))?;

        if value.starts_with("--") {
            return Err(CliError::InvalidArgument)
                .context(format!("invalid value `{value}` for `{key}`"));
        }

        insert_unique(&mut map, key, value.to_string())?;
    }

    Ok(map)
}

fn normalize_key(raw: &str) -> CliResult<String> {
    if !raw.starts_with("--") {
        Err(CliError::InvalidArgument)?;
    }

    Ok(raw.trim_start_matches("--").replace('-', "_"))
}

fn insert_unique(map: &mut HashMap<String, String>, key: String, value: String) -> CliResult<()> {
    if map.contains_key(&key) {
        return Err(CliError::InvalidArgument).context(format!("duplicate argument `{key}`"));
    }
    map.insert(key, value);
    Ok(())
}

/* ===========================
MAIN
=========================== */

fn main() -> CliResult<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() < 2 {
        return Err(CliError::InvalidArgument).context("usage: scarff <command> [options]");
    }

    let command = parse_args(&args)?;
    command.run()
}
