use std::{fmt::Display, io::Write};

#[derive(Debug)]
pub struct Response {
    body: String,
}

impl Response {
    pub fn new(body: String) -> Self {
        Response { body }
    }
}

impl Response {
    /// Get a reference to the response's body.
    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body())
    }
}
