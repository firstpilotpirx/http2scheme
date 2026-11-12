use std::sync::Arc;

use super::{AgentSupervisorService, MockAgentSupervisorService};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockAgentSupervisorService::new();
    mock.expect_is_loaded().returning(|_| Ok(true));
    let port: Arc<dyn AgentSupervisorService> = Arc::new(mock);
    assert!(port.is_loaded("a").await.unwrap());
}
