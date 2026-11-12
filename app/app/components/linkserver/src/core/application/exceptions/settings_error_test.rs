use super::SettingsError;
use crate::core::domain::settings_invalid_error::SettingsInvalidError;

#[test]
fn invalid_shows_the_settings_message() {
    let error = SettingsError::from(SettingsInvalidError::new("/s.toml", "port", "is wrong"));
    assert_eq!(error.to_string(), "/s.toml: port is wrong");
}

#[test]
fn io_names_the_path() {
    let error = SettingsError::Io { path: "/k".to_owned(), reason: "denied".to_owned() };
    assert_eq!(error.to_string(), "/k: denied");
}
