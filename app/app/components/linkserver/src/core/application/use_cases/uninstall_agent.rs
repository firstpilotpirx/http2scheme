//! Input port: stop and remove the background agent.

use async_trait::async_trait;

use crate::core::application::exceptions::agent_control_error::AgentControlError;

/// stop and remove the background agent.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UninstallAgent: Send + Sync {
    /// Runs the use case; `true` when an agent definition was removed.
    async fn execute(&self) -> Result<bool, AgentControlError>;
}
