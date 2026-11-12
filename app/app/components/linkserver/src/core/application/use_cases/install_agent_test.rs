use std::sync::Arc;

use super::{InstallAgent, MockInstallAgent};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockInstallAgent::new();
    mock.expect_execute().times(1).returning(|| Ok(()));
    let port: Arc<dyn InstallAgent> = Arc::new(mock);
    assert!(port.execute().await.is_ok());
}
