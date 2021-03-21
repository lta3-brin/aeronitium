use actix_web::{error, HttpResponse};
use actix_web::http::{StatusCode, header};
use actix_web::dev::HttpResponseBuilder;
use crate::app::AppError;
use crate::app::models::SimpleMessage;


impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
           _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = SimpleMessage::new(
            0,
            String::from("Kode kesalahan dari Aeronitium"),
            0,
            String::from("Tipe kesalahan dari Aeronitium"),
            self.to_string()
        );

        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .json(message)
    }
}
