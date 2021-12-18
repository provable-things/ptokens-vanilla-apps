#[cfg(feature = "file-logger")]
pub(crate) mod file_logger;
#[cfg(feature = "std-err-logger")]
pub(crate) mod std_err_logger;

#[cfg(feature = "file-logger")]
use crate::loggers::file_logger::initialize_file_logger as initialize_logger;
#[cfg(feature = "std-err-logger")]
use crate::loggers::std_err_logger::initialize_std_err_logger as initialize_logger;
use crate::types::Result;

pub fn init_logger() -> Result<()> {
    initialize_logger()
}
