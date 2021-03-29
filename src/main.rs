mod app;

use actix_cors::Cors;
use tokio::sync::Mutex;
use actix_web::rt::net::TcpStream;
use actix_web::{App, HttpServer, web};
use crate::app::AppError;
use crate::app::configs::get_configs;
use crate::app::helpers::display::display_banner;
use crate::app::routers::app_routers;


#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let conf = get_configs();

    let tcp_conn = web::Data::new(Mutex::new(
        TcpStream::connect(conf.get_dtc_addr()).await?
    ));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_header().allow_any_method())
            .app_data(tcp_conn.clone())
            .configure(app_routers)
    }).bind(conf.get_server_addr())?;

    println!("{}", display_banner());
    println!("Starting on {}", conf.get_server_addr().clone());
    Ok(server.run().await?)
}
