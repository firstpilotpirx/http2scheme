use std::sync::Arc;

use crate::core::domain::port_value_object::PortValueObject;

use super::{BrowserTabCloserService, MockBrowserTabCloserService};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockBrowserTabCloserService::new();
    mock.expect_close_open_tabs()
        .withf(|port| port.value() == 7777)
        .times(1)
        .returning(|_| Ok(()));
    let port: Arc<dyn BrowserTabCloserService> = Arc::new(mock);
    assert!(
        port.close_open_tabs(PortValueObject::new(7777).unwrap())
            .await
            .is_ok()
    );
}
