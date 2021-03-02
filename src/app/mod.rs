use std::io;
use std::array;

pub mod errors;
pub mod helpers;
pub mod decoders;
pub mod models;

#[derive(Debug)]
pub enum AppError {
    TokioIoError(io::Error),
    TrySliceError(array::TryFromSliceError)
}
