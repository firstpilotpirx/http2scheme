//! Starts the installed background agent.

use std::sync::Arc;

use async_trait::async_trait;

use super::start_agent::StartAgent;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::AgentSupervisorService;
use crate::core::domain::agent_definition_value_object::AgentDefinitionValueObject;

/// Loads the agent; refuses when it is not installed.
pub struct StartAgentUseCase {
    definition: AgentDefinitionValueObject,
    repository: Arc<dyn AgentDefinitionRepository>,
    supervisor: Arc<dyn AgentSupervisorService>,
}

impl StartAgentUseCase {
    /// A use case starting `definition`.
    #[must_use]
    pub fn new(
        definition: AgentDefinitionValueObject,
        repository: Arc<dyn AgentDefinitionRepository>,
        supervisor: Arc<dyn AgentSupervisorService>,
    ) -> Self {
        Self { definition, repository, supervisor }
    }
}

#[async_trait]
impl StartAgent for StartAgentUseCase {
    async fn execute(&self) -> Result<(), AgentControlError> {
        let label = self.definition.label();
        if !self.repository.exists(label).await? {
            return Err(AgentControlError::NotInstalled);
        }
        if self.supervisor.is_loaded(label).await? {
            return Ok(());
        }
        self.supervisor.load(&self.definition).await
    }
}

#[cfg(test)]
#[path = "start_agent_use_case_test.rs"]
mod start_agent_use_case_test;
