//! Input port: start the installed background agent.

use async_trait::async_trait;

use crate::core::application::exceptions::agent_control_error::AgentControlError;

/// start the installed background agent.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait StartAgent: Send + Sync {
    /// Runs the use case.
    async fn execute(&self) -> Result<(), AgentControlError>;
}

#[cfg(test)]
#[path = "start_agent_test.rs"]
mod start_agent_test;
