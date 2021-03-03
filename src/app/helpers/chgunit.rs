use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::models::SimpleMessage;
use crate::app::helpers::display;


#[allow(dead_code)]
pub async fn command(satuan: u8) -> Result<SimpleMessage<i32>, AppError> {
    let mut stream = TcpStream::connect("192.168.129.119:8400").await?;
    let cmd = format!("PC4 1 {};", satuan);
    let mut buffer = [0; 8];

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<i32>(buffer)?;

    Ok(message)
}
