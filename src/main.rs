use crate::app::AppError;
use crate::app::helpers::firmware;

mod app;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    println!("Connecting to Initium");

    firmware::get_version().await
}
