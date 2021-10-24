use crate::server::request::Request;

use super::{
    abstracts::RequestHandler, errors::TcpServerError, response::Response, SocketData,
    StreamHandlingError,
};
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
        let mut socket: SocketData = match listener.accept() {
            Ok(res) => res.into(),
            Err(e) => {
                println!("Error happened on accept connection: {}", e);
                continue;
            }
        };
        let handler = handler.clone();
        thread::spawn(move || {
            match handle(&mut socket, handler) {
                Ok(_) => println!("Request for socket: {} handled successfully", &socket),
                Err(e) => println!("Request for socket: {} handled with error: {}", &socket, e),
            };
        });
    }
}

fn handle(
    socket: &mut SocketData,
    handler: impl RequestHandler,
) -> Result<Response, StreamHandlingError> {
    let request = Request::try_from(socket)?;
    println!("Handling request: {}", &request);
    let result = handler.handle(request);
    println!("Request handled with result: {}", result);
    Ok(result)
}

fn bind(socket_address: &String) -> Result<TcpListener, TcpServerError> {
    match TcpListener::bind(socket_address) {
        Ok(listener) => Ok(listener),
        Err(e) => Err(TcpServerError(format!("Error during port binding: {}", e))),
    }
}
