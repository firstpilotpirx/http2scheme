//! Port: opening a link of the history again.

use async_trait::async_trait;

/// Opens a link again as the server would, without a key.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait LinkReplayService: Send + Sync {
    /// Opens `link` again; the result word of the request log.
    async fn replay(&self, link: &str) -> String;
}

#[cfg(test)]
#[path = "link_replay_service_test.rs"]
mod link_replay_service_test;
