use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use crate::app::AppError;
use crate::app::helpers::calzero;
use crate::app::configs::{TcpConnection, get_configs};
use actix_web::rt::net::TcpStream;


#[derive(Deserialize)]
pub struct DtcRezero {
    lrn: u8
}

#[post("/rezero")]
pub async fn calib_rezero(
    payload: web::Json<DtcRezero>,
    tcp: web::Data<TcpConnection>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.conn.lock().await;
    let stream = &mut *stream;

    let message = calzero::command(
        stream, buffer, payload.0.lrn
    ).await?;

    let configs = get_configs();
    *stream = TcpStream::connect(configs.get_dtc_addr()).await?;

    Ok(HttpResponse::Ok().json(message))
}
