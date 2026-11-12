//! Input port: open a scheme link from the link history again.

use async_trait::async_trait;

use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

/// open a scheme link from the link history again.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ReplayLink: Send + Sync {
    /// Runs the use case on the link text of a request log line.
    async fn execute(&self, link: &str) -> OpenOutcomeEnum;
}

#[cfg(test)]
#[path = "replay_link_test.rs"]
mod replay_link_test;
