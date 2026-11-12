use std::sync::Arc;

use super::StartAgentUseCase;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::MockAgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::MockAgentSupervisorService;
use crate::core::application::use_cases::agent_fixtures_test::{LABEL, definition};
use crate::core::application::use_cases::start_agent::StartAgent;

#[tokio::test]
async fn loads_an_installed_agent_that_is_not_loaded() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository
        .expect_exists()
        .withf(|l| l == LABEL)
        .returning(|_| Ok(true));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(false));
    supervisor
        .expect_load()
        .withf(|d| *d == definition())
        .times(1)
        .returning(|_| Ok(()));
    let use_case = StartAgentUseCase::new(definition(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Ok(()));
}

#[tokio::test]
async fn an_already_loaded_agent_is_left_alone() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_exists().returning(|_| Ok(true));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(true));
    supervisor.expect_load().never();
    let use_case = StartAgentUseCase::new(definition(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Ok(()));
}

#[tokio::test]
async fn refuses_when_not_installed() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_exists().returning(|_| Ok(false));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_load().never();
    let use_case = StartAgentUseCase::new(definition(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(
        use_case.execute().await,
        Err(AgentControlError::NotInstalled)
    );
}
