use std::fs::File;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use crate::app::models::SimpleMessage;
use crate::app::AppError;
use csv::WriterBuilder;


pub async fn build(
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
