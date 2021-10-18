use std::{
    error::Error,
    fmt::{Debug, Display},
    str::Utf8Error,
};

use crate::request::errors::OperationParseError;

// use super::OperationParseError;

pub enum RequestParseError {
    InvalidOperation(String),
    InvalidEncoding,
}

impl RequestParseError {
    fn repr(&self) -> String {
        match self {
            Self::InvalidOperation(m) => format!("Invalid Operation {}", m),
            Self::InvalidEncoding => format!("Request encoding is invalid"),
        }
    }
}

impl Display for RequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}

impl Debug for RequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}

impl Error for RequestParseError {}

impl From<OperationParseError> for RequestParseError {
    fn from(v: OperationParseError) -> Self {
        RequestParseError::InvalidOperation(v.0)
    }
}

impl From<Utf8Error> for RequestParseError {
    fn from(_: Utf8Error) -> Self {
        RequestParseError::InvalidEncoding
    }
}
