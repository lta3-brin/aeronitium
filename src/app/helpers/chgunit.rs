use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;


pub async fn command() -> Result<(), AppError> {
    let mut stream = TcpStream::connect("192.168.129.119:8400").await?;
    let command = b"PC4 1 3;";  // Ubah ke satuan Pa.
    stream.write_all(command).await?;

    let mut buffer = [0; 8];
    stream.read(&mut buffer).await?;

    println!("{:?}", buffer);

    Ok(())
}
