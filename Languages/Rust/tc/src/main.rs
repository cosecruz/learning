use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum TCError {
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

#[derive(Debug)]
enum Command<'a> {
    Parse(&'a [&'a str]),
}

struct Cli<'a> {
    command: Command<'a>,
}

/* =======================
COMMAND DISPATCH
======================= */

impl<'a> Command<'a> {
    fn run(&self) -> Result<(), TCError> {
        match self {
            Command::Parse(args) => Parse::run(args),
        }
    }
}

/* =======================
PARSE COMMAND
======================= */

struct Parse;

#[derive(Debug, Clone, Copy)]
enum OutputMode {
    Normal,
    Verbose,
    Quiet,
}

impl Parse {
    fn run(args: &[&str]) -> Result<(), TCError> {
        let parsed = Self::parse(args)?;
        Self::require_flags(&parsed)?;
        let mode = Self::output_mode(&parsed)?;

        match mode {
            OutputMode::Quiet => {
                // no output
            }
            OutputMode::Normal => {
                println!("Parsed successfully");
            }
            OutputMode::Verbose => {
                println!("Parsed arguments:");
                for (k, v) in &parsed {
                    println!("  {k:?} => {v:?}");
                }
            }
        }

        Ok(())
    }

    fn parse(args: &[&str]) -> Result<HashMap<String, Vec<Option<String>>>, TCError> {
        let mut map: HashMap<String, Vec<Option<String>>> = HashMap::new();

        for arg in args {
            let body = arg
                .strip_prefix("--")
                .ok_or_else(|| TCError::ParseError(format!("invalid flag: {arg}")))?;

            let (key, value) = match body.split_once('=') {
                Some((k, v)) => (k, Some(v.to_string())),
                None => (body, None),
            };

            let key = key.to_ascii_lowercase();

            map.entry(key).or_default().push(value);
        }

        Self::check_conflicts(&map)?;
        Ok(map)
    }

    fn output_mode(parsed: &HashMap<String, Vec<Option<String>>>) -> Result<OutputMode, TCError> {
        let verbose = parsed.contains_key("verbose");
        let quiet = parsed.contains_key("quiet");

        match (verbose, quiet) {
            (true, true) => Err(TCError::ParseError(
                "--verbose and --quiet cannot be used together".into(),
            )),
            (true, false) => Ok(OutputMode::Verbose),
            (false, true) => Ok(OutputMode::Quiet),
            (false, false) => Ok(OutputMode::Normal),
        }
    }

    fn check_conflicts(parsed: &HashMap<String, Vec<Option<String>>>) -> Result<(), TCError> {
        if parsed.contains_key("verbose") && parsed.contains_key("quiet") {
            return Err(TCError::ParseError(
                "--verbose and --quiet cannot be used together".into(),
            ));
        }
        Ok(())
    }

    fn require_flags(parsed: &HashMap<String, Vec<Option<String>>>) -> Result<(), TCError> {
        if parsed.is_empty() {
            return Err(TCError::ParseError(
                "parse requires at least one flag".into(),
            ));
        }
        Ok(())
    }
}

/* =======================
CLI PARSING
======================= */

impl<'a> Cli<'a> {
    fn cmd(args: &'a [&'a str]) -> Result<Self, TCError> {
        let command = args.first().ok_or(TCError::CliError)?;

        match *command {
            "parse" => Ok(Self {
                command: Command::Parse(&args[1..]),
            }),
            _ => Err(TCError::CliError),
        }
    }
}

/* =======================
MAIN
======================= */

fn main() -> Result<(), TCError> {
    let args: Vec<String> = std::env::args().collect();
    let args_str: Vec<&str> = args.iter().map(String::as_str).collect();

    if args_str.len() < 2 {
        return Err(TCError::CliError);
    }

    let cli = Cli::cmd(&args_str[1..])?;
    cli.command.run()
}
