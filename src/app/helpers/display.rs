use std::convert::TryInto;
use crate::app::models::SimpleMessage;
use crate::app::AppError;
use crate::app::decoders::{response_code, response_type, response_error};


pub trait ResponseData {
    fn display_data(data: [u8; 4], code: u8, kind: u8) -> Self;
}

impl ResponseData for String {
    fn display_data(data: [u8; 4], code: u8, kind: u8) -> Self {
        if kind == 4 {
            String::from("Ok")
        } else if kind == 8 {
            format!("{}", i32::from_be_bytes(data))
        } else if kind == 9 && code == 151 {
            let s;
            let r;
            let b;

            let bin = format!("{:b}", f32::from_be_bytes(data) as i32);

            if bin.chars().nth(0) == Some('1') { s = String::from("DTC Scanner attached"); }
            else { s = String::from("No DTC Scanner attached"); }

            if bin.chars().nth(1) == Some('1') { r = String::from("Valve at RUN position"); }
            else { r = String::from("Valve at CAL position"); }

            if bin.chars().nth(7) == Some('1') { b = String::from("Sensitive range"); }
            else { b = String::from("Sensitive normal"); }

            format!("{} - {} - {}", b, r, s)
        } else if kind == 9 {
            format!("v{}", f32::from_be_bytes(data))
        } else if kind == 33 {
            println!("{:?}", data);
            format!("v{}", 33)
        } else if kind == 128 {
            let err = i32::from_be_bytes(data);

            response_error::get(err)
        } else {
            String::from("Unknown response type")
        }
    }
}

pub fn display_message<U>(buffer: [u8; 8]) -> Result<SimpleMessage<U>, AppError>
    where U: ResponseData {
    let code: u8 = buffer[0];
    let kind: u8 = buffer[1];

    let bytes: [u8; 4] = buffer[4..].try_into()?;
    let message = SimpleMessage::<U>::new(
        code,
        response_code::get(code),
        kind,
        response_type::get(kind),
        U::display_data(bytes, code, kind)
    );

    Ok(message)
}
