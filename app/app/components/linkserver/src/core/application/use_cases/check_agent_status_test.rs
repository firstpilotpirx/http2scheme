use std::sync::Arc;

use crate::core::domain::agent_status_value_object::AgentStatusValueObject;

use super::{CheckAgentStatus, MockCheckAgentStatus};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockCheckAgentStatus::new();
    mock.expect_execute()
        .returning(|| Ok(AgentStatusValueObject::new(true, true, false)));
    let port: Arc<dyn CheckAgentStatus> = Arc::new(mock);
    assert!(!port.execute().await.unwrap().healthy());
}
