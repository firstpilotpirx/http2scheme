//! Port: the system service that runs the background agent.

use async_trait::async_trait;

use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

/// Loads, unloads and finds the agent.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AgentSupervisorService: Send + Sync {
    /// Loads (starts) the agent.
    async fn load(&self, definition: &AgentDefinitionValueObject) -> Result<(), AgentControlError>;
    /// Unloads (stops) the agent with this label.
    async fn unload(&self, label: &str) -> Result<(), AgentControlError>;
    /// Whether the agent with this label is loaded.
    async fn is_loaded(&self, label: &str) -> Result<bool, AgentControlError>;
}

#[cfg(test)]
#[path = "agent_supervisor_service_test.rs"]
mod agent_supervisor_service_test;
