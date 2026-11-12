//! Between the stored settings and the domain settings.

use std::fmt::Write as _;

use super::settings_record::SettingsRecord;
use crate::core::domain::port_value_object::PortValueObject;
use crate::core::domain::scheme_allowlist_policy::SchemeAllowlistPolicy;
use crate::core::domain::scheme_name_value_object::SchemeNameValueObject;
use crate::core::domain::server_settings_value_object::ServerSettingsValueObject;
use crate::core::domain::settings_invalid_error::SettingsInvalidError;

/// The domain settings of a record read from `file`.
///
/// # Errors
/// A port below 1024 or a bad scheme name names its field.
pub fn to_domain(
    record: &SettingsRecord,
    file: &str,
) -> Result<ServerSettingsValueObject, SettingsInvalidError> {
    let port = PortValueObject::new(record.port)
        .ok_or_else(|| SettingsInvalidError::new(file, "port", "must be between 1024 and 65535"))?;
    let schemes = record
        .allowed_schemes
        .iter()
        .map(|name| {
            SchemeNameValueObject::parse(name).map_err(|_| {
                SettingsInvalidError::new(
                    file,
                    "allowed_schemes",
                    &format!("has a bad scheme name: {name:?}"),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ServerSettingsValueObject::new(
        port,
        SchemeAllowlistPolicy::new(schemes),
        record.close_tab,
    ))
}

/// The settings file text for the domain settings.
#[must_use]
pub fn to_text(settings: &ServerSettingsValueObject) -> String {
    let schemes: Vec<String> = settings
        .allowlist()
        .schemes()
        .iter()
        .map(|name| format!("{:?}", name.as_str()))
        .collect();
    let mut text = String::from("# http2scheme settings. Restart the server after a change.\n\n");
    let _ = writeln!(
        text,
        "# The port the server listens on, from 1024 to 65535.\nport = {}\n",
        settings.port().value()
    );
    let _ = writeln!(
        text,
        "# The schemes the server opens; http, https and file are never opened.\nallowed_schemes = [{}]\n",
        schemes.join(", ")
    );
    let _ = writeln!(
        text,
        "# Close the browser tab after a link opens.\nclose_tab = {}",
        settings.close_tab()
    );
    text
}

#[cfg(test)]
#[path = "settings_record_to_domain_mapper_test.rs"]
mod settings_record_to_domain_mapper_test;
