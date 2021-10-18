use std::{
    convert::TryFrom,
    fmt::Display,
    net::{SocketAddr, TcpStream},
};

use super::{errors::RequestParseError, parser, Client, LockType, Metadata, Operation};

pub struct Request {
    operation: Operation,
    key: String,
    lock_type: LockType,
    client: Client,
    meta: Option<Metadata>,
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "operation: {}; key: {}; client: {}",
            self.operation, self.key, self.client
        )
    }
}

impl TryFrom<(&mut TcpStream, &SocketAddr)> for Request {
    type Error = RequestParseError;

    fn try_from(value: (&mut TcpStream, &SocketAddr)) -> Result<Self, Self::Error> {
        let body = parser::get_body(value.0)?;

        let (operation, body) = parser::get_line(&body);
        let (lock_key, body) = parser::get_line(&body);
        let _ = parser::get_line(&body);
        // let meta_lines = {
        //     let mut lines = Vec::<&str>::new();
        //     loop {
        //         let (line, body) = get_line(&body);
        //         lines.push(line);
        //         if body.len() <= 0 {
        //             break;
        //         }
        //     }
        //     lines
        // };

        let req_operation: Operation = operation.parse()?;
        Ok(Request {
            operation: req_operation,
            key: lock_key.to_string(),
            client: Client::new(value.1.to_string()),
            lock_type: LockType::All,
            meta: None,
        })
    }
}
