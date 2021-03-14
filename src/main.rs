mod app;
mod dtc;

use std::env;
use actix_web::{App, HttpServer};
use crate::app::AppError;
use crate::app::routers::app_routers;
use crate::app::middlewares::normalize_path;


#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let addr = env::var("APP_ADDRESS")?;

    let server = HttpServer::new(|| {
        App::new().wrap(normalize_path()).configure(app_routers)
    }).bind(addr.clone())?;

    println!("Running aeronitium on {}...", addr);
    let run = server.run().await?;

    Ok(run)
}
