use crate::server::request::Request;
use std::io::Write;

use super::{abstracts::RequestHandler, errors::TcpServerError, SocketData, StreamHandlingError};
use std::{convert::TryFrom, net::TcpListener, thread};

pub struct TcpServer {
    host: String,
    port: u16,
}

impl TcpServer {
    pub fn new(host: String, port: u16) -> Self {
        TcpServer { host, port }
    }

    pub fn run(&self, handler: impl RequestHandler + 'static) -> Result<(), TcpServerError> {
        let socket_addr = self.get_socket_address();
        let tcp_listener = bind(&socket_addr)?;
        println!("Created tcp listener on {}", socket_addr);
        serve(tcp_listener, handler);
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

fn serve(listener: TcpListener, handler: impl RequestHandler + 'static) {
    loop {
        let socket: SocketData = match listener.accept() {
            Ok(res) => res.into(),
            Err(e) => {
                println!("Error happened on accept connection: {}", e);
                continue;
            }
        };
        let handler = handler.clone();
        thread::spawn(move || {
            match handle(socket, handler) {
                Ok(_) => println!("Request handled successfully"),
                Err(e) => println!("Handled with error: {}", e),
            };
        });
    }
}

fn handle(mut socket: SocketData, handler: impl RequestHandler) -> Result<(), StreamHandlingError> {
    let request = Request::try_from(&mut socket)?;
    println!("Handling request: {}", &request);
    let response = handler.handle(request)?;
    println!("Request handled with result: {}", response);
    write!(&mut socket.stream, "{}", response)?;
    Ok(())
}

fn bind(socket_address: &String) -> Result<TcpListener, TcpServerError> {
    match TcpListener::bind(socket_address) {
        Ok(listener) => Ok(listener),
        Err(e) => Err(TcpServerError(format!("Error during port binding: {}", e))),
    }
}
