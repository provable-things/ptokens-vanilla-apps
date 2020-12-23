use std::fmt;
use std::error::Error;
use ptokens_core::errors::AppError as PtokensCoreError;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    IOError(std::io::Error),
    PbtcAppError(PtokensCoreError),
    RocksDBError(rocksdb::Error),
    SystemTimeError(std::time::SystemTimeError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) =>
                format!("{}", msg),
            AppError::IOError(ref e) =>
                format!("✘ I/O Error!\n✘ {}", e),
            AppError::PbtcAppError(ref e) =>
                format!("{}", e),
            AppError::RocksDBError(ref e) =>
                format!("✘ Rocks DB Error!\n✘ {}", e),
            AppError::SystemTimeError(ref e) =>
                format!("✘ System Time Error!\n✘ {}", e),
        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        "\n✘ Program Error!\n"
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::IOError(e)
    }
}

impl From<std::time::SystemTimeError> for AppError {
    fn from(e: std::time::SystemTimeError) -> AppError {
        AppError::SystemTimeError(e)
    }
}

impl From<PtokensCoreError> for AppError {
    fn from(e: PtokensCoreError) -> AppError {
        AppError::PbtcAppError(e)
    }
}

impl From<rocksdb::Error> for AppError {
    fn from(e: rocksdb::Error) -> AppError {
        AppError::RocksDBError(e)
    }
}
