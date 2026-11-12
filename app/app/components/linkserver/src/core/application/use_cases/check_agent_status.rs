//! Input port: report whether the agent is installed, loaded and answering.

use async_trait::async_trait;

use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::domain::agent_status_value_object::AgentStatusValueObject;

/// report whether the agent is installed, loaded and answering.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CheckAgentStatus: Send + Sync {
    /// Runs the use case.
    async fn execute(&self) -> Result<AgentStatusValueObject, AgentControlError>;
}
