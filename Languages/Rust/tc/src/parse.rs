use std::collections::HashMap;

use crate::error::TCError;

pub struct Parse;

#[derive(Debug, Clone, Copy)]
enum OutputMode {
    Normal,
    Verbose,
    Quiet,
}

impl Parse {
    pub fn run(args: &[&str]) -> Result<(), TCError> {
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
