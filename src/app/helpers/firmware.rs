use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::decoders::response_code;

pub async fn get_version() -> Result<(), AppError> {
    let mut stream = TcpStream::connect("192.168.129.119:8400").await?;
    let command = b"LA4 111;";
    stream.write_all(command).await?;

    let mut buffer = [0; 8];
    stream.read(&mut buffer).await?;

    let resp = response_code::get(buffer[0]);

    println!("The firmware of the DTC Initium: {:?}", resp);

    Ok(())
}
