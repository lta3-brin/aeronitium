use futures_util::SinkExt;
use tokio::net::TcpStream;
use async_nats::Connection;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use crate::error::AdapterError;


pub async fn send_message(
    stream: TcpStream,
    nc: Connection
) -> Result<(), AdapterError> {
    let mut ws = accept_async(stream).await?;
    let sub = nc.subscribe("dtc").await?;
    while let Some(message) = sub.next().await {
        let data = String::from_utf8(message.data)?;

        ws.send(Message::from(data)).await?;
    }

    Ok(())
}
