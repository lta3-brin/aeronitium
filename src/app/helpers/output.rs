use std::fs::File;
use csv::WriterBuilder;
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
) -> Result<(), AppError> {
    let cmd = format!("OP2 {} -{} {};", crs, stbl, sport);

    stream.write_all(cmd.as_bytes()).await?;
    stream.read(&mut buffer).await?;

    let array = display::display_message::<[u8; 2]>(buffer)?;

    build(stream, array).await?;

    Ok(())
}

async fn build(
    stream: &mut TcpStream,
    arr: SimpleMessage<[u8; 2]>
) -> Result<(), AppError> {
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

    {
        let file = File::create("calib_coeff.csv")?;
        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);

        for r in 0..row {
            writer.write_record(&array[r as usize][..]).unwrap();
        }
    }

    Ok(())
}
