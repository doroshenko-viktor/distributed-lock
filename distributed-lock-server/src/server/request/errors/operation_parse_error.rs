use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct OperationParseError(pub String);

impl Display for OperationParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for OperationParseError {}
