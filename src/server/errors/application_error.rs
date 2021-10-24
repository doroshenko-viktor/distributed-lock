use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ApplicationError {
    pub message: String,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for ApplicationError {}
