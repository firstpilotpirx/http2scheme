use std::sync::Arc;

use super::{MockSettingsRepository, SettingsRepository};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockSettingsRepository::new();
    mock.expect_read_settings().returning(|| Ok(None));
    let port: Arc<dyn SettingsRepository> = Arc::new(mock);
    assert!(port.read_settings().await.unwrap().is_none());
}
