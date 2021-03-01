use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::error::AppError;

mod app;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Connecting to Initium");

    let mut stream = TcpStream::connect("192.168.129.119:8400").await?;
    let command = b"LA4 111;";
    stream.write_all(command).await?;

    let mut buffer = [0; 8];
    stream.read(&mut buffer).await?;

    println!("The firmware of the DTC Initium: {:?}", buffer);

    Ok(())
}
