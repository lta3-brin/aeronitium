use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use crate::app::configs::TcpConnection;
use crate::app::AppError;
use crate::app::helpers::stream::stop;


#[post("/stopstream")]
pub async fn stop_stream(
    tcp: web::Data<TcpConnection>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.conn.lock().await;
    let stream = &mut *stream;

    let message = stop(stream, buffer).await?;

    Ok(HttpResponse::Ok().json(message))
}
