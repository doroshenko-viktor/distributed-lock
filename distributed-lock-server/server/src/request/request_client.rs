use std::fmt::Display;

pub struct Client {
    ip: String,
}

impl Client {
    pub fn new(ip: String) -> Self {
        Client { ip }
    }

    fn repr(&self) -> String {
        self.ip.clone()
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr())
    }
}
