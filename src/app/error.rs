use std::io;

#[derive(Debug)]
pub enum AppError {
    TokioIoError(io::Error)
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::TokioIoError(err)
    }
}
