use tokio::net::TcpStream;
use crate::app::AppError;
use crate::app::helpers::{
    initialization,
    liveaction,
    output,
    startarray,
    calzero
};


#[allow(dead_code)]
pub async fn command() -> Result<(), AppError> {
    let buffer = [0; 8];
    let mut stream = TcpStream::connect("192.168.129.119:8400").await?;

    // setup settings
    const CRS: &str        = "111";  // default CRS (111 - 118 are possible)
    const NUM_CHANNELS: u8 = 64;  //number of channels/port on the ESP. Allowed values include 32, 48, or 64
    const SCN_ADDRESS: u8  = 1;  // scanner address (1-8)
    const LRN: u8          = 1;  // logical range number
    const STBL_1: u8       = 1;  // filtered and averaged scanner table used (1-4, 5 is Initium generated)
    const STBL_2: u8       = 2;  // single point scanner table
    const SPORT: &str      = "101-164";  // 64 ports on first scanner

    // measurement settings
    const NFR_1: u8  = 64;  // number of frames averaged over for each measurement set
    const NFR_2: u8  = 1;  // no averaging -> rapid scanning for signal processing
    const FRD: u8    = 0;  // frame delay (should be 0)
    const NMS_1: u16  = 1;  // number of pressure measurement sets to take (0 = continuous until aborted)
    const NMS_2: u16  = 100;  // return a large set of frames to user
    const MSD_1: u16 = 500;  // milliseconds between start of sequential pressure measurement sets
    const MSD_2: u16  = 0;  // milliseconds between start of sequential pressure measurement sets
    const TRM: &str  = "FREE";  // trigger mode - free ignores trigger
    const SCM: &str  = "PAM";  // scan mode - parrallel address mode scans ports concurrently
    const OCF: u8    = 2;  // output convert format (2 = normal EU pressure)
    const UNX: u8    = 3;  // units index (1 = psi, 3 = Pa, 6 = atm, 7 = mmHg, 8 = mmH20, 9 = bar, 10 = kPa, 11 = mbar)

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

    // define data acquisition parameters for filtered table
    // let m = daparams::check(
    //     &mut stream,
    //     buffer,
    //     CRS,
    //     STBL_1,
    //     NFR_1,
    //     FRD,
    //     NMS_1,
    //     MSD_1,
    //     TRM,
    //     SCM,
    //     OCF
    // ).await?;

    // define data acquisition parameters for rapid scanning table
    let m = initialization::define(
        &mut stream,
        buffer,
        CRS,
        STBL_2,
        NFR_2,
        FRD,
        NMS_2,
        MSD_2,
        TRM,
        SCM,
        OCF
    ).await?;
    println!("{:?}", m);

    // define scan list for filtered table
    // let cmd = format!(
    //     "SD3 {} {} {};", CRS, STBL_1, SPORT
    // );
    // stream.write_all(cmd.as_bytes()).await?;
    // stream.read(&mut buffer).await?;
    // let message = display::display_message::<i32>(buffer)?;

    // define scan list for rapid scanning
    let m = initialization::scan(
        &mut stream,
        buffer,
        CRS,
        STBL_2,
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
        SCN_ADDRESS
    ).await?;
    println!("{:?}", m);

    // prints the group 0 coefficients
    let n = output::tabel_coef(
        &mut stream,
        buffer,
        CRS,
        STBL_2,
        SPORT
    ).await?;

    startarray::build(&mut stream, n).await?;

    // test acquisition
    // let cmd = format!("AD2 {};", STBL_2);
    // stream.write_all(cmd.as_bytes()).await?;
    // stream.read(&mut buffer).await?;
    // println!("{:?}", buffer);
    // let message = display::display_message::<String>(buffer)?;

    Ok(())
}
