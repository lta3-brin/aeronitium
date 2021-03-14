use std::{
    io,
    env,
    array,
};

pub mod errors;
pub mod helpers;
pub mod decoders;
pub mod models;
pub mod routers;
pub mod handlers;
pub mod middlewares;

#[derive(Debug)]
pub enum AppError {
    TokioIoError(io::Error),
    TrySliceError(array::TryFromSliceError),
    EnvError(env::VarError)
}
