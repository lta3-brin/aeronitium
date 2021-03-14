use actix_web::{get, HttpResponse};


#[get("/")]
pub fn root_handler() -> HttpResponse {
    HttpResponse::Ok().body("Halaman ini sengaja dikosongkan")
}