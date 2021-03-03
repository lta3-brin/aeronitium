use crate::app::AppError;
use crate::app::helpers::chgunit;

mod app;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Connecting to Initium");

    chgunit::command().await?;

    Ok(())
}
