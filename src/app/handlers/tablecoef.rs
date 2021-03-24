use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::rt::net::TcpStream;
use actix_web::{web, post, HttpResponse};
use crate::app::AppError;
use crate::app::helpers::output;
use crate::app::models::SimpleMessage;


#[derive(Deserialize)]
pub struct DtcTableCoef {
    crs: String,
    stbl: u8,
    sport: String
}

#[post("/tablecoef")]
pub async fn save_coef(
    payload: web::Json<DtcTableCoef>,
    tcp: web::Data<Mutex<TcpStream>>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.lock().await;
    let stream = &mut *stream;

    let o = output::tabel_coef(
        stream,
        buffer,
        &payload.0.crs,
        payload.0.stbl,
        &payload.0.sport
    ).await?;

    let message = SimpleMessage::new(
        0,
        String::from("Kode umum dari Aeronitium"),
        0,
        String::from("Tipe umum dari Aeronitium"),
        o
    );

    Ok(HttpResponse::Ok().json(message))
}
