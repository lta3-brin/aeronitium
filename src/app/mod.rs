use std::io;

pub mod errors;
pub mod helpers;
pub mod decoders;

#[derive(Debug)]
pub enum AppError {
    TokioIoError(io::Error)
}
