use crate::types::Result;

pub fn initialize_std_err_logger() -> Result<()> {
    match stderrlog::new()
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(2)
        .init()
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string().into()),
    }
}
