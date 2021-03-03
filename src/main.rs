use crate::app::AppError;
use crate::app::helpers::chgunit;

mod app;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Connecting to Initium");

    //Konversi satuan tekanan ke Pa (3)
    let resp = chgunit::command(3).await?;

    println!("{:?}", resp);

    Ok(())
}
