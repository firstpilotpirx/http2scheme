use std::sync::Arc;

use super::StopAgentUseCase;
use crate::core::application::use_cases::agent_fixtures_test::LABEL;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::MockAgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::MockAgentSupervisorService;
use crate::core::application::use_cases::stop_agent::StopAgent;

#[tokio::test]
async fn unloads_a_loaded_agent() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_exists().returning(|_| Ok(true));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(true));
    supervisor.expect_unload().withf(|l| l == LABEL).times(1).returning(|_| Ok(()));
    let use_case = StopAgentUseCase::new(LABEL.to_owned(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Ok(()));
}

#[tokio::test]
async fn a_stopped_agent_stays_stopped() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_exists().returning(|_| Ok(true));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(false));
    supervisor.expect_unload().never();
    let use_case = StopAgentUseCase::new(LABEL.to_owned(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Ok(()));
}

#[tokio::test]
async fn refuses_when_not_installed() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_exists().returning(|_| Ok(false));
    let use_case = StopAgentUseCase::new(LABEL.to_owned(), Arc::new(repository), Arc::new(MockAgentSupervisorService::new()));
    assert_eq!(use_case.execute().await, Err(AgentControlError::NotInstalled));
}
