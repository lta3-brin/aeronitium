use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::models::SimpleMessage;
use crate::app::helpers::display;


#[allow(dead_code)]
pub async fn command(
    stream: &mut TcpStream,
    mut buffer: [u8; 8]
) -> Result<SimpleMessage<String>, AppError> {
    let command = b"CA2;";

    stream.write_all(command).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}
