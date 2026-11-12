//! A menu action that failed.

/// Why a menu action did not happen.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MenuActionError {
    /// macOS refused to change or show the login item.
    #[error("login item: {0}")]
    LoginItem(String),
    /// The request log cannot be read.
    #[error("{path}: {reason}")]
    History {
        /// The request log.
        path: String,
        /// What went wrong.
        reason: String,
    },
    /// The first-launch record cannot be read or written.
    #[error("{path}: {reason}")]
    FirstLaunch {
        /// The record file.
        path: String,
        /// What went wrong.
        reason: String,
    },
}

#[cfg(test)]
#[path = "menu_action_error_test.rs"]
mod menu_action_error_test;
