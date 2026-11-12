//! Input port: stop the background agent.

use async_trait::async_trait;

use crate::core::application::exceptions::agent_control_error::AgentControlError;

/// stop the background agent.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait StopAgent: Send + Sync {
    /// Runs the use case.
    async fn execute(&self) -> Result<(), AgentControlError>;
}
