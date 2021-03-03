use crate::app::AppError;
use crate::app::helpers::stopstream;

mod app;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Connecting to Initium");

    let resp = stopstream::command().await?;

    println!("{:?}", resp);

    Ok(())
}
