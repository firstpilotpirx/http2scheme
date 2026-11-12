//! The input of the open-link use case.

use std::fmt;

/// The key and the scheme link of one open request, as the request carried them.
#[derive(Clone, PartialEq, Eq)]
pub struct OpenLinkCommand {
    key: Option<String>,
    link: Option<String>,
}

impl OpenLinkCommand {
    /// The request's key and link, each possibly missing.
    #[must_use]
    pub const fn new(key: Option<String>, link: Option<String>) -> Self {
        Self { key, link }
    }

    /// The key the request carried.
    #[must_use]
    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    /// The scheme link the request carried.
    #[must_use]
    pub fn link(&self) -> Option<&str> {
        self.link.as_deref()
    }
}

impl fmt::Debug for OpenLinkCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenLinkCommand").field("key", &"****").field("link", &self.link).finish()
    }
}

#[cfg(test)]
#[path = "open_link_command_test.rs"]
mod open_link_command_test;
