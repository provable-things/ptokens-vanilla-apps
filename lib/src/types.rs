use std::result;

use crate::errors::AppError;

pub type DataSensitivity = Option<u8>;
pub type Result<T> = result::Result<T, AppError>;
