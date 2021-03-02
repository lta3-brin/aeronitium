use std::convert::TryInto;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::models::SimpleMessage;
use crate::app::decoders::{response_code, response_type};
// use crate::app::decoders::{response_code, response_type};

pub async fn command() -> Result<SimpleMessage<i32>, AppError> {
    let mut stream = TcpStream::connect("192.168.129.119:8400").await?;
    let command = b"CA2;";
    stream.write_all(command).await?;

    let mut buffer = [0; 8];
    stream.read(&mut buffer).await?;
    let resp = &buffer;
    let bytes: [u8; 4] = resp[4..].try_into()?;

    let message = SimpleMessage::<i32>::new(
        resp[0],
        response_code::get(resp[0]),
        resp[1],
        response_type::get(resp[1]),
        i32::from_be_bytes(bytes)
    );

    Ok(message)
}
