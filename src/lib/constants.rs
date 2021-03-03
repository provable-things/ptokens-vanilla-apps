#[cfg(not(feature = "std-err-logger"))]
pub static LOG_FILE_PATH: &str = "logs/";

#[cfg(not(feature = "json-rpc-db"))]
pub static DATABASE_PATH: &str = "./database";

pub const APP_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
