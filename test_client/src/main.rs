use std::{io::Write, net::TcpStream};

fn main() {
    let address = "127.0.0.1:9876";
    let mut stream = TcpStream::connect(address).unwrap();

    let command = "LOCK\n<LOCK_KEY>\n\n<META>";
    let command_bytes = command.as_bytes();
    match stream.write_all(&command_bytes) {
        Ok(_) => {
            println!("Sent")
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    };
}
