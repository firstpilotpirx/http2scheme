use std::sync::Arc;

use mockall::Sequence;

use super::InstallAgentUseCase;
use crate::core::application::use_cases::agent_fixtures_test::{LABEL, definition};
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::MockAgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::MockAgentSupervisorService;
use crate::core::application::use_cases::install_agent::InstallAgent;

#[tokio::test]
async fn unloads_an_old_agent_writes_the_definition_and_loads_it() {
    let mut sequence = Sequence::new();
    let mut repository = MockAgentDefinitionRepository::new();
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().withf(|l| l == LABEL).times(1).in_sequence(&mut sequence).returning(|_| Ok(true));
    supervisor.expect_unload().withf(|l| l == LABEL).times(1).in_sequence(&mut sequence).returning(|_| Ok(()));
    repository.expect_save().withf(|d| *d == definition()).times(1).in_sequence(&mut sequence).returning(|_| Ok(()));
    supervisor.expect_load().withf(|d| *d == definition()).times(1).in_sequence(&mut sequence).returning(|_| Ok(()));
    let use_case = InstallAgentUseCase::new(definition(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Ok(()));
}

#[tokio::test]
async fn a_first_install_unloads_nothing() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_save().returning(|_| Ok(()));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(false));
    supervisor.expect_unload().never();
    supervisor.expect_load().times(1).returning(|_| Ok(()));
    let use_case = InstallAgentUseCase::new(definition(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Ok(()));
}

#[tokio::test]
async fn a_failed_load_is_reported() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_save().returning(|_| Ok(()));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(false));
    supervisor.expect_load().returning(|_| Err(AgentControlError::Supervisor { reason: "exit 5".to_owned() }));
    let use_case = InstallAgentUseCase::new(definition(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Err(AgentControlError::Supervisor { reason: "exit 5".to_owned() }));
}
