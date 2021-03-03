use std::convert::TryInto;
use crate::app::models::SimpleMessage;
use crate::app::AppError;
use crate::app::decoders::{response_code, response_type};


pub trait ResponseData {
    fn display_data(data: [u8; 4]) -> Self;
}

impl ResponseData for i32 {
    fn display_data(data: [u8; 4]) -> Self {
        Self::from_be_bytes(data)
    }
}

impl ResponseData for f32 {
    fn display_data(data: [u8; 4]) -> Self {
        Self::from_be_bytes(data)
    }
}

pub fn display_message<U>(buffer: [u8; 8]) -> Result<SimpleMessage<U>, AppError>
    where U: ResponseData {
    let bytes: [u8; 4] = buffer[4..].try_into()?;
    let message = SimpleMessage::<U>::new(
        buffer[0],
        response_code::get(buffer[0]),
        buffer[1],
        response_type::get(buffer[1]),
        U::display_data(bytes)
    );

    Ok(message)
}
