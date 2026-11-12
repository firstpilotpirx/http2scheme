use std::path::PathBuf;

use super::LaunchdAgentSupervisorGateway;
use crate::core::application::exceptions::agent_control_error::AgentControlError;
use crate::core::application::services::agent_supervisor_service::AgentSupervisorService;

#[tokio::test]
async fn a_missing_program_is_a_supervisor_error() {
    let gateway = LaunchdAgentSupervisorGateway::new(
        PathBuf::from("/nonexistent/launchctl"),
        501,
        PathBuf::from("/nonexistent/agents"),
    );
    let error = gateway.unload("io.example.agent").await.unwrap_err();
    assert!(
        matches!(error, AgentControlError::Supervisor { .. }),
        "{error}"
    );
}
