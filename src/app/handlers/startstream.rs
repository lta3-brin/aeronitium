use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::rt::spawn;
use async_nats::Connection;
use actix_web::rt::net::TcpStream;
use actix_web::{post, HttpResponse, web};
use crate::app::AppError;
use crate::app::models::SimpleMessage;
use crate::app::helpers::stream::start;


#[derive(Deserialize)]
pub struct DtcStartStream {
    stbl: u8
}

#[post("/startstream")]
pub async fn start_stream(
    payload: web::Json<DtcStartStream>,
    tcp_conn_spawn: web::Data<Mutex<TcpStream>>,
    nats_conn: web::Data<Mutex<Connection>>
) -> Result<HttpResponse, AppError> {
    let message = SimpleMessage::new(
        0,
        String::from("Kode umum dari Aeronitium"),
        0,
        String::from("Tipe umum dari Aeronitium"),
        String::from("Memulai streaming data...")
    );

    spawn(async move {
        let mut conn = tcp_conn_spawn.lock().await;
        let conn = &mut *conn;

        let mut natsconn = nats_conn.lock().await;
        let natsconn = &mut *natsconn;

        start(conn, payload.0.stbl, natsconn).await;
    });

    Ok(HttpResponse::Ok().json(message))
}
