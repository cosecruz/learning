use crate::command::Command;
use crate::error::TCError;

pub struct Cli<'a> {
    pub command: Command<'a>,
}

impl<'a> Cli<'a> {
    pub fn cmd(args: &'a [&'a str]) -> Result<Self, TCError> {
        let command = args.first().ok_or(TCError::CliError)?;

        match *command {
            "parse" => Ok(Self {
                command: Command::Parse(&args[1..]),
            }),
            _ => Err(TCError::CliError),
        }
    }
}
