use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::rt::net::TcpStream;
use actix_web::{web, post, HttpResponse};
use crate::app::AppError;
use crate::app::helpers::initialization;


#[derive(Deserialize)]
pub struct DtcEepromCoef {
    crs: String
}

#[post("/eeprom")]
pub async fn eeprom_coef(
    payload: web::Json<DtcEepromCoef>,
    tcp: web::Data<Mutex<TcpStream>>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.lock().await;
    let stream = &mut *stream;

    let message = initialization::load_store(
        stream,
        buffer,
        &payload.0.crs,
        -1,
        20
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
