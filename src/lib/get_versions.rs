use crate::lib::{constants::APP_VERSION, types::Result};
use ptokens_core::get_core_version;

pub fn get_versions() -> Result<String> {
    info!("✔ Getting core and app versions...");
    Ok(format!(
        "{{\"core_version\":\"{}\",\"app_version\":\"{}\"}}",
        get_core_version(),
        APP_VERSION.unwrap_or("Unknown").to_string()
    ))
}
