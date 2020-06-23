use std::num::ParseIntError;

#[cfg(feature = "gui")]
use regex::Regex;

#[cfg(feature = "gui")]
lazy_static! {
    static ref REPLACING_REG: Regex = Regex::new(r"[^0-9ABCDEF]").unwrap();
}

#[cfg(feature = "gui")]
pub fn cleanup_hex_string(value: String) -> String {
    REPLACING_REG.replace_all(value.to_uppercase().as_str(), "").to_string()
}

pub fn parse_hex(src: &str) -> Result<u64, ParseIntError> {
    u64::from_str_radix(src, 16)
}