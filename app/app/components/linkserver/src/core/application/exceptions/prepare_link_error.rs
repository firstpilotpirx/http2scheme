//! Errors preparing an open link.

use super::settings_error::SettingsError;
use crate::core::domain::link_rejected_error::LinkRejectedError;

/// Why no open link was printed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PrepareLinkError {
    /// The settings or key cannot be used.
    #[error(transparent)]
    Settings(#[from] SettingsError),
    /// The scheme link is refused.
    #[error(transparent)]
    Rejected(#[from] LinkRejectedError),
}

#[cfg(test)]
#[path = "prepare_link_error_test.rs"]
mod prepare_link_error_test;
