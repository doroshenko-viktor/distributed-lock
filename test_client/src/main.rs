use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let address = "127.0.0.1:9876";
    let mut stream = TcpStream::connect(address).unwrap();

    let command = "LOCK\n<LOCK_KEY>\n\n<META>!";
    let command_bytes = command.as_bytes();
    stream.write_all(&command_bytes).expect("error during send");
    println!("sent");
    let _ = stream.flush();
    println!("flushed");
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).expect("receive error");
    let response = std::str::from_utf8(&buf).expect("error during response parsing");
    println!("received: {}", response);
}
