#[cfg(feature = "json-rpc-db")]
use crate::lib::json_rpc_database::Database;
#[cfg(feature = "rocks-db")]
use crate::lib::rocks_database::Database;
use crate::lib::types::Result;

pub fn get_database() -> Result<Database> {
    Database::open()
}
