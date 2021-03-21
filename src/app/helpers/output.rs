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
) -> Result<Vec<String>, AppError> {
    let cmd = format!("OP2 {} -{} {};", crs, stbl, sport);

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let array = display::display_message::<[u8; 2]>(buffer)?;

    let result = build(stream, array).await?;

    Ok(result)
}

async fn build(
    stream: &mut TcpStream,
    arr: SimpleMessage<[u8; 2]>
) -> Result<Vec<String>, AppError> {
    let mut data = Vec::<String>::new();
    let mut buff = [0u8; 4];
    let row: u8 = arr.get_data()[0];
    let col: u8 = arr.get_data()[1];
    let mut array = vec![
        vec![String::new(); col as usize];
        row as usize
    ];

    for r in 0..row {
        for c in 0..col {
            stream.read(&mut buff).await?;

            array[r as usize][c as usize] = format!("{}", f32::from_be_bytes(buff))
        }
    }

    for r in 0..row {
        let dt = &array[r as usize][..];

        data.push(dt.join(","));
    }

    Ok(data)
}
