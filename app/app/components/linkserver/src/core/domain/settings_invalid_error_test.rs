use super::SettingsInvalidError;

#[test]
fn message_names_file_field_and_reason() {
    let error =
        SettingsInvalidError::new("/h/settings.toml", "port", "must be between 1024 and 65535");
    assert_eq!(
        error.to_string(),
        "/h/settings.toml: port must be between 1024 and 65535"
    );
    assert_eq!(error.field(), "port");
}
