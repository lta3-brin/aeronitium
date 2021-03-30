use chrono::Local;
use async_nats::Connection;
use std::io::{stdout, Write};
use tokio::{
    net::TcpStream,
    io::{AsyncWriteExt, AsyncReadExt}
};
use crate::app::AppError;
use crate::app::helpers::display;
use crate::app::models::SimpleMessage;
use crate::app::decoders::response_type::get;


#[allow(dead_code)]
pub async fn start(
    stream: &mut TcpStream,
    stbl: u8,
    nats: &mut Connection
) {
    let mut buff4 = [0u8; 4];
    let mut buff8 = [0u8; 8];
    let mut buff16 = [0u8; 16];

    let mut response_type = 0u8;
    let mut step = 0u8;
    let mut sensor: Vec<f32> = vec![];

    let mut i = 0u8;
    let cmd = format!("AD2 {};", stbl);
    stream.write_all(cmd.as_bytes()).await.unwrap();

    loop {
        if i == 0 || i % (step + 2) == 0 {
            stream.read(&mut buff8).await.unwrap();
            response_type = buff8[1];
            step = buff8[7];
            sensor = vec![];

            if response_type == 128 { println!("{}", get(response_type)) }
        } else if i % (step + 2) == 1 {
            stream.read(&mut buff16).await.unwrap();
            sensor = vec![];
        } else {
            if response_type == 19 {
                stream.read(&mut buff4).await.unwrap();
                sensor.push(f32::from_be_bytes(buff4));
            } else {
                let m = stop(stream, buff8).await.unwrap();
                println!("resp: {:?}", m)
            }
        }

        if sensor.len() == step as usize {
            let lokal = Local::now();
            let mut coll = vec![];
            coll.push(
                format!("{}", lokal.format("%Y-%m-%d %H:%M:%S%.6f"))
            );

            for sen in &sensor {
                coll.push(sen.to_string())
            }

            nats.publish("dtc", coll.join(",")).await.unwrap();
            print!("\r{:?}  ", coll);
            stdout().flush().unwrap();
        }

        i += 1;

        if i == step + 2 {
            i = 0;
        }
    }
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
