use std::{thread, time::Duration};

use super::router;
use crate::server::{abstracts::RequestHandler, request::Request, response::Response};

pub struct ApplicationRequestHandler {}

impl ApplicationRequestHandler {
    pub fn new() -> Self {
        ApplicationRequestHandler {}
    }
}

impl RequestHandler for ApplicationRequestHandler {
    fn handle(&self, request: Request) -> Response {
        // router::route(request)
        println!("Request: {} started", request);
        thread::sleep(Duration::from_secs(10));
        println!("Request: {} finished", request);
        Response {
            body: "response body".to_string(),
        }
    }
}

impl Clone for ApplicationRequestHandler {
    fn clone(&self) -> Self {
        Self::new()
    }
}
