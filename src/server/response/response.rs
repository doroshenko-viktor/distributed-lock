#[derive(Debug)]
pub struct Response {
    body: String,
}

impl Response {
    /// Get a reference to the response's body.
    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
