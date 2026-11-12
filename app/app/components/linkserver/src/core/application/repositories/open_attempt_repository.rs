//! Port: the request log.

use async_trait::async_trait;

use crate::core::application::exceptions::open_link_error::OpenLinkError;
use crate::core::domain::link_open_attempted_domain_event::LinkOpenAttemptedDomainEvent;

/// Appends one line per open request.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait OpenAttemptRepository: Send + Sync {
    /// Appends the event.
    async fn record(&self, event: &LinkOpenAttemptedDomainEvent) -> Result<(), OpenLinkError>;
}

#[cfg(test)]
#[path = "open_attempt_repository_test.rs"]
mod open_attempt_repository_test;
