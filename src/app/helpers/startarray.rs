use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use crate::app::models::SimpleMessage;
use crate::app::AppError;


pub async fn build(
    stream: &mut TcpStream,
    arr: SimpleMessage<[u8; 2]>
) -> Result<(), AppError> {
    let mut buff = [0u8; 4];
    let row: u8 = arr.get_data()[0];
    let col: u8 = arr.get_data()[1];

    for r in 0..row {
        for c in 0..col {
            stream.read(&mut buff).await?;
            println!("[{},{}] {:?}", r, c, f32::from_be_bytes(buff))
        }
    }

    Ok(())
}
