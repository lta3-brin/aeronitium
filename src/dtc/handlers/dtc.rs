use actix_web::{get, HttpResponse};


#[get("/dtc/")]
pub fn dtc_handler() -> HttpResponse {
    HttpResponse::Ok().body("Routers for DTC Initium.")
}
