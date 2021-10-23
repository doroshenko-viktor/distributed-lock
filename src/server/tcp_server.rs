use std::{net::TcpListener, thread};

use super::SocketData;
use super::{errors::TcpServerError, socket_handler};

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
        let mut socket: SocketData = match listener.accept() {
            Ok(res) => res.into(),
            Err(e) => {
                println!("Error happened on accept connection: {}", e);
                continue;
            }
        };
        thread::spawn(move || {
            match socket_handler::handle(&mut socket) {
                Ok(_) => println!("Request for socket: {} handled successfully", &socket),
                Err(e) => println!("Request for socket: {} handled with error: {}", &socket, e),
            };
        });
    }
}

fn bind(socket_address: &String) -> Result<TcpListener, TcpServerError> {
    match TcpListener::bind(socket_address) {
        Ok(listener) => Ok(listener),
        Err(e) => Err(TcpServerError(format!("Error during port binding: {}", e))),
    }
}
