//! Errors reading or creating the settings and the key.

use crate::core::domain::settings_invalid_error::SettingsInvalidError;

/// The settings or key cannot be used.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SettingsError {
    /// A file holds a value that breaks a rule.
    #[error(transparent)]
    Invalid(#[from] SettingsInvalidError),
    /// A file cannot be read or written.
    #[error("{path}: {reason}")]
    Io {
        /// The file.
        path: String,
        /// What went wrong.
        reason: String,
    },
}

#[cfg(test)]
#[path = "settings_error_test.rs"]
mod settings_error_test;
