use tokio::net::TcpStream;
use crate::app::AppError;
use crate::app::helpers::{
    initialization,
    liveaction,
    output
};


#[allow(dead_code)]
pub async fn command() -> Result<(), AppError> {
    let buffer = [0; 8];
    let mut stream = TcpStream::connect("192.168.129.119:8400").await?;

    // setup settings
    const CRS: &str        = "111";  // default CRS (111 - 118 are possible)
    const NUM_CHANNELS: u8 = 64;  //number of channels/port on the ESP. Allowed values include 32, 48, or 64
    const SCN_ADDRESS: &str  = "1";  // scanner address (1-8)
    const LRN: u8          = 1;  // logical range number
    const STBL: u8         = 2;  // single point scanner table
    const SPORT: &str      = "101-164";  // 64 ports on first scanner

    // measurement settings
    const NFR: u8          = 1;  // no averaging -> rapid scanning for signal processing
    const FRD: u8          = 0;  // frame delay (should be 0)
    const NMS: u16         = 0;  // continuous pressure measurement sets to take until aborted
    const MSD: u16       = 5000;  // milliseconds between start of sequential pressure measurement sets
    const TRM: &str        = "FREE";  // trigger mode - free ignores trigger
    const SCM: &str        = "PAM";  // scan mode - parrallel address mode scans ports concurrently
    const OCF: u8          = 2;  // output convert format (2 = normal EU pressure)
    const UNX: u8          = 3;  // units index (1 = psi, 3 = Pa, 6 = atm, 7 = mmHg, 8 = mmH20, 9 = bar, 10 = kPa, 11 = mbar)

    // let x = calzero::command(&mut stream, buffer).await?;
    // println!("{:?}", x);

    // define connected scanners
    let m = initialization::check(
        &mut stream,
        buffer,
        CRS,
        SCN_ADDRESS,
        NUM_CHANNELS,
        LRN
    ).await?;
    println!("{:?}", m);

    // define data acquisition parameters for rapid scanning table
    let m = initialization::define(
        &mut stream,
        buffer,
        CRS,
        STBL,
        NFR,
        FRD,
        NMS,
        MSD,
        TRM,
        SCM,
        OCF
    ).await?;
    println!("{:?}", m);

    // define scan list for rapid scanning
    let m = initialization::scan(
        &mut stream,
        buffer,
        CRS,
        STBL,
        SPORT
    ).await?;
    println!("{:?}", m);

    // load and store DTC scanners EEPROM coefficients
    let m = initialization::load_store(
        &mut stream,
        buffer,
        CRS,
        -1,
        20
    ).await?;
    println!("{:?}", m);

    // sets the engineering units
    let m = initialization::change_unit(
        &mut stream,
        buffer,
        LRN,
        UNX
    ).await?;
    println!("{:?}", m);

    // check firmware
    let m = liveaction::check_firmware(
        &mut stream,
        buffer,
        CRS
    ).await?;
    println!("{:?}", m);

    // look at scanner status
    let m = liveaction::lookat(
        &mut stream,
        buffer,
        CRS,
        1
    ).await?;
    println!("{:?}", m);

    // prints the group 0 coefficients
    output::tabel_coef(
        &mut stream,
        buffer,
        CRS,
        STBL,
        SPORT
    ).await?;

    Ok(())
}
