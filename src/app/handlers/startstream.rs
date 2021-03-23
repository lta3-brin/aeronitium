use actix_web::{get, HttpResponse, web};
use crate::app::models::SimpleMessage;
use crate::app::AppError;
use tokio::sync::Mutex;
use tokio::sync::watch::Sender;


#[get("/startstream")]
pub async fn start_stream(transmit: web::Data<Mutex<Sender<bool>>>)
    -> Result<HttpResponse, AppError> {
    let message = SimpleMessage::new(
        0,
        String::from("Kode umum dari Aeronitium"),
        0,
        String::from("Tipe umum dari Aeronitium"),
        String::from("Memulai streaming data...")
    );

    let mut tx = transmit.lock().await;
    let transmit = &mut *tx;

    transmit.broadcast(true).unwrap();

    Ok(HttpResponse::Ok().json(message))
}
