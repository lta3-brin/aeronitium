use tokio::net::TcpStream;
use crate::app::models::SimpleMessage;
use crate::app::AppError;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::helpers::display;


pub async fn daq(
    stream: &mut TcpStream,
    stbl: u8
) -> Result<(), AppError> {
    let mut buff4 = [0u8; 4];
    let mut buff8 = [0u8; 8];
    let mut buff16 = [0u8; 16];

    let cmd = format!("AD2 {};", stbl);
    stream.write_all(cmd.as_bytes()).await?;

    let mut response_type = 0u8;
    let mut step = 0i32;
    let mut id = 0i32;
    let mut sensor: Vec<f32> = vec![];
    for i in 0..264i32 {
        if i == 0 || i % (step + 2) == 0 {
            stream.read(&mut buff8).await?;
            response_type = buff8[1];
            step = buff8[7] as i32;
            sensor = vec![];
        } else if i % (step + 2) == 1 {
            stream.read(&mut buff16).await?;
            id = buff16[15] as i32;
            sensor = vec![];
        } else {
            if response_type == 19 {
                stream.read(&mut buff4).await?;
                sensor.push(f32::from_be_bytes(buff4));
            } else {
                let m = stop(stream, buff8).await?;
                println!("resp: {:?}", m)
            }
        }

        if sensor.len() == step as usize {
            println!("{} {:?}", id, sensor);
        }

        if i == 264 {
            let m = stop(stream, buff8).await?;
            println!("finish: {:?}", m);
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn stop(
    stream: &mut TcpStream,
    mut buffer: [u8; 8]
) -> Result<SimpleMessage<String>, AppError> {
    let cmd = b"AD0;";

    stream.write_all(cmd).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}
