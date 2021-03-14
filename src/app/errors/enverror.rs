use std::env;
use crate::app::AppError;

impl From<env::VarError> for AppError {
    fn from(err: env::VarError) -> Self {
        Self::EnvError(err)
    }
}
