use std::error::Error;
use std::fs;
use std::str::FromStr;

use anyhow::{Context, Result};
use thiserror::Error;

pub fn read_number_from_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid format in {path}: {msg}")]
    InvalidFormat { path: String, msg: String },

    #[error(
        "Value {actual} out of range [{min}, {max}] in {path}\nHint: adjust your input or fix the file"
    )]
    OutOfRange {
        path: String,
        min: i32,
        max: i32,
        actual: i32,
    },

    #[error("IO error while reading {path}: {source}")]
    IoWithPath {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

//manual way of writing this to make it better you use rust associated macros and thiserror then no need to write this;
// impl Display for ParseError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
//             ParseError::OutOfRange { min, max, actual } => {
//                 write!(f, "Value {} out of range [{}, {}]", actual, min, max)
//             }
//             ParseError::IoError((e)) => write!(f, "IO error: {}", e),
//         }
//     }
// }

// impl Error for ParseError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match self {
//             ParseError::IoError(e) => Some(e),
//             _ => None,
//         }
//     }
// }

// impl From<std::io::Error> for ParseError {
//     fn from(error: std::io::Error) -> Self {
//         ParseError::IoError(error)
//     }
// }

// pub fn read_and_parse_number(path: &str) -> Result<i32, ParseError> {
//     let contents =
//         fs::read_to_string(path).with_context(|| format!("Failed to read file: {path}"))?; // auto-converts io::Error

//     let trimmed = contents.trim();
//     if trimmed.is_empty() {
//         return Err(ParseError::InvalidFormat("Empty file".to_string()));
//     }

//     let num: i32 = trimmed
//         .parse()
//         .map_err(|_| ParseError::InvalidFormat("Not a valid integer".to_string()))?;

//     if !(0..=100).contains(&num) {
//         return Err(ParseError::OutOfRange {
//             min: 0,
//             max: 100,
//             actual: num,
//         });
//     }

//     Ok(num)
// }

// pub fn read_and_parse_number(path: &str) -> Result<i32, ParseError> {
//     let contents = fs::read_to_string(path).map_err(|e| ParseError::IoWithPath {
//         path: path.to_string(),
//         source: e,
//     })?;

//     let trimmed = contents.trim();
//     if trimmed.is_empty() {
//         return Err(ParseError::InvalidFormat {
//             path: path.to_string(),
//             msg: "Empty file".into(),
//         });
//     }

//     let num: i32 = trimmed.parse().map_err(|_| ParseError::InvalidFormat {
//         path: path.to_string(),
//         msg: "Not a valid integer".into(),
//     })?;

//     if !(0..=100).contains(&num) {
//         return Err(ParseError::OutOfRange {
//             path: path.to_string(),
//             min: 0,
//             max: 100,
//             actual: num,
//         });
//     }

//     Ok(num)
// }

pub fn read_and_parse_number(path: &str) -> Result<i32> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("Failed to read file: {path}"))?;

    let trimmed = contents.trim();
    if trimmed.is_empty() {
        anyhow::bail!("Empty file: {}", path);
    }

    let num: i32 = trimmed
        .parse()
        .with_context(|| format!("Not a valid integer in file: {path}"))?;

    if !(0..=100).contains(&num) {
        anyhow::bail!("Value {} out of range [0, 100] in {}", num, path);
    }

    Ok(num)
}

// pub fn read_and_parse_range<T>(path: &str, min: T, max: T) -> Result<T>
// where
//     T: FromStr + PartialOrd + ToString + Copy,
// {
//     let contents = std::fs::read_to_string(path)?;

//     let trimmed = contents.trim();

//     let value: T = trimmed
//         .parse()
//         .map_err(|_| ParseError::InvalidFormat(trimmed.into()))?;

//     if value < min || value > max {
//         return Err(ParseError::OutOfRange {
//             min: min.to_string(),
//             max: max.to_string(),
//             actual: value.to_string(),
//         });
//     }

//     Ok(value)
// }
