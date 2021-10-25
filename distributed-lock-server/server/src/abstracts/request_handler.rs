use super::super::{errors::ApplicationError, request::Request, response::Response};

pub trait RequestHandler: Send + Sync + Sized + Clone {
    fn handle(&self, request: Request) -> Result<Response, ApplicationError>;
}
