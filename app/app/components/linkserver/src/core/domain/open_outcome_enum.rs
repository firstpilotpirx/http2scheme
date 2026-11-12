//! The result of one open request.

use super::link_rejected_error::LinkRejectedError;

/// What happened to one open request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenOutcomeEnum {
    /// The link was handed to the opener.
    Opened,
    /// The key was missing or wrong.
    WrongKey,
    /// The link was malformed or its scheme not allowed.
    Rejected(LinkRejectedError),
    /// The opener failed.
    OpenFailed,
}

impl OpenOutcomeEnum {
    /// The result word written to the request log.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Opened => "opened",
            Self::WrongKey => "wrong key",
            Self::Rejected(reason) if reason.is_malformed() => "malformed link",
            Self::Rejected(_) => "scheme not allowed",
            Self::OpenFailed => "open failed",
        }
    }
}

#[cfg(test)]
#[path = "open_outcome_enum_test.rs"]
mod open_outcome_enum_test;
