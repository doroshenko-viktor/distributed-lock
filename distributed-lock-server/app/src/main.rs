mod application_request_handler;
mod controllers;
mod router;
use server::TcpServer;
use std::process;
use application_request_handler::ApplicationRequestHandler;

fn main() {
    let address = "127.0.0.1".to_string();
    let port: u16 = 9876;
    let server = TcpServer::new(address, port);
    let request_handler = ApplicationRequestHandler::new();
    match server.run(request_handler) {
        Ok(_) => {
            println!("Server shutting down...");
            process::exit(0);
        }
        Err(e) => {
            println!("Fatal error during server runtime: {}", e);
            process::exit(-1);
        }
    };
}
