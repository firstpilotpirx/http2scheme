//! Opens a link again through the replayer the app hands in.

use async_trait::async_trait;

use crate::core::application::services::link_replay_service::LinkReplayService;
use crate::menubar_hooks::LinkReplayer;

/// Calls the app's replayer, which runs the server's replay to its end.
pub struct InProcessLinkReplayGateway {
    replayer: LinkReplayer,
}

impl InProcessLinkReplayGateway {
    /// A gateway over `replayer`.
    #[must_use]
    pub fn new(replayer: LinkReplayer) -> Self {
        Self { replayer }
    }
}

#[async_trait]
impl LinkReplayService for InProcessLinkReplayGateway {
    async fn replay(&self, link: &str) -> String {
        (self.replayer)(link)
    }
}

#[cfg(test)]
#[path = "in_process_link_replay_gateway_test.rs"]
mod in_process_link_replay_gateway_test;
