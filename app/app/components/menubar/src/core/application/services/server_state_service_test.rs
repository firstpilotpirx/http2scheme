use std::sync::Arc;

use super::{MockServerStateService, ServerStateService};
use crate::core::domain::server_state_enum::ServerStateEnum;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockServerStateService::new();
    mock.expect_current()
        .return_const(ServerStateEnum::Starting);
    let port: Arc<dyn ServerStateService> = Arc::new(mock);
    assert_eq!(port.current().await, ServerStateEnum::Starting);
}
