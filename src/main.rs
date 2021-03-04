use crate::app::AppError;
use crate::app::helpers::simplesetup;

mod app;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Connecting to Initium");

    let resp = simplesetup::command().await?;

    println!("{:?}", resp);

    Ok(())
}
