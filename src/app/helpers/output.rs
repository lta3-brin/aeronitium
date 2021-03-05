use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::helpers::display;
use crate::app::models::SimpleMessage;


#[allow(dead_code)]
pub async fn tabel_coef(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    crs: &str,
    stbl: u8,
    sport: &str
) -> Result<SimpleMessage<[u8; 2]>, AppError> {
    let cmd = format!("OP2 {} -{} {};", crs, stbl, sport);

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<[u8; 2]>(buffer)?;

    Ok(message)
}
