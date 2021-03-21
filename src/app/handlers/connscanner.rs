use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use crate::app::configs::TcpConnection;
use crate::app::helpers::initialization;
use crate::app::AppError;


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
    tcp: web::Data<TcpConnection>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.conn.lock().await;
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
