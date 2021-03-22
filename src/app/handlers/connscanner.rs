use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::rt::net::TcpStream;
use actix_web::{web, post, HttpResponse};
use crate::app::AppError;
use crate::app::helpers::initialization;


#[derive(Deserialize)]
pub struct DtcConnScanner {
    crs: String,
    scn_address: String,
    num_channels: u8,
    lrn: u8
}

#[post("/connscanner")]
pub async fn connected_scanners(
    payload: web::Json<DtcConnScanner>,
    tcp: web::Data<Mutex<TcpStream>>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.lock().await;
    let stream = &mut *stream;

    let message = initialization::check(
        stream,
        buffer,
        &payload.0.crs,
        &payload.0.scn_address,
        payload.0.num_channels,
        payload.0.lrn
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
