use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use crate::app::configs::TcpConnection;
use crate::app::helpers::initialization;
use crate::app::AppError;


#[derive(Deserialize)]
pub struct DtcDaqParams {
    crs: String,
    stbl: u8,
    nfr: u8,
    frd: u8,
    nms: u16,
    msd: u16,
    trm: String,
    scm: String,
    ocf: u8
}

#[post("/daqparams")]
pub async fn daq_params(
    payload: web::Json<DtcDaqParams>,
    tcp: web::Data<TcpConnection>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.conn.lock().await;
    let stream = &mut *stream;

    let message = initialization::define(
        stream,
        buffer,
        &payload.0.crs,
        payload.0.stbl,
        payload.0.nfr,
        payload.0.frd,
        payload.0.nms,
        payload.0.msd,
        &payload.0.trm,
        &payload.0.scm,
        payload.0.ocf
    ).await?;

    Ok(HttpResponse::Ok().json(message))
}
