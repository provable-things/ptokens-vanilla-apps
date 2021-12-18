#[cfg(feature = "rocks-db")]
pub(crate) mod rocks_db;

#[cfg(feature = "json-rpc-db")]
pub(crate) mod json_rpc_database;

#[cfg(feature = "json-rpc-db")]
use crate::databases::json_rpc_database::Database;
#[cfg(feature = "rocks-db")]
use crate::databases::rocks_db::Database;
use crate::types::Result;

pub fn get_db() -> Result<Database> {
    Database::open()
}
