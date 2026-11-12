//! Stops the background agent.

use std::sync::Arc;

use async_trait::async_trait;

use super::stop_agent::StopAgent;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::AgentSupervisorService;

/// Unloads the agent; refuses when it is not installed.
pub struct StopAgentUseCase {
    label: String,
    repository: Arc<dyn AgentDefinitionRepository>,
    supervisor: Arc<dyn AgentSupervisorService>,
}

impl StopAgentUseCase {
    /// A use case stopping the agent with `label`.
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
impl StopAgent for StopAgentUseCase {
    async fn execute(&self) -> Result<(), AgentControlError> {
        if !self.repository.exists(&self.label).await? {
            return Err(AgentControlError::NotInstalled);
        }
        if self.supervisor.is_loaded(&self.label).await? {
            self.supervisor.unload(&self.label).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "stop_agent_use_case_test.rs"]
mod stop_agent_use_case_test;
