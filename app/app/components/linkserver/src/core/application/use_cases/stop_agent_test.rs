use std::sync::Arc;

use super::{MockStopAgent, StopAgent};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockStopAgent::new();
    mock.expect_execute().times(1).returning(|| Ok(()));
    let port: Arc<dyn StopAgent> = Arc::new(mock);
    assert!(port.execute().await.is_ok());
}
