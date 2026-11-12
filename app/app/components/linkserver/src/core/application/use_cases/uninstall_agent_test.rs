use std::sync::Arc;

use super::{MockUninstallAgent, UninstallAgent};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockUninstallAgent::new();
    mock.expect_execute().returning(|| Ok(false));
    let port: Arc<dyn UninstallAgent> = Arc::new(mock);
    assert!(!port.execute().await.unwrap());
}
