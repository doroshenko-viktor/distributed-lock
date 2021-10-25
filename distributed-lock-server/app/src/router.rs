use server::{
    request::{Operation, Request},
    response::Response,
};
use super::controllers::lock_controller;

pub fn route(request: Request) -> Response {
    match request.operation() {
        Operation::Lock => lock_controller::lock(&request),
        Operation::Release => lock_controller::release(&request),
    }
}
