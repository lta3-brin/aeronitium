use std::io;
use crate::app::AppError;

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::TcpError(err)
    }
}
