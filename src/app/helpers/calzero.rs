use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::models::SimpleMessage;
use crate::app::helpers::display;


#[allow(dead_code)]
pub async fn command(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    lrn: u8
) -> Result<SimpleMessage<String>, AppError> {
    let command = format!("CA2 {};", lrn);

    stream.write_all(command.as_ref()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}
