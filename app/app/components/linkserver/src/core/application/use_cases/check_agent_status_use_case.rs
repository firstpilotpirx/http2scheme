//! Reports the three facts of the background agent.

use std::sync::Arc;

use async_trait::async_trait;

use super::check_agent_status::CheckAgentStatus;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::AgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::AgentSupervisorService;
use crate::core::application::services::server_health_service::ServerHealthService;
use crate::core::domain::agent_status_value_object::AgentStatusValueObject;
use crate::core::domain::port_value_object::PortValueObject;

/// Installed, loaded, and answering on the port.
pub struct CheckAgentStatusUseCase {
    label: String,
    port: PortValueObject,
    repository: Arc<dyn AgentDefinitionRepository>,
    supervisor: Arc<dyn AgentSupervisorService>,
    health: Arc<dyn ServerHealthService>,
}

impl CheckAgentStatusUseCase {
    /// A use case checking the agent `label` and the server on `port`.
    #[must_use]
    pub fn new(
        label: String,
        port: PortValueObject,
        repository: Arc<dyn AgentDefinitionRepository>,
        supervisor: Arc<dyn AgentSupervisorService>,
        health: Arc<dyn ServerHealthService>,
    ) -> Self {
        Self { label, port, repository, supervisor, health }
    }
}

#[async_trait]
impl CheckAgentStatus for CheckAgentStatusUseCase {
    async fn execute(&self) -> Result<AgentStatusValueObject, AgentControlError> {
        let installed = self.repository.exists(&self.label).await?;
        let loaded = self.supervisor.is_loaded(&self.label).await?;
        let answering = self.health.is_answering(self.port).await;
        Ok(AgentStatusValueObject::new(installed, loaded, answering))
    }
}

#[cfg(test)]
#[path = "check_agent_status_use_case_test.rs"]
mod check_agent_status_use_case_test;
