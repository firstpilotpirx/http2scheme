use std::sync::Arc;

use super::UninstallAgentUseCase;
use crate::core::application::repositories::agent_definition_repository::MockAgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::MockAgentSupervisorService;
use crate::core::application::use_cases::agent_fixtures_test::LABEL;
use crate::core::application::use_cases::uninstall_agent::UninstallAgent;

#[tokio::test]
async fn unloads_and_deletes_an_installed_agent() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository
        .expect_exists()
        .withf(|l| l == LABEL)
        .returning(|_| Ok(true));
    repository
        .expect_delete()
        .withf(|l| l == LABEL)
        .times(1)
        .returning(|_| Ok(()));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(true));
    supervisor
        .expect_unload()
        .withf(|l| l == LABEL)
        .times(1)
        .returning(|_| Ok(()));
    let use_case =
        UninstallAgentUseCase::new(LABEL.to_owned(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Ok(()));
}

#[tokio::test]
async fn is_harmless_when_nothing_is_installed() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_exists().returning(|_| Ok(false));
    repository.expect_delete().never();
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Ok(false));
    supervisor.expect_unload().never();
    let use_case =
        UninstallAgentUseCase::new(LABEL.to_owned(), Arc::new(repository), Arc::new(supervisor));
    assert_eq!(use_case.execute().await, Ok(()));
}
