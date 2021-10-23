use std::process;

use server::TcpServer;

pub mod server;

fn main() {
    let address = "127.0.0.1".to_string();
    let port: u16 = 9876;
    let server = TcpServer::new(address, port);
    match server.run() {
        Ok(_) => {
            println!("Server finished its work");
            process::exit(0);
        }
        Err(e) => {
            println!("Error during server runtime: {}", e);
            process::exit(-1);
        }
    };
}
