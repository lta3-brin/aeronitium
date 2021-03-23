use tokio::sync::Mutex;
use actix_web::rt::spawn;
use actix_web::rt::net::TcpStream;
use actix_web::{get, HttpResponse, web};
use crate::app::models::SimpleMessage;
use crate::app::AppError;
use crate::app::helpers::stream::start;


#[get("/startstream")]
pub async fn start_stream(
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

        start(conn, 2).await;
    });

    Ok(HttpResponse::Ok().json(message))
}
