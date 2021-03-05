use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::helpers::display;
use crate::app::models::SimpleMessage;


#[allow(dead_code)]
pub async fn check_firmware(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    crs: &str
) -> Result<SimpleMessage<String>, AppError> {
    let cmd = format!("LA4 {};", crs);

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}

#[allow(dead_code)]
pub async fn lookat(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    crs: &str,
    scnaddr: u8
) -> Result<SimpleMessage<String>, AppError> {
    let cmd = format!("LA1 {} -{}97;", crs, scnaddr);

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}
