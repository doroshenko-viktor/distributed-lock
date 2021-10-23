use std::{error::Error};

use crate::server::request::errors::RequestParseError;

#[derive(Debug)]
pub enum StreamHandlingError {
    ParsingError(String),
    ApplicationError(String),
}

impl StreamHandlingError {
    fn repr(&self) -> String {
        match self {
            StreamHandlingError::ParsingError(e) => format!("Parsing error: {}", e),
            StreamHandlingError::ApplicationError(e) => format!("Application error: {}", e),
        }
    }
}

impl std::fmt::Display for StreamHandlingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}

impl Error for StreamHandlingError {}

impl From<RequestParseError> for StreamHandlingError {
    fn from(e: RequestParseError) -> Self {
        match e {
            RequestParseError::InvalidOperation(_) => {
                StreamHandlingError::ParsingError(e.to_string())
            }
            RequestParseError::InvalidEncoding => StreamHandlingError::ParsingError(e.to_string()),
        }
    }
}
