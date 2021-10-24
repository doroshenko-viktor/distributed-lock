use super::super::{request::Request, response::Response};

pub trait RequestHandler: Send+Sync+Sized+Clone {
    fn handle(&self, request: Request) -> Response;
}
