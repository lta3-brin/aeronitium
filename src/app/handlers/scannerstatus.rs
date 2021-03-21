use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use crate::app::configs::TcpConnection;
use crate::app::helpers::liveaction;
use crate::app::AppError;


#[derive(Deserialize)]
pub struct DtcScannerStatus {
    crs: String,
    scanner_number: u8
}

#[post("/scannerstatus")]
pub async fn get_status(
    payload: web::Json<DtcScannerStatus>,
    tcp: web::Data<TcpConnection>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.conn.lock().await;
    let stream = &mut *stream;

    let message = liveaction::lookat(
        stream,
        buffer,
        &payload.0.crs,
        payload.0.scanner_number
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
