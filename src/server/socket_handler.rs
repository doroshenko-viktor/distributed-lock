use super::{request::Request, SocketData, StreamHandlingError};
use std::convert::TryFrom;

pub fn handle(socket: &mut SocketData) -> Result<(), StreamHandlingError> {
    let request = Request::try_from(socket)?;
    println!("Handling request: {}", &request);
    Ok(())
}
