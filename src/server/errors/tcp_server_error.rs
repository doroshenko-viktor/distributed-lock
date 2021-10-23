use std::error::Error;

#[derive(Debug)]
pub struct TcpServerError(pub String);

impl std::fmt::Display for TcpServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TcpServerError {}
