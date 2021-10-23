use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub struct SocketData {
    pub stream: TcpStream,
    pub client_addr: SocketAddr,
}

impl std::fmt::Display for SocketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "client: {}", self.client_addr)
    }
}

impl SocketData {
    pub fn new(d: (TcpStream, SocketAddr)) -> Self {
        SocketData {
            stream: d.0,
            client_addr: d.1,
        }
    }
}

impl From<(TcpStream, SocketAddr)> for SocketData {
    fn from(d: (TcpStream, SocketAddr)) -> Self {
        SocketData {
            stream: d.0,
            client_addr: d.1,
        }
    }
}
