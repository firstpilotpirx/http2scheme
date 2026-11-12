//! Errors of the ports used while opening a link.

/// A failed step while opening a link.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OpenLinkError {
    /// The link opener failed.
    #[error("the link opener failed: {reason}")]
    OpenFailed {
        /// What went wrong.
        reason: String,
    },
    /// Closing the browser tab failed.
    #[error("closing the browser tab failed: {reason}")]
    TabCloseFailed {
        /// What went wrong.
        reason: String,
    },
    /// Bringing back the app the user left for the browser failed.
    #[error("bringing back the left app failed: {reason}")]
    ReorderFailed {
        /// What went wrong.
        reason: String,
    },
    /// Writing the request log failed.
    #[error("writing the request log failed: {reason}")]
    RecordFailed {
        /// What went wrong.
        reason: String,
    },
}

#[cfg(test)]
#[path = "open_link_error_test.rs"]
mod open_link_error_test;
