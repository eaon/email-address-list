use std::convert::From;
use std::error;
use std::fmt;
use Rule;

#[derive(Debug)]
pub enum Error {
    PestRuleError(pest::error::Error<Rule>),
    UnexpectedError(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnexpectedError(ref e) => e,
            _ => self.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            _ => Some(self),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(s: pest::error::Error<Rule>) -> Error {
        Error::PestRuleError(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
