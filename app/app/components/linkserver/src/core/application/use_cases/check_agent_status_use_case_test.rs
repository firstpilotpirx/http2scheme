use std::sync::Arc;

use super::CheckAgentStatusUseCase;
use crate::core::application::use_cases::agent_fixtures_test::LABEL;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::repositories::agent_definition_repository::MockAgentDefinitionRepository;
use crate::core::application::services::agent_supervisor_service::MockAgentSupervisorService;
use crate::core::application::services::server_health_service::MockServerHealthService;
use crate::core::application::use_cases::check_agent_status::CheckAgentStatus;
use crate::core::domain::agent_status_value_object::AgentStatusValueObject;
use crate::core::domain::port_value_object::PortValueObject;

fn use_case(installed: bool, loaded: bool, answering: bool) -> CheckAgentStatusUseCase {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_exists().withf(|l| l == LABEL).returning(move |_| Ok(installed));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(move |_| Ok(loaded));
    let mut health = MockServerHealthService::new();
    health.expect_is_answering().withf(|p| p.value() == 7777).returning(move |_| answering);
    CheckAgentStatusUseCase::new(LABEL.to_owned(), PortValueObject::default(), Arc::new(repository), Arc::new(supervisor), Arc::new(health))
}

#[tokio::test]
async fn reports_the_three_facts() {
    assert_eq!(use_case(true, true, true).execute().await, Ok(AgentStatusValueObject::new(true, true, true)));
    assert_eq!(use_case(true, true, false).execute().await, Ok(AgentStatusValueObject::new(true, true, false)));
    assert_eq!(use_case(false, false, false).execute().await, Ok(AgentStatusValueObject::new(false, false, false)));
}

#[tokio::test]
async fn passes_supervisor_errors_on() {
    let mut repository = MockAgentDefinitionRepository::new();
    repository.expect_exists().returning(|_| Ok(true));
    let mut supervisor = MockAgentSupervisorService::new();
    supervisor.expect_is_loaded().returning(|_| Err(AgentControlError::Supervisor { reason: "x".to_owned() }));
    let use_case = CheckAgentStatusUseCase::new(
        LABEL.to_owned(),
        PortValueObject::default(),
        Arc::new(repository),
        Arc::new(supervisor),
        Arc::new(MockServerHealthService::new()),
    );
    assert!(use_case.execute().await.is_err());
}
