use std::sync::Arc;

use super::{MockOperateMenu, OperateMenu};
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::menu_command_enum::MenuCommandEnum;
use crate::core::domain::menu_view_value_object::MenuViewValueObject;
use crate::core::domain::server_state_enum::ServerStateEnum;

#[tokio::test]
async fn the_port_is_object_safe_and_forwards_its_calls() {
    let view =
        MenuViewValueObject::new("0.1.0", &ServerStateEnum::Starting, LoginItemStateEnum::Off);
    let answer = view.clone();
    let mut mock = MockOperateMenu::new();
    mock.expect_execute().returning(move |_| Ok(answer.clone()));
    let port: Arc<dyn OperateMenu> = Arc::new(mock);
    assert_eq!(port.execute(MenuCommandEnum::Show).await, Ok(view));
}
