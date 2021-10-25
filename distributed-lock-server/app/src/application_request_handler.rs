use std::{thread, time::Duration};

use super::router;
use server::{
    abstracts::RequestHandler, errors::ApplicationError, request::Request, response::Response,
};

pub struct ApplicationRequestHandler {}

impl ApplicationRequestHandler {
    pub fn new() -> Self {
        ApplicationRequestHandler {}
    }
}

impl RequestHandler for ApplicationRequestHandler {
    fn handle(&self, request: Request) -> Result<Response, ApplicationError> {
        // router::route(request)
        println!("Request: {} started", request);
        thread::sleep(Duration::from_secs(1));
        println!("Request: {} finished", request);
        Ok(Response::new("response body".to_string()))
    }
}

impl Clone for ApplicationRequestHandler {
    fn clone(&self) -> Self {
        Self::new()
    }
}
