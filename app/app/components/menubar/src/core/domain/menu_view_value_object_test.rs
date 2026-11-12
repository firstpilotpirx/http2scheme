use super::MenuViewValueObject;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::server_state_enum::ServerStateEnum;

#[test]
fn the_view_holds_the_title_the_status_and_the_login_item() {
    let server = ServerStateEnum::Running("http://127.0.0.1:7777".to_owned());
    let view = MenuViewValueObject::new("0.1.0", &server, LoginItemStateEnum::On);
    assert_eq!(view.title(), "http2scheme 0.1.0");
    assert_eq!(view.status_line(), "Running on http://127.0.0.1:7777");
    assert_eq!(view.login_title(), "Start at Login");
    assert!(view.login_checked());
    assert!(view.login_enabled());
}

#[test]
fn a_program_outside_a_bundle_disables_the_login_item() {
    let view = MenuViewValueObject::new(
        "0.1.0",
        &ServerStateEnum::Starting,
        LoginItemStateEnum::Unavailable,
    );
    assert!(!view.login_checked());
    assert!(!view.login_enabled());
}
