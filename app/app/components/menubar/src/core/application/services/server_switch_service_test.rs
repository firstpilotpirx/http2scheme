use std::sync::Arc;

use super::{MockServerSwitchService, ServerSwitchService};
use crate::core::domain::server_state_enum::ServerStateEnum;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockServerSwitchService::new();
    mock.expect_turn_off().times(1).return_const(());
    mock.expect_turn_on().times(1).return_const(());
    mock.expect_current().return_const(ServerStateEnum::Off);
    let port: Arc<dyn ServerSwitchService> = Arc::new(mock);
    port.turn_off().await;
    port.turn_on().await;
    assert_eq!(port.current().await, ServerStateEnum::Off);
}
