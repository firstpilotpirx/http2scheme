//! Why a scheme link was refused.

/// A reason a requested scheme link is not opened.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LinkRejectedError {
    /// The request carried no link.
    #[error("the link is missing")]
    Missing,
    /// The link text is empty.
    #[error("the link is empty")]
    Empty,
    /// The link text is longer than the limit.
    #[error("the link is longer than 4096 characters")]
    TooLong,
    /// The link holds a space or a control character.
    #[error("the link holds a space or a control character")]
    ForbiddenCharacter,
    /// The link has no valid scheme before its first colon.
    #[error("the link has no valid scheme")]
    NoScheme,
    /// The scheme is not on the allowed list.
    #[error("scheme not allowed: {scheme}")]
    SchemeNotAllowed {
        /// The refused scheme, lower case.
        scheme: String,
    },
}

impl LinkRejectedError {
    /// True for every reason except a well-formed link with a scheme not allowed.
    #[must_use]
    pub const fn is_malformed(&self) -> bool {
        !matches!(self, Self::SchemeNotAllowed { .. })
    }
}

#[cfg(test)]
#[path = "link_rejected_error_test.rs"]
mod link_rejected_error_test;
