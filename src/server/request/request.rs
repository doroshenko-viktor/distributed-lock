use std::{convert::TryFrom, fmt::Display};

use super::super::SocketData;
use super::{errors::RequestParseError, parser, Client, LockType, Metadata, Operation};

pub type Key = String;

pub struct Request {
    operation: Operation,
    key: Key,
    lock_type: LockType,
    client: Client,
    meta: Option<Metadata>,
}

impl Request {
    pub fn operation(&self) -> &Operation {
        &self.operation
    }
    pub fn key(&self) -> &Key {
        &self.key
    }
    pub fn lock_type(&self) -> &LockType {
        &self.lock_type
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub fn meta(&self) -> Option<&Metadata> {
        self.meta.as_ref()
    }
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

impl TryFrom<&mut SocketData> for Request {
    type Error = RequestParseError;

    fn try_from(value: &mut SocketData) -> Result<Self, Self::Error> {
        let body = parser::get_body(&mut value.stream)?;

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
            client: Client::new(value.client_addr.to_string()),
            lock_type: LockType::All,
            meta: None,
        })
    }
}
