//! Stops and removes the background agent.

use std::sync::Arc;

use async_trait::async_trait;

use super::uninstall_agent::UninstallAgent;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::AgentSupervisorService;

/// Unloads the agent when loaded and deletes its definition when present.
pub struct UninstallAgentUseCase {
    label: String,
    repository: Arc<dyn AgentDefinitionRepository>,
    supervisor: Arc<dyn AgentSupervisorService>,
}

impl UninstallAgentUseCase {
    /// A use case removing the agent with `label`.
    #[must_use]
    pub fn new(
        label: String,
        repository: Arc<dyn AgentDefinitionRepository>,
        supervisor: Arc<dyn AgentSupervisorService>,
    ) -> Self {
        Self {
            label,
            repository,
            supervisor,
        }
    }
}

#[async_trait]
impl UninstallAgent for UninstallAgentUseCase {
    async fn execute(&self) -> Result<(), AgentControlError> {
        if self.supervisor.is_loaded(&self.label).await? {
            self.supervisor.unload(&self.label).await?;
        }
        if self.repository.exists(&self.label).await? {
            self.repository.delete(&self.label).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "uninstall_agent_use_case_test.rs"]
mod uninstall_agent_use_case_test;
