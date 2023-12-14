//! Common utilities for Advent of Code

use thiserror::Error;
use std::num::ParseIntError;

#[derive(Clone, PartialEq, Debug, Error)]
/// The common error type for parsing.
pub enum ParseError {
    #[error("Failed to find {0} in input")]
    /// Parsing has failed to find
    /// a required part of the input format.
    InvalidFormat(&'static str),
    #[error("Failed to parse number")]
    /// A call to `String::parse` has failed.
    ExpectedNumber(#[from] ParseIntError),
}

pub type ParseResult<T> = Result<T, ParseError>;
