use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use crate::app::configs::TcpConnection;
use crate::app::helpers::initialization;
use crate::app::AppError;


#[derive(Deserialize)]
pub struct DtcScanList {
    crs: String,
    stbl: u8,
    sport: String
}

#[post("/scanlist")]
pub async fn scan_list(
    payload: web::Json<DtcScanList>,
    tcp: web::Data<TcpConnection>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.conn.lock().await;
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
