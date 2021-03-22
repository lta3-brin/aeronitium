use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use actix_web::rt::net::TcpStream;
use crate::app::AppError;
use crate::app::helpers::liveaction;


#[derive(Deserialize)]
pub struct DtcFirmwareParams {
    crs: String
}

#[post("/firmware")]
pub async fn get_firmware(
    payload: web::Json<DtcFirmwareParams>,
    tcp: web::Data<Mutex<TcpStream>>
) -> Result<HttpResponse, AppError> {
    let crs = payload.0.crs;

    let buffer = [0u8; 8];
    let mut stream = tcp.lock().await;
    let stream = &mut *stream;

    let message = liveaction::check_firmware(
        stream,
        buffer,
        &crs
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
