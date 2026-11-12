use std::sync::Arc;

use super::OperateMenuUseCase;
use crate::core::application::exceptions::menu_action_error::MenuActionError;
use crate::core::application::services::login_item_service::MockLoginItemService;
use crate::core::application::services::server_switch_service::MockServerSwitchService;
use crate::core::application::use_cases::operate_menu::OperateMenu;
use crate::core::domain::login_item_state_enum::LoginItemStateEnum;
use crate::core::domain::menu_command_enum::MenuCommandEnum;
use crate::core::domain::server_state_enum::ServerStateEnum;

fn use_case(login_item: MockLoginItemService) -> OperateMenuUseCase {
    let running = ServerStateEnum::Running("http://127.0.0.1:7777".to_owned());
    use_case_with_server(running, MockServerSwitchService::new(), login_item)
}

fn use_case_with_server(
    state: ServerStateEnum,
    mut switch: MockServerSwitchService,
    login_item: MockLoginItemService,
) -> OperateMenuUseCase {
    switch.expect_current().return_const(state);
    OperateMenuUseCase::new("0.1.0".to_owned(), Arc::new(switch), Arc::new(login_item))
}

fn login_item(state: LoginItemStateEnum) -> MockLoginItemService {
    let mut mock = MockLoginItemService::new();
    mock.expect_status().return_const(state);
    mock
}

#[tokio::test]
async fn show_answers_the_current_view_and_changes_nothing() {
    let view = use_case(login_item(LoginItemStateEnum::On))
        .execute(MenuCommandEnum::Show)
        .await
        .unwrap();
    assert_eq!(view.title(), "http2scheme 0.1.0");
    assert_eq!(view.status_line(), "Running on http://127.0.0.1:7777");
    assert!(view.login_checked());
}

#[tokio::test]
async fn toggle_registers_a_login_item_that_is_off() {
    let mut mock = login_item(LoginItemStateEnum::Off);
    mock.expect_register().times(1).returning(|| Ok(()));
    let result = use_case(mock)
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn toggle_unregisters_a_login_item_that_is_on() {
    let mut mock = login_item(LoginItemStateEnum::On);
    mock.expect_unregister().times(1).returning(|| Ok(()));
    let result = use_case(mock)
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn toggle_opens_system_settings_for_a_login_item_waiting_for_approval() {
    let mut mock = login_item(LoginItemStateEnum::NeedsApproval);
    mock.expect_open_system_settings()
        .times(1)
        .returning(|| Ok(()));
    let result = use_case(mock)
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn toggle_does_nothing_outside_a_bundle() {
    let mock = login_item(LoginItemStateEnum::Unavailable);
    let view = use_case(mock)
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await
        .unwrap();
    assert!(!view.login_enabled());
}

#[tokio::test]
async fn a_refused_toggle_is_an_error() {
    let mut mock = login_item(LoginItemStateEnum::Off);
    mock.expect_register()
        .returning(|| Err(MenuActionError::LoginItem("denied".to_owned())));
    let result = use_case(mock)
        .execute(MenuCommandEnum::ToggleLoginItem)
        .await;
    assert_eq!(result, Err(MenuActionError::LoginItem("denied".to_owned())));
}

#[tokio::test]
async fn toggle_server_turns_a_serving_server_off() {
    let mut switch = MockServerSwitchService::new();
    switch.expect_turn_off().times(1).return_const(());
    switch.expect_turn_on().never();
    let running = ServerStateEnum::Running("http://127.0.0.1:7777".to_owned());
    let result = use_case_with_server(running, switch, login_item(LoginItemStateEnum::Off))
        .execute(MenuCommandEnum::ToggleServer)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn toggle_server_turns_a_server_that_is_off_on() {
    let mut switch = MockServerSwitchService::new();
    switch.expect_turn_on().times(1).return_const(());
    switch.expect_turn_off().never();
    let view = use_case_with_server(
        ServerStateEnum::Off,
        switch,
        login_item(LoginItemStateEnum::Off),
    )
    .execute(MenuCommandEnum::ToggleServer)
    .await
    .unwrap();
    assert_eq!(view.status_line(), "Off");
}
