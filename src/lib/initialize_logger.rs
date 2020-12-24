#[cfg(feature = "file-logger")]
use crate::lib::file_logger::initialize_file_logger as init_logger;
#[cfg(feature = "std-err-logger")]
use crate::lib::std_err_logger::initialize_std_err_logger as init_logger;
use crate::lib::types::Result;

pub fn initialize_logger() -> Result<()> {
    init_logger()
}
