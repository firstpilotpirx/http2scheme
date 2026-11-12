use std::sync::Arc;

use crate::core::application::exceptions::agent_control_error::AgentControlError;

use super::{MockStartAgent, StartAgent};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockStartAgent::new();
    mock.expect_execute()
        .returning(|| Err(AgentControlError::NotInstalled));
    let port: Arc<dyn StartAgent> = Arc::new(mock);
    assert!(matches!(
        port.execute().await,
        Err(AgentControlError::NotInstalled)
    ));
}
