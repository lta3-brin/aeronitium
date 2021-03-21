use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use crate::app::configs::TcpConnection;
use crate::app::helpers::liveaction;
use crate::app::AppError;


#[derive(Deserialize)]
pub struct DtcFirmwareParams {
    crs: String
}

#[post("/firmware")]
pub async fn get_firmware(
    payload: web::Json<DtcFirmwareParams>,
    tcp: web::Data<TcpConnection>
) -> Result<HttpResponse, AppError> {
    let crs = payload.0.crs;

    let buffer = [0u8; 8];
    let mut stream = tcp.conn.lock().await;
    let stream = &mut *stream;

    let message = liveaction::check_firmware(
        stream,
        buffer,
        &crs
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
