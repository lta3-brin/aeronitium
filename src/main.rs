mod app;

use actix_cors::Cors;
use tokio::sync::Mutex;
use actix_web::rt::net::TcpStream;
use actix_web::{App, HttpServer, web};
use crate::app::AppError;
use crate::app::configs::get_configs;
use crate::app::routers::app_routers;
use crate::app::helpers::process::run_script;
use crate::app::helpers::display::display_banner;


#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let conf = get_configs();

    let tcp_conn = web::Data::new(Mutex::new(
        TcpStream::connect(conf.get_dtc_addr()).await?
    ));

    let nats_conn = web::Data::new(Mutex::new(
        async_nats::connect(&*conf.get_nats_addr()).await?
    ));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_header().allow_any_method())
            .app_data(nats_conn.clone())
            .app_data(tcp_conn.clone())
            .configure(app_routers)
    }).bind(conf.get_server_addr())?;

    println!("{}", display_banner());
    println!("Menjalankan server di {}", conf.get_server_addr().clone());
    run_script()?;

    Ok(server.run().await?)
}
