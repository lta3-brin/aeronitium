use std::{
    io,
    env,
    string
};
use tokio_tungstenite::tungstenite;


#[derive(Debug)]
pub enum AdapterError {
    IoError(io::Error),
    EnvError(env::VarError),
    WsError(tungstenite::Error),
    Utf8Error(string::FromUtf8Error)
}

impl From<io::Error> for AdapterError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<env::VarError> for AdapterError {
    fn from(err: env::VarError) -> Self {
        Self::EnvError(err)
    }
}

impl From<tungstenite::Error> for AdapterError {
    fn from(err: tungstenite::Error) -> Self {
        Self::WsError(err)
    }
}

impl From<string::FromUtf8Error> for AdapterError {
    fn from(err: string::FromUtf8Error) -> Self {
        Self::Utf8Error(err)
    }
}
