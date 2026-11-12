use std::sync::Arc;

use crate::core::domain::port_value_object::PortValueObject;

use super::{MockServerHealthService, ServerHealthService};

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockServerHealthService::new();
    mock.expect_is_answering()
        .returning(|port| port.value() == 7777);
    let port: Arc<dyn ServerHealthService> = Arc::new(mock);
    assert!(port.is_answering(PortValueObject::new(7777).unwrap()).await);
    assert!(!port.is_answering(PortValueObject::new(7778).unwrap()).await);
}
