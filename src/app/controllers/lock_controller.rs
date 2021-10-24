use crate::server::{request::Request, response::Response};

pub fn lock(request: &Request) -> Response {
    Response::new("response body".to_string())
}

pub fn release(request: &Request) -> Response {
    Response::new("response body".to_string())
}

pub fn get_lock_status(request: &Request) -> Response {
    Response::new("response body".to_string())
}
