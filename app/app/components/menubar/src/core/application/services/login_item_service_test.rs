use std::sync::Arc;

use super::{LoginItemService, MockLoginItemService};
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let mut mock = MockLoginItemService::new();
    mock.expect_status().return_const(LoginItemStateEnum::Off);
    mock.expect_register().returning(|| Ok(()));
    let port: Arc<dyn LoginItemService> = Arc::new(mock);
    assert_eq!(port.status().await, LoginItemStateEnum::Off);
    assert_eq!(port.register().await, Ok(()));
}
