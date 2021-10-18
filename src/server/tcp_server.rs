use std::{convert::TryFrom, error::Error, net::TcpListener};

use crate::request::Request;

#[derive(Debug)]
pub struct TcpServerError(pub String);

impl std::fmt::Display for TcpServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for TcpServerError {}

pub struct TcpServer {
    host: String,
    port: u16,
}

impl TcpServer {
    pub fn new(host: String, port: u16) -> Self {
        TcpServer { host, port }
    }

    pub fn run(&self) -> Result<(), TcpServerError> {
        let socket_addr = self.get_socket_address();
        let tcp_listener = bind(&socket_addr)?;
        println!("Created tcp listener on {}", socket_addr);
        serve(tcp_listener);
        Ok(())
    }

    fn get_socket_address(&self) -> String {
        format!(
            "{host}:{port}",
            host = self.host,
            port = self.port.to_string()
        )
    }
}

fn serve(listener: TcpListener) {
    loop {
        let (mut stream, addr) = match listener.accept() {
            Ok(res) => res,
            Err(e) => {
                println!("Error happened on accept connection: {}", e);
                continue;
            }
        };
        let request = match Request::try_from((&mut stream, &addr)) {
            Ok(r) => r,
            Err(e) => {
                println!("Error during handling request: {}", e);
                continue;
            }
        };
        println!("Handling request: {}", request);
    }
}

fn bind(socket_address: &String) -> Result<TcpListener, TcpServerError> {
    match TcpListener::bind(socket_address) {
        Ok(listener) => Ok(listener),
        Err(e) => Err(TcpServerError(format!("Error during port binding: {}", e))),
    }
}
