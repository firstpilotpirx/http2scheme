//! Input port: open a scheme link from a keyed request.

use async_trait::async_trait;

use super::open_link_command::OpenLinkCommand;
use crate::core::domain::open_outcome_enum::OpenOutcomeEnum;

/// open a scheme link from a keyed request.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait OpenLink: Send + Sync {
    /// Runs the use case.
    async fn execute(&self, command: OpenLinkCommand) -> OpenOutcomeEnum;
}
