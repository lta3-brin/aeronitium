use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use actix_web::rt::spawn;
use crate::app::AppError;
use crate::app::models::SimpleMessage;
use crate::app::configs::TcpConnection;
use crate::app::helpers::stream::start;


#[derive(Deserialize)]
pub struct DtcStreamSensor {
    stbl: u8
}

#[post("/startstream")]
pub async fn start_stream(
    payload: web::Json<DtcStreamSensor>,
    tcp: web::Data<TcpConnection>
) -> Result<HttpResponse, AppError> {
    let mut stream = tcp.conn.lock().await;
    let stream = &mut *stream;

    spawn(async move {
        start(stream, payload.0.stbl).await;
    });

    let message = SimpleMessage::new(
        0,
        String::from("Kode umum dari Aeronitium"),
        0,
        String::from("Tipe umum dari Aeronitium"),
        "Memulai streaming sensor tekanan"
    );

    Ok(HttpResponse::Ok().json(message))
}
