use std::sync::Arc;

use super::{AgentDefinitionRepository, MockAgentDefinitionRepository};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockAgentDefinitionRepository::new();
    mock.expect_exists().returning(|label| Ok(label == "a"));
    let port: Arc<dyn AgentDefinitionRepository> = Arc::new(mock);
    assert!(port.exists("a").await.unwrap());
    assert!(!port.exists("b").await.unwrap());
}
