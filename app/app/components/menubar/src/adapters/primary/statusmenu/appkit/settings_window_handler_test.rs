use super::still_starting;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::menu_view_value_object::MenuViewValueObject;
use crate::core::domain::server_state_enum::ServerStateEnum;

fn view(server: &ServerStateEnum) -> MenuViewValueObject {
    MenuViewValueObject::new("0.1.0", server, LoginItemStateEnum::Off)
}

#[test]
fn only_a_starting_server_is_read_again() {
    assert!(still_starting(Some(&view(&ServerStateEnum::Starting))));
    let running = ServerStateEnum::Running("http://127.0.0.1:7777".to_owned());
    assert!(!still_starting(Some(&view(&running))));
    assert!(!still_starting(Some(&view(&ServerStateEnum::Off))));
    assert!(!still_starting(None));
}
