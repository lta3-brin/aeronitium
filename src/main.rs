use crate::app::AppError;
use crate::app::helpers::calzero;

mod app;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Connecting to Initium");

    let message = calzero::command().await?;

    println!("{:?}", message);

    Ok(())
}
