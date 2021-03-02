use crate::app::AppError;
use crate::app::helpers::firmware;

mod app;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Connecting to Initium");

    let message = firmware::command().await?;

    println!("{:?}", message);

    Ok(())
}
