use std::{
    io::Read,
    net::TcpStream,
    str::{self, Utf8Error},
};

pub fn get_line(text: &str) -> (&str, &str) {
    for (index, c) in text.chars().enumerate() {
        if c == '\n' || c == '\r' {
            return (&text[..index], &text[index + 1..]);
        }
    }
    (&text[..], &text[..0])
}

pub fn get_body(stream: &mut TcpStream) -> Result<String, Utf8Error> {
    let mut body = Vec::<u8>::with_capacity(1024);
    let mut buf = [0u8; 1024];
    loop {
        let position = match stream.read(&mut buf) {
            Ok(position) if position == 0 => break,
            Err(_) => break,
            Ok(position) => position,
        };
        body.extend_from_slice(&buf[..position]);
    }
    let s = str::from_utf8(&body)?.to_string();
    println!("Request body parsed: {}", s);
    Ok(s)
}
