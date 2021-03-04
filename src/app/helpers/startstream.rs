use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::models::SimpleMessage;
use crate::app::helpers::display;


#[allow(dead_code)]
pub async fn command(setup_table: u8) -> Result<SimpleMessage<String>, AppError> {
    let mut stream = TcpStream::connect("192.168.129.119:8400").await?;
    let cmd = format!("AD2 {};", setup_table);
    let mut buffer = [0; 8];

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}
