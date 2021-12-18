#[cfg(feature = "json-rpc-db")]
pub fn maybe_strip_hex_prefix(hex: &str) -> &str {
    let lowercase_hex_prefix = "0x";
    let uppercase_hex_prefix = "0X";
    match hex.starts_with(lowercase_hex_prefix) || hex.starts_with(uppercase_hex_prefix) {
        true => hex
            .trim_start_matches(lowercase_hex_prefix)
            .trim_start_matches(uppercase_hex_prefix),
        false => hex,
    }
}
