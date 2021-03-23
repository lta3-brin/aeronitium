mod app;

use actix_web::rt::spawn;
use tokio::sync::{Mutex, watch};
use actix_web::rt::net::TcpStream;
use actix_web::{App, HttpServer, web};
use crate::app::AppError;
use crate::app::configs::get_configs;
use crate::app::helpers::display::display_banner;
use crate::app::routers::app_routers;
use crate::app::helpers::stream::start;


#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let conf = get_configs();
    let (tx, rx) = watch::channel(false);
    let transmit = web::Data::new(Mutex::new(tx));

    let tcp_conn = web::Data::new(Mutex::new(
        TcpStream::connect(conf.get_dtc_addr()).await?
    ));
    let tcp_conn_spawn = tcp_conn.clone();

    spawn(async move {
        let mut conn = tcp_conn_spawn.lock().await;
        let conn = &mut *conn;

        start(conn, 2, rx).await;
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(tcp_conn.clone())
            .app_data(transmit.clone())
            .configure(app_routers)
    }).bind(conf.get_server_addr())?;

    println!("{}", display_banner());
    println!("Starting on {}", conf.get_server_addr().clone());
    Ok(server.run().await?)
}
