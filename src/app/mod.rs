use std::{
    io,
    env,
    array,
};

pub mod errors;
pub mod helpers;
pub mod decoders;
pub mod models;
pub mod layouts;
pub mod components;
pub mod pages;


#[derive(Debug)]
pub enum AppError {
    TokioIoError(io::Error),
    TrySliceError(array::TryFromSliceError),
    EnvError(env::VarError)
}
