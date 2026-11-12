//! Port: the background agent's definition file.

use async_trait::async_trait;

use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

/// Writes, finds and deletes the agent definition.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AgentDefinitionRepository: Send + Sync {
    /// Whether a definition with this label exists.
    async fn exists(&self, label: &str) -> Result<bool, AgentControlError>;
    /// Writes the definition, replacing an old one.
    async fn save(&self, definition: &AgentDefinitionValueObject) -> Result<(), AgentControlError>;
    /// Deletes the definition with this label.
    async fn delete(&self, label: &str) -> Result<(), AgentControlError>;
}

#[cfg(test)]
#[path = "agent_definition_repository_test.rs"]
mod agent_definition_repository_test;
