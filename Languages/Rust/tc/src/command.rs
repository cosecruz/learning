use crate::error::TCError;
use crate::parse::Parse;

#[derive(Debug)]
pub enum Command<'a> {
    Parse(&'a [&'a str]),
}

impl<'a> Command<'a> {
    pub fn run(&self) -> Result<(), TCError> {
        match self {
            Command::Parse(args) => Parse::run(args),
        }
    }
}
