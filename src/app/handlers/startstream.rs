use tokio::sync::Mutex;
use actix_web::rt::spawn;
use actix_web::rt::net::TcpStream;
use actix_web::{post, HttpResponse, web};
use crate::app::models::SimpleMessage;
use crate::app::AppError;
use crate::app::helpers::stream::start;


#[derive(Deserialize)]
pub struct DtcSimpleSetup {
    stbl: u8
}

#[post("/startstream")]
pub async fn start_stream(
    payload: web::Json<DtcSimpleSetup>,
    tcp_conn_spawn: web::Data<Mutex<TcpStream>>
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

        start(conn, payload.0.stbl).await;
    });

    Ok(HttpResponse::Ok().json(message))
}
