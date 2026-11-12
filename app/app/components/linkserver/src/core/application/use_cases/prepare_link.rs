//! Input port: build the open link of a scheme link.

use async_trait::async_trait;

use crate::core::application::exceptions::prepare_link_error::PrepareLinkError;
use crate::core::domain::prepared_link_value_object::PreparedLinkValueObject;

/// build the open link of a scheme link.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait PrepareLink: Send + Sync {
    /// Runs the use case.
    async fn execute(&self, link: &str) -> Result<PreparedLinkValueObject, PrepareLinkError>;
}

#[cfg(test)]
#[path = "prepare_link_test.rs"]
mod prepare_link_test;
