use actix_web::{get, HttpResponse};


#[get("/")]
pub fn version_one() -> HttpResponse {
    HttpResponse::Ok().body("Ini untuk router v1")
}
