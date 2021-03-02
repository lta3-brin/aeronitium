use std::array;
use crate::app::AppError;

impl From<array::TryFromSliceError> for AppError {
    fn from(err: array::TryFromSliceError) -> Self {
        Self::TrySliceError(err)
    }
}
