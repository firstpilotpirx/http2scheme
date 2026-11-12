use std::sync::Arc;

use super::{AppOrderService, MockAppOrderService};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockAppOrderService::new();
    mock.expect_bring_back_left_app()
        .times(1)
        .returning(|| Ok(()));
    let port: Arc<dyn AppOrderService> = Arc::new(mock);
    assert!(port.bring_back_left_app().await.is_ok());
}
