use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::rt::net::TcpStream;
use actix_web::{web, post, HttpResponse};
use crate::app::AppError;
use crate::app::helpers::initialization;


#[derive(Deserialize)]
pub struct DtcScanList {
    crs: String,
    stbl: u8,
    sport: String
}

#[post("/scanlist")]
pub async fn scan_list(
    payload: web::Json<DtcScanList>,
    tcp: web::Data<Mutex<TcpStream>>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.lock().await;
    let stream = &mut *stream;

    let message = initialization::scan(
        stream,
        buffer,
        &payload.0.crs,
        payload.0.stbl,
        &payload.0.sport
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
