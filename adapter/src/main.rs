mod error;
mod transfer;

use std::env;
use dotenv::dotenv;
use tokio::net::TcpListener;
use crate::error::AdapterError;
use crate::transfer::send_message;


#[allow(irrefutable_let_patterns)]
#[tokio::main]
async fn main() -> Result<(), AdapterError> {
    dotenv().ok();
    println!("Menjalankan Aeronitium adapter");

    let nats_conn = env::var("NATS_ADDRESS")?;
    let nc = async_nats::connect(nats_conn.as_str()).await?;

    let server = TcpListener::bind("0.0.0.0:3000").await?;
    while let conn = server.accept().await? {
        let nc = nc.clone();
        let stream = conn.0;

        tokio::spawn(send_message(stream, nc));
    };

    Ok(())
}
