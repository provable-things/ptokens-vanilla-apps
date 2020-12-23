use std::{
    path::Path,
    fs::create_dir_all,
};
use crate::{
    types::Result,
    constants::DATABASE_PATH,
};

pub fn maybe_initialize_database() -> Result<()> {
    info!("✔ Maybe initializing databaseu...");
    if !Path::new(&DATABASE_PATH).exists() {
        info!("✔ No database directory found, creating...");
        create_dir_all(&DATABASE_PATH)?;
    };
    Ok(())
}
