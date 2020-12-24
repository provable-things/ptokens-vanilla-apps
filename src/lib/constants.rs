#[cfg(not(feature = "std-err-logger"))]
pub static LOG_FILE_PATH: &'static str = "logs/";

#[cfg(not(feature = "json-rpc-db"))]
pub static DATABASE_PATH: &'static str = "./database";
