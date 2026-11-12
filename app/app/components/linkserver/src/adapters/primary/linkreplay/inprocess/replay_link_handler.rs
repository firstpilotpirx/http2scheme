//! Opens a link of the link history again and answers the result word.

use std::sync::Arc;

use crate::core::application::use_cases::replay_link::ReplayLink;

/// The ▶ of the History tab, called in the app's process.
#[derive(Clone)]
pub struct ReplayLinkHandler {
    use_case: Arc<dyn ReplayLink>,
}

impl ReplayLinkHandler {
    /// A handler over the use case.
    #[must_use]
    pub fn new(use_case: Arc<dyn ReplayLink>) -> Self {
        Self { use_case }
    }

    /// Opens `link` again; the result word of the request log.
    pub async fn replay(&self, link: &str) -> String {
        self.use_case.execute(link).await.label().to_owned()
    }
}

#[cfg(test)]
#[path = "replay_link_handler_test.rs"]
mod replay_link_handler_test;
