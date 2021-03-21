use actix_web::{get, HttpResponse};
use crate::app::models::SimpleMessage;
use crate::app::AppError;


#[get("/")]
pub async fn get_root() -> Result<HttpResponse, AppError> {
    let message = SimpleMessage::new(
        0,
        String::from("Kode umum dari Aeronitium"),
        0,
        String::from("Tipe umum dari Aeronitium"),
        String::from("Halaman ini sengaja dikosongkan")
    );

    Ok(HttpResponse::Ok().json(message))
}
