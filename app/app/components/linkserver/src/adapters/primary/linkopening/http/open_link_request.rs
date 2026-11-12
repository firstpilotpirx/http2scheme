//! The query of one open request.

use serde::Deserialize;

use crate::core::application::use_cases::open_link_command::OpenLinkCommand;

/// The `key` and `url` query parameters, each possibly missing.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct OpenLinkRequest {
    key: Option<String>,
    url: Option<String>,
}

impl OpenLinkRequest {
    /// The scheme link the request carried.
    #[must_use]
    pub fn link(&self) -> Option<&str> {
        self.url.as_deref()
    }

    /// The use case command.
    #[must_use]
    pub fn into_command(self) -> OpenLinkCommand {
        OpenLinkCommand::new(self.key, self.url)
    }
}

#[cfg(test)]
#[path = "open_link_request_test.rs"]
mod open_link_request_test;
