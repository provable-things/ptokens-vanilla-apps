use lib::{get_core_version, types::Result};

pub fn get_versions() -> Result<String> {
    info!("✔ Getting core and app versions...");
    Ok(format!(
        "{{\"core_version\":\"{}\",\"app_version\":\"{}\"}}",
        get_core_version(),
        option_env!("CARGO_PKG_VERSION").unwrap_or("Unknown").to_string()
    ))
}
