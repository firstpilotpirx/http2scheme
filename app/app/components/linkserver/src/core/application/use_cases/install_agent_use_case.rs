//! Installs the background agent and starts it.

use std::sync::Arc;

use async_trait::async_trait;

use super::install_agent::InstallAgent;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::AgentSupervisorService;
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

/// Unloads an old agent, writes the definition and loads it.
pub struct InstallAgentUseCase {
    definition: AgentDefinitionValueObject,
    repository: Arc<dyn AgentDefinitionRepository>,
    supervisor: Arc<dyn AgentSupervisorService>,
}

impl InstallAgentUseCase {
    /// A use case installing `definition`.
    #[must_use]
    pub fn new(
        definition: AgentDefinitionValueObject,
        repository: Arc<dyn AgentDefinitionRepository>,
        supervisor: Arc<dyn AgentSupervisorService>,
    ) -> Self {
        Self {
            definition,
            repository,
            supervisor,
        }
    }
}

#[async_trait]
impl InstallAgent for InstallAgentUseCase {
    async fn execute(&self) -> Result<(), AgentControlError> {
        let label = self.definition.label();
        if self.supervisor.is_loaded(label).await? {
            self.supervisor.unload(label).await?;
        }
        self.repository.save(&self.definition).await?;
        self.supervisor.load(&self.definition).await
    }
}

#[cfg(test)]
#[path = "install_agent_use_case_test.rs"]
mod install_agent_use_case_test;
