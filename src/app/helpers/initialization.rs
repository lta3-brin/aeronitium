use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use crate::app::AppError;
use crate::app::helpers::display;
use crate::app::models::SimpleMessage;


#[allow(dead_code)]
pub async fn check(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    crs: &str,
    scnaddr: u8,
    numchan: u8,
    lrn: u8
) -> Result<SimpleMessage<String>, AppError> {
    let cmd = format!(
        "SD1 {} ({}, {}, {});", crs, scnaddr, numchan, lrn
    );

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}

#[allow(dead_code)]
pub async fn define(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    crs: &str,
    stbl: u8,
    nfr: u8,
    frd: u8,
    nms: u16,
    msd: u16,
    trm: &str,
    scm: &str,
    ocf: u8
) -> Result<SimpleMessage<String>, AppError> {
    let cmd = format!(
        "SD2 {} {} ({}, {}) ({}, {}) ({}, {}) {};",
        crs, stbl, nfr, frd, nms, msd, trm, scm, ocf
    );

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}

#[allow(dead_code)]
pub async fn scan(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    crs: &str,
    stbl: u8,
    sport: &str
) -> Result<SimpleMessage<String>, AppError> {
    let cmd = format!(
        "SD3 {} {} {};", crs, stbl, sport
    );

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}

#[allow(dead_code)]
pub async fn load_store(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    crs: &str,
    stbl: i8,
    actx: u8
) -> Result<SimpleMessage<String>, AppError> {
    let cmd = format!(
        "SD5 {} {} {};", crs, stbl, actx
    );

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}

#[allow(dead_code)]
pub async fn change_unit(
    stream: &mut TcpStream,
    mut buffer: [u8; 8],
    lrn: u8,
    unx: u8
) -> Result<SimpleMessage<String>, AppError> {
    let cmd = format!("PC4 {} {};", lrn, unx);

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let message = display::display_message::<String>(buffer)?;

    Ok(message)
}
