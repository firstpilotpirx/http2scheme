//! A settings or key file that cannot be used.

/// Names the file, the field and what is wrong.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{file}: {field} {reason}")]
pub struct SettingsInvalidError {
    file: String,
    field: String,
    reason: String,
}

impl SettingsInvalidError {
    /// An error naming the file, field and reason.
    #[must_use]
    pub fn new(file: &str, field: &str, reason: &str) -> Self {
        Self { file: file.to_owned(), field: field.to_owned(), reason: reason.to_owned() }
    }

    /// The field at fault.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }
}

#[cfg(test)]
#[path = "settings_invalid_error_test.rs"]
mod settings_invalid_error_test;
