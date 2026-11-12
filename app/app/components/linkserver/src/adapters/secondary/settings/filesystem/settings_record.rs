//! The settings file as it is stored.

use serde::Deserialize;

/// The fields every settings file may hold.
pub const FIELDS: [&str; 3] = ["port", "allowed_schemes", "close_tab"];

/// One settings file, read with the TOML format.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettingsRecord {
    /// The listening port.
    pub port: u16,
    /// The schemes the server opens.
    pub allowed_schemes: Vec<String>,
    /// Whether the browser tab closes after a link opens.
    #[serde(default = "close_tab_default")]
    pub close_tab: bool,
}

const fn close_tab_default() -> bool {
    true
}

#[cfg(test)]
#[path = "settings_record_test.rs"]
mod settings_record_test;
