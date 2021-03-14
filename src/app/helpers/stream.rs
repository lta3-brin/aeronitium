use std::sync::{Arc};
use tokio::{
    spawn,
    net::TcpStream,
    sync::{Mutex, mpsc},
    io::{AsyncWriteExt, AsyncReadExt}
};
use crate::app::models::SimpleMessage;
use crate::app::AppError;
use crate::app::helpers::display;


pub async fn daq(stream: Arc<Mutex<TcpStream>>, stbl: u8) {
    let (tx, mut rx) = mpsc::channel(32);
    let stream = Arc::clone(&stream);

    // Sending task to thread
    spawn(async move {
        get_data(stream, stbl, tx).await;
    });

    while let Some(sensor) = rx.recv().await {
        println!("{}", sensor);
    }
}

async fn get_data(
    stream: Arc<Mutex<TcpStream>>,
    stbl: u8,
    transmit: mpsc::Sender<String>
) {
    let mut buff4 = [0u8; 4];
    let mut buff8 = [0u8; 8];
    let mut buff16 = [0u8; 16];

    let mut stream_lock = stream.lock().await;
    let mut stream = &mut *stream_lock;

    let cmd = format!("AD2 {};", stbl);
    stream.write_all(cmd.as_bytes()).await.unwrap();

    let mut response_type = 0u8;
    let mut step = 0u8;
    let mut id = 0u8;
    let mut sensor: Vec<f32> = vec![];

    let mut i = 0u8;
    loop {
        if i == 0 || i % (step + 2) == 0 {
            stream.read(&mut buff8).await.unwrap();
            response_type = buff8[1];
            step = buff8[7];
            sensor = vec![];
        } else if i % (step + 2) == 1 {
            stream.read(&mut buff16).await.unwrap();
            id = buff16[15];
            sensor = vec![];
        } else {
            if response_type == 19 {
                stream.read(&mut buff4).await.unwrap();
                sensor.push(f32::from_be_bytes(buff4));
            } else {
                let m = stop(&mut stream, buff8).await.unwrap();
                println!("resp: {:?}", m)
            }
        }

        if sensor.len() == step as usize {
            let mut coll = vec![];
            coll.push(id.to_string());

            for sen in &sensor {
                coll.push(sen.to_string())
            }

            let dt = coll.join(", ");

            transmit.send(dt).await.unwrap();
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
