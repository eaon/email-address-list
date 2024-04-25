use crate::parser::Rule;
use std::convert::From;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    PestRuleError(Box<pest::error::Error<Rule>>),
    UnexpectedError(String),
    Empty,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(s: pest::error::Error<Rule>) -> Error {
        Error::PestRuleError(Box::new(s))
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn invalid_nesting(rule: &str) -> Error {
    Error::UnexpectedError(format!("Invalid nesting in {} rule", rule))
}

pub(crate) fn invalid_empty(rule: &str) -> Error {
    Error::UnexpectedError(format!("{} cannot be empty", rule))
}
