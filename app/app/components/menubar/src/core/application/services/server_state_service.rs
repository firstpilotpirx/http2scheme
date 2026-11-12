//! Port: the state of the server the app runs.

use crate::core::domain::server_state_enum::ServerStateEnum;
use async_trait::async_trait;

/// Tells where the app's server is.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ServerStateService: Send + Sync {
    /// The state now.
    async fn current(&self) -> ServerStateEnum;
}

#[cfg(test)]
#[path = "server_state_service_test.rs"]
mod server_state_service_test;
