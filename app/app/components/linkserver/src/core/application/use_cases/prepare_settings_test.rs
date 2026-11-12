use std::sync::Arc;

use crate::core::application::exceptions::settings_error::SettingsError;

use super::{MockPrepareSettings, PrepareSettings};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockPrepareSettings::new();
    mock.expect_execute().times(1).returning(|| {
        Err(SettingsError::Io {
            path: "/k".to_owned(),
            reason: "denied".to_owned(),
        })
    });
    let port: Arc<dyn PrepareSettings> = Arc::new(mock);
    assert!(port.execute().await.is_err());
}
