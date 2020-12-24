use crate::lib::errors::AppError;
use std::result;

pub type DataSensitivity = Option<u8>;
pub type Result<T> = result::Result<T, AppError>;
