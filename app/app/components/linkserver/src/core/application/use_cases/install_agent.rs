//! Input port: install the background agent and start it.

use async_trait::async_trait;

use crate::core::application::exceptions::agent_control_error::AgentControlError;

/// install the background agent and start it.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait InstallAgent: Send + Sync {
    /// Runs the use case.
    async fn execute(&self) -> Result<(), AgentControlError>;
}

#[cfg(test)]
#[path = "install_agent_test.rs"]
mod install_agent_test;
