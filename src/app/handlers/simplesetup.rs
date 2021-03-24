use tokio::sync::Mutex;
use serde::Deserialize;
use actix_web::{web, post, HttpResponse};
use actix_web::rt::net::TcpStream;
use crate::app::AppError;
use crate::app::models::SimpleMessage;
use crate::app::helpers::{initialization, output};


#[derive(Deserialize)]
pub struct DtcSimpleSetup {
    crs: String,
    num_channels: u8,
    scn_address: String,
    lrn: u8,
    stbl: u8,
    sport: String,
    nfr: u8,
    frd: u8,
    nms: u16,
    msd: u16,
    trm: String,
    scm: String,
    ocf: u8,
    unx: u8,
}

#[post("/simplesetup")]
pub async fn simple_setup(
    payload: web::Json<DtcSimpleSetup>,
    tcp: web::Data<Mutex<TcpStream>>
) -> Result<HttpResponse, AppError> {
    let buffer = [0u8; 8];
    let mut stream = tcp.lock().await;
    let stream = &mut *stream;

    // define connected scanners
    initialization::check(
        stream,
        buffer,
        &payload.0.crs,
        &payload.0.scn_address,
        payload.0.num_channels,
        payload.0.lrn
    ).await?;

    // define data acquisition parameters for rapid scanning table
    initialization::define(
        stream,
        buffer,
        &payload.0.crs,
        payload.0.stbl,
        payload.0.nfr,
        payload.0.frd,
        payload.0.nms,
        payload.0.msd,
        &payload.0.trm,
        &payload.0.scm,
        payload.0.ocf
    ).await?;

    // define scan list for rapid scanning
    initialization::scan(
        stream,
        buffer,
        &payload.0.crs,
        payload.0.stbl,
        &payload.0.sport
    ).await?;

    // load and store DTC scanners EEPROM coefficients
    initialization::load_store(
        stream,
        buffer,
        &payload.0.crs,
        -1,
        20
    ).await?;

    // sets the engineering units
    initialization::change_unit(
        stream,
        buffer,
        payload.0.lrn,
        payload.0.unx
    ).await?;

    // prints the group 0 coefficients
    let o = output::tabel_coef(
        stream,
        buffer,
        &payload.0.crs,
        payload.0.stbl,
        &payload.0.sport
    ).await?;

    let message = SimpleMessage::new(
        0,
        String::from("Kode umum dari Aeronitium"),
        0,
        String::from("Tipe umum dari Aeronitium"),
        o
    );

    Ok(HttpResponse::Ok().json(message))
}
