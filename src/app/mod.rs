use std::{
    array,
    env,
    io,
};
use derive_more::{Display, Error};

pub mod errors;
pub mod helpers;
pub mod decoders;
pub mod models;
pub mod configs;
pub mod handlers;
pub mod routers;


#[derive(Debug, Display, Error)]
pub enum AppError {
    TcpError(io::Error),
    TrySliceError(array::TryFromSliceError),
    EnvError(env::VarError)
}
